use crate::crypto::verify;

use sqlx::FromRow;

use chrono::NaiveDateTime;

#[derive(FromRow)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub hash: String,
	pub created_at: NaiveDateTime,
	pub role: Role
}

impl User {
	fn check_credentials(&self, password: &str) -> bool {
		verify(password, &self.hash)
	}
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "ROLE", rename_all = "lowercase")]
pub enum Role {
	User,
	Admin
}