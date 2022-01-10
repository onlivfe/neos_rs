# Neos API in rust

<img align="right" width="256" height="256" src="https://git.ljoonal.xyz/ljoonal/neos_rs/raw/logo.png"/>

[![License](https://img.shields.io/crates/l/neos.svg)](https://git.ljoonal.xyz/ljoonal/neos_rs/src/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/neos.svg)](https://crates.io/crates/neos)
[![Docs](https://docs.rs/neos/badge.svg)](https://docs.rs/crate/neos/)

Rust models of [NeosVR's API](https://wiki.neosvr.com/docfx/api/).

Featuring full serde support, chrono for datetimes, and strum for better enums.

Any official documentation of Neos' API is lacking, and the API is still changing too.
So this crate can't guarantee correctness.
Some of the types are based solely on educated guesses even.

This crate provides a blocking API client with the optional `api_client` feature.

## License

The LICENSE is MPL-2.0.
However a license change can be negotiated if the Neos team wants to use this crate or adopt it into a more official one with a different license.
