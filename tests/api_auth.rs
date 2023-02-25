#![cfg(feature = "http_client")]

mod common;

#[tokio::test]
#[ignore]
async fn extend_session() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_auth();

	let extend_session = neos::query::ExtendUserSession;
	client.query(extend_session).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn friends() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_auth();

	let friends_query = neos::query::Friends::default();
	let friends = client.query(friends_query).await?;

	// Neos bot will always be at least one friend of yours
	assert!(!friends.is_empty());

	Ok(())
}

#[tokio::test]
#[ignore]
async fn get_messages() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_auth();

	let messages_query = neos::query::Messages::default();
	let messages = client.query(messages_query).await?;

	//println!("Messages: {:?}", messages);

	// Test user should have at least a single message
	assert!(!messages.is_empty());

	Ok(())
}
