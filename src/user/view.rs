use super::User;

use chrono::NaiveDateTime;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserView {
	pub username: String,
	pub email: String,
	pub created_at: NaiveDateTime
}

impl From<User> for UserView {
	fn from(user: User) -> Self {
		UserView {
			username: user.username,
			email: user.email,
			created_at: user.created_at
		}
	}
}