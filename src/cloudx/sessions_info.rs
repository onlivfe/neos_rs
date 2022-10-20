use time::{serde::rfc3339, OffsetDateTime};

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
/// A Neos session.
#[cfg_attr(feature = "api_client", doc = "")]
#[cfg_attr(
	feature = "api_client",
	doc = "Can be gotten with
	[`Neos::get_sessions`](crate::api_client::Neos::get_sessions) &
	[`Neos::get_session`](crate::api_client::Neos::get_session)."
)]
pub struct SessionInfo {
	/// The name of the session
	pub name: String,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// The description of the session
	///
	/// Defaulted to empty string if the API returns none for the session.
	pub description: String,
	#[serde(rename = "correspondingWorldId")]
	/// The ID of the session's world
	pub world: Option<crate::RecordId>,
	/// The tags of the session
	pub tags: Vec<String>,
	#[serde(rename = "sessionId")]
	/// The ID of the session (`S-{uuid}` for example)
	pub id: crate::id::Session,
	#[serde(rename = "normalizedSessionId")]
	/// Normalized (capitalization) version of the session's id (`s-{uuid}` for
	/// example)
	pub normalized_id: String,
	#[serde(rename = "hostUserId")]
	/// The ID of the session's host (`U-{uuid}` for example)
	pub host_id: Option<crate::id::User>,
	/// The ID of the session's host's machine (`{uuid}`)
	pub host_machine_id: String,
	/// The username of the session's host
	pub host_username: String,
	/// A hash to check if the session is compatible (version, plugins, etc)
	pub compatibility_hash: String,
	/// The version of Neos that session is hosting
	pub neos_version: String,
	#[serde(rename = "headlessHost")]
	/// If the host is a headless (server) instance or not.
	pub is_headless_host: bool,
	#[serde(rename = "sessionURLs")]
	/// Links to the session, in custom protocols such as `lnl-nat:///` and
	/// `neos-steam://`
	pub urls: Vec<String>,
	#[serde(rename = "sessionUsers")]
	/// A list of the session's users very basic details.
	pub users: Vec<crate::SessionUser>,
	/// A link to the thumbnail of the session.
	///
	/// Can be `https://` or `neosdb://` for example
	pub thumbnail: Option<crate::AssetUrl>,
	/// The amount of users that have joined the session
	pub joined_users: u8,
	/// The amount of users that are focused on the session
	pub active_users: u8,
	/// Total of `joined_users`..?
	pub total_joined_users: u8,
	/// Total of `active_users`...?
	pub total_active_users: u8,
	/// The max limit of users in the session
	pub max_users: u8,
	#[serde(rename = "mobileFriendly")]
	/// If the session is suitable for mobile clients
	pub is_mobile_friendly: bool,
	#[serde(with = "rfc3339")]
	/// When the session began
	pub session_begin_time: OffsetDateTime,
	#[serde(rename = "lastUpdate")]
	#[serde(with = "rfc3339")]
	/// When the session was last updated
	pub last_update_time: OffsetDateTime,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// Since when has the host been away most likely
	pub away_since: Option<OffsetDateTime>,
	/// Who can access the session
	pub access_level: crate::SessionAccessLevel,
	/// If the session has ended
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default = "has_ended_default")]
	pub has_ended: bool,
	/// If the session is valid
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	pub is_valid: bool,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Sessions that this session is a child of
	///
	/// Defaulted to empty vector if the API returns none for the session.
	pub parent_session_ids: Vec<crate::id::Session>,
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	#[serde(default)]
	/// Sessions that are the child of this session
	///
	/// Defaulted to empty vector if the API returns none for the session.
	pub nested_session_ids: Vec<crate::id::Session>,
}

// If the field is missing, it probably has ended...
const fn has_ended_default() -> bool {
	true
}

#[must_use]
/// Tries to strip XML tags out of a string.
///
/// Not using an actual XML parser though, just a simple `<` and `>` character
/// search.
fn bad_xml_strip(str: &str) -> String {
	let start_indexes = str.match_indices('<');
	let end_indexes = str.match_indices('>');

	let mut stripped_name = str.to_owned();
	start_indexes.rev().zip(end_indexes.rev()).for_each(|((start, _), (end, _))| {
		if start < end {
			stripped_name.replace_range(start..=end, "");
		}
	});

	stripped_name
}

impl SessionInfo {
	#[must_use]
	/// Tries to remove the XML notation-like parts from a session's name.
	///
	/// Note that this is imperfect and not using an actual XML parser to remain
	/// lightweight.
	pub fn stripped_name(&self) -> String {
		bad_xml_strip(&self.name)
	}
}
