use mio::Event;
use mio::Events;
use mio::Poll;
use mio::PollOpt;
use mio::Ready;
use mio::net::TcpListener;
use mio::Token;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use client::Client;

const LISTENER: Token = Token(0);

pub struct HttpServer {
    listener: TcpListener,
    poll: Poll,
    clients: HashMap<Token, Client>,
    next_token: usize,
}

impl HttpServer {
    pub fn new(addr: SocketAddr) -> io::Result<Self> {

        Ok(HttpServer{
            listener: TcpListener::bind(&addr)?,
            poll: Poll::new()?,
            clients: HashMap::new(),
            next_token: 1,
        })
    }

    pub fn run(&mut self) -> () {
        self.poll.register(&self.listener, LISTENER, Ready::readable(),
                      PollOpt::edge()).unwrap();

        let mut events = Events::with_capacity(1024);

        loop {
            self.poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                self.handle(event);
            }
        }
    }

    fn handle(&mut self, event: Event) -> () {
        match event.token() {
            LISTENER => {
                loop {
                    match self.listener.accept() {
                        Ok((socket, _)) => {
                            let token = Token(self.next_token);
                            self.next_token += 1;

                            println!("Connection: {:?} - Connect", &token);

                            self.poll.register(&socket,
                                          token,
                                          Ready::readable() | Ready::writable(),
                                          PollOpt::edge()).unwrap();
                            self.clients.insert(token, Client::new(token,socket));

                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        e => panic!("err={:?}", e), // Unexpected error
                    }
                }
            },
            token => {
                {
                    let client = self.clients.get_mut(&token);
                    match client {
                        Some(mut client) => {
                            client.process();
                        }
                        None => {
                            println!("No socket found for token")
                        }
                    }
                }
                {
                    self.clients.remove(&token);
                }
            }
        }
    }
}
