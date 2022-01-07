#![doc(html_logo_url = "https://git.ljoonal.xyz/ljoonal/neos_rs/raw/logo.png")]
//! Typed models for [Neos's api](https://wiki.neosvr.com/docfx/api/) with serde deserialization support.
//!
//! Featuring chrono for datetimes and strum for better enums.
//!
//! Actual documentation of the API is lacking, and the API is still changing
//! too. Thusly this crate can't guarantee that it's necessarily fully correct.
//! Some of the types are based solely on educated guesses.
//!
//! Check out <https://github.com/PolyLogiX-Studio/NeosVR-API> if you're not using Rust and just want to learn about the API.
//!
//! ## Future plans
//!
//! - Beter documentation in general
//! - Splitting some linked `Option<T>` fields into their own sub-structs
//! - Better documentation about the API request paths
//! - In the future this crate might also provide an API client
//!
//! ## Examples
//!
//! ```rust
//!     extern crate serde_json;
//!     use neos::{NeosSessionUser, NeosOutputDevice};
//!
//!     // Normally you'd get the data by calling the API
//!     let data = r#"{
//!         "username": "ljoonal",
//!         "userID": "U-ljoonal",
//!         "isPresent": true,
//!         "outputDevice": 2
//!      }"#;
//!
//!     let session_user: NeosSessionUser = serde_json::from_str(data).unwrap();
//!
//!     assert_eq!(session_user.output_device, NeosOutputDevice::Screen);
//! ```

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// Strum macros would cause warnings
#![allow(clippy::use_self)]

// The models are split into slightly smaller files in order to avoid a really
// long single file.
mod auth;
mod records;
mod sessions;
mod users;

// They are re-exported at the top level though to make importing them easier /
// less confusing.
pub use auth::*;
pub use records::*;
pub use sessions::*;
pub use users::*;
