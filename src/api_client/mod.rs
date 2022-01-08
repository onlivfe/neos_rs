//! An API client for Neos.
//!
//! Currently a minimal blocking implementation.
//!
//! # Example usage
//!
//! ```rust
//!     use neos::api_client::{Neos, NeosUnauthenticated};
//!     let neos_api_client = NeosUnauthenticated::new(
//!         format!("NeosRS/{} (test runner)", env!("CARGO_PKG_VERSION")).to_string()
//!     );
//!     let online_users_count = neos_api_client.online_users_count();
//!     match online_users_count {
//!         Ok(online_users_count) => {
//!             println!("Neos currently has {} online users", online_users_count)
//!         }
//!         Err(err) => {
//!             println!("Couldn't get the online users count: {} ", err)
//!         }
//!     };
//! ```

// Pretty much all API calls can fail with repetitively similar errors,
// documenting them all only adds bloat without much more value.
#![allow(clippy::missing_errors_doc)]

use minreq::{Method, Request, Response};

const API_BASE: &str = "https://www.neosvr-api.com/api/";

mod error;
mod inner;
pub use error::*;

use inner::NeosApiClient;

mod auth;
mod noauth;
mod req_models;
pub use auth::*;
pub use noauth::*;
pub use req_models::*;

/// A Neos API client
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
	fn ping(&self) -> Result<(), RequestError> {
		self.api_request(Method::Get, "testing/ping", &mut Ok)?;
		Ok(())
	}
	/// Gets the amount of users that are online.
	fn online_users_count(&self) -> Result<u32, RequestError> {
		let resp =
			self.api_request(Method::Get, "stats/onlineUsers", &mut Ok)?;

		// The API responds in JSON due to our Accept header, so need to go
		// trough a string
		match resp.json::<String>()?.parse::<u32>() {
			Ok(num) => Ok(num),
			Err(err) => Err(RequestError::Deserialization(err.to_string())),
		}
	}
}

/// Neos API client with authentication
///
/// # Example usage
///
/// ```rust
///     use neos::api_client::{Neos, NeosUnauthenticated};
///     let neos_api_client = NeosUnauthenticated::new(
///         format!("NeosRS/{} (test runner)", env!("CARGO_PKG_VERSION")).to_string()
///     );
///     match neos_api_client.ping() {
///         Ok(_) => println!("Neos' API is reachable!"),
///         Err(err) => println!("Couldn't reach Neos' API because: {}", err)
///     }
/// ```
pub struct NeosUnauthenticated {
	inner: NeosApiClient,
}

// Must be defined here for module visibility reasons :/
impl NeosUnauthenticated {
	// Destructing self isn't available for const fns
	#[allow(clippy::missing_const_for_fn)]
	#[must_use]
	/// Adds authentication to the client without any additional checks, meant
	/// for internal use.
	fn add_authentication(
		self,
		token: String,
		user_id: String,
	) -> NeosAuthenticated {
		NeosAuthenticated { inner: self.inner, token, user_id }
	}
}

/// Neos API client with authentication
///
/// # Example usage
///
/// ```rust
///     use neos::api_client::{Neos, NeosAuthenticated};
///     let neos_api_client: NeosAuthenticated = todo!();
///     let online_users_count = neos_api_client.get_friends();
/// ```
pub struct NeosAuthenticated {
	inner: NeosApiClient,
	/// The secret user session token of the session
	token: String,
	/// The user id the user session is for.
	user_id: String,
}

// Must be defined here for module visibility reasons :/
impl NeosAuthenticated {
	// Destructing self isn't available for const fns
	#[allow(clippy::missing_const_for_fn)]
	#[must_use]
	/// Removes the authentication from the client without any additional
	/// checks, meant for internal use.
	fn remove_authentication(self) -> NeosUnauthenticated {
		NeosUnauthenticated { inner: self.inner }
	}
}

/// Any Neos API client
pub enum AnyNeos {
	/// The API client without authentication
	Unauthenticated(NeosUnauthenticated),
	/// The API client with authentication
	Authenticated(NeosAuthenticated),
}

impl AnyNeos {
	/// If the API client instance is authenticated or not.
	pub const fn is_authenticated(&self) -> bool {
		match self {
			Self::Authenticated(_) => true,
			Self::Unauthenticated(_) => false,
		}
	}
}

/// Use the inner client directly
impl Neos for AnyNeos {
	fn api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(Request) -> Result<Request, minreq::Error>,
	) -> Result<Response, RequestError> {
		match self {
			Self::Authenticated(api) => api.api_request(method, url, build),
			Self::Unauthenticated(api) => api.api_request(method, url, build),
		}
	}
}
