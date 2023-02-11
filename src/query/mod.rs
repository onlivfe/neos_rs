//! Models of the queries for Neos' API.

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to Neos' API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct NoAuthentication {}

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// With authentication
pub struct Authentication {
	/// The secret authentication token
	pub token: String,
	/// The user that the authentication token is for
	pub user_id: crate::id::User,
}

impl From<UserSession> for Authentication {
	fn from(user_session: UserSession) -> Self {
		Self { token: user_session.token, user_id: user_session.user_id }
	}
}

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

use crate::model::UserSession;
