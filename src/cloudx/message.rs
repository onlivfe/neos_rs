use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

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
	#[serde(with = "rfc3339")]
	/// When the message was sent
	pub send_time: OffsetDateTime,
	#[serde(with = "rfc3339")]
	/// When the message was sent
	pub last_update_time: OffsetDateTime,
	#[serde(default)]
	#[serde(with = "crate::util::opt_rfc3339")]
	/// When the message was sent
	pub read_time: Option<OffsetDateTime>,
}

impl Message {
	#[must_use]
	/// Gets the ID recipient's if the owners ID doesn't match it, otherwise the
	/// sender's id is returned
	pub fn non_owner_id(&self) -> &crate::id::User {
		if self.owner_id == self.recipient_id {
			&self.sender_id
		} else {
			&self.recipient_id
		}
	}

	#[cfg(feature = "rand_util")]
	#[cfg_attr(nightly, doc(cfg(feature = "rand_util")))]
	#[must_use]
	/// Creates a new message with a random id and time set to now
	pub fn new(
		content: MessageContents,
		owner_and_sender: crate::id::User,
		recipient: crate::id::User,
	) -> Self {
		let now = OffsetDateTime::now_utc();

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

	#[cfg(feature = "rand_util")]
	#[cfg_attr(nightly, doc(cfg(feature = "rand_util")))]
	#[must_use]
	/// Generates a new pseudorandom ID for a message
	pub fn new_id() -> String {
		"MSG-".to_owned() + &crate::util::random_ascii_string(24)
	}
}

#[serde_with::serde_as]
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, strum::EnumVariantNames)]
#[serde(tag = "messageType", content = "content")]
/// The contents of a message combined with the `MessageType`
pub enum MessageContents {
	/// A normal message
	Text(String),
	/// ???
	Object(#[serde_as(as = "serde_with::json::JsonString")] Box<crate::Record>),
	/// Voice recording
	Sound(#[serde_as(as = "serde_with::json::JsonString")] Box<crate::Record>),
	/// Invite to a session
	SessionInvite(
		#[serde_as(as = "serde_with::json::JsonString")] Box<crate::SessionInfo>,
	),
	/// NCR/KFC related most likely
	CreditTransfer(
		#[serde_as(as = "serde_with::json::JsonString")] crate::CreditTransaction,
	),
	/// Kofi/tipping related..?
	SugarCubes(String),
}
