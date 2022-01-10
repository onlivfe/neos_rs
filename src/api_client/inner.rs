use std::{
	sync::{Arc, RwLock},
	time::{Duration, Instant},
};

use super::{RequestError, API_BASE};
use chrono::{DateTime, Local};
use minreq::{Method, Request, Response};

#[derive(Clone)]
#[warn(clippy::module_name_repetitions)]
/// Neos API client internals.
pub struct NeosApiClient {
	/// The user agent to use for the requests
	user_agent: String,
	/// When the last request was send.
	last_request_time: Arc<RwLock<Instant>>,
	/// Arc since we want all the API clients to share the same rate limit if
	/// possible. RwLock to enable modifying it.
	rate_limit_expiration: Arc<RwLock<Instant>>,
}

impl NeosApiClient {
	const MIN_BETWEEN_REQUESTS: Duration = Duration::from_millis(100);
	pub fn new(user_agent: impl Into<String>) -> Self {
		Self {
			user_agent: user_agent.into(),
			last_request_time: Arc::new(RwLock::new(
				Instant::now() - Self::MIN_BETWEEN_REQUESTS,
			)),
			rate_limit_expiration: Arc::new(RwLock::new(Instant::now())),
		}
	}

	pub fn basic_api_request(
		&self,
		method: Method,
		url: &str,
		build: &mut dyn FnMut(Request) -> Result<Request, minreq::Error>,
	) -> Result<Response, RequestError> {
		self.sleep_if_ratelimited();
		self.sleep_between_requests();

		// 2^20 ~=1MB, 2^22~=4MB
		let response = build(
			Request::new(method, &(API_BASE.to_owned() + url))
				.with_header("Accept", "application/json")
				.with_header("Content-Type", "application/json")
				.with_header("User-Agent", &self.user_agent)
				.with_max_redirects(5)
				.with_max_status_line_length(Some(2usize.pow(20)))
				.with_max_headers_size(Some(2usize.pow(22)))
				.with_timeout(120),
		)?
		.send()?;

		self.handle_response(response)
	}

	/// Makes the thread sleep until a certain time has passed between the last
	/// request
	fn sleep_between_requests(&self) {
		let mut waiting_from_last_request = false;

		while waiting_from_last_request {
			let mut last_request_time = self.last_request_time.write().unwrap();
			if Instant::now()
				.checked_duration_since(*last_request_time + Self::MIN_BETWEEN_REQUESTS)
				.is_some()
			{
				*last_request_time = Instant::now();
				waiting_from_last_request = false;
			} else {
				drop(last_request_time);
				std::thread::sleep(Self::MIN_BETWEEN_REQUESTS);
			}
		}
	}

	/// Makes the thread sleep until the ratelimit has expired
	fn sleep_if_ratelimited(&self) {
		// TODO: set a max limit for the sleeping, and/or a request cancel
		// mechanism?
		if let Some(since) = self
			.rate_limit_expiration
			.read()
			.unwrap()
			.checked_duration_since(Instant::now())
		{
			std::thread::sleep(since);
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
				if let Ok(duration) = (rate_limit_resets - Local::now()).to_std() {
					*self.rate_limit_expiration.write().unwrap() =
						Instant::now() + duration;
				}
			} else if let Some(Ok(retry_after)) =
				response.headers.get("Retry-After").map(|time| time.parse::<u64>())
			{
				*self.rate_limit_expiration.write().unwrap() =
					Instant::now() + Duration::from_secs(retry_after);
			} else {
				*self.rate_limit_expiration.write().unwrap() =
					Instant::now() + Duration::from_secs(2);
			}
		};

		if res.status_code == 429 {
			apply_rate_limit(&res);
			return Err(RequestError::ResponseCode((
				res.status_code,
				res.as_str().unwrap_or("").to_owned(),
			)));
		}

		if let Some(Ok(rate_limit_remaining)) =
			res.headers.get("X-Rate-Limit-Remaining").map(|limit| limit.parse::<u32>())
		{
			if rate_limit_remaining == 0 {
				apply_rate_limit(&res);
			}
		}

		if res.status_code < 200 || res.status_code >= 300 {
			return Err(RequestError::ResponseCode((
				res.status_code,
				res.as_str().unwrap_or("").to_owned(),
			)));
		}

		Ok(res)
	}
}
