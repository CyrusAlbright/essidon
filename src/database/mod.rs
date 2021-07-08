use tokio::time::interval;
use std::time::Duration;

use sqlx::{ postgres::PgPool, Executor };

use chrono::Utc;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Database {
	pub pool: PgPool
}

impl Database {
	pub async fn initialize(config: &Config) -> Result<Self, anyhow::Error> {
		log::trace!("Initializing database...");

		log::trace!("Connecting to database...");
		let pool = PgPool::connect(&config.database_url).await?;
		log::trace!("Done connecting to database.");

		let database = Database {
			pool
		};

		if let Err(error) = database.migrate().await {
			log::error!("Error while migrating database: {}", error);
		}

		let database_clone = database.clone();

		tokio::spawn(async move {
			let mut interval = interval(Duration::from_secs(600));
			interval.tick().await;
		
			loop {
				interval.tick().await;
				if let Err(error) = database_clone.cleanup().await {
					log::error!("Error while cleaning up session database: {}", error);
				}
			}
		});

		log::trace!("Done initializing database.");

		Ok(database)
	}
	pub async fn migrate(&self) -> Result<(), anyhow::Error> {
		log::trace!("Migrating database...");

		log::trace!("Migrating sessions database...");
		let mut connection = self.pool.acquire().await?;
		connection.execute(r#"
			CREATE TABLE IF NOT EXISTS sessions (
				id VARCHAR NOT NULL PRIMARY KEY,
				expires TIMESTAMP NULL,
				session TEXT NOT NULL
			)
		"#)
		.await?;
		log::trace!("Done migrating sessions database.");
		
		log::trace!("Migrating users database...");
		connection.execute(r#"
			DO $$ BEGIN
				CREATE TYPE USER_ROLE AS ENUM ('user', 'admin');
			EXCEPTION
				WHEN duplicate_object THEN null;
			END $$;
		"#)
		.await?;
		connection.execute(r#"
			CREATE TABLE IF NOT EXISTS users (
				id SERIAL PRIMARY KEY,
				username VARCHAR(64) NOT NULL UNIQUE,
				email VARCHAR(128) NOT NULL UNIQUE,
				hash VARCHAR(128) NOT NULL,
				created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
				role USER_ROLE NOT NULL DEFAULT 'user'
			)
		"#)
		.await?;
		log::trace!("Done migrating users database.");

		log::trace!("Done migrating database.");

		Ok(())
	}
	pub async fn cleanup(&self) -> Result<(), anyhow::Error> {
		log::trace!("Cleaning up sessions database...");

		let mut connection = self.pool.acquire().await?;
		sqlx::query(r#"
			DELETE FROM sessions WHERE expires < $1
		"#)
		.bind(Utc::now())
		.execute(&mut connection)
		.await?;

		log::trace!("Done cleaning up sessions database.");

		Ok(())
	}
}