use crate::database::Database;
use super::UserStore;
use super::User;
use super::error::{ EntryError, Field, Issue };

static MIN_USERNAME_LENGTH: u32 = 4;
static MAX_USERNAME_LENGTH: u32 = 64;

static MAX_EMAIL_LENGTH: u32 = 128;

static MIN_PASSWORD_LENGTH: u32 = 8;
static MAX_PASSWORD_LENGTH: u32 = 32;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserRegistration {
	pub username: String,
	pub email: String,
	pub password: String,
	pub password_confirmation: String
}

impl UserRegistration {
	fn validate_password(&self) -> Result<(), EntryError> {
		let length = self.password.len();

		if self.password != self.password_confirmation {
			Err(EntryError {
				field: Field::Password,
				issue: Issue::Mismatch
			})
		} else if length < MIN_PASSWORD_LENGTH as usize {
			Err(EntryError {
				field: Field::Password,
				issue: Issue::TooShort
			})
		} else if length > MAX_PASSWORD_LENGTH as usize {
			Err(EntryError {
				field: Field::Password,
				issue: Issue::TooLong
			})
		} else {
			Ok(())
		}
	}

	fn validate_email(&self) -> Result<(), EntryError> {
		let length = self.email.len();

		if length > MAX_EMAIL_LENGTH as usize {
			Err(EntryError {
				field: Field::Email,
				issue: Issue::TooLong
			})
		} else {
			Ok(())
		}
	}

	fn validate_username(&self) -> Result<(), EntryError> {
		let length = self.username.len();

		if length < MIN_USERNAME_LENGTH as usize {
			Err(EntryError {
				field: Field::Username,
				issue: Issue::TooShort
			})
		} else if length > MAX_USERNAME_LENGTH as usize {
			Err(EntryError {
				field: Field::Username,
				issue: Issue::TooLong
			})
		} else {
			Ok(())
		}
	}

	pub fn validate(&self) -> Result<(), EntryError> {
		self.validate_username()?;
		self.validate_email()?;
		self.validate_password()?;

		Ok(())
	}

	pub async fn register(self, database: Database) -> Result<User, anyhow::Error> {
		let result = database.register_user(self).await;

		match result {
			Ok(user) => Ok(user),
			Err(error) => match error.downcast_ref::<sqlx::Error>() {
				Some(sqlx_error) => match sqlx_error {
					sqlx::Error::Database(db_error) => match db_error.constraint() {
						Some(constraint) => match constraint {
							"users_username_key" => Err(EntryError {
								field: Field::Username,
								issue: Issue::Taken
							}.into()),
							"users_email_key" => Err(EntryError {
								field: Field::Email,
								issue: Issue::Taken
							}.into()),
							_ => {
								Err(error)
							}
						},
						None => Err(error)
					},
					_ => Err(error)
				},
				None => Err(error)
			}
		}
	}
}