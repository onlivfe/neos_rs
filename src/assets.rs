#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// An URI for a Neos asset such as a profile picture.
pub struct AssetUrl {
	/// The last URL part without the file extension
	id: String,
	/// The file extension
	ext: Option<String>,
	is_neosdb: bool,
	/// The URL before the last URL part
	url_prefix: String,
}

impl AssetUrl {
	const URL_PREFIX: &'static str = "https://assets.neos.com/assets/";

	fn from_url(url: impl AsRef<str>, is_neosdb: bool) -> Result<Self, &'static str> {
		// Extract the last / part and put the rest back together
		let mut path_split = url.as_ref().split('/').rev();
		let last_path = path_split.next().ok_or("Couldn't parse url path")?;
		let url_prefix = path_split.rev().collect::<Vec<&str>>().join("/") + "/";

		// Extract the last . part and put the rest back together from the last path
		// part
		let mut ext_split = last_path.split('.').rev();
		let ext_split_last = ext_split.next().ok_or("Couldn't parse url ext")?.to_owned();
		let ext_split_rest = ext_split.rev().collect::<Vec<&str>>().join(".");

		// If there was no ext handle that, map the split result to the id and ext
		let (id, ext) = if ext_split_rest.is_empty() {
			(ext_split_last, None)
		} else {
			(ext_split_rest, Some(ext_split_last))
		};

		Ok(Self { id, ext, is_neosdb, url_prefix })
	}
}

impl TryFrom<&str> for AssetUrl {
	type Error = &'static str;
	fn try_from(url: &str) -> Result<Self, Self::Error> {
		if url.starts_with("neosdb:///") {
			if let Some(split) = url.split_once("neosdb:///") {
				if split.0.is_empty() && !split.1.is_empty() {
					return Self::from_url(Self::URL_PREFIX.to_owned() + split.1, true);
				}
			}
		}

		if url.starts_with("https://") {
			return Self::from_url(url, false);
		}

		Err(concat!("should start with `neosdb:///` `https://`"))
	}
}

impl AssetUrl {
	#[must_use]
	/// Gets the file's name
	pub fn filename(&self) -> String {
		match &self.ext {
			Some(ext) => self.id.clone() + ext,
			None => self.id.clone(),
		}
	}

	#[must_use]
	/// Gets the file's name without the extension
	pub fn id(&self) -> &str {
		&self.id
	}

	#[must_use]
	/// Gets the extension
	pub const fn ext(&self) -> &Option<String> {
		&self.ext
	}
}

impl ToString for AssetUrl {
	/// The `https://` URL needed to retrieve the asset.
	fn to_string(&self) -> String {
		match (self.is_neosdb, &self.ext) {
			(false, Some(ext)) => self.url_prefix.clone() + &self.id + "." + ext,
			_ => self.url_prefix.clone() + &self.id,
		}
	}
}

impl<'de> serde::de::Deserialize<'de> for AssetUrl {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		struct IdVisitor;

		impl<'de> serde::de::Visitor<'de> for IdVisitor {
			type Value = AssetUrl;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str(concat!("an AssetUrl string"))
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				AssetUrl::try_from(v).map_err(|err| {
					serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &err)
				})
			}
		}

		deserializer.deserialize_str(IdVisitor)
	}
}

impl serde::ser::Serialize for AssetUrl {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		let og_url = match (self.is_neosdb, &self.ext) {
			(true, Some(ext)) => "neosdb:///".to_owned() + &self.id + "." + ext,
			(true, None) => "neosdb:///".to_owned() + &self.id,
			(false, Some(ext)) => self.url_prefix.clone() + &self.id + "." + ext,
			(false, None) => self.url_prefix.clone() + &self.id,
		};
		serializer.serialize_str(&og_url)
	}
}
