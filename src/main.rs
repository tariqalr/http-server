use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3708").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

		handle_connection(stream);
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

	if http_request.first().unwrap() == "GET / HTTP/1.1" {
		let contents = fs::read_to_string("src/htdocs/index.html").unwrap();
		let response = format!("HTTP/1.1 200 OK\r\n\r\nContent-Length: {}\r\n\r\n{contents}",contents.len());
		stream.write_all(response.as_bytes()).unwrap();
	} else {
		let response = "HTTP/1.1 404 Not Found";
		stream.write_all(response.as_bytes()).unwrap();
	}

}