//! Models of the queries for Neos' API.

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to Neos' API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct NoAuthentication {}

impl From<&NoAuthentication> for NoAuthentication {
	fn from(_: &NoAuthentication) -> Self {
		Self {}
	}
}

/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// With authentication
pub struct Authentication {
	/// The secret authentication token
	pub token: String,
	/// The user that the authentication token is for
	pub user_id: crate::id::User,
}

impl From<&Authentication> for Authentication {
	fn from(auth: &Authentication) -> Self {
		Self { token: auth.token.clone(), user_id: auth.user_id.clone() }
	}
}

impl From<&UserSession> for Authentication {
	fn from(user_session: &UserSession) -> Self {
		Self { token: user_session.token.clone(), user_id: user_session.user_id.clone() }
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