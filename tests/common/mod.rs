#![cfg(feature = "api_client")]

use neos::{
	api_client::{NeosAuthenticated, NeosUnauthenticated},
	NeosUserSession,
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

pub static UNAUTHENTICATED_API_CLIENT: Lazy<NeosUnauthenticated> =
	Lazy::new(|| NeosUnauthenticated::new(USER_AGENT.to_string()));

pub static USER_SESSION: Lazy<NeosUserSession> = Lazy::new(|| {
	let user_session: NeosUserSession = serde_json::from_slice(
		&std::fs::read("user-session.json")
			.expect("must have a prepared `user-session.json` file for live API testing"),
	)
	.expect("`user-session.json` file to parse into a user session");

	assert!(user_session.secret_machine_id.is_some());

	user_session
});

pub static AUTHENTICATED_API_CLIENT: Lazy<NeosAuthenticated> =
	Lazy::new(|| UNAUTHENTICATED_API_CLIENT.clone().upgrade(USER_SESSION.clone()));
