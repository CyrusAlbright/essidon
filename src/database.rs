use std::env;
use std::fmt;

use postgres::{Client, Row, Error};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

pub struct Database {
	client: Client
}

impl Database {
	pub fn new(dbcreds: DatabaseCredentials) -> Result<Database, String> {
		let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
		builder.set_verify(SslVerifyMode::NONE);
		let connector = MakeTlsConnector::new(builder.build());

		let client = Client::connect(
			format!("{}", dbcreds).as_str(), 
			connector
		).map_err(|_| "Connection error")?;

		Ok(Database { client })
	}

	pub fn fetch(& mut self) -> Result<Vec<Row>, Error> {
		self.client.query("SELECT * FROM users", &[])
	}
}

pub struct DatabaseCredentials {
	host: String,
	name: String,
	user: String,
	port: i32,
	pass: String
}

impl fmt::Display for DatabaseCredentials {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		format!(
			"host={} dbname={} user={} port={} password={}",
			self.host,
			self.name,
			self.user,
			self.port,
			self.pass
		)
	}
}