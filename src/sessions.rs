//! Models that relate to sessions.

#![allow(clippy::struct_excessive_bools)]

use crate::{AssetUrl, NeosOutputDevice, NeosRecordId};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Short description of a session's user.
///
/// Found for example in [`NeosSession`](NeosSession::session_users)
pub struct NeosSessionUser {
	/// The username of the user
	pub username: String,
	#[serde(rename = "userID")]
	/// Always always exists, but rarely inexplicably missing
	pub user_id: Option<crate::id::User>,
	/// If the user is focused on this session
	pub is_present: bool,
	/// The output device type of the user
	pub output_device: NeosOutputDevice,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// A Neos session.
///
/// The response from the API at `sessions` & `sessions/{session_id}`.
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`api_client::Neos::get_sessions`](crate::api_client::Neos::get_sessions) &
	[`api_client::Neos::get_session`](crate::api_client::Neos::get_session)."
)]
/// Found for example in
/// [`NeosUserStatus`](crate::NeosUserStatus::active_sessions)
pub struct NeosSession {
	/// The name of the session
	pub name: String,
	/// The ID of the session's world
	pub corresponding_world_id: Option<NeosRecordId>,
	/// The tags of the session
	pub tags: Vec<String>,
	/// The ID of the session (`S-{uuid}` for example)
	pub session_id: crate::id::Session,
	/// Normalized (capitalization) version of the session's id (`s-{uuid}` for
	/// example)
	pub normalized_session_id: String,
	/// The ID of the session's host (`U-{uuid}` for example)
	pub host_user_id: Option<crate::id::User>,
	/// The ID of the session's host's machine (`{uuid}`)
	pub host_machine_id: String,
	/// The username of the session's host
	pub host_username: String,
	/// A hash to check if the sessios is compatible (version, plugins, etc)
	pub compatibility_hash: String,
	/// The version of Neos that session is hosting
	pub neos_version: String,
	/// If the host is a headless (server) instance or not.
	pub headless_host: bool,
	#[serde(rename = "sessionURLs")]
	/// Links to the session, in custom protocols such as `lnl-nat:///` and
	/// `neos-steam://`
	pub session_urls: Vec<String>,
	/// A list of the session's users very basic details.
	pub session_users: Vec<NeosSessionUser>,
	/// A link to the thumbnail of the session.
	///
	/// Can be https:// or neosdb:// for example
	pub thumbnail: Option<AssetUrl>,
	/// The amount of users that have joined the session
	pub joined_users: u8,
	/// The amount of users that are focused on the session
	pub active_users: u8,
	/// Total of joined_users..?
	pub total_joined_users: u8,
	/// Total of active_users...?
	pub total_active_users: u8,
	/// The max limit of users in the session
	pub max_users: u8,
	/// If the session is suitable for mobile clients
	pub mobile_friendly: bool,
	/// When the session began
	pub session_begin_time: chrono::DateTime<chrono::Utc>,
	/// When the session was last updated
	pub last_update: chrono::DateTime<chrono::Utc>,
	/// Who can access the session
	pub access_level: SessionAccessLevel,
	/// If the session has ended
	pub has_ended: bool,
	/// If the session is valid
	pub is_valid: bool,
}

#[must_use]
/// Tries to strip XML tags out of a string.
///
/// Not using an actual XML parser though, just a simple '<' and '>' char
/// search.
fn bad_xml_strip(str: &str) -> String {
	let start_indexes = str.match_indices('<');
	let end_indexes = str.match_indices('>');

	let mut stripped_name = str.to_owned();
	start_indexes.rev().zip(end_indexes.rev()).for_each(|((start, _), (end, _))| {
		stripped_name.replace_range(start..=end, "");
	});

	stripped_name
}

impl NeosSession {
	#[must_use]
	/// Tries to remove the XML notation-like parts from a session's name.
	///
	/// Note that this is imperfect and not using an actual XML parser to remain
	/// lightweight.
	pub fn stripped_name(&self) -> String {
		bad_xml_strip(&self.name)
	}
}

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	strum::Display,
	strum::FromRepr,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[repr(u8)]
/// A Neos session's access level.
///
/// The API is inconsistent, sometimes representing this as a string and
/// sometimes as a number.
///
/// Found for example in [`NeosSession`](NeosSession::access_level)
pub enum SessionAccessLevel {
	/// The session is private
	Private = 0,
	/// The session is accessible trough LAN
	Lan = 1,
	/// The session is accessible to the friends of the host
	Friends = 2,
	/// The session is accessible to anyone with friends in the session
	FriendsOfFriends = 3,
	/// The session is accessible to anyone who has registered an user account
	RegisteredUsers = 4,
	/// The session is accessible to anyone
	Anyone = 5,
}

// Allow the SessionAccessLevel to be either represented as a string or number
// in JSON.
impl<'de> Deserialize<'de> for SessionAccessLevel {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct SessionAccessLevelVisitor;

		impl<'de> Visitor<'de> for SessionAccessLevelVisitor {
			type Value = SessionAccessLevel;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter
					.write_str("a string or number matching the SessionAccessLevel enum")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				SessionAccessLevel::from_repr(v).ok_or_else(|| {
					de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v.into()),
						&"enum u8 repr",
					)
				})
			}

			fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					de::Error::invalid_value(
						serde::de::Unexpected::Signed(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: de::Error,
			{
				SessionAccessLevel::try_from(v).map_err(|_| {
					de::Error::invalid_value(
						serde::de::Unexpected::Str(v),
						&"enum str repr",
					)
				})
			}
		}

		deserializer.deserialize_any(SessionAccessLevelVisitor)
	}
}
