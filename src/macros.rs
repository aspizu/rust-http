macro_rules! Application {
    (
		host: $host:literal
        $(
            $method:ident $path:expr => $handler:expr
        )*
    ) => {
        {
            use crate::application::Application;
            use crate::method::Method;
            let mut app = Application::new($host)?;
            $(
                app = app.with_route(Method::$method, $path, Box::new($handler));
            )+
            app
        }
    };
}

pub(crate) use Application;
