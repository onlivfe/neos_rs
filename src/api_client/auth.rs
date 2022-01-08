use super::{Neos, NeosAuthenticated, NeosUnauthenticated, RequestError};
use crate::NeosFriend;
use minreq::{Method, Request, Response};

impl Neos for NeosAuthenticated {
	fn api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(Request) -> Result<Request, minreq::Error>,
	) -> Result<Response, RequestError> {
		self.inner.basic_api_request(method, url, &mut |req: Request| {
			build(req.with_header(
				"Authorization",
				&("neos ".to_owned() + &self.user_id + ":" + &self.token),
			))
		})
	}
}

impl NeosAuthenticated {
	/// Sends a logout request to the API.
	///
	/// If you just want to get rid of the authentication, consider just
	/// creating a new session.
	pub fn logout(self) -> Result<NeosUnauthenticated, (Self, RequestError)> {
		let res = self.api_request(
			Method::Delete,
			&("userSessions/".to_owned() + &self.user_id),
			&mut Ok,
		);

		match res {
			Ok(_) => Ok(self.remove_authentication()),
			Err(e) => Err((self, e)),
		}
	}

	/// Extends the current user session
	pub fn extend_session(&self) -> Result<(), RequestError> {
		self.api_request(Method::Patch, "userSessions", &mut Ok)?;
		Ok(())
	}

	/// Gets the current session's user's friends.
	pub fn get_friends(&self) -> Result<Vec<NeosFriend>, RequestError> {
		let response = self.api_request(
			Method::Get,
			&("users/".to_owned() + &self.user_id + "/friends"),
			&mut Ok,
		)?;

		Ok(response.json()?)
	}
}
