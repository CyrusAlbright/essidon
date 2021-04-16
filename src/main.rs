//mod database;

//use std::sync::Mutex;
//use std::sync::Arc;

//use database::Database;

use std::path::PathBuf;

use actix_service::Service;
use actix_files::NamedFile;
use actix_http::http::{Uri, PathAndQuery, Method};
use actix_web::{get, post, web, App, Error, HttpResponse, HttpRequest, Result, Responder, HttpServer};
use actix_web::dev::{ServiceRequest, ServiceResponse};

/*#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> impl Responder {
	let path: PathBuf = format!("./srv/{}", match req.match_info().query("filename") {
		"" => "index.html",
		a => a
	}).parse().unwrap();
	NamedFile::open(path)
}

#[get("/style.css")]
async fn style(_req: HttpRequest) -> Result<NamedFile, Error> {
	let path: PathBuf = "./srv/style.css".parse::<PathBuf>().unwrap();
	Ok(NamedFile::open(path)?)
}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let port = std::env::var("PORT").expect("Env var PORT has to be set")
		.parse::<u16>().expect("Env var PORT has to be an integer");
	let addr = format!("0.0.0.0:{}", port);

	HttpServer::new(|| {
		App::new()
			.wrap_fn(|mut req, srv| {
				let head = req.head();
				let path = head.uri.path().to_string();
				let mut path_changed = false;

				let mut new_path = path.trim_end_matches("/").to_owned();
				if req.head().method == Method::GET
					&& !(path.ends_with(".html") 
					|| path.ends_with(".js")
					|| path.ends_with(".css")) {
					if PathBuf::from(format!("./srv{}.html", new_path)).exists() {
						new_path += ".html";
						path_changed = true;
					}
				}

				if path_changed {
					let mut parts = head.uri.clone().into_parts();
					let query = parts.path_and_query.as_ref().and_then(|pq| pq.query());

					let new_path = if let Some(q) = query {
						format!("{}?{}", new_path, q)
					} else {
						new_path
					};
					parts.path_and_query = Some(PathAndQuery::from_maybe_shared(new_path).unwrap());

					let uri = Uri::from_parts(parts).unwrap();
					req.match_info_mut().get_mut().update(&uri);
					req.head_mut().uri = uri;
				}

				srv.call(req)
			})
			.default_service(
				actix_files::Files::new("", "./srv")
					.index_file("index.html")
					.default_handler(|req: ServiceRequest| {
						let (http_req, _payload) = req.into_parts();

						async {
							let response = NamedFile::open("./srv/404.html")?
								.set_status_code(actix_web::http::StatusCode::from_u16(404).unwrap())
								.into_response(&http_req)?;
							Ok(ServiceResponse::new(http_req, response))
						}
					})
			)
	}).bind(addr)?.run().await
	/*
	let database = Arc::new(Mutex::new(database::Database::new().expect("Database init failed")));
	let database_mutex_clone = Arc::clone(&database);
	pool.execute(|| handle_connection(database_mutex_clone, stream));
	*/
}