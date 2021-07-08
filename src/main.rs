mod auth;
mod config;
mod database;
mod routes;
mod user;
mod error;
mod session;

use warp::{ Filter, Rejection };

use database::Database;

use config::Config;

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
async fn main() -> Result<(), anyhow::Error> {
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	let config = Config::new()?;

	let database = Database::initialize(&config).await?;

	let routes = routes::routes(database);

	warp::serve(routes).run(config.ip).await;

	Ok(())

	// let store = state.database.create_session_store().await;
	// store.migrate().await.expect("Failed to migrate");

	// app.with(tide::sessions::SessionMiddleware::new(
	//  store,
	// 	&config.secret
	// ));
	// app.at("/register").post(register_user);
	// app.at("/users").get(get_user);
	// app.at("/login").post(login_user);
}
