use postgres::{Client, NoTls, Row, Error, Config};

pub struct Database {
	client: Client
}

impl Database {
	pub fn new() -> Result<Database, String> {
		let client = Client::connect("host=ec2-52-7-115-250.compute-1.amazonaws.com dbname=deb6b3ucs3l9cg user=pltegjiermmuku port=5432 password=placeholder", NoTls).map_err(|_| "Connection error")?;

		Ok(Database { client })
	}

	pub fn fetch(& mut self) -> Result<Vec<Row>, Error> {
		self.client.query("SELECT * FROM users", &[])
	}
}