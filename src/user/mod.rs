pub mod error;
mod registration;
mod store;
mod login;
mod view;

pub use registration::*;
pub use store::*;
pub use login::*;
pub use view::*;

use chrono::NaiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub hash: String,
	pub created_at: NaiveDateTime,
	pub role: Role
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "USER_ROLE", rename_all = "lowercase")]
pub enum Role {
	User,
	Admin
}