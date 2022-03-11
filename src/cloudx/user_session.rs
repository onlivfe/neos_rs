use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// An users login/auth session.
///
/// Not to be confused with a Neos session that's "an instance of a world".
/// This is the response to logging in for example.
///
/// The response from the API at POST `userSessions`.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSession {
	/// The Neos user that this session is for
	pub user_id: crate::id::User,
	/// The secret token of this session
	pub token: String,
	#[serde(rename = "created")]
	/// When the user session was created
	pub creation_time: DateTime<Utc>,
	#[serde(rename = "expire")]
	/// When the user session is set to expire
	pub expiration: DateTime<Utc>,
	/// If the user session has the remember me checked (lives longer)
	pub remember_me: bool,
	#[serde(rename = "sourceIP")]
	/// The IP address that created the user session
	pub source_ip: String,
	/// Assumed to be a Neos internal field
	pub partition_key: String,
	/// Assumed to be a Neos internal field
	pub row_key: String,
	/// A timestamp of the session
	pub timestamp: DateTime<Utc>,
	/// A standard etag, useful for caching
	pub e_tag: String,
	/// Returned when creating a new session
	pub secret_machine_id: Option<String>,
}

impl UserSession {
	#[must_use]
	/// The `Authorization` header required to use this `NeosUserSession`.
	pub fn auth_header(&self) -> String {
		"neos ".to_owned() + self.user_id.as_ref() + ":" + &self.token
	}
}

// Need to do manual impl to censor out secret token.
impl std::fmt::Debug for UserSession {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NeosUserSession")
			.field("user_id", &self.user_id)
			.field("token", &"*****")
			.field("created", &self.creation_time)
			.field("expire", &self.expiration)
			.field("remember_me", &self.remember_me)
			.field("source_ip", &self.source_ip)
			.field("partition_key", &self.partition_key)
			.field("row_key", &self.row_key)
			.field("timestamp", &self.timestamp)
			.field("e_tag", &self.e_tag)
			.finish()
	}
}