fn main() {
    // let string = String::from("🚚🎱👸🔥");
    // // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[..5];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";
    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);

    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;



    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
    pub struct Server {
        addr: String,
    
    }
    impl Server {
        pub fn new(addr: String) -> Self {
            Self {
                addr
             }
        }
    
        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {
    mod request {
        struct Request {
            path: String,
            query_string: Option<String>,
            method: super::method::Method,
        }
    }
    
    mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
            
        }
    }
}