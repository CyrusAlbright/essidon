use warp::{ Filter, Reply, Rejection, http::{ Uri, StatusCode } };
use warp_sessions::{ CookieOptions, SessionWithStore };

use serde::de::DeserializeOwned;

use crate::user::{ User, UserStore, UserView, UserLogin, UserRegistration, error::{ Field, Issue, EntryError } };
use crate::database::Database;
use crate::session;
use crate::routes::database_route;

pub fn routes(database: Database, cookie_options: CookieOptions) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	warp::path("register").and(register_route(database.clone(), cookie_options.clone()))
		.or(warp::path("login").and(login_route(database.clone(), cookie_options.clone())))
}

pub fn register_route(database: Database, cookie_options: CookieOptions) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	warp::post()
		.and(database_route(database.clone()))
		.and(warp_sessions::request::with_session(database, Some(cookie_options)))
		.and(extract_body::<UserRegistration>())
		.and_then(|database: Database, mut session: SessionWithStore<Database>, registration: UserRegistration| async move {
			type RegisterReply = (Box<dyn Reply>, SessionWithStore<Database>);
			let result = registration.register(database.clone()).await;

			match result {
				Ok(user) => {
					log::trace!("Registered as {}", user.username);

					if let Err(error) = session.session.insert("user", user.clone()) {
						log::error!("Error while logging in user: {}", error);
					}

					let view: UserView = user.into();
					Ok::<RegisterReply, Rejection>((
						Box::new(warp::reply::json(&view)),
						session
					))
				},
				Err(error) => match error.downcast_ref::<EntryError>() {
					Some(entry_error) => Ok::<RegisterReply, Rejection>((
						Box::new(warp::reply::with_status(warp::reply::json(&entry_error), StatusCode::PRECONDITION_FAILED)),
						session
					)),
					None => {
						log::error!("Error while registering user: {}", error);
						Ok::<RegisterReply, Rejection>((
							Box::new(warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR)),
							session
						))
					}
				}
			}
		})
		.untuple_one()
		.and_then(warp_sessions::reply::with_session)
}

pub fn login_route(database: Database, cookie_options: CookieOptions) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	warp::post()
		.and(database_route(database.clone()))
		.and(warp_sessions::request::with_session(database, Some(cookie_options)))
		.and(extract_body::<UserLogin>())
		.and_then(|database, mut session: SessionWithStore<Database>, login: UserLogin| async move {
			type LoginReply = (Box<dyn Reply>, SessionWithStore<Database>);
			let result = login.login(database).await;

			match result {
				Ok(maybe_user) => match maybe_user {
					Some(user) => {
						log::trace!("Logged in as {}", user.username);

						if let Err(error) = session.session.insert("user", user.clone()) {
							log::error!("Error while logging in user: {}", error);
						}

						let view: UserView = user.into();
						Ok::<LoginReply, Rejection>((
							Box::new(warp::reply::json(&view)),
							session
						))
					},
					None => {
						Ok::<LoginReply, Rejection>((
							Box::new(warp::reply::with_status(warp::reply(), StatusCode::NOT_FOUND)),
							session
						))
					}
				},
				Err(error) => match error.downcast_ref::<EntryError>() {
					Some(entry_error) => Ok::<LoginReply, Rejection>((
						Box::new(warp::reply::with_status(warp::reply::json(&entry_error), StatusCode::UNAUTHORIZED)),
						session
					)),
					None => {
						log::error!("Error while logging in user: {}", error);
						Ok::<LoginReply, Rejection>((
							Box::new(warp::reply::with_status(warp::reply(), StatusCode::INTERNAL_SERVER_ERROR)),
							session
						))
					}
				}
			}
		})
		.untuple_one()
		.and_then(warp_sessions::reply::with_session)
}

pub fn extract_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
	warp::body::content_length_limit(1024 * 32)
		.and(warp::body::json().or(warp::body::form()).unify())
}