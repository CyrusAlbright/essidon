use warp::{ Filter, Reply, Rejection };
use warp_sessions::{ CookieOptions, SameSiteCookieOption };

use crate::database::Database;

mod auth;
mod spa;
mod templates;
mod global;
mod database;
mod chat;

pub use database::*;

pub fn routes(database: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	let cookie_options = CookieOptions {
		cookie_name: "sid",
		cookie_value: None,
		max_age: Some(60 * 60 * 24),
		domain: None,
		path: None,
		secure: true,
		http_only: true,
		same_site: Some(SameSiteCookieOption::Strict)
	};

	

	global::routes()
		.or(warp::path("spa").and(spa::routes()))
		.or(auth::routes(database, cookie_options))
}