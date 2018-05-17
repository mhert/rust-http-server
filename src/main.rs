extern crate mio;

use mio::*;
use mio::net::TcpListener;
use std::collections::HashMap;
use std::io::{self, Read};
use std::str;

mod server;

fn main() {

    let http_server = server::HttpServer::new();

    const LISTENER: Token = Token(0);
    let mut next_token = 1;
    let mut sockets = HashMap::new();

    let addr = "0.0.0.0:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    let poll = Poll::new().unwrap();

    poll.register(&listener, LISTENER, Ready::readable(),
                  PollOpt::edge()).unwrap();


    let mut events = Events::with_capacity(1024);
    let mut buf = [0; 256];

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
        }
    }}
