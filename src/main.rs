mod crypto;
mod database;
mod auth;
mod config;

use std::env;

use database::Database;
use database::UserRegistration;

// use tide::http::mime;

#[derive(Clone)]
struct AppState {
	database: Database,
}

impl AppState {
	async fn new() -> AppState {
		let database_url = env::var("DATABASE_URL").expect("Env var DATABASE_URL has to be set");
		let database = Database::connect(&database_url)
			.await
			.expect("Failed to connect to database");

		AppState { database }
	}
}

type Request = tide::Request<AppState>;

async fn register_user(mut req: Request) -> tide::Result {
	let reg: UserRegistration = req.body_form().await?;

	match req.state().database.register_user(reg).await {
		Ok(user) => Ok(format!(
			r#"User "{}" ({}) registered with email "{}", hash "{}""#,
			user.username,
			user.id,
			user.email,
			user.hash
		).into()),
		Err(_) => Ok("Failed!".into())
	}
}

/*async fn login_user(mut req: Request) -> tide::Result {

}*/

async fn index(_req: Request) -> tide::Result {
	Ok("Hello".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
	let config = config::Config::new()?;

	let mut app = tide::with_state(AppState::new().await);
	app.at("/").get(index);
	app.at("/register").post(register_user);
	// app.at("/login").post(login_user);
	app.listen(config.address).await?;
	Ok(())

	/*.wrap_fn(|mut req: ServiceRequest, srv| {
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
