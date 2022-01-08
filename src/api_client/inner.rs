use std::sync::RwLock;

use super::{RequestError, API_BASE};
use chrono::{DateTime, Local};
use minreq::{Method, Request, Response};

#[warn(clippy::module_name_repetitions)]
pub struct NeosApiClient {
	user_agent: String,
	rate_limit_expiration: RwLock<Option<chrono::DateTime<chrono::Local>>>,
}

impl NeosApiClient {
	pub fn new(
		user_agent: impl Into<String>,
		rate_limit_expiration: impl Into<Option<chrono::DateTime<chrono::Local>>>,
	) -> Self {
		Self {
			user_agent: user_agent.into(),
			rate_limit_expiration: RwLock::new(rate_limit_expiration.into()),
		}
	}

	pub fn basic_api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(Request) -> Result<Request, minreq::Error>,
	) -> Result<Response, RequestError> {
		self.sleep_if_ratelimited();

		let response = build(
			Request::new(method, &(API_BASE.to_owned() + url))
			.with_header("Accept", "application/json")
			.with_header("Content-Type", "application/json")
			.with_header("User-Agent", &self.user_agent)
			.with_max_redirects(5)
			// ~1MB
			.with_max_status_line_length(2^20)
			// ~8MB
			.with_max_headers_size(2^23)
			.with_timeout(120),
		)?
		.send()?;

		self.handle_response(response)
	}

	/// Makes the thread sleep until the ratelimit has expired
	fn sleep_if_ratelimited(&self) {
		// TODO: set a max limit for the sleeping, and/or a request cancel
		// mechanism?
		if let Some(rate_limited_until) =
			*self.rate_limit_expiration.read().unwrap()
		{
			let millis = u64::try_from(
				rate_limited_until.timestamp_millis()
					- Local::now().timestamp_millis(),
			);

			if let Ok(millis) = millis {
				println!("Neos' API rate limited, sleeping: {}ms", millis);
				std::thread::sleep(std::time::Duration::from_millis(millis));
			}
			*self.rate_limit_expiration.write().unwrap() = None;
		}
	}

	/// Handles updating the ratelimit and figuring out other errors
	fn handle_response(&self, res: Response) -> Result<Response, RequestError> {
		let apply_rate_limit = |response: &Response| {
			if let Some(Ok(rate_limit_resets)) = response
				.headers
				.get("X-Rate-Limit-Reset")
				.map(|time| time.parse::<DateTime<Local>>())
			{
				*self.rate_limit_expiration.write().unwrap() =
					Some(rate_limit_resets);
			} else if let Some(Ok(rate_limit_resets_after)) = response
				.headers
				.get("Retry-After")
				.map(|time| time.parse::<i64>())
			{
				*self.rate_limit_expiration.write().unwrap() = Some(
					Local::now()
						+ chrono::Duration::seconds(rate_limit_resets_after),
				);
			} else {
				*self.rate_limit_expiration.write().unwrap() =
					Some(Local::now() + chrono::Duration::seconds(2));
			}
		};

		if res.status_code == 429 {
			apply_rate_limit(&res);
			return Err(RequestError::ResponseCode(res.status_code));
		}

		if let Some(Ok(rate_limit_remaining)) = res
			.headers
			.get("X-Rate-Limit-Remaining")
			.map(|limit| limit.parse::<u32>())
		{
			if rate_limit_remaining == 0 {
				apply_rate_limit(&res);
			}
		}

		if res.status_code < 200 || res.status_code >= 300 {
			return Err(RequestError::ResponseCode(res.status_code));
		}

		Ok(res)
	}
}
