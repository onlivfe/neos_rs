use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::FromRepr,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[repr(u8)]
/// A description of the type of transaction that happens with credits in Neos
pub enum TransactionType {
	/// An user sending monies to another user
	User2User,
	/// Taking monies out
	Withdrawal,
	/// Putting monies in
	Deposit,
	/// Giving monies to express gratitude
	Tip,
	/// Exchanging monies for goods/services
	Purchase,
}
