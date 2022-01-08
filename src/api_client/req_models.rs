use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
/// An identifier to use when requesting a session from the Neos API.
///
/// So used when logging in for example, in
/// [`NeosRequestUserSession`](NeosRequestUserSession::identifier).
pub enum NeosRequestUserSessionIdentifier {
	/// Identify using the username
	Username(String),
	#[serde(rename = "ownerID")]
	/// Identify using the UserID
	OwnerID(String),
	/// Identify using an email address
	Email(String),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Data for a user session request to the Neos api.
pub struct NeosRequestUserSession {
	/// The way to identify the user account the request is for
	pub identifier: NeosRequestUserSessionIdentifier,
	/// The password of the account
	pub password: String,
	/// Two factor authenticator code, if such is required
	pub totp: Option<String>,
	/// A secret machine ID for the session.
	///
	/// Note that without this, the request will log out all other sessions if
	/// successful.
	pub secret_machine_id: Option<String>,
	/// If the session should be remembered. Meaning it'll last for a longer
	/// time.
	pub remember_me: bool,
}
