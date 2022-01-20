use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Details about a Neos group.
///
/// The response from the API at `groups/{group_id}`.
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`api_client::Neos::get_group`](crate::api_client::Neos::get_group)."
)]
pub struct NeosGroup {
	/// The G-groupname form of ID
	pub id: crate::id::Group,
	#[serde(rename = "adminUserId")]
	/// The U-username form of ID
	pub admin_id: crate::id::User,
	/// The name of the group
	pub name: String,
	/// How much large is the group's storage quota.
	pub quota_bytes: u64,
	/// How much storage quota the group has used.
	pub used_bytes: u64,
}
