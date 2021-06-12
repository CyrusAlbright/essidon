use crate::crypto::CryptoError;

pub enum DatabaseError {
	DatabaseError
}

impl From<sqlx::Error> for DatabaseError {
	fn from(_error: sqlx::Error) -> Self {
		DatabaseError::DatabaseError
	}
}

pub enum UserRegistrationError {
	EntryError(EntryError),
	DatabaseError(DatabaseError),
}

pub struct EntryError {
	pub field: Field,
	pub issue: Issue,
}

pub enum Field {
	Username,
	Email,
	Password,
}

pub enum Issue {
	TooLong,
	TooShort,
	Invalid,
	Taken,
	Mismatch
}

impl From<CryptoError> for UserRegistrationError {
	fn from(error: CryptoError) -> Self {
		match error {
			CryptoError::InvalidPassword => {
				EntryError {
					field: Field::Password,
					issue: Issue::Invalid
				}.into()
			}
			CryptoError::SystemError => DatabaseError::DatabaseError.into(),
		}
	}
}

impl From<EntryError> for UserRegistrationError {
	fn from(error: EntryError) -> Self {
		UserRegistrationError::EntryError(error)
	}
}

impl From<DatabaseError> for UserRegistrationError {
	fn from(error: DatabaseError) -> Self {
		UserRegistrationError::DatabaseError(error)
	}
}

impl From<sqlx::Error> for UserRegistrationError {
	fn from(error: sqlx::Error) -> Self {
		match error {
			sqlx::Error::Database(db_error) => match db_error.constraint() {
				Some(constraint) => match constraint {
					"users_username_key" => EntryError {
						field: Field::Username,
						issue: Issue::Taken
					}.into(),
					"users_email_key" => EntryError {
						field: Field::Email,
						issue: Issue::Taken
					}.into(),
					_ => DatabaseError::DatabaseError.into()
				},
				None => DatabaseError::DatabaseError.into()
			},
			_ => DatabaseError::DatabaseError.into()
		}
	}
}