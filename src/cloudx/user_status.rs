use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A Neos user/friend's status.
///
/// The response from the API at `users/{user_id}/status`.
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`Neos::get_user_status`](crate::api_client::Neos::get_user_status)"
)]
pub struct UserStatus {
	/// "Online" / "Offline" and so on
	pub online_status: crate::OnlineStatus,
	#[serde(rename = "lastStatusChange")]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// When the user's status last changed
	///
	/// Wrong/Invalid dates such as `2018-01-01T00:00:00` are expressed as None
	pub last_status_change_time: Option<DateTime<Utc>>,
	/// The id of the session that the user is currently in
	pub current_session_id: Option<crate::id::Session>,
	/// The access level of the session that the user is currently in
	pub current_session_access_level: crate::SessionAccessLevel,
	#[serde(rename = "currentSessionHidden")]
	/// If the session that the user is currently in is hidden
	pub is_current_session_hidden: bool,
	#[serde(rename = "currentHosting")]
	/// If the user is currently hosting a session
	pub is_current_hosting: bool,
	/// "Screen" or "VR" for example
	pub output_device: crate::OutputDevice,
	/// Only seems to exist when the user is online
	pub compatibility_hash: Option<String>,
	/// Only seems to exist when the user is online
	pub neos_version: Option<String>,
	/// Only seems to exist when the user is online
	#[serde(rename = "publicRSAKey")]
	pub public_rsa_key: Option<crate::RSAParametersData>,
	/// If the user is using a mobile client.
	pub is_mobile: bool,
	/// Only seems to exist when the user is online
	#[serde(default)]
	pub active_sessions: Vec<crate::SessionInfo>,
}
