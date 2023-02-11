use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Deserialize,
	Serialize,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
/// The online status of a Neos user.
pub enum OnlineStatus {
	/// The user is online
	Online,
	/// The user is invisible
	Invisible,
	/// The user is away
	Away,
	/// The user is busy offline
	Busy,
	/// The user is offline
	Offline,
}

impl OnlineStatus {
	/// (R,G,B) colors that are estimated from the game's UI
	#[must_use]
	pub const fn color(&self) -> (u8, u8, u8) {
		match &self {
			Self::Online => (0, 255, 0),
			Self::Away => (255, 200, 0),
			Self::Busy => (255, 0, 0),
			Self::Offline | Self::Invisible => (127, 127, 127),
		}
	}
}
