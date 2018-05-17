extern crate mio;

mod server;
mod client;

fn main() {

    let addr = "0.0.0.0:8080".parse().unwrap();

    let mut http_server = server::HttpServer::new(addr);
    http_server.run();
}
