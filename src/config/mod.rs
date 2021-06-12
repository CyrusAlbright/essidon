use std::error::Error;
use std::env::*;
use std::fmt;

#[derive(Clone)]
pub struct Config {
	pub address: String,
	pub database_url: String,
	pub secret: Vec<u8>,
}

pub struct ConfigError {
	pub message: String
}

impl ConfigError {
	pub fn new<T>(message: T) -> ConfigError 
	where T: Into<String> {
		ConfigError {
			message: message.into()
		}
	}
}

impl fmt::Debug for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Config Error: {}", self.message)
	}
}

impl fmt::Display for ConfigError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Config Error: {}", self.message)
	}
}

impl Error for ConfigError {
	fn description(&self) -> &str {
		&self.message
	}
}

impl Config {
	pub fn new() -> Result<Self, ConfigError> {
		let port = var("PORT")
			.map_err(|_| ConfigError::new("Env var PORT must be set"))?
			.parse::<u16>()
			.map_err(|_| ConfigError::new("Env var PORT must be an integer"))?;
		
		let ip = var("ADDR")
			.map_err(|_| ConfigError::new("Env var ADDR must be set"))?;
		
		let address = format!("{}:{}", ip, port);

		let secret = var("SECRET")
			.map_err(|_| ConfigError::new("Env var SECRET must be set"))?
			.as_bytes()
			.to_vec();

		let database_url = var("DATABASE_URL")
			.map_err(|_| ConfigError::new("Env var DATABASE_URL must be set"))?;

		Ok(Config {
			address,
			secret,
			database_url
		})
	}
}