#![cfg(feature = "api_client")]

mod common;

#[test]
#[ignore]
fn extend_session() -> Result<(), neos::api_client::RequestError> {
	common::AUTHENTICATED_API_CLIENT.extend_session()
}

#[test]
#[ignore]
fn friends() -> Result<(), neos::api_client::RequestError> {
	let friends = common::AUTHENTICATED_API_CLIENT.get_friends()?;

	// Neos bot will always be at least one friend of yours
	assert!(!friends.is_empty());

	Ok(())
}
