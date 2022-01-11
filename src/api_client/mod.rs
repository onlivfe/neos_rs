//! An API client for Neos.
//!
//! Currently a minimal blocking implementation.
//!
//! # Example usage
//!
//! ```no_run
//! const USER_AGENT: &str = concat!(
//! 	env!("CARGO_PKG_NAME"),
//! 	"/",
//! 	env!("CARGO_PKG_VERSION"),
//! 	" (",
//! 	env!("CARGO_PKG_REPOSITORY"),
//! 	")",
//! );
//!
//! use neos::api_client::{Neos, NeosUnauthenticated};
//! let neos_api_client = NeosUnauthenticated::new(USER_AGENT.to_string());
//! let online_users_count = neos_api_client.online_user_count();
//! match online_users_count {
//! 	Ok(online_users_count) => {
//! 		println!("Neos currently has {} online users", online_users_count);
//! 	}
//! 	Err(err) => {
//! 		println!("Couldn't get the online users count: {} ", err);
//! 	}
//! };
//! ```

// Pretty much all API calls can fail with repetitively similar errors,
// documenting them all only adds bloat without much more value.
#![allow(clippy::missing_errors_doc)]

use minreq::{Method, Request, Response};

const API_BASE: &str = "https://www.neosvr-api.com/api/";

mod error;
mod inner;
pub use error::*;

mod any;
mod auth;
mod noauth;
mod req_models;
pub use any::*;
pub use auth::*;
pub use noauth::*;
pub use req_models::*;

use crate::{NeosGroup, NeosSession, NeosUser, NeosUserStatus};

/// A Neos API client
///
/// # Example usage
///
/// Counts the amount of users that are active in publicly listed sessions
///
/// ```no_run
/// use neos::api_client::{Neos, NeosUnauthenticated};
/// # let USER_AGENT = String::new();
/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
/// let sessions = neos_api_client.get_sessions();
/// match sessions {
/// 	Ok(sessions) => {
/// 		let mut count = 0;
/// 		for session in sessions {
/// 			count += session.active_users;
/// 		}
/// 		println!("{} users focused on public sessions", count);
/// 	}
/// 	Err(err) => {
/// 		println!("Couldn't get the session details: {} ", err);
/// 	}
/// };
/// ```
pub trait Neos {
	#[doc(hidden)]
	// Meant for internal use only.
	fn api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(Request) -> Result<Request, minreq::Error>,
	) -> Result<Response, RequestError>;

	/// Pings the API
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
	fn ping(&self) -> Result<(), RequestError> {
		self.api_request(Method::Get, "testing/ping", &mut Ok)?;
		Ok(())
	}

	/// Gets the amount of users that are online.
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let online_users_count = neos_api_client.online_user_count();
	/// match online_users_count {
	/// 	Ok(online_users_count) => {
	/// 		println!("Neos currently has {} online users", online_users_count);
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the online users count: {} ", err);
	/// 	}
	/// };
	/// ```
	fn online_user_count(&self) -> Result<u32, RequestError> {
		let resp = self.api_request(Method::Get, "stats/onlineUsers", &mut Ok)?;

		// The API responds in JSON due to our Accept header, so need to go
		// trough a string
		match resp.json::<&str>()?.parse::<u32>() {
			Ok(num) => Ok(num),
			Err(err) => Err(RequestError::Deserialization(err.to_string())),
		}
	}

	/// Gets the amount of online instances
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let online_instance_count = neos_api_client.online_instance_count();
	/// match online_instance_count {
	/// 	Ok(online_instance_count) => {
	/// 		println!("Neos currently has {} online instances", online_instance_count);
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the online instance count: {} ", err);
	/// 	}
	/// };
	/// ```
	fn online_instance_count(&self) -> Result<u32, RequestError> {
		let resp = self.api_request(Method::Get, "stats/onlineInstances", &mut Ok)?;

		// The API responds in JSON due to our Accept header, so need to go
		// trough a string
		match resp.json::<&str>()?.parse::<u32>() {
			Ok(num) => Ok(num),
			Err(err) => Err(RequestError::Deserialization(err.to_string())),
		}
	}

	/// Gets details of publicly listed sessions.
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let sessions = neos_api_client.get_sessions();
	/// match sessions {
	/// 	Ok(sessions) => {
	/// 		println!("{} public sessions", sessions.len());
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the session details: {} ", err);
	/// 	}
	/// };
	/// ```
	fn get_sessions(&self) -> Result<Vec<NeosSession>, RequestError> {
		let resp = self.api_request(Method::Get, "sessions", &mut Ok)?;

		Ok(resp.json()?)
	}

	/// Gets the an user
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let neos_bot = neos_api_client.get_user("Neos");
	/// match neos_bot {
	/// 	Ok(neos_bot) => {
	/// 		println!("Fetched the account of {}", &neos_bot.username);
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the session details: {} ", err);
	/// 	}
	/// };
	/// ```
	fn get_user(
		&self,
		user: impl Into<UserIdOrUsername>,
	) -> Result<NeosUser, RequestError> {
		let user = user.into();
		let resp = self.api_request(
			Method::Get,
			&("users/".to_owned()
				+ user.as_ref() + "?byUsername="
				+ &(!user.is_id()).to_string()),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}

	/// Searches users by name
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let matching_users = neos_api_client.search_users("Neos").unwrap();
	/// let neos_bot = matching_users.iter().find(|user| user.username == "Neos").unwrap();
	/// println!("Fetched the account of {}", &neos_bot.username);
	/// ```
	fn search_users(&self, name: &str) -> Result<Vec<NeosUser>, RequestError> {
		let resp =
			self.api_request(Method::Get, &("users?name=".to_owned() + name), &mut Ok)?;

		Ok(resp.json()?)
	}

	/// Gets the status of a user
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let neos_bot_status = neos_api_client
	/// 	.get_user_status(neos::id::User::try_from("U-neos".to_string()).unwrap());
	/// match neos_bot_status {
	/// 	Ok(neos_bot_status) => {
	/// 		println!("Neos bot account is: {}", &neos_bot_status.online_status);
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the session details: {} ", err);
	/// 	}
	/// };
	/// ```
	fn get_user_status(
		&self,
		user_id: crate::id::User,
	) -> Result<NeosUserStatus, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("users/".to_owned() + user_id.as_ref() + "/status"),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}

	/// Gets details of a session.
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let session = neos_api_client.get_session(
	/// 	neos::id::Session::try_from("S-totally-legit-id".to_string()).unwrap(),
	/// );
	/// match session {
	/// 	Ok(session) => {
	/// 		println!("Session has {} active_users users", &session.active_users);
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the session details: {} ", err);
	/// 	}
	/// };
	/// ```
	fn get_session(
		&self,
		session_id: crate::id::Session,
	) -> Result<NeosSession, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("sessions/".to_owned() + session_id.as_ref()),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}

	/// Gets details of a group.
	///
	/// # Example usage
	///
	/// ```no_run
	/// use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let group = neos_api_client
	/// 	.get_group(neos::id::Group::try_from("G-totally-legit-id".to_string()).unwrap());
	/// match group {
	/// 	Ok(group) => {
	/// 		println!(
	/// 			"The admin of the group {} is {}",
	/// 			&group.name,
	/// 			group.admin_user_id.as_ref()
	/// 		);
	/// 	}
	/// 	Err(err) => {
	/// 		println!("Couldn't get the session details: {} ", err);
	/// 	}
	/// };
	/// ```
	fn get_group(&self, group_id: crate::id::Group) -> Result<NeosGroup, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("groups/".to_owned() + group_id.as_ref()),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}
}
