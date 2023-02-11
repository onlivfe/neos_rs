use racal::Queryable;

use super::NoAuthentication;

/// Gets the amount of users that are online
pub struct OnlineUserCount;

impl Queryable<NoAuthentication, u32> for OnlineUserCount {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/stats/onlineUsers", crate::API_BASE_URI)
	}
}

/// Gets the amount of online instances
pub struct OnlineInstances;

impl Queryable<NoAuthentication, u32> for OnlineInstances {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/stats/onlineInstances", crate::API_BASE_URI)
	}
}
