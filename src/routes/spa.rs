use warp::{ Filter, Reply, Rejection };

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
	let single_page_app = warp::fs::dir("spa");

	return single_page_app;
}