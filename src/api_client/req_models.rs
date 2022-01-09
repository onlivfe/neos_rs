use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum::AsRefStr)]
#[serde(rename_all = "camelCase")]
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

impl NeosRequestUserSessionIdentifier {
	#[must_use]
	/// Gets the inner string
	pub const fn inner(&self) -> &String {
		match self {
			Self::Username(s) | Self::Email(s) | Self::OwnerID(s) => s,
		}
	}

	#[must_use]
	/// Gets the inner string
	pub fn inner_mut(&mut self) -> &mut String {
		match self {
			Self::Username(s) | Self::Email(s) | Self::OwnerID(s) => s,
		}
	}

	#[must_use]
	/// If is username
	pub const fn is_username(&self) -> bool {
		matches!(self, Self::Username(_))
	}

	#[must_use]
	/// If is email based
	pub const fn is_email(&self) -> bool {
		matches!(self, Self::Email(_))
	}

	#[must_use]
	/// If is owner's ID based
	pub const fn is_ownerid(&self) -> bool {
		matches!(self, Self::OwnerID(_))
	}
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
/// Data for a user session request to the Neos api.
pub struct NeosRequestUserSession {
	#[serde(flatten)]
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
