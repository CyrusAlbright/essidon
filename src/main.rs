//mod database;

//use std::env;
//use std::fs;

//use std::io::prelude::*;

//use std::sync::Mutex;
//use std::sync::Arc;

use std::path::PathBuf;

//use database::Database;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpRequest, HttpServer, Responder};

use config::{Config, Environment};

#[get("/")]
async fn index(_req: HttpRequest) -> Result<NamedFile, Error> {
	let path: PathBuf = "./srv/index.html".parse::<PathBuf>().unwrap();
	Ok(NamedFile::open(path)?)
}

#[get("/style.css")]
async fn style(_req: HttpRequest) -> Result<NamedFile, Error> {
	let path: PathBuf = "./srv/style.css".parse::<PathBuf>().unwrap();
	Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let mut settings = config::Config::default();
	settings.merge(config::Environment::default()).unwrap();

	let port = settings.get_int("PORT").unwrap();
	let addr = format!("0.0.0.0:{}", port);

	HttpServer::new(|| {
		App::new().service(index).service(style)
	}).bind(addr)?.run().await
	/*
	let database = Arc::new(Mutex::new(database::Database::new().expect("Database init failed")));
	let database_mutex_clone = Arc::clone(&database);
	pool.execute(|| handle_connection(database_mutex_clone, stream));
	}
	*/
}

/*
			"/" | "/index.html" => fetch_and_send(stream, "HTTP/1.1 200 OK", "html/index.html"),
			"/about.html" => fetch_and_send(stream, "HTTP/1.1 200 OK", "html/about.html"),
			"/style.css" => fetch_and_send(stream, "HTTP/1.1 200 OK", "css/style.css"),
			"/users" => {
				let rows = db.lock().unwrap().fetch().expect("Failed to fetch rows");
				let data = rows.iter().map(|row: &postgres::Row| {
					let id: i32 = row.get(0);
					let username: &str = row.get(1);
					let email: &str = row.get(2);

					format!(
						"{{\r\n\"id\" : \"{}\",\r\n\"username\" : \"{}\",\r\n\"email\" : \"{}\"\r\n}}",
						id,
						username,
						email
					)
				}).collect::<Vec<String>>().join(",\r\n");

				let response = format!(
					"{}\r\n{}\r\n\r\n{{\r\n\"rows\" : [{}]\r\n}}",
					"HTTP/1.1 200 OK",
					"Content-Type: application/json; charset=UTF-8",
					data
				);

				send(stream, response.as_ref());
			},
			_ => fetch_and_send(stream, "HTTP/1.1 404 NOT FOUND", "html/404.html")
}*/