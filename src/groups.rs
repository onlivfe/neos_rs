use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Details about a Neos group.
///
/// The response from the API at `groups/{group_id}`.
pub struct NeosGroup {
	/// The G-groupname form of ID
	pub id: crate::id::Group,
	/// The U-username form of ID
	pub admin_user_id: crate::id::User,
	/// The name of the group
	pub name: String,
	/// How much large is the group's storage quota.
	pub quota_bytes: u64,
	/// How much storage quota the group has used.
	pub used_bytes: u64,
}
