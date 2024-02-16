pub mod simple_api {
    use std::collections::HashMap;
    use std::net::TcpStream;
    use std::io::prelude::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::musicianservice::Musician;
    use crate::painterservice::Painter;

    pub struct Router{
        routes: HashMap<String, (&'static str, fn() -> String)>
    }

    impl Router {
        pub fn new() -> Self {
            let mut routes: HashMap<String, (&'static str, fn() -> String)> = HashMap::new();
            routes.insert("/api/painters".to_string(), ("HTTP/1.1 200 OK", Painter::get_painters_json) );
            routes.insert("/api/musicians".to_string(), ("HTTP/1.1 200 OK", Musician::get_musicians_json) );
            routes.insert("/".to_string(), ("HTTP/1.1 404 NOT FOUND", || {"{ \"message\" : \"Resource not found\" }".to_string()}) );
            Router { routes }
        }
        
        fn route(&self, request: &str) -> &(&str, fn() -> String) {
            match self.routes.get(request) {
                Some(value) => value,
                None => {
                    self.routes
                        .get("/")
                        .expect("Default route not found")
                }
            }
        }
    }

    pub struct Request {
        buffer: [u8; 1024],
        method: String,
        path: String
    }

    impl Request {
        pub fn handle_request(mut stream: TcpStream, router: Arc<Mutex<Router>>) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let request_str = String::from_utf8_lossy(&buffer).to_string();
            let parts: Vec<&str> = request_str.split_whitespace().collect();
            let ( method, path) = if parts.len() > 1 { (parts[0], parts[1]) } else { ("GET", "/") };
            
            #[cfg(debug_assertions)]{
                println!("{}", request_str);
            }

            let response = Request {
                buffer,
                method : method.to_string(),
                path : path.to_string(),
            }
            .router(router);

            response.send_response(&stream);
        }

        fn router(&self, router: Arc<Mutex<Router>>) -> Response {
            let router = router.lock().unwrap();
            
            let (status, json_data) = router.route(&self.path);
            
            let response_content = json_data();

            Response{
                status : status.to_string(),
                contents_size: response_content.len(),
                contents : response_content
                }
        }
    }

    pub struct Response {
        status: String,
        contents: String,
        contents_size: usize,
    }

    impl Response {

        pub fn send_response(&self, mut stream: &TcpStream){
            let response = self.deserialize_into_string();
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();        
        }

        fn deserialize_into_string(&self) -> String {
            format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", self.status,"application/json" , self.contents_size, self.contents)
        }
    }
}
