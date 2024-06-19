use crate::{method::Method, request::Request, response::Response};

pub struct Route {
	pub method: Method,
	patterns: Vec<String>,
	pub endpoint: Box<dyn Endpoint + Send + Sync>,
}

impl Route {
	pub fn new(
		method: Method,
		pattern: &str,
		endpoint: Box<dyn Endpoint + Send + Sync>,
	) -> Self {
		Self {
			method,
			patterns: pattern
				.strip_prefix('/')
				.unwrap()
				.split('/')
				.map(Into::into)
				.collect(),
			endpoint,
		}
	}

	pub fn matches(&self, request: &Request) -> Option<Vec<String>> {
		let mut params = vec![];
		let segments =
			request.path.strip_prefix('/').unwrap().split('/').collect::<Vec<_>>();
		for i in 0..self.patterns.len() {
			if self.patterns[i] == "*" {
				break;
			}
			if i >= segments.len() {
				return None;
			}
			if self.patterns[i] == "%s" {
				params.push(segments[i].to_owned());
			}
			if self.patterns[i] != segments[i] {
				return None;
			}
		}
		if self.patterns.last().is_some_and(|it| it == "*") {
			params.push(segments[self.patterns.len() - 1..].join("/"));
		} else if self.patterns.len() != segments.len() {
			return None;
		}
		Some(params)
	}
}

pub trait Endpoint {
	fn call(&self, request: &Request) -> Response;
}
