use racal::Queryable;

use super::NoAuthentication;

/// Pings the API
pub struct Ping;

impl Queryable<NoAuthentication, ()> for Ping {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/testing/ping", crate::API_BASE_URI)
	}

	fn deserialize(&self, _data: &[u8]) -> serde_json::Result<()> { Ok(()) }
}
