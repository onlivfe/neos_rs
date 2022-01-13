#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// An url for a neos asset such as a profile picture.
pub struct AssetUrl(String);

impl AssetUrl {
	const URL_PREFIX: &'static str =
		"https://cloudxstorage.blob.core.windows.net/assets/";
}

impl TryFrom<&str> for AssetUrl {
	type Error = &'static str;
	fn try_from(url: &str) -> Result<Self, Self::Error> {
		if let Some(split) = url.split_once("neosdb:///") {
			if split.0.is_empty() && !split.1.is_empty() {
				return Ok(AssetUrl(split.1.to_owned()));
			}
		}

		if let Some(split) = url.split_once(Self::URL_PREFIX) {
			if split.0.is_empty() && !split.1.is_empty() {
				return Ok(AssetUrl(split.1.to_owned()));
			}
		}

		Err(concat!(
			"should start with `neosdb:///` `https://cloudxstorage.blob.core.windows.net/assets/`"
		))
	}
}

impl AssetUrl {
	#[must_use]
	/// Gets the filename/"ID" part of this url.
	pub fn filename(&self) -> &str {
		&self.0
	}
}

impl ToString for AssetUrl {
	fn to_string(&self) -> String {
		Self::URL_PREFIX.to_owned() + self.0.split('.').next().unwrap()
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
