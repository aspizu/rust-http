use std::{
	collections::HashMap,
	io::{self, Write},
	net::TcpStream,
};

use crate::{cookies::Cookie, headers::Headers, status::Status};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Response {
	body: Vec<u8>,
	status_code: Status,
	headers: Headers,
	cookies: HashMap<String, Cookie>,
}

impl Response {
	pub fn new(status_code: Status, body: Vec<u8>) -> Self {
		let mut headers = Headers::new();
		headers.insert("content-length".to_string(), body.len().to_string());
		Self { body, status_code, headers, ..Default::default() }
	}

	pub fn with_header(mut self, name: &str, value: &str) -> Self {
		self.headers.insert(name.to_string(), value.to_string());
		self
	}

	pub fn with_cookie(mut self, name: &str, cookie: Cookie) -> Self {
		self.cookies.insert(name.to_string(), cookie);
		self
	}

	pub fn send(&self, buffer: &mut TcpStream) -> io::Result<()> {
		write!(
			buffer,
			"HTTP/1.1 {} {}\r\n",
			self.status_code as u16,
			self.status_code.reason_phrase()
		)?;
		for (name, value) in &self.headers {
			write!(buffer, "{}: {}\r\n", name, value)?;
		}
		for (name, cookie) in &self.cookies {
			write!(buffer, "Set-Cookie: {}={}\r\n", name, cookie)?;
		}
		write!(buffer, "\r\n")?;
		buffer.write_all(&self.body)?;
		Ok(())
	}
}
