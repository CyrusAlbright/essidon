use postgres::{Client, Row, Error};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

pub struct Database {
	client: Client
}

impl Database {
	pub fn new() -> Result<Database, String> {
<<<<<<< HEAD
		let client = Client::connect("host=ec2-52-7-115-250.compute-1.amazonaws.com dbname=deb6b3ucs3l9cg user=pltegjiermmuku port=5432 password=placeholder", NoTls).map_err(|_| "Connection error")?;
=======
		let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
		builder.set_verify(SslVerifyMode::NONE);
		let connector = MakeTlsConnector::new(builder.build());

		let client = Client::connect(
			"host=ec2-52-7-115-250.compute-1.amazonaws.com dbname=deb6b3ucs3l9cg user=pltegjiermmuku port=5432 password=placeholder", 
			connector
		).map_err(|_| "Connection error")?;
>>>>>>> fbe9392... Fix OpenSSL DB integration

		Ok(Database { client })
	}

	pub fn fetch(& mut self) -> Result<Vec<Row>, Error> {
		self.client.query("SELECT * FROM users", &[])
	}
}