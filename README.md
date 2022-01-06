# Neos API in rust

<img align="right" width="256" height="256" src="./logo.png"/>

Rust models of [NeosVR's API](https://wiki.neosvr.com/docfx/api/).

Featuring chrono for datetimes and strum for better enums.

Actual documentation of Neos' API is lacking, and the API is still changing too.
So this crate doesn't guarantee correctness.
Some of the types are based solely on educated guesses even.

Also bring your own API client, this crate does not yet provide one.

Though adding an API client as an optional feature is planned.

## Status

Very much work in progress, check the docs for more details.

Currently due to a lack of client and/or access to all endpoints without donating to the patreon, some of the types are missing still.
