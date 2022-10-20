use minreq::{Method, Request, Response};

use super::{Neos, NeosAuthenticated, NeosUnauthenticated, RequestError};

// Re-exported in super module
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
/// Any Neos API client
pub enum AnyNeos {
	/// The API client without authentication
	Unauthenticated(NeosUnauthenticated),
	/// The API client with authentication
	Authenticated(NeosAuthenticated),
}

impl From<NeosUnauthenticated> for AnyNeos {
	fn from(api: NeosUnauthenticated) -> Self {
		Self::Unauthenticated(api)
	}
}

impl From<NeosAuthenticated> for AnyNeos {
	fn from(api: NeosAuthenticated) -> Self {
		Self::Authenticated(api)
	}
}

impl From<AnyNeos> for NeosUnauthenticated {
	fn from(api: AnyNeos) -> Self {
		match api {
			AnyNeos::Unauthenticated(v) => v,
			AnyNeos::Authenticated(v) => v.downgrade(),
		}
	}
}

impl AnyNeos {
	#[must_use]
	/// If the API client instance has been authenticated.
	pub const fn is_authenticated(&self) -> bool {
		match self {
			Self::Authenticated(_) => true,
			Self::Unauthenticated(_) => false,
		}
	}

	#[must_use]
	/// If the API client instance hasn't been authenticated.
	pub const fn is_unauthenticated(&self) -> bool {
		match self {
			Self::Authenticated(_) => false,
			Self::Unauthenticated(_) => true,
		}
	}

	#[must_use]
	/// Gets the API client instance if it has been authenticated.
	pub const fn authenticated(&self) -> Option<&NeosAuthenticated> {
		match self {
			Self::Authenticated(api) => Some(api),
			Self::Unauthenticated(_) => None,
		}
	}

	#[must_use]
	/// Gets the API client instance if it has not been authenticated.
	pub const fn unauthenticated(&self) -> Option<&NeosUnauthenticated> {
		match self {
			Self::Authenticated(_) => None,
			Self::Unauthenticated(api) => Some(api),
		}
	}

	#[must_use]
	/// Gets the API client instance as mutable if it has been authenticated.
	pub fn authenticated_mut(&mut self) -> Option<&mut NeosAuthenticated> {
		match self {
			Self::Authenticated(api) => Some(&mut *api),
			Self::Unauthenticated(_) => None,
		}
	}

	#[must_use]
	/// Gets the API client instance as mutable if it has not been
	/// authenticated.
	pub fn unauthenticated_mut(&mut self) -> Option<&mut NeosUnauthenticated> {
		match self {
			Self::Authenticated(_) => None,
			Self::Unauthenticated(api) => Some(api),
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
