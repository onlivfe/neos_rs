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

use std::borrow::Borrow;

use minreq::{Method, Request, Response};

const API_BASE: &str = "https://api.neos.com/api/";

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

/// A Neos API client
///
/// # Example usage
///
/// Counts the amount of users that are active in publicly listed sessions
///
/// ```no_run
/// # use neos::api_client::{Neos, NeosUnauthenticated};
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
	/// Meant for internal use only.
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
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
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

	/// Gets the amount of users that are online
	///
	/// # Example usage
	///
	/// ```no_run
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let online_users_count = neos_api_client
	/// 	.online_user_count()
	/// 	.expect("to be able to get the online user count from Neos");
	/// println!("Neos currently has {} online users", online_users_count);
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
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let online_instance_count = neos_api_client
	/// 	.online_instance_count()
	/// 	.expect("to be able to get the online instance count from Neos");
	/// println!("Neos currently has {} online instances", online_instance_count);
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

	/// Gets details of publicly listed sessions
	///
	/// # Example usage
	///
	/// ```no_run
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let sessions = neos_api_client
	/// 	.get_sessions()
	/// 	.expect("to be able to get publicly visible sessions from Neos");
	/// for session in sessions {
	/// 	if session.active_users > 0 {
	/// 		println!(
	/// 			"Session {} has {} active users",
	/// 			session.name, session.active_users
	/// 		);
	/// 	}
	/// }
	/// ```
	fn get_sessions(&self) -> Result<Vec<crate::SessionInfo>, RequestError> {
		let resp = self.api_request(Method::Get, "sessions", &mut Ok)?;

		Ok(resp.json()?)
	}

	/// Gets details of an user by either username or ID
	///
	/// # Example usage
	///
	/// ```no_run
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let neos_bot = neos_api_client
	/// 	.get_user("Neos")
	/// 	.expect("to be able to get the Neos bot account from Neos");
	/// println!("The Neos bot supposedly registered on {}", &neos_bot.registration_time);
	/// ```
	fn get_user(
		&self,
		user: impl Into<UserIdOrUsername>,
	) -> Result<crate::User, RequestError> {
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
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let matching_users = neos_api_client
	/// 	.search_users("Neos")
	/// 	.expect("to be able to get search for the Neos bot account from Neos");
	/// let neos_bot = matching_users
	/// 	.iter()
	/// 	.find(|user| user.username == "Neos")
	/// 	.expect("for the search results to contain the Neos bot account");
	/// println!("Fetched the account of {}", &neos_bot.username);
	/// ```
	fn search_users(
		&self,
		name: impl Borrow<str>,
	) -> Result<Vec<crate::User>, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("users?name=".to_owned() + name.borrow()),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}

	/// Gets the status of an user
	///
	/// # Example usage
	///
	/// ```no_run
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let neos_bot_status = neos_api_client
	/// 	.get_user_status(neos::id::User::try_from("U-neos").unwrap())
	/// 	.expect("to be able to get the Neos bot account from Neos");
	/// println!("Neos bot account is: {}", &neos_bot_status.online_status);
	/// ```
	fn get_user_status(
		&self,
		user_id: impl Borrow<crate::id::User>,
	) -> Result<crate::UserStatus, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("users/".to_owned() + user_id.borrow().as_ref() + "/status"),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}

	/// Gets details of a session
	///
	/// # Example usage
	///
	/// ```no_run
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let session = neos_api_client
	/// 	.get_session(neos::id::Session::try_from("S-totally-legit-id").unwrap())
	/// 	.expect("to be able to get a session from the invalid id from Neos... yeh no");
	/// println!("Session has {} active_users users", &session.active_users);
	/// ```
	fn get_session(
		&self,
		session_id: impl Borrow<crate::id::Session>,
	) -> Result<crate::SessionInfo, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("sessions/".to_owned() + session_id.borrow().as_ref()),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}

	/// Gets details of a group
	///
	/// # Example usage
	///
	/// ```no_run
	/// # use neos::api_client::{Neos, NeosUnauthenticated};
	/// # let USER_AGENT = String::new();
	/// # let neos_api_client = NeosUnauthenticated::new(USER_AGENT);
	/// let group = neos_api_client
	/// 	.get_group(neos::id::Group::try_from("G-Neos").unwrap())
	/// 	.expect("to be able to get the Neos group details from Neos");
	/// println!("The admin of the group {} is {}", &group.name, group.admin_id.as_ref());
	/// ```
	fn get_group(
		&self,
		group_id: impl Borrow<crate::id::Group>,
	) -> Result<crate::Group, RequestError> {
		let resp = self.api_request(
			Method::Get,
			&("groups/".to_owned() + group_id.borrow().as_ref()),
			&mut Ok,
		)?;

		Ok(resp.json()?)
	}
}
