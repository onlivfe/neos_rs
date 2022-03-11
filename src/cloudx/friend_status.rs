use serde::{Deserialize, Serialize};
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Serialize,
	Deserialize,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
/// The friendship status with a neos user
pub enum FriendStatus {
	/// Not friends
	None,
	/// Apparently possible value too..?
	SearchResult,
	/// The user has requested friendship
	Requested,
	/// Ignored the friendship request
	Ignored,
	/// User has been blocked
	Blocked,
	/// Accepted the user as a friend
	Accepted,
}
