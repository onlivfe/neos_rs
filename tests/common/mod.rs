#![cfg(feature = "http_client")]
// Something's funky with checking if these are used or not.
#![allow(dead_code)]

use neos::{
	api_client::{AuthenticatedNeos, UnauthenticatedNeos},
	model::UserSession,
};
use once_cell::sync::Lazy;

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"-TestRunner/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	")",
);

pub fn api_no_auth() -> UnauthenticatedNeos {
	UnauthenticatedNeos::new(USER_AGENT.to_string()).unwrap()
}

pub static USER_SESSION: Lazy<UserSession> = Lazy::new(|| {
	let user_session: UserSession =
		serde_json::from_slice(&std::fs::read("user-session.json").expect(
			"must have a prepared `user-session.json` file for live API testing",
		))
		.expect("`user-session.json` file to parse into a user session");

	assert!(user_session.secret_machine_id.is_some());

	user_session
});

pub fn api_auth() -> AuthenticatedNeos {
	AuthenticatedNeos::new(USER_AGENT.to_string(), &USER_SESSION.clone()).unwrap()
}
