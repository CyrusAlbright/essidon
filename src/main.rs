//#[macro_use] extern crate lazy_static;
//extern crate regex;

mod worker_pool;
mod database;

use std::env;
use std::fs;

use std::io::prelude::*;

use std::net::TcpListener;
use std::net::TcpStream;

use worker_pool::WorkerPool;
use database::Database;

//use regex::Regex;

fn main() {

	let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());

	let addr = format!("0.0.0.0:{}", port);

	let listener = TcpListener::bind(addr).unwrap();

	let pool = WorkerPool::new(4);

	let database = Database::new().expect("Database init failed");

	for stream in listener.incoming() {
		let stream = stream.unwrap();
	
		pool.execute(|| handle_connection(database, stream));
	}
}

fn handle_connection(db: Database, mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	//let get = b"GET / HTTP/1.1\r\n";

	let raw = String::from_utf8_lossy(&buffer[..]);
	let request = raw.as_ref();

	//println!("Request: {}", request);

	let url = get_url(request);

	let error_page = ("HTTP/1.1 404 NOT FOUND", "html/404.html");

	match url {
		Some(page) => match page.as_ref() {
			"/" | "/index.html" => fetch_and_send(stream, "HTTP/1.1 200 OK", "html/index.html"),
			"/about.html" => fetch_and_send(stream, "HTTP/1.1 200 OK", "html/about.html"),
			"/style.css" => fetch_and_send(stream, "HTTP/1.1 200 OK", "css/style.css"),
			"/users" => {
				for row in db.fetch().expect("Failed to fetch rows") {
					let id: i32 = row.get(0);
					let username: &str = row.get(1);
					let email: &str = row.get(2);
					let hash: &str = row.get(3);
					
					println!("Person: {} {} {} {}", id, username, email, hash);
				}
				
				//send(stream, format!("HTTP/1.1 200 OK\r\n\r\n{}", row))
			},
			_ => fetch_and_send(stream, "HTTP/1.1 404 NOT FOUND", "html/404.html")
		},
		None => fetch_and_send(stream, "HTTP/1.1 404 NOT FOUND", "html/404.html")
	}
}

fn fetch_and_send(mut stream: TcpStream, status_line: &str, page: &str) {
	let contents = fs::read_to_string(page).unwrap();

	let response = format!(
		"{}\r\n\r\n{}",
		status_line,
		contents
	);

	send(stream, response.as_ref());	
}

fn send(mut stream: TcpStream, response: &str) {
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn get_url(request: &str) -> Option<&str> {
	/*lazy_static! {
		static ref URL_GRABBER: Regex = Regex::new("^GET ([A-Za-z0-9\\-\\._~:\\?#\\[\\]@!\\$\\&'\\(\\)\\*\\+,;%=/]+) HTTP/1.1\r\n").unwrap();
	}

	Some(URL_GRABBER.captures(request)?[1].to_string())*/

	let split_by_whitespace = Some(request.split_whitespace());

	split_by_whitespace.map(|mut list| list.nth(1).map(|url| url)).flatten()
}