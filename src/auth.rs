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
	pub user_id: String,
	pub token: String,
	pub created: DateTime<Utc>,
	pub expire: DateTime<Utc>,
	pub remember_me: bool,
	pub source_ip: String,
	pub partition_key: String,
	pub row_key: String,
	pub timestamp: DateTime<Utc>,
	pub e_tag: String,
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
/// Found for example in [NeosUser::public_ban_type][crate::NeosUser::public_ban_type]
pub enum NeosPublicBanType {
	Standard,
	Soft,
	Hard,
}
