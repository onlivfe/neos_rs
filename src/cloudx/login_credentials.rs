use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl LoginCredentials {
	#[must_use]
	/// Creates a new instance of `LoginCredentials` with the identifier,
	/// password and other values set to defaults.
	pub fn new(
		identifier: impl Into<LoginCredentialsIdentifier>,
		password: impl Into<String>,
	) -> Self {
		Self {
			identifier: identifier.into(),
			password: password.into(),
			totp: None,
			secret_machine_id: None,
			remember_me: false,
		}
	}

	#[must_use]
	/// Sets the totp field's value
	pub fn totp(mut self, totp: impl Into<Option<String>>) -> Self {
		self.totp = totp.into();
		self
	}
	#[must_use]
	/// Sets the secret machine ID field's value
	pub fn machine_id(mut self, machine_id: impl Into<Option<String>>) -> Self {
		self.secret_machine_id = machine_id.into();
		self
	}
	#[must_use]
	/// Sets the remember me field's value
	pub fn remember_me(mut self, remember_me: impl Into<bool>) -> Self {
		self.remember_me = remember_me.into();
		self
	}

	#[cfg(feature = "rand_util")]
	#[cfg_attr(nightly, doc(cfg(feature = "rand_util")))]
	#[must_use]
	/// Sets the `machine_id` to a not cryptographically safe generated
	/// pseudorandom value.
	pub fn use_generated_machine_id(mut self) -> Self {
		self.secret_machine_id = Some(crate::util::random_ascii_string(32));
		self
	}
}

impl std::fmt::Debug for LoginCredentials {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("LoginCredentials")
			.field("identifier", &self.identifier)
			.field("password", &"*****")
			.field("totp", &self.totp)
			.field(
				"secret_machine_id",
				match &self.secret_machine_id {
					Some(_) => &"Some(*****)",
					None => &"None",
				},
			)
			.field("remember_me", &self.remember_me)
			.finish()
	}
}

#[allow(clippy::module_name_repetitions)]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
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
