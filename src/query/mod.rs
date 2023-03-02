//! Models of the queries for Neos' API.

use racal::FromApiState;

use crate::model::UserSession;

mod friends;
pub use friends::*;
mod groups;
pub use groups::*;
mod messages;
pub use messages::*;
mod sessions;
pub use sessions::*;
mod stats;
pub use stats::*;
mod testing;
pub use testing::*;
mod user_sessions;
pub use user_sessions::*;
mod users;
pub use users::*;

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to Neos' API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct NoAuthentication {}

impl racal::FromApiState<Self> for NoAuthentication {
	fn from_state(state: &Self) -> &Self { state }
}

impl racal::FromApiState<Authentication> for NoAuthentication {
	fn from_state(_: &Authentication) -> &Self { &NoAuthentication {} }
}

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// With authentication
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Authentication {
	/// The secret authentication token
	pub token: String,
	/// The user that the authentication token is for
	pub user_id: crate::id::User,
}

impl std::fmt::Debug for Authentication {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Authentication")
			.field("token", &"*****")
			.field("user_id", &self.user_id)
			.finish()
	}
}

impl FromApiState<Self> for Authentication {
	fn from_state(state: &Self) -> &Self { state }
}

impl From<&UserSession> for Authentication {
	fn from(user_session: &UserSession) -> Self {
		Self {
			token: user_session.token.clone(),
			user_id: user_session.user_id.clone(),
		}
	}
}
