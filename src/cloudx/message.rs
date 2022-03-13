use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::SessionInfo;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

#[cfg(feature = "rand_util")]
#[cfg_attr(nightly, doc(cfg(feature = "rand_util")))]
impl Message {
	#[must_use]
	/// Creates a new message with a random id and time set to now
	pub fn new(
		content: MessageContents,
		owner_and_sender: crate::id::User,
		recipient: crate::id::User,
	) -> Self {
		let now = Utc::now();

		Message {
			owner_id: owner_and_sender.clone(),
			sender_id: owner_and_sender,
			recipient_id: recipient,
			content,
			id: Self::new_id(),
			send_time: now,
			last_update_time: now,
			read_time: None,
		}
	}

	#[must_use]
	/// Generares a new pseudorandom ID for a message
	pub fn new_id() -> String {
		use nanorand::Rng;
		let rand = [0u8; 48];
		// Not crypto safe, but good enough for the ID, and fast
		nanorand::tls_rng().fill_bytes(rand);
		"MSG-".to_owned() + &String::from_utf8_lossy(&rand)
	}
}

#[allow(clippy::module_name_repetitions)]
#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, strum::EnumVariantNames)]
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
