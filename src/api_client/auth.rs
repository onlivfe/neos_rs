use std::borrow::Borrow;

use super::{inner::NeosApiClient, Neos, NeosUnauthenticated, RequestError};
use chrono::{DateTime, Utc};
use minreq::{Method, Request, Response};

/// Neos API client with authentication
///
/// # Example usage
///
/// ```no_run
/// use neos::api_client::{Neos, NeosAuthenticated, NeosUnauthenticated};
/// # let USER_AGENT = String::new();
/// # let user_session_request: neos::LoginCredentials = todo!();
///
/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
/// let user_session =
/// 	neos_api_client.login(user_session_request).expect("to be able to login to Neos");
/// let neos_api_client = neos_api_client.upgrade(user_session);
///
/// let friends =
/// 	neos_api_client.get_friends(None).expect("to be able to fetch friends from Neos");
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
	pub fn get_friends(
		&self,
		last_status_update: impl Borrow<Option<DateTime<Utc>>>,
	) -> Result<Vec<crate::Friend>, RequestError> {
		self.get_friends_for(&self.user_id, last_status_update)
	}

	/// Get the friends for a specific user
	pub fn get_friends_for(
		&self,
		user: impl Borrow<crate::id::User>,
		last_status_update: impl Borrow<Option<DateTime<Utc>>>,
	) -> Result<Vec<crate::Friend>, RequestError> {
		let response = self.api_request(
			Method::Get,
			&("users/".to_owned() + user.borrow().as_ref() + "/friends"),
			&mut |mut req| {
				if let Some(last_status_update) = last_status_update.borrow() {
					req = req
						.with_param("lastStatusUpdate", last_status_update.to_string());
				}

				Ok(req)
			},
		)?;

		Ok(response.json()?)
	}

	/// Sends a friend request
	pub fn add_friend(
		&self,
		user_id: impl Borrow<crate::id::User>,
	) -> Result<(), RequestError> {
		let response = self.api_request(
			Method::Put,
			&("users/".to_owned()
				+ self.user_id.as_ref()
				+ "/friends/" + user_id.borrow().as_ref()),
			&mut |req| {
				// Body is a partial of Friend
				req.with_json(&serde_json::json!({
					"ownerId": self.user_id.as_ref(),
					"friendStatus": crate::FriendStatus::Accepted
				}))
			},
		)?;

		Ok(response.json()?)
	}

	/// Sends a request to remove a friend
	pub fn remove_friend(
		&self,
		user_id: impl Borrow<crate::id::User>,
	) -> Result<(), RequestError> {
		let response = self.api_request(
			Method::Delete,
			&("users/".to_owned()
				+ self.user_id.as_ref()
				+ "/friends/" + user_id.borrow().as_ref()),
			&mut |req| {
				// Body is a partial of Friend
				req.with_json(&serde_json::json!({
					"ownerId": self.user_id.as_ref(),
					"friendStatus": crate::FriendStatus::Ignored
				}))
			},
		)?;

		Ok(response.json()?)
	}

	/// Sends a message
	pub fn send_message(
		&self,
		message: impl Borrow<crate::Message>,
	) -> Result<(), RequestError> {
		let message = message.borrow();
		let response = self.api_request(
			Method::Delete,
			&("users/".to_owned() + message.recipient_id.as_ref() + "/messages"),
			&mut |req| req.with_json(message),
		)?;

		Ok(response.json()?)
	}

	/// Fetches messages from the API
	pub fn get_messages(
		&self,
		max_amount: u16,
		unread_only: bool,
		from_time: impl Borrow<Option<DateTime<Utc>>>,
		user: impl Borrow<Option<crate::id::User>>,
	) -> Result<Vec<crate::Message>, RequestError> {
		let response = self.api_request(
			Method::Get,
			&("users/".to_owned() + self.user_id.as_ref() + "/messages"),
			&mut |mut req| {
				if let Some(from_time) = from_time.borrow() {
					req = req.with_param("fromTime", from_time.to_string());
				}
				if let Some(user) = user.borrow() {
					req = req.with_param("user", user.as_ref());
				}
				if unread_only {
					req = req.with_param("unread", "true");
				}

				Ok(req.with_param("maxItems", max_amount.to_string()))
			},
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
