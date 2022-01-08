use minreq::{Method, Request, Response};

use super::{Neos, NeosAuthenticated, NeosUnauthenticated, RequestError};

// Re-exported in super module
#[allow(clippy::module_name_repetitions)]
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

impl AnyNeos {
	/// If the API client instance has been authenticated.
	pub const fn is_authenticated(&self) -> bool {
		match self {
			Self::Authenticated(_) => true,
			Self::Unauthenticated(_) => false,
		}
	}

	/// If the API client instance hasn't been authenticated.
	pub const fn is_unauthenticated(&self) -> bool {
		match self {
			Self::Authenticated(_) => false,
			Self::Unauthenticated(_) => true,
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
