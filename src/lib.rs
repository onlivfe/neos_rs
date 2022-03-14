#![doc(html_logo_url = "https://git.ljoonal.xyz/ljoonal/neos_rs/raw/logo.png")]
//! Typed models for [Neos's api](https://wiki.neosvr.com/docfx/api) with serde support.
//!
//! Featuring chrono for datetimes and strum for better enums.
//!
//! Actual documentation of the API is lacking, and the API is still changing
//! too. Thusly this crate can't guarantee that it's necessarily fully correct.
//! Some of the types are based solely on educated guesses.
//!
//! Check out <https://wiki.neosvr.com/docfx/api> if you're not using Rust and just want to learn about the API.
//!
//! ## Example usage
//!
//! ```rust
//! extern crate serde_json;
//!
//! // Normally you'd get the data by calling the API
//! let data = r#"{
//!         "username": "ljoonal",
//!         "userID": "U-ljoonal",
//!         "isPresent": true,
//!         "outputDevice": 2
//!      }"#;
//!
//! let session_user: neos::SessionUser = serde_json::from_str(data).unwrap();
//!
//! assert_eq!(session_user.output_device, neos::OutputDevice::Screen);
//! ```

#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Strum macros would cause warnings
#![allow(clippy::use_self)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]

pub mod id;

// The models are split into slightly smaller files in order to avoid a really
// long single file.
mod assets;

// Models that should match up with Neos' CloudX ones.
mod cloudx;

// They are re-exported at the top level though to make importing them easier /
// less confusing.
pub use assets::*;
pub use cloudx::*;

#[cfg(feature = "api_client")]
#[cfg_attr(nightly, doc(cfg(feature = "api_client")))]
pub mod api_client;

#[must_use]
/// Generates a new (not cryptographically safe) pseudorandom string
///
/// The output string's char count is `bytes_count` multiplied by two
pub fn random_ascii_string(bytes_count: u8) -> String {
	// By using nanorand we avoid pulling in really heavy deps.
	use nanorand::Rng;

	const DICT: &[char; 16] =
		&['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];

	let mut bits = [0u8].repeat(bytes_count as usize);

	nanorand::tls_rng().fill(&mut bits);

	let mut string = String::with_capacity(bytes_count as usize * 2);

	for byte in bits {
		let first_char_index = byte & 0xF;
		let second_char_index = byte >> 4;
		string.push(DICT[first_char_index as usize]);
		string.push(DICT[second_char_index as usize]);
	}

	string
}
