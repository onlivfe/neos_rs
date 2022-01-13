#[derive(Debug, Clone, serde::Deserialize)]
/// An url for a neos asset such as a profile picture.
pub struct AssetUrl(String);

impl AssetUrl {
	const URL_PREFIX: &'static str =
		"https://cloudxstorage.blob.core.windows.net/assets/";
}

impl TryFrom<&str> for AssetUrl {
	type Error = &'static str;
	fn try_from(url: &str) -> Result<Self, Self::Error> {
		if let Some(split) = url.split_once(Self::URL_PREFIX) {
			if split.0.is_empty() && !split.1.is_empty() {
				return Ok(AssetUrl(split.1.to_owned()));
			}
		}

		if let Some(split) = url.split_once("neosdb:///") {
			if split.0.is_empty() && !split.1.is_empty() {
				return Ok(AssetUrl(split.1.to_owned()));
			}
		}

		Err(concat!("should start with `neosdb:///` `https://cloudxstorage.blob.core.windows.net/assets/`"))
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
		Self::URL_PREFIX.to_owned() + &self.0
	}
}
