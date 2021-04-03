//#[macro_use] extern crate lazy_static;
//extern crate regex;

mod thread_pool;

use std::env;
use std::fs;

use std::io::prelude::*;

use std::net::TcpListener;
use std::net::TcpStream;

use thread_pool::ThreadPool;

//use regex::Regex;

fn main() {

	let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());

	let addr = format!("0.0.0.0:{}", port);

	let listener = TcpListener::bind(addr).unwrap();

	let pool = ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream = stream.unwrap();
	
		pool.execute(|| handle_connection(stream));
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

	let error_page = ("HTTP/1.1 404 NOT FOUND", "html/404.html");

	let (status_line, page) = match url {
		Some(page) => match page.as_ref() {
			"/" | "/index.html" => ("HTTP/1.1 200 OK", "html/index.html"),
			"/astro.html" => ("HTTP/1.1 200 OK", "html/astro.html"),
			"/style.css" => ("HTTP/1.1 200 OK", "css/style.css"),
			_ => error_page
		},
		None => error_page
	};

	fetch_and_send(stream, status_line, page)
}

fn fetch_and_send(mut stream: TcpStream, status_line: &str, page: &str) {
	let contents = fs::read_to_string(page).unwrap();

	let response = format!(
		"{}\r\n\r\n{}",
		status_line,
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