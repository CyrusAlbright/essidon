use std::env;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

	let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());

	let addr = format!("0.0.0.0:{}", port);

	let listener = TcpListener::bind(addr).unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	let get = b"GET / HTTP/1.1\r\n";

	println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

	if buffer.starts_with(get) {
		let contents = fs::read_to_string("main.html").unwrap();

		let response = format!(
			"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
			contents.len(),
			contents
		);

		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	} else {
		let status_line = "HTTP/1.1 404 NOT FOUND \r\n\r\n";
		let contents = fs::read_to_string("404.html").unwrap();

		let response = format!("{}{}", status_line, contents);

		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	}
	
}