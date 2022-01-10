//! Wrappers for Neos IDs.
//!
//! Wrapping them IDs in newtypes makes sure you aren't trying to accidentally
//! compare different types of Neos IDs with each other like so:
//!
//! ```compile_fail,E0308
//! let user_id = neos::id::User::try_from("U-totally-legit-id".to_string()).unwrap();
//! let record_id = neos::id::Record::try_from("R-totally-legit-id".to_string()).unwrap();
//! assert!(user_id != record_id, "can't compare different types of IDs")
//! ```
//!
//! The deserializers are also made to check that the strings start with the
//! correct ID prefix.
//!
//! Note that the IDs seem to be handled as case-sensitive, so any normalized
//! versions are represented as strings instead of IDs.

use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize};

macro_rules! add_id {
	(
		$(#[$meta:meta])*
		$name:ident,
		$prefix:expr
	) => {
		#[doc = concat!("An ID of a Neos ", stringify!($name), "(`", $prefix, "{id}`)")]
		///
		/// # Example usage
		///
		/// ```
		#[doc = concat!("use neos::id::", stringify!($name), ";")]
		#[doc = concat!("let id1 =", stringify!($name), "::try_from(\"", $prefix, "totally-legit-id\".to_string()).unwrap();")]
		#[doc = concat!("let id2 =", stringify!($name), "::try_from(\"", $prefix, "other-legit-id\".to_string()).unwrap();")]
		/// assert!(id1 != id2);
		/// ```
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

		impl TryFrom<String> for $name {
			type Error = &'static str;
			fn try_from(v: String) -> Result<Self, Self::Error> {
				if !v.starts_with($prefix) {
					return Err(concat!("should start with `", $prefix , "`"));
				}
				Ok($name(v))
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
						$name::try_from(v.to_owned()).map_err(|_| {
							de::Error::invalid_value(
								serde::de::Unexpected::Str(v),
								&concat!("start with `", $prefix , "`"),
							)
						})
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
/// Any of the Neos IDs
///
/// # Example usage
///
/// ```
/// let id1 = neos::id::User::try_from("U-totally-legit-id".to_string()).unwrap();
/// let id1: neos::id::Any = id1.into();
/// let id2 = neos::id::Record::try_from("R-totally-legit-id".to_string()).unwrap();
/// let id2: neos::id::Any = id2.into();
/// assert!(id1 != id2);
/// ```
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
///
/// # Example usage
///
/// ```
/// let id1 = neos::id::User::try_from("U-totally-legit-id".to_string()).unwrap();
/// let id1: neos::id::Owner = id1.into();
/// let id2 = neos::id::Group::try_from("G-totally-legit-id".to_string()).unwrap();
/// let id2: neos::id::Owner = id2.into();
/// assert!(id1 != id2);
/// ```
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