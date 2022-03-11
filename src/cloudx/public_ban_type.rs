use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Deserialize,
	Serialize,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
/// The type of a ban.
pub enum PublicBanType {
	/// A standard ban
	Standard,
	/// A soft ban
	Soft,
	/// A hard ban
	Hard,
}
