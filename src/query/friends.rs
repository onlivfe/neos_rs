use racal::Queryable;
use time::OffsetDateTime;

use super::Authentication;

/// Get the friends for a specific user
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Friends {
	/// Possibly limit to friends only with newer status updates
	pub last_status_update: Option<OffsetDateTime>,
}

impl Default for Friends {
	/// Creates a new friends query based on the ID
	fn default() -> Self { Self { last_status_update: None } }
}

// TODO: VecSkipError
impl Queryable<Authentication, Vec<crate::model::Friend>> for Friends {
	fn url(&self, auth: &Authentication) -> String {
		let mut query = format!(
			"{}/users/{}/friends",
			crate::API_BASE_URI,
			auth.user_id.as_ref()
		);

		if let Some(last_status_update) = self.last_status_update {
			query = query + "?lastStatusUpdate=" + &last_status_update.to_string();
		}

		query
	}
}

/// Removes a friend
pub struct RemoveFriend {
	/// The user's ID that's being removed
	pub to: crate::id::User,
}

impl RemoveFriend {
	/// Creates a new friend request query based on the ID
	pub fn new(to: impl Into<crate::id::User>) -> Self { Self { to: to.into() } }
}

impl Queryable<Authentication, crate::model::User> for RemoveFriend {
	fn url(&self, auth: &Authentication) -> String {
		format!(
			"{}/users/{}/friends/{}",
			crate::API_BASE_URI,
			auth.user_id.as_ref(),
			self.to.as_ref()
		)
	}

	fn body(&self, auth: &Authentication) -> Option<serde_json::Result<Vec<u8>>> {
		let value = serde_json::json!({
			"ownerId": auth.user_id.as_ref(),
			"friendStatus": crate::model::FriendStatus::Ignored
		});
		Some(serde_json::to_vec(&value))
	}

	fn method(&self, _: &Authentication) -> racal::RequestMethod {
		racal::RequestMethod::Delete
	}
}
