use std::{
	io,
	net::{TcpListener, TcpStream, ToSocketAddrs},
	panic,
	sync::{Arc, RwLock},
	thread,
};

use crate::{
	method::Method,
	request::Request,
	response::Response,
	route::{Endpoint, Route},
	status::Status,
};

pub struct Application {
	listener: TcpListener,
	routes: Arc<RwLock<Vec<Route>>>,
}

impl Application {
	pub fn new<A>(addr: A) -> io::Result<Self>
	where A: ToSocketAddrs {
		Ok(Self { listener: TcpListener::bind(addr)?, routes: Default::default() })
	}

	pub fn with_route(
		self,
		method: Method,
		pattern: &'static str,
		endpoint: Box<dyn Endpoint + Send + Sync>,
	) -> Self {
		self.routes.write().unwrap().push(Route::new(method, pattern, endpoint));
		self
	}

	pub fn run(self) {
		for mut stream in self.listener.incoming().flatten() {
			let routes = self.routes.clone();
			thread::spawn(move || {
				let mut stream2 = stream.try_clone().unwrap();
				if let Err(err) = panic::catch_unwind(move || {
					if let Err(err) = Self::handle_requests(routes, &mut stream) {
						eprintln!("{:?}", err);
						let _ = Response::new(
							Status::InternalServerError,
							b"<h1>500 Internal Server Error</h1>".to_vec(),
						)
						.with_header("content-type", "text/html")
						.with_header("connection", "close")
						.send(&mut stream);
					}
				}) {
					eprintln!("{:?}", err);
					let _ = Response::new(
						Status::InternalServerError,
						b"<h1>500 Internal Server Error</h1>".to_vec(),
					)
					.with_header("content-type", "text/html")
					.with_header("connection", "close")
					.send(&mut stream2);
				}
			});
		}
	}

	fn handle_requests(
		routes: Arc<RwLock<Vec<Route>>>,
		stream: &mut TcpStream,
	) -> io::Result<()> {
		loop {
			let Ok(mut request) = Request::parse(stream) else {
				return Err(io::Error::new(
					io::ErrorKind::InvalidData,
					"Failed to parse request",
				));
			};
			let keep_alive =
				request.headers.get("connection").is_some_and(|it| it == "keep-alive");
			let routes = routes.read().unwrap();
			let response = if let Some((route, params)) = routes
				.iter()
				.find_map(|route| route.matches(&request).map(|params| (route, params)))
			{
				if route.method != request.method {
					Response::new(
						Status::MethodNotAllowed,
						b"<h1>405 Method Not Allowed</h1>".to_vec(),
					)
					.with_header("content-type", "text/html")
				} else {
					request.params = params;
					route.endpoint.call(&request)
				}
			} else {
				Response::new(Status::NotFound, b"<h1>404 Not Found</h1>".to_vec())
					.with_header("content-type", "text/html")
			};
			let _ = response
				.with_header(
					"connection",
					if keep_alive { "keep-alive" } else { "close" },
				)
				.send(stream);
			if !keep_alive {
				return Ok(());
			}
		}
	}
}
