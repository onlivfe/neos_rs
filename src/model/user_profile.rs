use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Partial profile of a Neos user.
pub struct UserProfile {
	/// The URI seems to be in a Neos' own `neosdb://` format
	pub icon_url: Option<crate::AssetUrl>,
	/// If the user has opted out of "NCR" or "KCR" for example.
	pub token_opt_out: Option<Vec<String>>,
}
