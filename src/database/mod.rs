mod user;
pub use user::*;

mod user_registration;
pub use user_registration::*;

mod error;
pub use error::*;

use sqlx::postgres::PgPool;

use crate::crypto::hash;

#[derive(Clone)]
pub struct Database {
	connection_pool: PgPool
}

impl Database {
	pub async fn connect(url: &str) -> Result<Self, ()> {
		PgPool::connect(url).await
			.map(|connection_pool| Database { connection_pool })
			.map_err(|_| ())
	}

	pub async fn register_user(&self, req: UserRegistration) -> Result<User, UserRegistrationError> {
		req.validate()?;
		
		let hash = hash(&req.password)?;

		let user = sqlx::query_as!(
			User, 
			r#"
				INSERT INTO users ( username, email, hash ) 
				VALUES ( $1, $2, $3 )
				RETURNING id, username, email, hash, created_at, role as "role!: Role"
			"#, 
			req.username,
			req.email,
			hash
		)
		.fetch_one(&self.connection_pool)
		.await?;
		
		Ok(user)
	}

	pub async fn get_user_by_id(&self, id: i32) -> Option<User> {
		let user = sqlx::query_as!(
			User, 
			r#"
				SELECT id, username, email, hash, created_at, role as "role!: Role" FROM users
				WHERE id=$1
			"#,
			id
		)
		.fetch_optional(&self.connection_pool)
		.await?;

		Some(user)
	}

	pub async fn get_user_by_username(&self, username: &str) -> Option<User> {
		let user = sqlx::query_as!(
			User, 
			r#"
				SELECT id, username, email, hash, created_at, role as "role!: Role" FROM users
				WHERE username=$1
			"#,
			username
		)
		.fetch_optional(&self.connection_pool)
		.await?;

		Some(user)
	}

	pub async fn get_user_by_email(&self, email: &str) -> Option<User> {
		let user = sqlx::query_as!(
			User, 
			r#"
				SELECT id, username, email, hash, created_at, role as "role!: Role" FROM users
				WHERE email=$1
			"#,
			email
		)
		.fetch_optional(&self.connection_pool)
		.await?;

		Some(user)
	}
}