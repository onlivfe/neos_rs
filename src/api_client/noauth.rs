use std::borrow::Borrow;

use crate::UserSession;

use super::Neos;
use super::{inner::NeosApiClient, NeosAuthenticated, RequestError};
use crate::LoginCredentials;
use minreq::Method;

/// Neos API client without authentication
///
/// # Example usage
///
/// ```no_run
/// use neos::api_client::{Neos, NeosUnauthenticated};
/// # let USER_AGENT = String::new();
/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
/// if neos_api_client.ping().is_ok() {
/// 	println!("Neos is reachable :)");
/// } else {
/// 	println!("Couldn't reach neos :(");
/// }
/// ```
#[derive(Clone)]
pub struct NeosUnauthenticated {
	inner: NeosApiClient,
}

impl Neos for NeosUnauthenticated {
	fn api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(minreq::Request) -> Result<minreq::Request, minreq::Error>,
	) -> Result<minreq::Response, RequestError> {
		self.inner.basic_api_request(method, url, build)
	}
}

impl NeosUnauthenticated {
	#[must_use]
	/// Creates a new unauthenticated Neos API client instance
	pub fn new(user_agent: String) -> Self {
		Self { inner: NeosApiClient::new(user_agent) }
	}

	/// Sends a login request to the API.
	pub fn login(
		&self,
		login_credentials: impl Borrow<LoginCredentials>,
	) -> Result<UserSession, RequestError> {
		let res = self.api_request(Method::Post, "userSessions", &mut |req| {
			req.with_json(login_credentials.borrow())
		})?;

		Ok(res.json()?)
	}

	#[must_use]
	/// Upgrades the client to an authenticated version with an user session.
	/// Does not check the user session validity.
	pub fn upgrade(self, user_session: UserSession) -> NeosAuthenticated {
		NeosAuthenticated::from((self.inner, user_session))
	}
}

impl From<NeosApiClient> for NeosUnauthenticated {
	fn from(inner: NeosApiClient) -> Self {
		NeosUnauthenticated { inner }
	}
}
