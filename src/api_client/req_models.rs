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

impl NeosRequestUserSession {
	#[must_use]
	/// Creates a user session request with a certain identifier
	pub fn with_identifier(
		identifier: impl Into<NeosRequestUserSessionIdentifier>,
		password: impl Into<String>,
	) -> NeosRequestUserSession {
		Self {
			identifier: identifier.into(),
			password: password.into(),
			totp: None,
			secret_machine_id: None,
			remember_me: false,
		}
	}

	#[must_use]
	/// Creates a user session request with an username.
	pub fn with_username(
		username: impl Into<String>,
		password: impl Into<String>,
	) -> NeosRequestUserSession {
		Self::with_identifier(
			NeosRequestUserSessionIdentifier::Username(username.into()),
			password,
		)
	}

	#[must_use]
	/// Creates a user session request from an email.
	pub fn with_email(
		email: impl Into<String>,
		password: impl Into<String>,
	) -> NeosRequestUserSession {
		Self::with_identifier(
			NeosRequestUserSessionIdentifier::Email(email.into()),
			password,
		)
	}

	#[must_use]
	/// Creates a user session request from the owner's ID.
	pub fn with_owner_id(
		owners_id: impl Into<String>,
		password: impl Into<String>,
	) -> NeosRequestUserSession {
		Self::with_identifier(
			NeosRequestUserSessionIdentifier::OwnerID(owners_id.into()),
			password,
		)
	}

	#[must_use]
	/// Sets the totp field's value
	pub fn totp(mut self, totp: impl Into<Option<String>>) -> Self {
		self.totp = totp.into();
		self
	}

	#[must_use]
	/// Sets the totp field's value
	pub fn machine_id(mut self, machine_id: impl Into<Option<String>>) -> Self {
		self.secret_machine_id = machine_id.into();
		self
	}

	#[must_use]
	/// Sets the totp field's value
	pub fn remember_me(mut self, remember_me: impl Into<bool>) -> Self {
		self.remember_me = remember_me.into();
		self
	}
}
