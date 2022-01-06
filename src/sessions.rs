//! Models that relate to sessions.

use crate::{NeosOutputDevice, NeosRecordId};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Short description of a session's user.
///
/// Found for example in [NeosSession::session_users]
pub struct NeosSessionUser {
	/// The username of the user
	pub username: String,
	#[serde(rename = "userID")]
	/// The ID of the user (`U-{uuid}` for example)
	pub user_id: String,
	/// If the user is focused on this session
	pub is_present: bool,
	/// The output device type of the user
	pub output_device: NeosOutputDevice,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// A NeosVR session.
///
/// Found for example in [NeosUserStatus::active_sessions](crate::NeosUserStatus::active_sessions)
pub struct NeosSession {
	pub name: String,
	pub corresponding_world_id: NeosRecordId,
	pub tags: Vec<String>,
	/// The ID of the session (`S-{uuid}` for example)
	pub session_id: String,
	/// Normalized (capitalization) version of the session's id (`s-{uuid}` for example)
	pub normalized_session_id: String,
	/// The ID of the session's host (`U-{uuid}` for example)
	pub host_user_id: String,
	/// The ID of the session's host's machine (`i-{rand}` for example)
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
	/// Links to the session, in custom protocols such as `lnl-nat:///` and `neos-steam://`
	pub session_urls: Vec<String>,
	/// A list of the session's users very basic details.
	pub session_users: Vec<NeosSessionUser>,
	/// A link to the thumbnail of the session.
	///
	/// Can be https:// or neosdb:// for example
	pub thumbnail: String,
	pub joined_users: u8,
	pub active_users: u8,
	pub total_joined_users: u8,
	pub total_active_users: u8,
	pub max_users: u8,
	pub mobile_friendly: bool,
	pub session_begin_time: chrono::DateTime<chrono::Utc>,
	pub last_update: chrono::DateTime<chrono::Utc>,
	pub access_level: SessionAccessLevel,
	pub has_ended: bool,
	pub is_valid: bool,
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
/// Found for example in [NeosSession::access_level]
pub enum SessionAccessLevel {
	Private = 0,
	Lan = 1,
	Friends = 2,
	FriendsOfFriends = 3,
	RegisteredUsers = 4,
	Anyone = 5,
}

// Allow the SessionAccessLevel to be either represented as a string or number in JSON.
impl<'de> Deserialize<'de> for SessionAccessLevel {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		struct SessionAccessLevelVisitor;

		impl<'de> Visitor<'de> for SessionAccessLevelVisitor {
			type Value = SessionAccessLevel;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str(
					"a string or number matching the SessionAccessLevel enum",
				)
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
