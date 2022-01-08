use super::Neos;
use minreq::Method;

use super::{
	inner::NeosApiClient,
	NeosAuthenticated,
	NeosRequestUserSession,
	NeosUnauthenticated,
	RequestError,
};

impl Neos for NeosUnauthenticated {
	fn api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(
			minreq::Request,
		) -> Result<minreq::Request, minreq::Error>,
	) -> Result<minreq::Response, RequestError> {
		self.inner.basic_api_request(method, url, build)
	}
}

impl NeosUnauthenticated {
	#[must_use]
	/// Creates a new Unauthenticated Neos API client instance
	pub fn new(user_agent: String) -> Self {
		Self { inner: NeosApiClient::new(user_agent, None) }
	}

	/// Sends a login request to the API.
	pub fn login(
		self,
		user_session_request: &NeosRequestUserSession,
	) -> Result<NeosAuthenticated, (Self, RequestError)> {
		let res = self.api_request(Method::Post, "userSessions", &mut |req| {
			req.with_json(&user_session_request)
		});

		let res = match res {
			Ok(v) => v,
			Err(e) => return Err((self, e)),
		};

		let res: crate::NeosUserSession = match res.json() {
			Ok(v) => v,
			Err(e) => {
				return Err((self, e.into()));
			}
		};

		Ok(self.add_authentication(res.token, res.user_id))
	}

	/// Extends an old user session to check that it works and returns the
	/// authenticated API client on success.
	pub fn extend_session(
		self,
		token: String,
		user_id: String,
	) -> Result<NeosAuthenticated, (Self, RequestError)> {
		let api = self.add_authentication(token, user_id);

		match api.extend_session() {
			Ok(_) => Ok(api),
			Err(err) => Err((api.remove_authentication(), err)),
		}
	}
}
