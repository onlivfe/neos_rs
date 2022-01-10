use crate::NeosUserSession;

use super::Neos;
use minreq::Method;

use super::{
	inner::NeosApiClient,
	NeosAuthenticated,
	NeosRequestUserSession,
	RequestError,
};

/// Neos API client with authentication
///
/// # Example usage
///
/// ```rust
/// use neos::api_client::{Neos, NeosUnauthenticated};
/// let neos_api_client = NeosUnauthenticated::new(
/// 	format!("NeosRS/{} (test runner)", env!("CARGO_PKG_VERSION")).to_string(),
/// );
/// match neos_api_client.ping() {
/// 	Ok(_) => println!("Neos' API is reachable!"),
/// 	Err(err) => println!("Couldn't reach Neos' API because: {}", err),
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
		user_session_request: &NeosRequestUserSession,
	) -> Result<NeosUserSession, RequestError> {
		let res = self.api_request(Method::Post, "userSessions", &mut |req| {
			req.with_json(&user_session_request)
		})?;

		Ok(res.json()?)
	}

	#[must_use]
	/// Upgrades the client to an authenticated version with an user session.
	/// Does not check the user session validity.
	pub fn upgrade(self, user_session: NeosUserSession) -> NeosAuthenticated {
		NeosAuthenticated::from((self.inner, user_session))
	}
}

impl From<NeosApiClient> for NeosUnauthenticated {
	fn from(inner: NeosApiClient) -> Self {
		NeosUnauthenticated { inner }
	}
}
