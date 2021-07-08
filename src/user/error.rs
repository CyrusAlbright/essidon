#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntryError {
	pub field: Field,
	pub issue: Issue,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Field {
	Username,
	Email,
	Password,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Issue {
	TooLong,
	TooShort,
	Invalid,
	Taken,
	Incorrect,
	DoesNotExist,
	Mismatch
}

impl std::fmt::Display for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Field::Username => write!(f, "username"),
			Field::Email => write!(f, "email"),
			Field::Password => write!(f, "password")
		}
	}
}

impl std::fmt::Display for Issue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Issue::TooLong => write!(f, "too long"),
			Issue::TooShort => write!(f, "too short"),
			Issue::Invalid => write!(f, "invalid"),
			Issue::Taken => write!(f, "taken"),
			Issue::Incorrect => write!(f, "incorrect"),
			Issue::DoesNotExist => write!(f, "does not exist"),
			Issue::Mismatch => write!(f, "mismatch")
		}
	}
}

impl std::fmt::Display for EntryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "entry error: {} {}", self.field, self.issue)
	}
}

impl std::error::Error for EntryError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}