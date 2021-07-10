use std::collections::HashMap;
use std::sync::{ Arc, RwLock };

use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use warp::{ Filter, Reply, Rejection, sse::Event };
use warp_sessions::{ CookieOptions };

use crate::database::Database;
use crate::session;
use crate::routes::database_route;
use crate::user::User;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct ChatMessage {
	text: String
}

type ChatUsers = Arc<RwLock<HashMap<String, (User, UnboundedReceiverStream<ChatMessage>)>>>;

// pub fn routes(database: Database, cookie_options: CookieOptions) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {

// }

// pub fn get_message_route(database: Database, cookie_options: CookieOptions, users: ChatUsers) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {

// }

// pub fn post_message_route(database: Database, cookie_options: CookieOptions, users: ChatUsers) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
// 	warp::post()
// 		.and(warp::body::content_length_limit(1024 * 32))
// 		.and(warp::body::json::<ChatMessage>())
		
// }