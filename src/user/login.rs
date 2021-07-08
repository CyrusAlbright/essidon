use crate::auth::verify_password;
use crate::user::User;
use crate::user::store::UserStore;
use crate::user::error::{ Field, Issue, EntryError };

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserLogin {
	login: String,
	password: String
}

impl UserLogin {
	pub async fn login(&self, store: impl UserStore) -> Result<Option<User>, anyhow::Error> {
		let maybe_user = store.get_user_by_username_or_email(&self.login).await?;

		match maybe_user {
			Some(user) => match verify_password(&self.password, &user.hash) {
				Ok(correct) => match correct {
					true => Ok(Some(user)),
					false => Err(EntryError {
						field: Field::Password,
						issue: Issue::Incorrect
					}.into())
				},
				Err(error) => Err(error)
			},
			None => Ok(None)
		}
	}
}