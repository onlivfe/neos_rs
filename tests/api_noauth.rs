#![cfg(feature = "api_client")]

mod common;

use neos::api_client::Neos;

#[test]
#[ignore]
fn ping() -> Result<(), neos::api_client::RequestError> {
	common::UNAUTHENTICATED_API_CLIENT.ping()
}

#[test]
#[ignore]
fn online_user_count() -> Result<(), neos::api_client::RequestError> {
	assert!(common::UNAUTHENTICATED_API_CLIENT.online_users_count()? > 0);

	Ok(())
}

#[test]
#[ignore]
fn sessions() -> Result<(), neos::api_client::RequestError> {
	// Test that listing public sessions work
	let sessions = common::UNAUTHENTICATED_API_CLIENT.get_sessions()?;

	// Test that the sessions response isn't just an empty vec
	let first_session = sessions.first().expect("there should be at least one session");

	// Test that getting a specific session works.
	let session = common::UNAUTHENTICATED_API_CLIENT
		.get_session(first_session.session_id.clone())?;

	// Some basic sanity checks, can't do full eq since some data might've changed
	assert!(session.session_id == first_session.session_id);
	assert!(session.host_user_id == first_session.host_user_id);
	assert!(session.compatibility_hash == first_session.compatibility_hash);

	Ok(())
}
