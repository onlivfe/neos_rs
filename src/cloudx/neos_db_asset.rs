use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a Neos stored asset
pub struct NeosDBAsset {
	/// The hash of the asset
	pub hash: String,
	/// How large the asset is
	pub bytes: u64,
}
