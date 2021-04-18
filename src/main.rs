//mod database;

//use std::sync::Mutex;
//use std::sync::Arc;

//use database::Database;

use std::env;
use std::fmt;

use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use actix_files::NamedFile;
use actix_http::http::{Method, PathAndQuery, StatusCode, Uri};
use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{
    get, post, web::Data, App, Error, HttpResponse, HttpRequest, HttpServer, Responder,
};

use sqlx::postgres::{PgPool as DbPool, PgRow};
use sqlx::{FromRow, Row};

use chrono::NaiveDateTime;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
    hash: String,
    created_at: NaiveDateTime,
}

impl fmt::Display for User {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, 
			"ID: {}, username: {}, email: {}, hash: {}, created_at: {}",
			self.id,
			self.username,
			self.email,
			self.hash,
			self.created_at
		)
	}
}

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
    NamedFile::open(path)
}*/

#[get("/test")]
async fn test(_req: HttpRequest, db_pool: Data<DbPool>) -> Result<String, Error> {
    let row = sqlx::query_as!(User, "SELECT * FROM users")
		.fetch_one(db_pool.get_ref())
		.await
		.expect("Failed");

    Ok(format!("{}", row))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = env::var("PORT")
        .expect("Env var PORT has to be set")
        .parse::<u16>()
        .expect("Env var PORT has to be an integer");
    let ip = env::var("ADDR").expect("Env var ADDR has to be set");
    let addr = format!("{}:{}", ip, port);

    let db_url = env::var("DATABASE_URL")
		.expect("Env var DATABASE_URL has to be set");
    let db_pool = DbPool::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap_fn(|mut req: ServiceRequest, srv| {
                let head = req.head();
                let mut path = head.uri.path().to_string();
                let mut path_changed = false;

                if req.head().method == Method::GET
                    && !(path.ends_with(".html") || path.ends_with(".js") || path.ends_with(".css"))
                    && PathBuf::from(format!("./srv{}.html", path)).exists()
                {
                    path += ".html";
                    path_changed = true;
                }

                if path_changed {
                    let mut parts = head.uri.clone().into_parts();
                    let query = parts.path_and_query.as_ref().and_then(|pq| pq.query());

                    let new_path = if let Some(q) = query {
                        format!("{}?{}", path, q)
                    } else {
                        path
                    };
                    parts.path_and_query = Some(PathAndQuery::from_maybe_shared(new_path).unwrap());

                    let uri = Uri::from_parts(parts).unwrap();
                    req.match_info_mut().get_mut().update(&uri);
                    req.head_mut().uri = uri;
                }

                srv.call(req)
            })
            .service(test)
            .default_service(
                actix_files::Files::new("", "./srv")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest| {
                        let (http_req, _payload) = req.into_parts();

                        async {
                            let mut response =
                                NamedFile::open("./srv/404.html")?.into_response(&http_req)?;

                            *response.status_mut() = StatusCode::NOT_FOUND;
							
                            Ok(ServiceResponse::new(http_req, response))
                        }
                    }),
            )
    })
    .bind(addr)?
    .run()
    .await
}

/*
let database = Arc::new(Mutex::new(database::Database::new().expect("Database init failed")));
let database_mutex_clone = Arc::clone(&database);
pool.execute(|| handle_connection(database_mutex_clone, stream));
*/
