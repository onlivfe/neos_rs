use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Data for a user session request to the Neos api.
#[cfg_attr(feature = "api_client", doc = "")]
#[cfg_attr(
	feature = "api_client",
	doc = "Used in
	[`NeosUnauthenticated::login`](crate::api_client::NeosUnauthenticated::login)"
)]
pub struct LoginCredentials {
	#[serde(flatten)]
	/// The way to identify the user account the request is for
	pub identifier: LoginCredentialsIdentifier,
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

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize, strum::AsRefStr)]
#[serde(rename_all = "camelCase")]
/// An identifier to use when requesting a session from the Neos API.
///
/// Used when logging in for example in
/// [`LoginCredentials`](LoginCredentials::identifier).
pub enum LoginCredentialsIdentifier {
	/// Identify using the username
	Username(String),
	#[serde(rename = "ownerID")]
	/// Identify using the UserID
	OwnerID(String),
	/// Identify using an email address
	Email(String),
}

impl LoginCredentialsIdentifier {
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
