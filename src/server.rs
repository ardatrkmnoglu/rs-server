use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Server is listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(0) => println!("Connection closed by client."),
                        Ok(n) => {
                            println!(
                                "Received request from {}: {}",
                                addr,
                                String::from_utf8_lossy(&buf[..n])
                            );
                        }
                        Err(err) => println!("Failed to read from {}: {}", addr, err),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}
