use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	serde::Serialize,
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
