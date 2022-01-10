//! Wrappers for Neos IDs.
//!
//! Wrapping them IDs in newtypes makes sure you aren't trying to accidentally
//! compare different types of Neos IDs with each other like so:
//!
//! ```compile_fail,E0308
//!     let user_id = neos::id::User("U-example-only-invalid-id");
//!     let record_id = neos::id::Group("G-example-only-invalid-id");
//!     if user_id == record_id {
//!         println!("Logic error!");
//!     }
//! ```
//!
//! The deserializers are also made to check that the strings start with the
//! correct ID prefix.

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};

macro_rules! add_id {
	(
		$(#[$meta:meta])*
		$name:ident,
		$prefix:expr
	) => {
		#[doc = concat!("An ID of a Neos ", stringify!($name), "(`", $prefix, "{id}`)")]
		#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
		#[serde(transparent)]
		$(#[$meta])*
		pub struct $name(String);

		impl AsRef<str> for $name {
			/// Extracts a string slice containing the entire inner String.
			#[must_use]
			fn as_ref(&self) -> &str {
				&self.0
			}
		}

		impl From<$name> for String {
			fn from(id: $name) -> String {
				id.0
			}
		}

		impl From<$name> for Any {
			fn from(id: $name) -> Any {
				Any::$name(id)
			}
		}

		/// The deserializer will give an error if the inner String doesn't start with the proper prefix.
		/// Also always normalizes strings to lowercase for consistency.
		impl<'de> serde::de::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct IdVisitor;

				impl<'de> Visitor<'de> for IdVisitor {
					type Value = $name;

					fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
						formatter
							.write_str(concat!("a string, ", stringify!($name), "that must start with `", $prefix, "`"))
					}

					fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
					where
						E: de::Error,
					{
						let normalized = v.to_lowercase();
						if !normalized.starts_with($prefix) {
							return Err(
								de::Error::invalid_value(
								serde::de::Unexpected::Str(v),
								&"enum str repr",
							)
							);
						}
						Ok($name(normalized))
					}
				}

				deserializer.deserialize_str(IdVisitor)
			}
		}

	};
}

add_id!(User, "U-");
add_id!(Group, "G-");
add_id!(Session, "S-");
add_id!(Record, "R-");

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
/// Any Neos ID
pub enum Any {
	/// An user ID
	User(User),
	/// A group ID
	Group(Group),
	/// A session ID
	Session(Session),
	/// A record ID
	Record(Record),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
/// Neos IDs that can own records for example
pub enum Owner {
	/// An user ID
	User(User),
	/// A group ID
	Group(Group),
}

impl From<User> for Owner {
	fn from(user: User) -> Self {
		Owner::User(user)
	}
}

impl From<Group> for Owner {
	fn from(group: Group) -> Self {
		Owner::Group(group)
	}
}
