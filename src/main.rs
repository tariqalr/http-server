use std::{
	net::{TcpListener, TcpStream},
	io::{prelude::*, BufReader},
	fs,
};
use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3708").expect("Could not bind to port");
	let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let Ok(stream)=stream else {continue;};

		pool.execute(|| {
			handle_connection(stream);
		});
    }
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


	let (status, file) = match http_request.first().map(|s| s.as_str()).unwrap_or("") {
		"GET / HTTP/1.1" => ("HTTP/1.1 200 OK","htdocs/index.html"),
		"GET /cat.html HTTP/1.1" => ("HTTP/1.1 200 OK","htdocs/cat.html"),
		_ => ("HTTP/1.1 404 Not Found",""), 
	};

	let response = if !file.is_empty() {
		match fs::read_to_string(file) {
            Ok(contents) => &format!(
                "{status}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{contents}",
                contents.len()
            )[..],
            Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n",
        }
	} else {
		status
	};

	stream.write_all(response.as_bytes()).expect("Could not send response");
}