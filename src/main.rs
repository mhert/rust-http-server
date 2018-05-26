extern crate mio;
extern crate memchr;

mod server;
mod client;
mod parser;
mod request;

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let mut http_server = server::HttpServer::new(addr).unwrap();
    http_server.run();
}
