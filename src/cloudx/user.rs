use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::{serde::rfc3339, OffsetDateTime};

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
pub struct User {
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
	#[serde(rename = "registrationDate")]
	#[serde(with = "rfc3339")]
	/// When the user registered their Neos account.
	pub registration_time: OffsetDateTime,
	/// If the account is verified
	pub is_verified: bool,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the account ban expires
	pub account_ban_expiration: Option<OffsetDateTime>,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the public ban expires
	pub public_ban_expiration: Option<OffsetDateTime>,
	/// The type of public ban
	pub public_ban_type: Option<crate::PublicBanType>,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the spectator ban expires
	pub spectator_ban_expiration: Option<OffsetDateTime>,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the mute ban expires
	pub mute_ban_expiration: Option<OffsetDateTime>,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the listing ban expires
	pub listing_ban_expiration: Option<OffsetDateTime>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	/// How much large is the users storage quota.
	///
	/// The API returns -1 for no permissions, which is de-serialized into None
	/// here.
	pub quota_bytes: Option<u64>,
	/// If the account is prevented from logging in
	pub is_locked: bool,
	/// If ban evasion is suppressed for the user.
	pub supress_ban_evasion: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnError")]
	/// How much storage quota the user has used.
	///
	/// The API returns -1 for no permissions, which is de-serialized into None
	/// here.
	pub used_bytes: Option<u64>,
	#[serde(rename = "2fa_login")]
	/// If the user has two factor authentication turned on.
	pub two_factor_login: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Tags of the user. Seem to match up with the badges.
	pub tags: Vec<String>,
	/// The profile of the user
	pub profile: Option<crate::UserProfile>,
	/// NCR related referral id probably
	pub referral_id: Option<String>,
	/// Data about the user's Patreon subscription
	pub patreon_data: Option<crate::UserPatreonData>,
	/// Credits, seems to exist only when authenticated.
	pub credits: Option<HashMap<String, f64>>,
	#[serde(rename = "NCRdepositAddress")]
	/// NCR address, seems to exist only when authenticated.
	pub ncr_deposit_address: Option<String>,
}
