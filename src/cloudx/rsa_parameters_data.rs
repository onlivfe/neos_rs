use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
/// A Neos users public RSA keypair...for... session authentication?
pub struct RSAParametersData {
	/// The exponent component of the RSA pubkey
	pub exponent: String,
	/// The modulus component of the RSA pubkey
	pub modulus: String,
}
