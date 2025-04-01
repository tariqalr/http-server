use std::{
	fs, 
	io::{prelude::*, BufReader}, 
	net::{TcpListener, TcpStream}, 
	path::PathBuf,
};
use http_server::ThreadPool;

fn main() {
	const IP: &str = "0.0.0.0";
	const PORT: &str = "3708";
    let listener = TcpListener::bind(format!("{IP}:{PORT}")).expect("Could not bind to port");
	
	let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let Ok(stream)=stream else {continue;};

		pool.execute(|| {
			handle_connection(stream);
		});
    }
}

enum Response {
	Text(String),
	Binary(Vec<u8>),
	Status,
	InternalError,
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&stream);
	let http_request: Vec<_> = buf_reader
		.lines()
		.map_while(|result| match result {
			Ok(line) if line.is_empty() => None,
			Ok(line) => Some(line),
			Err(_) => None,
		})
		.collect();

	if http_request.is_empty() {
		println!("Connection closed early");
		return;
	}

	println!("Request: {http_request:#?}\n");

	let mut filepath = PathBuf::new();

	let mut http_request = http_request.first().map(|s| s.as_str()).unwrap_or("").split(" ");
	let (status, content_type) = match http_request.next().unwrap_or("") {
		"GET" => {
			match http_request.next().unwrap_or("").split_once(".") {
				None => {
					filepath.push("htdocs/index.html");
					("HTTP/1.1 200 OK","text/html")
				},
				Some((prefix,"jpg")) => {
					filepath.push(format!("{}.jpg",&prefix[1..])); //big security issue: user can input absolute path/'.' char encoding to traverse filesystem, or can use null terminator to modify target file extension
					("HTTP/1.1 200 OK","image/jpeg")
				},
				Some((prefix,"html")) => {
					filepath.push(format!("htdocs{}.html",prefix)); //less of a security issue
					("HTTP/1.1 200 OK","text/html")
				},
				_ => ("HTTP/1.1 404 Not Found",""), 
			}
		}, 
		_ => ("HTTP/1.1 404 Not Found",""), 
	};


	let response = match content_type {
		"text/html" => {
			match fs::read_to_string(filepath) {
				Ok(contents) => Response::Text(format!("{status}\r\nContent-Length: {}\r\nContent-Type: {content_type}; charset=utf-8\r\n\r\n{contents}",contents.len())),
				Err(_) => Response::InternalError
			}
		},
		"image/jpeg" => {
			match fs::read(filepath) {
				Ok(contents) => Response::Binary([format!("{status}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\n\r\n",contents.len()).into_bytes(), contents].concat()),
				Err(_) => Response::InternalError,
			}
		},
		_ => Response::Status
	};

	match response {
		Response::Text(text) => stream.write_all(text.as_bytes()),
		Response::Binary(data) => stream.write_all(&data),
		Response::Status => stream.write_all(status.as_bytes()),
		Response::InternalError => stream.write_all("HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes()),
	}.expect("Could not send response");
}