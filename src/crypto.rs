use bcrypt::BcryptError;

pub enum CryptoError {
	InvalidPassword,
	SystemError,
}

pub fn hash(password: &str) -> Result<String, CryptoError> {
	let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
	Ok(hash)
}

pub fn verify(password: &str, hash: &str) -> bool {
	bcrypt::verify(password, hash).unwrap_or(false)
}

impl From<bcrypt::BcryptError> for CryptoError {
	fn from(error: BcryptError) -> Self {
		match error {
			BcryptError::InvalidPassword => CryptoError::InvalidPassword,
			_ => CryptoError::SystemError,
		}
	}
}
