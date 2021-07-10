use warp::{ Filter, Reply, Rejection };

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	let global_file_service = warp::path("global").and(warp::fs::dir("global"));

	return global_file_service;
}