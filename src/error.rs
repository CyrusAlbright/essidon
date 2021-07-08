#[derive(Debug)]
pub struct ErrorWrapper(pub anyhow::Error);

impl warp::reject::Reject for ErrorWrapper {}

impl From<anyhow::Error> for ErrorWrapper {
	fn from(e: anyhow::Error) -> Self {
		ErrorWrapper(e)
	}
}

impl From<ErrorWrapper> for anyhow::Error {
	fn from(wrapper: ErrorWrapper) -> Self {
		wrapper.0
	}
}