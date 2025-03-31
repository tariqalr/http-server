use std::{
	net::{TcpListener, TcpStream},
	io::{prelude::*, BufReader},
	fs,
};
use http_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3708").unwrap();
	let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
    }
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader = BufReader::new(&stream);
	let http_request: Vec<_> = buf_reader
		.lines()
		.map(|result| result.unwrap())
		.take_while(|line| !line.is_empty())
		.collect();

	println!("Request: {http_request:#?}");


	let (status, file) = match &http_request.first().unwrap()[..] {
		"GET / HTTP/1.1" =>
			(
				"HTTP/1.1 200 OK",
				"src/htdocs/index.html"
			),
		_ => 
			(
				"HTTP/1.1 404 Not Found",
				""
			), 
	};

	let response = if !file.is_empty() {
		let contents = fs::read_to_string(file).unwrap();
		&format!("{status}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{contents}",contents.len())
	} else {
		status
	};

	stream.write_all(response.as_bytes()).unwrap();
}