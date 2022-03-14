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
	let friends = common::AUTHENTICATED_API_CLIENT.get_friends(None)?;

	// Neos bot will always be at least one friend of yours
	assert!(!friends.is_empty());

	Ok(())
}

#[test]
#[ignore]
fn get_messages() -> Result<(), neos::api_client::RequestError> {
	let messages =
		common::AUTHENTICATED_API_CLIENT.get_messages(100, false, &None, &None)?;

	//println!("Messages: {:?}", messages);

	// Test user should have at least a single message
	assert!(!messages.is_empty());

	Ok(())
}
