use std::env;
use std::fmt;

use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use tide::{Request};

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

async fn test(mut req: Request<DbPool>) -> tide::Result {
    let row = sqlx::query_as!(User, "SELECT * FROM users")
		.fetch_one(&req.state())
		.await
		.expect("Failed");

    Ok(format!("{}", row).into())
}

async fn index(mut req: Request<DbPool>) -> tide::Result {
	Ok("Hello".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
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

	let mut app = tide::with_state(db_pool);
	app.at("/").get(index);
	app.at("/test").get(test);
	app.listen(addr).await?;
	Ok(())

    /*.data(db_pool.clone())
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
            )*/
}