use std::{fmt::Display, str::FromStr};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub enum Method {
	OPTIONS,
	#[default]
	GET,
	HEAD,
	POST,
	PUT,
	DELETE,
	TRACE,
	CONNECT,
}

impl Display for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::OPTIONS => "OPTIONS",
				Self::GET => "GET",
				Self::HEAD => "HEAD",
				Self::POST => "POST",
				Self::PUT => "PUT",
				Self::DELETE => "DELETE",
				Self::TRACE => "TRACE",
				Self::CONNECT => "CONNECT",
			}
		)
	}
}

#[derive(Debug)]
pub struct InvalidMethod;

impl FromStr for Method {
	type Err = InvalidMethod;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"OPTIONS" => Ok(Self::OPTIONS),
			"GET" => Ok(Self::GET),
			"HEAD" => Ok(Self::HEAD),
			"POST" => Ok(Self::POST),
			"PUT" => Ok(Self::PUT),
			"DELETE" => Ok(Self::DELETE),
			"TRACE" => Ok(Self::TRACE),
			"CONNECT" => Ok(Self::CONNECT),
			_ => Err(InvalidMethod),
		}
	}
}
