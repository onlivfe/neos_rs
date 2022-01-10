use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A generic Neos record, used for storage related things.
///
/// Found for example in
/// [`NeosSession`](crate::NeosSession::corresponding_world_id)
pub struct NeosRecordId {
	/// The ID of the record (`R-{uuid}` for example)
	pub record_id: crate::id::Record,
	/// The ID of the owner (`U-{uuid}` or `G-{uuid}` for example)
	pub owner_id: crate::id::Owner,
}
