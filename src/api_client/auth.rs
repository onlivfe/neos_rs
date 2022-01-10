use super::{inner::NeosApiClient, Neos, NeosUnauthenticated, RequestError};
use crate::{NeosFriend, NeosUserSession};
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
/// let user_session = neos_api_client.login(user_session_request).unwrap();
/// let neos_api_client = neos_api_client.upgrade(user_session);
///
/// let friends = neos_api_client.get_friends().unwrap();
/// println!("Neos friendcount: {} ", friends.len());
/// ```
#[derive(Clone)]
pub struct NeosAuthenticated {
	inner: NeosApiClient,
	user_session: NeosUserSession,
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
				&("neos ".to_owned()
					+ self.user_session.user_id.as_ref()
					+ ":" + &self.user_session.token),
			))
		})
	}
}

impl NeosAuthenticated {
	#[must_use]
	/// Creates a new authenticated Neos API client instance. Does not check the
	/// user session validity.
	pub fn new(user_agent: String, session: NeosUserSession) -> Self {
		Self::from((NeosApiClient::new(user_agent), session))
	}

	/// Sends a logout request to the API.
	///
	/// If you just want to get rid of the authentication, consider just
	/// creating a new session.
	pub fn logout(&self) -> Result<(), RequestError> {
		self.api_request(
			Method::Delete,
			&("userSessions/".to_owned() + self.user_session.user_id.as_ref()),
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
	pub fn get_friends(&self) -> Result<Vec<NeosFriend>, RequestError> {
		let response = self.api_request(
			Method::Get,
			&("users/".to_owned() + self.user_session.user_id.as_ref() + "/friends"),
			&mut Ok,
		)?;

		Ok(response.json()?)
	}

	/// Tells the Neos API that the user session is online with the machine id.
	pub fn notify_instance_online(&self) -> Result<(), RequestError> {
		let response =
			self.api_request(Method::Get, "stats/instanceOnline/machineId", &mut Ok)?;

		Ok(response.json()?)
	}

	#[must_use]
	/// Downgrades the client to an unauthenticated version without an user
	/// session.
	pub fn downgrade(self) -> (NeosUnauthenticated, NeosUserSession) {
		(NeosUnauthenticated::from(self.inner), self.user_session)
	}
}

/// Downgrade an authenticated API client to an unauthenticated one, losing the
/// user session.
impl From<(NeosApiClient, NeosUserSession)> for NeosAuthenticated {
	fn from((inner, user_session): (NeosApiClient, NeosUserSession)) -> Self {
		NeosAuthenticated { inner, user_session }
	}
}
