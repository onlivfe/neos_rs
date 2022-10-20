// Re-exported in super module.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
/// An error that might happen when communicating with the Neos API
pub enum RequestError {
	/// The response code of the request indicated a failure.
	ResponseCode((i32, String)),
	/// The response data could not be de-serialized.
	Deserialization(String),
	/// An unexpected/unknown/other error happened.
	Other(String),
}

impl std::error::Error for RequestError {}

impl std::fmt::Display for RequestError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::ResponseCode((code, msg)) => {
				write!(f, "RequestError: Status code {} - {}", code, msg)
			}
			Self::Deserialization(err) => {
				write!(f, "RequestError: {}", err)
			}
			Self::Other(err) => write!(f, "RequestError: {}", err),
		}
	}
}

impl From<minreq::Error> for RequestError {
	fn from(err: minreq::Error) -> Self {
		match err {
			minreq::Error::InvalidUtf8InResponse
			| minreq::Error::InvalidUtf8InBody(_)
			| minreq::Error::SerdeJsonError(_) => RequestError::Deserialization(err.to_string()),
			_ => RequestError::Other(err.to_string()),
		}
	}
}
