use chrono::{DateTime, Utc};
use serde::Deserialize;

/// An users login/auth session.
///
/// Not to be confused with a Neos session that's "an instance of a world".
/// This is the response to logging in for example.
///
/// The response from the API at POST `userSessions`.
#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeosUserSession {
	/// The Neos user that this session is for
	pub user_id: String,
	/// The secret token of this session
	pub token: String,
	/// When the session was created
	pub created: DateTime<Utc>,
	/// When the session is set to expire
	pub expire: DateTime<Utc>,
	/// If the session has the remember me checked (lives longer)
	pub remember_me: bool,
	/// The IP address that created the session
	pub source_ip: String,
	/// Assumed to be a Neos internal field
	pub partition_key: String,
	/// Assumed to be a Neos internal field
	pub row_key: String,
	/// A timestamp of the session
	pub timestamp: DateTime<Utc>,
	/// A standard etag, useful for caching
	pub e_tag: String,
}

impl NeosUserSession {
	#[must_use]
	/// The `Authorization` header required to use this `NeosUserSession`.
	pub fn auth_header(&self) -> String {
		"neos ".to_owned() + &self.user_id + ":" + &self.token
	}
}

// Need to do manual impl to censor out secret token.
impl std::fmt::Debug for NeosUserSession {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NeosUserSession")
			.field("user_id", &self.user_id)
			.field("token", &"*****")
			.field("created", &self.created)
			.field("expire", &self.expire)
			.field("remember_me", &self.remember_me)
			.field("source_ip", &self.source_ip)
			.field("partition_key", &self.partition_key)
			.field("row_key", &self.row_key)
			.field("timestamp", &self.timestamp)
			.field("e_tag", &self.e_tag)
			.finish()
	}
}

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Deserialize,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
/// The type of a ban.
///
/// Found for example in
/// [`NeosUser`](crate::NeosUser::public_ban_type)
pub enum NeosPublicBanType {
	/// A standard ban
	Standard,
	/// A soft ban
	Soft,
	/// A hard ban
	Hard,
}
