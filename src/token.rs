use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Token {
	pub expires_in: usize,
	pub access_token: String,
	pub refresh_token: String,
}
