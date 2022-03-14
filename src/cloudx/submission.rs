use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A Neos record's submission to a group
pub struct Submission {
	#[serde(flatten)]
	/// The id of the submission
	pub id: String,
	/// The group that this submission is to
	pub owner_id: crate::id::Group,
	/// The id of the record that this submission is for
	pub target_record_id: crate::id::Record,
	/// When the submission was created
	pub submission_time: DateTime<Utc>,
	/// The ID of the user that created the submission
	pub submitted_by_id: crate::id::User,
	/// The name of the submitter
	pub submitted_by_name: String,
	/// If the submission should be featured or not
	pub featured: bool,
	/// The ID of the user that enabled featuring this submission
	pub featured_by_user_id: Option<crate::id::User>,
	#[serde(rename = "featuredTimestamp")]
	/// When featuring this submission was enabled
	pub featured_time: Option<DateTime<Utc>>,
}
