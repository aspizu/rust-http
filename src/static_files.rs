use crate::{request::Request, response::Response, route::Endpoint, status::Status};

pub struct StaticFiles {
	path: String,
}

impl StaticFiles {
	pub fn new(path: &str) -> Self {
		Self { path: path.to_owned() }
	}
}

impl Endpoint for StaticFiles {
	fn call(&self, request: &Request) -> Response {
		let path = &request.params[0];
		if path.is_empty() {
			match std::fs::read(format!("{}/index.html", self.path)) {
				Ok(content) => {
					return Response::new(Status::Ok, content)
						.with_header("content-type", "text/html")
				}
				Err(_) => {
					return Response::new(Status::NotFound, b"404 Not Found".to_vec())
				}
			}
		}
		let mut real_path = String::new();
		real_path.push_str(&self.path);
		for each in path.split('/').filter(|&each| each != "..") {
			real_path.push('/');
			real_path.push_str(each);
		}
		println!("real_path: {}", real_path);
		match std::fs::read(&real_path) {
			Ok(content) => Response::new(Status::Ok, content),
			Err(_) => {
				Response::new(Status::NotFound, b"StaticFiles: 404 Not Found".to_vec())
			}
		}
	}
}
