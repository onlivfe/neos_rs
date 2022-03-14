use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A Neos record, used for all kinds of storage objects
pub struct Record {
	/// The id of the record
	pub id: crate::id::Record,
	/// The owner of the record
	pub owner_id: crate::id::Owner,
	/// The URI that this record points to
	pub asset_uri: crate::AssetUrl,
	/// The version of the asset, in the global scope
	pub global_version: u32,
	/// The version of the asset, in the local scope
	pub local_version: u32,
	/// Who last modified the record
	pub last_modifying_user_id: crate::id::User,
	/// The machine ID of whoever last modified the record.
	///
	/// Might not always start with `M-` though.
	pub last_modifying_machine_id: String,
	/// The user readable name of the record
	pub name: String,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// The user readable description of the record
	///
	/// Defaults to an empty string if null/none in the API.
	pub description: String,
	/// The type of the record
	pub record_type: String,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// The user readable name of the owner
	///
	/// Defaults to an empty string if null/none in the API.
	pub owner_name: String,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// The tags of the record
	pub tags: Vec<String>,
	#[serde(with = "serde_with::rust::default_on_error")]
	#[serde(default)]
	/// The path to this record
	///
	/// Defaulted to empty string if it doesn't exist.
	pub path: String,
	/// The URI that this record's thumbnail is at
	pub thumbnail_uri: Option<crate::AssetUrl>,
	/// When the record was last modified at
	pub last_modification_time: DateTime<Utc>,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// When the record was created at
	pub creation_time: Option<DateTime<Utc>>,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// When the record was first published at
	pub first_publish_time: Option<DateTime<Utc>>,
	/// If the record is public or not
	pub is_public: bool,
	/// If the record is intended for patrons
	pub is_for_patrons: bool,
	/// If the record should be publicly findable
	pub is_listed: bool,
	/// If the record should be publicly findable
	pub visits: u32,
	/// The rating of the record
	pub rating: f32,
	/// Number for random ordering
	pub random_order: u32,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	/// The record's submissions to groups
	pub submissions: Vec<crate::Submission>,
	#[serde(default)]
	#[serde(with = "serde_with::rust::default_on_error")]
	#[serde(rename = "neosDBmanifest")]
	/// Details about the asset
	pub neos_db_manifest: Vec<crate::NeosDBAsset>,
}
