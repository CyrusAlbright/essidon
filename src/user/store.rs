use crate::database::Database;

use crate::auth::encrypt_password;

use super::User;
use super::registration::UserRegistration;

#[async_trait::async_trait]
pub trait UserStore {
	async fn register_user(&self, user_reg: UserRegistration) -> Result<User, anyhow::Error>;
	async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, anyhow::Error>;
	async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
	async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error>;
	async fn get_user_by_username_or_email(&self, token: &str) -> Result<Option<User>, anyhow::Error>;
}

#[async_trait::async_trait]
impl UserStore for Database {
	async fn register_user(&self, user_reg: UserRegistration) -> Result<User, anyhow::Error> {
		user_reg.validate()?;
		
		let hash = encrypt_password(&user_reg.password)?;

		let mut connection = self.pool.acquire().await?;

		let user = sqlx::query_as(
			r#"
				INSERT INTO users ( username, email, hash ) 
				VALUES ( $1, $2, $3 )
				RETURNING id, username, email, hash, created_at, role
			"#
		)
		.bind(&user_reg.username)
		.bind(&user_reg.email)
		.bind(&hash)
		.fetch_one(&mut connection)
		.await?;

		Ok(user)
	}
	async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, anyhow::Error> {
		let user = sqlx::query_as(
			r#"
				SELECT id, username, email, hash, created_at, role FROM users
				WHERE id=$1
			"#
		)
		.bind(&id)
		.fetch_optional(&self.pool)
		.await?;

		Ok(user)
	}
	async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
		let user = sqlx::query_as(
			r#"
				SELECT id, username, email, hash, created_at, role FROM users
				WHERE email=$1
			"#
		)
		.bind(&email)
		.fetch_optional(&self.pool)
		.await?;

		Ok(user)
	}
	async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, anyhow::Error> {
		let user = sqlx::query_as(
			r#"
				SELECT id, username, email, hash, created_at, role FROM users
				WHERE username=$1
			"#
		)
		.bind(&username)
		.fetch_optional(&self.pool)
		.await?;

		Ok(user)
	}
	async fn get_user_by_username_or_email(&self, token: &str) -> Result<Option<User>, anyhow::Error> {
		let user = sqlx::query_as(
			r#"
				SELECT id, username, email, hash, created_at, role FROM users
				WHERE username=$1 OR email=$1
			"#
		)
		.bind(&token)
		.fetch_optional(&self.pool)
		.await?;

		Ok(user)
	}
}