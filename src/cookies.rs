use std::{
	collections::HashMap,
	fmt::{self, Display, Formatter, Write},
};

pub fn parse_cookie(cookie: &str) -> HashMap<String, String> {
	let mut cookies: HashMap<String, String> = Default::default();
	for chunk in cookie.split(';') {
		let (key, value) = chunk.split_once('=').unwrap_or(("", chunk));
		let (key, value) = (key.trim(), value.trim());
		if !(key.is_empty() && value.is_empty()) {
			if let Some(value) = unquote(value) {
				cookies.insert(key.to_owned(), value);
			}
		}
	}
	cookies
}

pub fn unquote(str: &str) -> Option<String> {
	if !(str.len() > 2 && str.starts_with('"') && str.ends_with('"')) {
		return Some(str.to_owned());
	}
	let mut state = 0;
	let mut out = String::with_capacity(3);
	let mut oct = String::with_capacity(str.len() - 2);
	for ch in str.strip_prefix('"').unwrap().strip_suffix('"').unwrap().chars() {
		match ch {
			'\\' => match state {
				0 => state = 1,
				1 => {
					out.push('\\');
					state = 0;
				}
				_ => unreachable!(),
			},
			_ => match state {
				0 => out.push(ch),
				1 => match ch {
					'0'..='3' => {
						oct.push(ch);
						state = 2;
					}
					_ => {
						out.push(ch);
						state = 0;
					}
				},
				2 | 3 => match ch {
					'0'..='7' => {
						oct.push(ch);
						state += 1;
						if state == 4 {
							let Ok(octch) = u8::from_str_radix(&oct, 8) else {
								return None;
							};
							out.push(octch as char);
							state = 0;
						}
					}
					_ => {
						return None;
					}
				},
				_ => unreachable!(),
			},
		}
	}
	Some(out)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SameSite {
	Strict,
	Lax,
	None,
}

impl Display for SameSite {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Strict => f.write_str("Strict"),
			Self::Lax => f.write_str("Lax"),
			Self::None => f.write_str("None"),
		}
	}
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Cookie {
	pub value: String,
	pub max_age: Option<u64>,
	pub expires: Option<String>,
	pub path: Option<String>,
	pub domain: Option<String>,
	pub secure: bool,
	pub http_only: bool,
	pub same_site: Option<SameSite>,
}

impl Cookie {
	pub fn new(value: String) -> Self {
		Self { value, ..Default::default() }
	}

	pub fn with_max_age(mut self, max_age: u64) -> Self {
		self.max_age = Some(max_age);
		self
	}

	pub fn with_expires(mut self, expires: String) -> Self {
		self.expires = Some(expires);
		self
	}

	pub fn with_path(mut self, path: String) -> Self {
		self.path = Some(path);
		self
	}

	pub fn with_domain(mut self, domain: String) -> Self {
		self.domain = Some(domain);
		self
	}

	pub fn with_secure(mut self, secure: bool) -> Self {
		self.secure = secure;
		self
	}

	pub fn with_http_only(mut self, http_only: bool) -> Self {
		self.http_only = http_only;
		self
	}

	pub fn with_same_site(mut self, same_site: SameSite) -> Self {
		self.same_site = Some(same_site);
		self
	}
}

impl Display for Cookie {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("\"")?;
		for ch in self.value.chars() {
			match ch {
				'\\' => f.write_str("\\\\")?,
				'"' => f.write_str("\\\"")?,
				| 'a'..='z'
				| 'A'..='Z'
				| '0'..='9'
				| ' '
				| '('
				| ')'
				| '/'
				| '<'
				| '='
				| '>'
				| '?'
				| '@'
				| '['
				| ']'
				| '{'
				| '}'
				| '!'
				| '#'
				| '$'
				| '%'
				| '&'
				| '\''
				| '*'
				| '+'
				| '-'
				| '.'
				| '^'
				| '_'
				| '`'
				| '|'
				| '~'
				| ':' => f.write_char(ch)?,
				_ => {
					write!(f, "\\{:o}", ch as u8)?;
				}
			}
		}
		f.write_str("\"")?;
		if let Some(max_age) = self.max_age {
			write!(f, "; Max-Age={}", max_age)?;
		}
		if let Some(expires) = &self.expires {
			write!(f, "; Expires={}", expires)?;
		}
		if let Some(path) = &self.path {
			write!(f, "; Path={}", path)?;
		}
		if let Some(domain) = &self.domain {
			write!(f, "; Domain={}", domain)?;
		}
		if self.secure {
			f.write_str("; Secure")?;
		}
		if self.http_only {
			f.write_str("; HttpOnly")?;
		}
		if let Some(same_site) = &self.same_site {
			write!(f, "; SameSite={}", same_site)?;
		}
		Ok(())
	}
}
