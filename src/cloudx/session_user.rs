#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
/// Short description of a session's user.
pub struct SessionUser {
	/// The username of the user
	pub username: String,
	#[serde(rename = "userID")]
	/// Almost always exists, but rarely inexplicably missing
	pub id: Option<crate::id::User>,
	/// If the user is focused on this session
	pub is_present: bool,
	/// The output device type of the user
	pub output_device: crate::OutputDevice,
}
