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

mod any;
mod auth;
mod noauth;
mod req_models;
pub use any::*;
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
		let resp = self.api_request(Method::Get, "stats/onlineUsers", &mut Ok)?;

		// The API responds in JSON due to our Accept header, so need to go
		// trough a string
		match resp.json::<String>()?.parse::<u32>() {
			Ok(num) => Ok(num),
			Err(err) => Err(RequestError::Deserialization(err.to_string())),
		}
	}
}
