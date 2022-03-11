use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};

use crate::SessionInfo;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Short description of a session's user.
pub struct Message {
	/// An UUID prefixed with `MSG-`
	pub id: String,
	/// The owner, so most likely the logged in user
	pub owner_id: crate::id::User,
	/// The sender of the message
	pub sender_id: crate::id::User,
	/// If the user is focused on this session
	pub recipient_id: crate::id::User,
	/// The contents of the message
	#[serde(flatten)]
	pub content: MessageContents,
	/// When the message was sent
	pub send_time: DateTime<Utc>,
	/// When the message was sent
	pub last_update_time: DateTime<Utc>,
	/// When the message was sent
	pub read_time: Option<DateTime<Utc>>,
}

#[allow(clippy::module_name_repetitions)]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "messageType", content = "content")]
/// The contents of a message combined with the `MessageType`
pub enum MessageContents {
	/// A normal message
	Text(String),
	/// ???
	Object(String),
	/// Voice recording
	Sound(String),
	/// Invite to a session
	SessionInvite(Box<SessionInfo>),
	/// NCR/KFC related most likely
	#[serde(with = "serde_with::json::nested")]
	CreditTransfer(crate::CreditTransaction),
	/// Kofi/tipping related..?
	SugarCubes(String),
}
