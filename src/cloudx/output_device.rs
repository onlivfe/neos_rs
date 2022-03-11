use serde::Deserialize;

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	strum::FromRepr,
	strum::Display,
	strum::EnumString,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[repr(u8)]
/// The type of output device that the user is using.
///
/// The API is inconsistent, sometimes representing this as a string and
/// sometimes as a number.
pub enum OutputDevice {
	/// Output device not known
	Unknown = 0,
	/// No visual output, server machine
	/// No visual output, server machine
	Headless = 1,
	/// Desktop
	Screen = 2,
	#[strum(to_string = "VR")]
	/// Virtual Reality
	Vr = 3,
	/// In game camera
	Camera = 4,
}

// Allow the OutputDevice to be either represented as a string or number in
// JSON.
impl<'de> Deserialize<'de> for OutputDevice {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		struct OutputDeviceVisitor;

		impl<'de> serde::de::Visitor<'de> for OutputDeviceVisitor {
			type Value = OutputDevice;

			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("a string or number matching the OutputDevice enum")
			}

			fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				OutputDevice::from_repr(v).ok_or_else(|| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v.into()),
						&"enum u8 repr",
					)
				})
			}

			fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Unsigned(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				u8::try_from(v).map(|v| self.visit_u8(v)).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Signed(v),
						&"enum u8 repr",
					)
				})?
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: serde::de::Error,
			{
				OutputDevice::try_from(v).map_err(|_| {
					serde::de::Error::invalid_value(
						serde::de::Unexpected::Str(v),
						&"enum str repr",
					)
				})
			}
		}

		deserializer.deserialize_any(OutputDeviceVisitor)
	}
}
