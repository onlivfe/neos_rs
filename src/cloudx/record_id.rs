use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A generic Neos record, used for storage related things.
pub struct RecordId {
	#[serde(rename = "recordId")]
	/// The ID of the record (`R-{uuid}` for example)
	pub id: crate::id::Record,
	/// The ID of the owner (`U-{uuid}` or `G-{uuid}` for example)
	pub owner_id: crate::id::Owner,
}
