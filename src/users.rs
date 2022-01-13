//! Models that relate to the users.

#![allow(clippy::struct_excessive_bools)]

use crate::{
	sessions::{NeosSession, SessionAccessLevel},
	AssetUrl,
	NeosPublicBanType,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Details about a friend/contact.
///
/// The response from the API at `users/{user_id}/friends`.
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`crate::api_client::NeosAuthenticated::get_friends`]."
)]
pub struct NeosFriend {
	/// The U-username form of ID
	pub id: crate::id::User,
	/// The actual username
	pub friend_username: String,
	/// The status of the friendship
	pub friend_status: String,
	/// If the friendship has been accepted
	pub is_accepted: bool,
	/// The status of the user
	pub user_status: NeosUserStatus,
	/// The profile of the user
	pub profile: Option<NeosUserProfile>,
	#[serde(with = "serde_with::rust::default_on_error")]
	/// When the latest message with the friend was at.
	///
	/// Wrong/Invalid dates such as `0001-01-01T00:00:00` are expressed as
	/// None.
	pub latest_message_time: Option<DateTime<Utc>>,
	/// The U-username form of ID of whose friend the details are for.
	pub owner_id: crate::id::Owner,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Details about a Neos user.
///
/// The response from the API at `users/{user_id}`.
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`api_client::Neos::search_users`](crate::api_client::Neos::search_users) &
	[`api_client::Neos::get_user`](crate::api_client::Neos::get_user)."
)]
pub struct NeosUser {
	/// The U-username form of ID
	pub id: crate::id::User,
	/// The actual username
	pub username: String,
	/// Normalized (capitalization) version of the username.
	pub normalized_username: String,
	/// Possible alternatives to the normalized username
	pub alternate_normalized_names: Option<Vec<String>>,
	/// The email address of the user. Only visible when logged in.
	pub email: Option<String>,
	/// When the user registered their Neos account.
	pub registration_date: DateTime<Utc>,
	/// If the account is verified
	pub is_verified: bool,
	/// When the account ban expires
	pub account_ban_expiration: Option<DateTime<Utc>>,
	/// When the public ban expires
	pub public_ban_expiration: Option<DateTime<Utc>>,
	/// The type of public ban
	pub public_ban_type: Option<NeosPublicBanType>,
	/// When the spectator ban expires
	pub spectator_ban_expiration: Option<DateTime<Utc>>,
	/// When the mute ban expires
	pub mute_ban_expiration: Option<DateTime<Utc>>,
	/// When the listing ban expires
	pub listing_ban_expiration: Option<DateTime<Utc>>,
	#[serde(with = "serde_with::rust::default_on_error")]
	/// How much large is the users storage quota.
	///
	/// The api returns -1 for no permissions, which is deserialized into None
	/// here.
	pub quota_bytes: Option<u64>,
	/// If the account is prevented from logging in
	pub is_locked: bool,
	/// If ban evasion is supressed for the user.
	pub supress_ban_evasion: bool,
	#[serde(with = "serde_with::rust::default_on_error")]
	/// How much storage quota the user has used.
	///
	/// The api returns -1 for no permissions, which is deserialized into None
	/// here.
	pub used_bytes: Option<u64>,
	#[serde(rename = "2fa_login")]
	/// If the user has two factor authentication turned on.
	pub two_factor_login: bool,
	#[serde(default)]
	/// Tags of the user. Seem to match up with the badges.
	pub tags: Vec<String>,
	/// The profile of the user
	pub profile: Option<NeosUserProfile>,
	/// NCR crypto referral id probably
	pub referral_id: Option<String>,
	/// Data about the user's patreon subscription
	pub patreon_data: Option<NeosUserPatreonData>,
	/// Credits, seems to exist only when authenticated.
	pub credits: Option<NeosUserCredits>,
	#[serde(rename = "NCRdepositAddress")]
	/// NCR crypto address, seems to exist only when authenticated.
	pub ncr_deposit_address: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A Neos user/friend's status.
///
/// The response from the API at `users/{user_id}/status`.
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`api_client::Neos::get_user_status`](crate::api_client::Neos::get_user_status)"
)]
/// Also found in [`NeosFriend`](NeosFriend::user_status).
pub struct NeosUserStatus {
	/// "Online" / "Offline" and so on
	pub online_status: NeosUserOnlineStatus,
	#[serde(with = "serde_with::rust::default_on_error")]
	/// When the user's status last changed
	///
	/// Wrong/Invalid dates such as `2018-01-01T00:00:00` are expressed as None
	pub last_status_change: Option<DateTime<Utc>>,
	/// The access level of the session that the user is currently in
	pub current_session_access_level: SessionAccessLevel,
	/// If the session that the user is currently in is hidden
	pub current_session_hidden: bool,
	/// If the user is currently hosting a session
	pub current_hosting: bool,
	/// "Screen" or "VR" for example
	pub output_device: NeosOutputDevice,
	/// Only seems to exist when the user is online
	pub compatibility_hash: Option<String>,
	/// Only seems to exist when the user is online
	pub neos_version: Option<String>,
	/// Only seems to exist when the user is online
	#[serde(rename = "publicRSAKey")]
	pub public_rsa_key: Option<NeosUserPublicRSA>,
	/// If the user is using a mobile client.
	pub is_mobile: bool,
	/// Only seems to exist when the user is online
	#[serde(default)]
	pub active_sessions: Vec<NeosSession>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// A Neos users public RSA keypair...for... session authentication?
///
/// Found for example in [`NeosUserStatus`](NeosUserStatus::public_rsa_key).
pub struct NeosUserPublicRSA {
	/// The exponent component of the RSA pubkey
	pub exponent: String,
	/// The modulus component of the RSA pubkey
	pub modulus: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Partial profile of a Neos user.
///
/// Found for example in [`NeosFriend`](NeosFriend::profile)
pub struct NeosUserProfile {
	/// The url seems to be in a Neos' own neosdb:// format
	pub icon_url: Option<AssetUrl>,
	/// If the user has opted out of "NCR" or "KCR" for example.
	pub token_opt_out: Option<Vec<String>>,
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
/// The online status of a Neos user.
///
/// Found for example in [`NeosUserStatus`](NeosUserStatus::online_status).
pub enum NeosUserOnlineStatus {
	/// The user is online
	Online,
	/// The user is away
	Away,
	/// The user is busy offline
	Busy,
	/// The user is offline
	Offline,
}

impl NeosUserOnlineStatus {
	/// (R,G,B) colors that are estimated from the game's UI
	#[must_use]
	pub const fn color(&self) -> (u8, u8, u8) {
		match &self {
			NeosUserOnlineStatus::Online => (0, 255, 0),
			NeosUserOnlineStatus::Away => (255, 200, 0),
			NeosUserOnlineStatus::Busy => (255, 0, 0),
			NeosUserOnlineStatus::Offline => (127, 127, 127),
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Data about a Neos user's patreon subscription.
///
/// Found for example in [`NeosUser`](NeosUser::patreon_data).
/// Some fields missing due to seemingly lack of purpose of them.
pub struct NeosUserPatreonData {
	/// If the user is current supporting Neos on Patreon
	pub is_patreon_supporter: bool,
	/// If the user has supported Neos on Patreon.
	pub has_supported: bool,
	/// Guess: If the user has donated enough to be a board member
	pub last_is_anorak: bool,
	/// The ID of the github issue that this user has set as their priority
	pub priority_issue: u32,
	#[serde(with = "serde_with::rust::default_on_error")]
	/// Guess: The second last time when the user last activated their Patreon
	/// subscription
	///
	/// Wrong/Invalid dates such as `0001-01-01T00:00:00` are expressed as
	/// None.
	pub last_plus_activation_time: Option<DateTime<Utc>>,
	#[serde(with = "serde_with::rust::default_on_error")]
	/// When the user last activated their Patreon subscription
	///
	/// Wrong/Invalid dates such as `0001-01-01T00:00:00` are expressed as
	/// None.
	pub last_activation_time: Option<DateTime<Utc>>,
}

/// The amount of credits that a neos user has.
///
/// Found for example in [`NeosUser`](NeosUser::credits).
/// Although only when querying the logged in user's details.
#[derive(Debug, Clone, Deserialize)]
pub struct NeosUserCredits {
	#[serde(rename = "KFC")]
	/// Neos testing credits
	pub kfc: Option<f64>,
	#[serde(rename = "NCR")]
	/// Neos credits
	pub ncr: Option<f64>,
}

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	strum::FromRepr,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[repr(u8)]
/// The type of output device that the user is using.
///
/// The API is inconsistent, sometimes representing this as a string and
/// sometimes as a number.
///
/// Found for example in
/// [`NeosUserStatus`](NeosUserStatus::output_device) &
/// [`NeosSessionUser`](crate::NeosSessionUser::output_device)
pub enum NeosOutputDevice {
	/// Output device not known
	Unknown = 0,
	/// No visual output, server machine
	Headless = 1,
	/// Desktop
	Screen = 2,
	#[strum(to_string = "VR")]
	/// Virtual Reality
	Vr = 3,
	/// In game camera
	Camera = 4,
}

// Allow the NeosOutputDevice to be either represented as a string or number in
// JSON.
impl<'de> Deserialize<'de> for NeosOutputDevice {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		struct NeosOutputDeviceVisitor;

		impl<'de> serde::de::Visitor<'de> for NeosOutputDeviceVisitor {
			type Value = NeosOutputDevice;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter
					.write_str("a string or number matching the NeosOutputDevice enum")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				NeosOutputDevice::from_repr(v).ok_or_else(|| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v.into()),
						&"enum u8 repr",
					)
				})
			}

			fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Signed(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				NeosOutputDevice::try_from(v).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Str(v),
						&"enum str repr",
					)
				})
			}
		}

		deserializer.deserialize_any(NeosOutputDeviceVisitor)
	}
}
