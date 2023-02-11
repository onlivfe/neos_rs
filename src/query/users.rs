use racal::Queryable;

use super::NoAuthentication;

/// An user's ID or their username
///
/// Used in [`UserInfo`](neos::query::UserInfo).
#[derive(Clone, PartialEq, Eq, Hash)]
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

/// Gets details of an user by either username or ID
///
/// # Example usage
///
/// ```no_run
/// # use neos::{api_client::{Neos, NeosUnauthenticated}, query::UserSearch};
/// # let USER_AGENT = String::new();
/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
/// let neos_user_search_query = UserSearch::new("Neos")
/// let neos_bot = neos_api_client.query(neos_user_search_query)
/// 	.expect("to be able to get the Neos bot account from Neos");
/// println!("The Neos bot supposedly registered on {}", &neos_bot.registration_time);
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UserInfo {
	/// The ID or username to query information about
	pub user: UserIdOrUsername,
}

impl UserInfo {
	/// Creates a new user info query based on the username or ID
	pub fn new(user: impl Into<UserIdOrUsername>) -> Self {
		Self { user: user.into() }
	}
}

impl Queryable<NoAuthentication, crate::model::User> for UserInfo {
	fn url(&self, _: &NoAuthentication) -> String {
		format!(
			"{}/users/{}?byUsername={}",
			crate::API_BASE_URI,
			self.user.as_ref(),
			&(!self.user.is_id()).to_string()
		)
	}
}

/// Gets details of an user by either username or ID
///
/// # Example usage
///
/// ```no_run
/// # use neos::{api_client::{Neos, NeosUnauthenticated}, query::UserSearch};
/// # let USER_AGENT = String::new();
/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
/// let neos_user_status_query = UserStatus::new(eos::id::User::try_from("U-neos").unwrap())
/// let neos_bot_status = neos_api_client.query(neos_user_search_query)
/// 	.expect("to be able to get the Neos bot account from Neos");
/// println!("Neos bot account is: {}", &neos_bot_status.online_status);
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UserStatus {
	/// The ID of the user whose status the query is for
	pub user_id: crate::id::User,
}

impl UserStatus {
	/// Creates a new user status query
	pub fn new(user_id: impl Into<crate::id::User>) -> Self {
		Self { user_id: user_id.into() }
	}
}

impl Queryable<NoAuthentication, crate::model::UserStatus> for UserStatus {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/users/{}/status", crate::API_BASE_URI, self.user_id.as_ref(),)
	}
}

#[derive(Clone, PartialEq, Eq, Hash)]
/// Searches users by name
pub struct UserSearch {
	/// The name to search for
	pub name: String,
}

impl UserSearch {
	/// Creates a new user search query
	pub fn new(name: impl Into<String>) -> Self {
		Self { name: name.into() }
	}
}

impl Queryable<NoAuthentication, crate::model::Users> for UserSearch {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/users?name={}", crate::API_BASE_URI, self.name)
	}
}
