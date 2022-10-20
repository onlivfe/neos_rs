use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

/// An users login/authorization session.
///
/// Not to be confused with a Neos session that's "an instance of a world".
/// This is the response to logging in for example.
///
/// The response from the API at POST `userSessions`.
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSession {
	/// The Neos user that this session is for
	pub user_id: crate::id::User,
	/// The secret token of this session
	pub token: String,
	#[serde(rename = "created")]
	#[serde(with = "rfc3339")]
	/// When the user session was created
	pub creation_time: OffsetDateTime,
	#[serde(rename = "expire")]
	#[serde(with = "rfc3339")]
	/// When the user session is set to expire
	pub expiration: OffsetDateTime,
	/// If the user session has the remember me checked (lives longer)
	pub remember_me: bool,
	#[serde(default)]
	#[serde(rename = "sourceIP")]
	/// The IP address that created the user session
	///
	///Not found in standard CloudX models, defaults to an empty String if
	/// none.
	pub source_ip: String,
	#[serde(with = "rfc3339")]
	/// A time-stamp of the session
	pub timestamp: OffsetDateTime,
	#[serde(default)]
	/// A standard etag, useful for caching
	///
	/// Not found in standard CloudX models, defaults to an empty String if
	/// none.
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
			.field(
				"secret_machine_id",
				match &self.secret_machine_id {
					Some(_) => &"Some(*****)",
					None => &"None",
				},
			)
			.field("creation_time", &self.creation_time)
			.field("expiration", &self.expiration)
			.field("remember_me", &self.remember_me)
			.field("source_ip", &self.source_ip)
			.field("timestamp", &self.timestamp)
			.field("e_tag", &self.e_tag)
			.finish()
	}
}
