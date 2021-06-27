mod crypto;
mod database;
mod auth;
mod config;

use std::error::Error;

use warp::Filter;

// use database::Database;
// use database::UserRegistration;

use config::Config;

/*#[derive(Clone)]
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
}*/

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
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv()?;

	let config = Config::new()?;

	let index = warp::path::end().map(|| "Hello world!");

	let routes = warp::get().and(index);

	warp::serve(routes).run(config.ip).await;

	Ok(())

	// let state = AppState::new(&config).await;

	// let mut app = tide::with_state(state.clone());

	// let store = state.database.create_session_store().await;
	// store.migrate().await.expect("Failed to migrate");

	// app.with(tide::sessions::SessionMiddleware::new(
	// store,
	// 	&config.secret
	// ));
	// app.at("/register").post(register_user);
	// app.at("/users").get(get_user);
	// app.at("/login").post(login_user);
}
