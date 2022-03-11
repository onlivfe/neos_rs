/// An user's ID or their username
///
/// Used in [`Neos::search_users`](super::Neos::search_users).
pub enum UserIdOrUsername {
	/// An user's ID
	Id(crate::id::User),
	/// An user's username
	Username(String),
}

impl UserIdOrUsername {
	#[must_use]
	/// If it's an ID
	pub const fn is_id(&self) -> bool {
		matches!(self, Self::Id(_))
	}

	#[must_use]
	/// If it's an username
	pub const fn is_username(&self) -> bool {
		matches!(self, Self::Username(_))
	}
}

impl AsRef<str> for UserIdOrUsername {
	fn as_ref(&self) -> &str {
		match self {
			Self::Id(v) => v.as_ref(),
			Self::Username(v) => v,
		}
	}
}

/// For easier scripting, should use String otherwise.
impl From<&'static str> for UserIdOrUsername {
	fn from(v: &'static str) -> Self {
		Self::Username(v.to_owned())
	}
}

impl From<String> for UserIdOrUsername {
	fn from(v: String) -> Self {
		Self::Username(v)
	}
}

impl From<crate::id::User> for UserIdOrUsername {
	fn from(v: crate::id::User) -> Self {
		Self::Id(v)
	}
}
