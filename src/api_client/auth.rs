use super::{inner::NeosApiClient, Neos, NeosUnauthenticated, RequestError};
use minreq::{Method, Request, Response};

/// Neos API client with authentication
///
/// # Example usage
///
/// ```no_run
/// use neos::api_client::{Neos, NeosAuthenticated, NeosUnauthenticated};
/// # let USER_AGENT = String::new();
/// # let user_session_request = todo!();
///
/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
/// let user_session =
/// 	neos_api_client.login(user_session_request).expect("to be able to login to Neos");
/// let neos_api_client = neos_api_client.upgrade(user_session);
///
/// let friends =
/// 	neos_api_client.get_friends().expect("to be able to fetch friends from Neos");
/// println!("Neos friendcount: {} ", friends.len());
/// ```
#[derive(Clone)]
pub struct NeosAuthenticated {
	inner: NeosApiClient,
	user_id: crate::id::User,
	token: String,
}

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
				&("neos ".to_owned() + self.user_id.as_ref() + ":" + &self.token),
			))
		})
	}
}

impl NeosAuthenticated {
	#[must_use]
	/// Creates a new authenticated Neos API client instance. Does not check the
	/// user session validity.
	pub fn new(user_agent: String, session: crate::UserSession) -> Self {
		Self::from((NeosApiClient::new(user_agent), session))
	}

	/// Sends a logout request to the API.
	///
	/// If you just want to get rid of the authentication, consider using
	/// [`downgrade`](Self::downgrade)
	pub fn logout(&self) -> Result<(), RequestError> {
		self.api_request(
			Method::Delete,
			&("userSessions/".to_owned() + self.user_id.as_ref()),
			&mut Ok,
		)?;

		Ok(())
	}

	/// Extends the current user session
	pub fn extend_session(&self) -> Result<(), RequestError> {
		self.api_request(Method::Patch, "userSessions", &mut Ok)?;
		Ok(())
	}

	/// Gets the current session's user's friends.
	pub fn get_friends(&self) -> Result<Vec<crate::Friend>, RequestError> {
		let response = self.api_request(
			Method::Get,
			&("users/".to_owned() + self.user_id.as_ref() + "/friends"),
			&mut Ok,
		)?;

		Ok(response.json()?)
	}

	#[must_use]
	/// Removes the authentication from the API client.
	pub fn downgrade(self) -> NeosUnauthenticated {
		NeosUnauthenticated::from(self.inner)
	}
}

impl From<(NeosApiClient, crate::UserSession)> for NeosAuthenticated {
	fn from((inner, user_session): (NeosApiClient, crate::UserSession)) -> Self {
		NeosAuthenticated {
			inner,
			token: user_session.token,
			user_id: user_session.user_id,
		}
	}
}
