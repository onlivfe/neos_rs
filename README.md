# Neos API in rust

<img align="right" width="256" height="256" src="https://git.ljoonal.xyz/ljoonal/neos_rs/raw/logo.png"/>

[![License](https://img.shields.io/crates/l/neos.svg)](https://git.ljoonal.xyz/ljoonal/neos_rs/src/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/neos.svg)](https://crates.io/crates/neos)
[![Docs](https://docs.rs/neos/badge.svg)](https://docs.rs/crate/neos/)

Rust models of [NeosVR's API](https://wiki.neosvr.com/docfx/api).

Featuring full serde support, chrono for datetimes, and strum for better enums.

Any official documentation of Neos' API is lacking, and the API is still changing too.
So this crate can't guarantee correctness.
Some of the types are based solely on educated guesses even.

This crate provides a blocking API client with the optional `api_client` feature.

## Future plans

- Better documentation in general
- Splitting some linked `Option<T>` fields into their own sub-structs

## Testing

The integration tests contact the live API.
That's why they are ignored by default.

Some of them also require authentication.

Sadly not all the things can even be reliably tested without creating a mock API.
Which in turn defeats the purpose of the tests in the first place.

### Creating a user session manually

You can generate a `user-sesion.json` file with logging in via curl for example:

```shell
curl --request POST \
  --url https://www.neosvr-api.com/api/userSessions \
  --header 'Content-Type: application/json' \
  --header 'Accept: application/json' \
  --data '{
  "password": "pa$$word",
  "secretMachineId": "string",
  "rememberMe": true,
  "ownerId": "string",
  "email": "user@example.com",
  "username": "string"
}' > user-session.json
```

Only use a single identification method (username/email/ownerId).
Also be sure to replace the rest of the values with your own.
Using a secretMachineId is also recommended to not log out your other sessions.
You can generate a random one for example with: `openssl rand -hex 32`

### Running ignored tests

Make sure that you've got:

- an internet connection
- a valid `user-sesion.json`

Then just run `cargo test --all-features -- --ignored`

## License

Note that the license is `MPL-2.0` instead of the more common `MIT OR Apache-2.0`.
A license change however can be negotiated if the Neos team wants to use this crate or adopt this crate into a more official one with a different license.
