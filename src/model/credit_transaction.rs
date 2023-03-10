use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details a transfer of credits (KFC/NCR)
pub struct CreditTransaction {
	/// The name of the token ("NCR" for example)
	pub token: String,
	/// Who sent the credits
	pub from_user_id: Option<crate::id::User>,
	/// How much of the token was involved in the transaction
	pub amount: f64,
	/// Who received the credits
	pub transaction_type: crate::model::TransactionType,
	/// A message attached to the transaction
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub comment: String,
	/// If the transaction is anonymous or not
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub anonymous: bool,
}
