use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{NeosUserProfile, NeosUserStatus};

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
	#[serde(rename = "friendUsername")]
	/// The actual username
	pub username: String,
	#[serde(rename = "friendStatus")]
	/// The status of the friendship
	pub friendship_status: String,
	/// If the friendship has been accepted
	pub is_accepted: bool,
	#[serde(rename = "userStatus")]
	/// The status of the user
	pub status: NeosUserStatus,
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
