use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {

	let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());

	let addr = format!("127.0.0.1:{}", port);

	let listener = TcpListener::bind(addr).unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	
	stream.read(&mut buffer).unwrap();
	println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}