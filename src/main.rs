mod auth;
mod config;
mod database;
mod routes;
mod user;
mod error;
mod session;

use database::Database;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	dotenv::dotenv().ok();
	pretty_env_logger::init();

	let config = Config::new()?;

	let database = Database::initialize(&config).await?;

	let routes = routes::routes(database);

	warp::serve(routes).run(config.ip).await;

	Ok(())

	// app.with(tide::sessions::SessionMiddleware::new(
	//  store,
	// 	&config.secret
	// ));
}
