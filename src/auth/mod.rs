pub use bcrypt::BcryptError::InvalidPassword as InvalidPasswordError;

pub fn encrypt_password(password: &str) -> Result<String, anyhow::Error> {
	let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
	Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, anyhow::Error> {
	Ok(bcrypt::verify(password, hash)?)
}