use std::{
	collections::HashMap,
	io::{self, Read},
	str::{FromStr, Utf8Error},
};

use crate::{
	cookies::parse_cookie, headers::Headers, method::Method, query::parse_query,
};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Request {
	pub method: Method,
	pub path: String,
	pub headers: Headers,
	pub body: Vec<u8>,
	pub query: HashMap<String, String>,
	pub params: Vec<String>,
	pub cookies: HashMap<String, String>,
}

#[derive(Debug)]
pub enum RequestParseError {
	InvalidMethod,
	InvalidVersion,
	Utf8Error(Utf8Error),
	InvalidEncoding,
	IOError(io::Error),
}

impl Request {
	pub fn parse<T>(stream: &mut T) -> Result<Request, RequestParseError>
	where T: Read {
		let mut buf: Vec<u8> = vec![];
		let mut byte = [0; 1];
		loop {
			stream.read_exact(&mut byte).map_err(RequestParseError::IOError)?;
			if byte[0] == b' ' {
				break;
			}
			buf.push(byte[0]);
		}
		let method = Method::from_str(
			std::str::from_utf8(&buf).map_err(|_| RequestParseError::InvalidMethod)?,
		)
		.map_err(|_| RequestParseError::InvalidMethod)?;
		buf.clear();
		loop {
			stream.read_exact(&mut byte).map_err(RequestParseError::IOError)?;
			if byte[0] == b'\r' {
				break;
			}
			buf.push(byte[0]);
		}
		stream.read_exact(&mut byte).map_err(RequestParseError::IOError)?;
		if byte[0] != b'\n' {
			return Err(RequestParseError::InvalidEncoding);
		}
		let path = std::str::from_utf8(
			buf.strip_suffix(b" HTTP/1.1").ok_or(RequestParseError::InvalidVersion)?,
		)
		.map_err(RequestParseError::Utf8Error)?
		.to_owned();
		let mut headers = Headers::new();
		loop {
			buf.clear();
			loop {
				stream.read_exact(&mut byte).map_err(RequestParseError::IOError)?;
				if byte[0] == b'\r' {
					break;
				}
				buf.push(byte[0]);
			}
			stream.read_exact(&mut byte).map_err(RequestParseError::IOError)?;
			if byte[0] != b'\n' {
				return Err(RequestParseError::InvalidEncoding);
			}
			if buf.is_empty() {
				break;
			}
			let mut iter = buf.splitn(2, |&byte| byte == b':');
			let name = iter
				.next()
				.ok_or(RequestParseError::InvalidEncoding)?
				.iter()
				.map(|&byte| byte as char)
				.collect::<String>();
			let value = iter
				.next()
				.ok_or(RequestParseError::InvalidEncoding)?
				.iter()
				.skip(1)
				.map(|&byte| byte as char)
				.collect::<String>();
			headers.insert(name.to_lowercase(), value);
		}
		let body = if let Some(content_length) = headers.get("content-length") {
			let content_length = content_length
				.parse::<usize>()
				.map_err(|_| RequestParseError::InvalidEncoding)?;
			let mut body = vec![0; content_length];
			stream
				.read_exact(&mut body)
				.map_err(|_| RequestParseError::InvalidEncoding)?;
			body
		} else {
			vec![]
		};
		let (path, query) = parse_query(&path);
		let cookies =
			headers.get("cookie").map(|it| parse_cookie(it)).unwrap_or_default();
		Ok(Request {
			method,
			path,
			headers,
			body,
			query,
			cookies,
			..Default::default()
		})
	}
}
