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
	pub output_device: crate::model::OutputDevice,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Deserialize)]
/// A list of a session's users that skips deserializing items with errors when
/// not in debug mode
pub struct SessionUsers(
	#[cfg_attr(not(feature = "debug"), serde_as(as = "serde_with::VecSkipError<_>"))]
	pub Vec<SessionUser>,
);
