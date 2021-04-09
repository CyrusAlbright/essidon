use std::fs;

use postgres::{Client, Row, Error};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

pub struct Database {
	client: Client
}

impl Database {
	pub fn new() -> Result<Database, String> {
		let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
		builder.set_verify(SslVerifyMode::NONE);
		let connector = MakeTlsConnector::new(builder.build());

		let client = Client::connect(
			format!("host=ec2-52-7-115-250.compute-1.amazonaws.com dbname=deb6b3ucs3l9cg user=pltegjiermmuku port=5432 password={}", fs::read_to_string("db.info").unwrap()), 
			connector
		).map_err(|_| "Connection error")?;

		Ok(Database { client })
	}

	pub fn fetch(& mut self) -> Result<Vec<Row>, Error> {
		self.client.query("SELECT * FROM users", &[])
	}
}