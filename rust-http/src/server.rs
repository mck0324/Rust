use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

fn arr(a: &[u8]) {

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream,_)) => {
                    let a = [1,3,4,2,2,3,4];
                    arr(&a[1..3]);
                    // stream.read();
                },
                Err(e) => println!("Failed to establish a connect {}", e),
            }
            
        }
    }
}