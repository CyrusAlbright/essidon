use postgres::{Client, NoTls, Row, Error};

pub struct Database {
	client: Client
}

impl Database {
	pub fn new() -> Result<Database, &'static str> {
		let mut client = Client::connect("host=0.0.0.0 user=postgres", NoTls).map_err(|_| "Error")?;

		Ok(Database { client })
	}

	pub fn fetch(&self) -> Result<Vec<Row>, Error> {
		self.client.query("SELECT * FROM users", &[])
	}
}