use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Data about a Neos user's patreon subscription.
pub struct UserPatreonData {
	/// If the user is current supporting Neos on Patreon
	pub is_patreon_supporter: bool,
	/// If the user has supported Neos on Patreon.
	pub has_supported: bool,
	/// Guess: If the user has donated enough to be a board member
	pub last_is_anorak: bool,
	/// The ID of the github issue that this user has set as their priority
	pub priority_issue: u32,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// Guess: The second last time when the user last activated their Patreon
	/// subscription
	///
	/// Wrong/Invalid dates such as `0001-01-01T00:00:00` are expressed as
	/// None.
	pub last_plus_activation_time: Option<OffsetDateTime>,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the user last activated their Patreon subscription
	///
	/// Wrong/Invalid dates such as `0001-01-01T00:00:00` are expressed as
	/// None.
	pub last_activation_time: Option<OffsetDateTime>,
}
