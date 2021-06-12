mod crypto;
mod database;
mod auth;
mod config;

use database::Database;
use database::UserRegistration;

// use tide::http::mime;

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

/*async fn login_user(mut req: Request) -> tide::Result {

}*/

async fn index(mut req: Request) -> tide::Result {
	let session = req.session_mut();
	let visits: usize = session.get::<usize>("visits").unwrap_or_default() + 1;
    session.insert("visits", visits).unwrap();
	Ok(format!("Site visited {} times", visits).into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
	let config = config::Config::new()?;

	let state = AppState::new(&config).await;

	let mut app = tide::with_state(state.clone());

	app.with(tide::sessions::SessionMiddleware::new(
		state.database.create_session_store().await,
		&config.secret
	));

	app.at("/").get(index);
	app.at("/register").post(register_user);
	app.at("/users").get(get_user);
	// app.at("/login").post(login_user);

	app.listen(config.address).await?;

	Ok(())
}
