mod crypto;
mod database;
mod auth;
mod config;

use warp::Filter;

use database::Database;
use database::UserRegistration;

use config::Config;

#[derive(Clone)]
struct AppState {
	database: Database,
}

impl AppState {
	async fn new(config: &config::Config) -> AppState {
		let database = Database::connect(&config.database_url)
			.await
			.expect("Failed to connect to database");

		AppState { database }
	}
}

/*async fn register_user(mut req: Request) -> tide::Result {
	let reg: UserRegistration = req.body_form().await?;

	match req.state().database.register_user(reg).await {
		Ok(user) => {
			let session = req.session_mut();
			session.insert("id", user.id).unwrap();
			Ok(format!(
				r#"User "{}" ({}) registered with email "{}", hash "{}""#,
				user.username,
				user.id,
				user.email,
				user.hash
			).into())
		},
		Err(e) => match e {
			database::UserRegistrationError::EntryError(entry) => Ok("Entry error".into()),
			database::UserRegistrationError::DatabaseError(_) => Ok("Database error!".into())
		}
	}
}

async fn get_user(req: Request) -> tide::Result {
	let result = req.state().database.get_user_by_username("cyrus").await;

	match result {
		Ok(value) => match value {
			Some(user) => Ok(format!(r#"User "{}" found, email "{}""#, user.username, user.email).into()),
			None => Ok("None found!".into())
		},
		Err(_) => Ok("Error!".into())
	}
}

async fn index(mut req: Request) -> tide::Result {
	let session = req.session_mut();
	let visits: usize = session.get::<usize>("visits").unwrap_or_default() + 1;
    session.insert("visits", visits).unwrap();
	Ok(format!("Site visited {} times", visits).into())
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv::dotenv()?;

	let config = Config::new()?;

	let routes = warp::any().map(|| "Hello world!");

	warp::serve(routes).run(config.ip).await;

	Ok(())

	// let config = config::Config::new()?;

	// let state = AppState::new(&config).await;

	// let mut app = tide::with_state(state.clone());

	// let store = state.database.create_session_store().await;
	// store.migrate().await.expect("Failed to migrate");

	// app.with(tide::sessions::SessionMiddleware::new(
	// 	store,
	// 	&config.secret
	// ));

	// app.at("/api").nest({
	// 	let mut app = tide::new();
	// 	app.at("/");
	// 	app
	// });
	// app.at("/srv/*").serve_dir("srv/")?;
	// app.at("/").serve_file("srv/index.html")?;
	// app.at("/about").serve_file("srv/index.html")?;
	// app.at("/contact").serve_file("srv/index.html")?;
	// app.at("/register").post(register_user);
	// app.at("/users").get(get_user);
	// app.at("/login").post(login_user);

	// app.listen(config.address).await?;
}
