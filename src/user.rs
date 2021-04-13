pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub hash: String,
	pub role: Role
}

pub enum Role {
	Admin,
	User
}

impl Role {
	pub fn from_str(role: &str) -> Role {
		match role {
			"Admin" => Role::Admin,
			_ => Role::User
		}
	}
}

impl fmt::Display for Role {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Role::Admin => write!(f, "Admin"),
			Role::User => write!(f, "User")
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
struct Claim {
	sub: String,
	role: String,
	exp: usize
}