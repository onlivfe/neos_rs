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
	assert!(common::UNAUTHENTICATED_API_CLIENT.online_user_count()? > 0);

	Ok(())
}

#[test]
#[ignore]
fn online_instance_count() -> Result<(), neos::api_client::RequestError> {
	assert!(common::UNAUTHENTICATED_API_CLIENT.online_instance_count()? > 0);

	Ok(())
}

#[test]
#[ignore]
fn get_user() -> Result<(), neos::api_client::RequestError> {
	let user_id = neos::id::User::try_from("U-Neos".to_string()).unwrap();
	let _user_status = common::UNAUTHENTICATED_API_CLIENT.get_user(user_id)?;

	Ok(())
}

#[test]
#[ignore]
fn get_user_status() -> Result<(), neos::api_client::RequestError> {
	let user_id = neos::id::User::try_from("U-Neos".to_string()).unwrap();
	let _user_status = common::UNAUTHENTICATED_API_CLIENT.get_user_status(user_id)?;

	Ok(())
}

#[test]
#[ignore]
fn sessions() -> Result<(), neos::api_client::RequestError> {
	// Test that listing public sessions work
	let sessions = common::UNAUTHENTICATED_API_CLIENT.get_sessions()?;

	let public_session = sessions
		.iter()
		.find(|session| {
			session.access_level == neos::SessionAccessLevel::Anyone && session.is_valid
		})
		.expect("there should be at least one public session");

	// Test that getting a specific session works.
	let session = common::UNAUTHENTICATED_API_CLIENT
		.get_session(public_session.session_id.clone())?;

	// Some basic sanity checks, can't do full eq since some data might've changed
	assert!(session.session_id == public_session.session_id);
	assert!(session.host_user_id == public_session.host_user_id);
	assert!(session.compatibility_hash == public_session.compatibility_hash);

	Ok(())
}
