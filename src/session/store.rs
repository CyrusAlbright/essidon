use async_session::{ Session, SessionStore, Result };

use chrono::Utc;

use crate::database::Database;

#[async_trait::async_trait]
impl SessionStore for Database {
	async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
		let id = Session::id_from_cookie_value(&cookie_value)?;
		let mut connection = self.pool.acquire().await?;

		let maybe_session: Option<(String,)> = sqlx::query_as(
			r#"
				SELECT session FROM sessions
				WHERE id=$1 AND (expires IS NULL OR expires > $2)
			"#
		)
		.bind(&id)
		.bind(Utc::now())
		.fetch_optional(&mut connection)
		.await?;

		Ok(maybe_session
			.map(|(session,)| serde_json::from_str(&session))
			.transpose()?)
	}
	async fn store_session(&self, session: Session) -> Result<Option<String>> {
		let id = session.id();
		let string = serde_json::to_string(&session)?;
		let mut connection = self.pool.acquire().await?;

		sqlx::query(r#"
			INSERT INTO sessions
				(id, session, expires) SELECT $1, $2, $3
			ON CONFLICT(id) DO UPDATE SET
				expires = EXCLUDED.expires, session = EXCLUDED.session
		"#)
		.bind(&id)
		.bind(&string)
		.bind(&session.expiry())
		.execute(&mut connection)
		.await?;

		Ok(session.into_cookie_value())
	}
	async fn destroy_session(&self, session: Session) -> Result {
		let id = session.id();
		let mut connection = self.pool.acquire().await?;

		sqlx::query(r#"
			DELETE FROM sessions WHERE id = $1
		"#)
		.bind(&id)
		.execute(&mut connection)
		.await?;

		Ok(())
	}
	async fn clear_store(&self) -> Result {
		let mut connection = self.pool.acquire().await?;
		sqlx::query(r#"
			TRUNCATE sessions
		"#)
		.execute(&mut connection)
		.await?;

		Ok(())
	}
}