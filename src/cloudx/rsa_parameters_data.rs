use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A Neos users public RSA key pair...for... session authentication?
pub struct RSAParametersData {
	/// The exponent component of the RSA public key
	pub exponent: String,
	/// The modulus component of the RSA public key
	pub modulus: String,
}
