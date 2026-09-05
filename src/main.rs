fn main() {
    let address = "127.0.0.1:8080";
    let server = Server::new(address.to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Server is listening on {}", self.addr);
    }
}

