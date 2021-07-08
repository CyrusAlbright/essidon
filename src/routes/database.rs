use warp::Filter;

use crate::database::Database;

pub fn database_route(database: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
	warp::any().map(move || { let db_clone = database.clone(); db_clone })
}