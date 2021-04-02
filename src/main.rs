#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs;

use std::io::prelude::*;

use std::net::TcpListener;
use std::net::TcpStream;

use regex::Regex;

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

	//let get = b"GET / HTTP/1.1\r\n";

	let raw = String::from_utf8_lossy(&buffer[..]);
	let request = raw.as_ref();

	println!("Request: {}", request);

	let url = get_url(request);

	match url {
		Some(page) => {
			match page.as_ref() {
				"/" => fetch_item(stream, "index.html"),
				"/astro.html" => fetch_item(stream, "astro.html"),
				"/css/style.css" => fetch_item(stream, "css/style.css"),
				_ => error_page(stream)
			}
		},
		None => error_page(stream)
	}

}

fn fetch_item(mut stream: TcpStream, page: &str) {
	let contents = fs::read_to_string(page).unwrap();

	let response = format!(
		"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
		contents.len(),
		contents
	);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn error_page(mut stream: TcpStream) {
	let contents = fs::read_to_string("404.html").unwrap();
	
	let response = format!(
		"HTTP/1.1 404 NOT FOUND \r\n\r\n{}", 
		contents
	);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn get_url(request: &str) -> Option<String> {
	/*lazy_static! {
		static ref URL_GRABBER: Regex = Regex::new("^GET ([A-Za-z0-9\\-\\._~:\\?#\\[\\]@!\\$\\&'\\(\\)\\*\\+,;%=/]+) HTTP/1.1\r\n").unwrap();
	}

	Some(URL_GRABBER.captures(request)?[1].to_string())*/

	let split_by_whitespace = Some(request.split_whitespace());
	
	split_by_whitespace.map(|mut list| list.nth(1).map(|url| url.to_string())).flatten()
}