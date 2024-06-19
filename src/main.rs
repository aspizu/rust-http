mod application;
mod cookies;
mod headers;
mod macros;
mod method;
mod query;
mod request;
mod response;
mod route;
mod static_files;
mod status;

use std::io;

use cookies::Cookie;
use macros::Application;
use request::Request;
use response::Response;
use route::Endpoint;
use static_files::StaticFiles;
use status::Status;

struct Login;

impl Endpoint for Login {
	fn call(&self, request: &Request) -> Response {
		let Some(username) = request.query.get("username") else {
			return Response::new(Status::BadRequest, b"username is required".to_vec());
		};
		if request.body != b"password1" {
			return Response::new(Status::Unauthorized, b"invalid password".to_vec());
		}
		Response::new(Status::Ok, format!("Welcome, {}!", username).into_bytes())
			.with_cookie("token", Cookie::new("123-456-789".to_owned()))
	}
}

struct Session;

impl Endpoint for Session {
	fn call(&self, request: &Request) -> Response {
		let Some(token) = request.cookies.get("token") else {
			return Response::new(
				Status::Unauthorized,
				b"You are not looged in".to_vec(),
			);
		};
		Response::new(Status::Ok, format!("Your token is: {}", token).into_bytes())
	}
}

fn main() -> io::Result<()> {
	let app = Application!(
		host: "127.0.0.1:8000"

		POST "/login" => Login
		GET "/session" => Session
		GET "/*" => StaticFiles::new("static")
	);
	app.run();
	Ok(())
}
