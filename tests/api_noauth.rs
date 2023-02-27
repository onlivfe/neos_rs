#![cfg(feature = "http_client")]

mod common;

#[tokio::test]
#[ignore]
async fn ping() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	client.query(neos::query::Ping).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn online_user_count() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	assert!(client.query(neos::query::OnlineUserCount).await? > 0);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn online_instance_count() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	assert!(client.query(neos::query::OnlineInstanceCount).await? > 0);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn get_user() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	let user_id = neos::id::User::try_from("U-Neos").unwrap();
	let user_id_query = neos::query::UserInfo::new(user_id);
	let user_from_id = client.query(user_id_query).await?;
	let user_name_query = neos::query::UserInfo::new("Neos");
	let user_from_username = client.query(user_name_query).await?;

	assert_eq!(user_from_id.id, user_from_username.id);
	assert_eq!(user_from_id.username, user_from_username.username);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn get_user_status() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	let user_id = neos::id::User::try_from("U-Neos").unwrap();
	let user_status_query = neos::query::UserStatus::new(user_id);
	let _user_status = client.query(user_status_query).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn search_users() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	let user_search_query = neos::query::UserSearch::new("Neos");
	let users = client.query(user_search_query).await?;

	assert!(!users.is_empty());

	let neos_bot_user = users.iter().find(|user| user.username == "Neos");

	assert!(neos_bot_user.is_some());

	Ok(())
}

#[tokio::test]
#[ignore]
async fn sessions() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	let sessions = client.query(neos::query::Sessions).await?;

	let public_session = sessions
		.iter()
		.find(|session| {
			session.access_level == neos::model::SessionAccessLevel::Anyone
				&& session.is_valid
		})
		.expect("there should be at least one public session");

	// Test that getting a specific session works.
	let session = client
		.query(neos::query::SessionInfo::new(public_session.id.clone()))
		.await?;

	// Some basic sanity checks, can't do full eq since some data might've changed
	assert!(session.id == public_session.id);
	assert!(session.host_id == public_session.host_id);
	assert!(session.compatibility_hash == public_session.compatibility_hash);

	Ok(())
}

#[tokio::test]
#[ignore]
async fn get_group() -> Result<(), neos::api_client::ApiError> {
	let client = common::api_no_auth();

	let group_id = neos::id::Group::try_from("G-Neos").unwrap();
	let group_query = neos::query::GroupInfo::new(group_id);
	let _group = client.query(group_query).await?;

	Ok(())
}
