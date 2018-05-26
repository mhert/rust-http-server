
use std::io;
use std::str;
use mio::Token;
use std::io::Read;
use std::io::Write;
use mio::net::TcpStream;
use parser::Parser;

pub struct Client {
    token: Token,
    socket: TcpStream,
    parser: Parser,
}

impl Client {
    pub fn new(token: Token, socket: TcpStream) -> Self {
        Client{
            token,
            socket,
            parser: Parser::new(),
        }
    }

    pub fn process(&mut self) -> () {
        self.parser.parse(&mut self.socket);
        self.socket.write("HTTP/1.1 200 OK
Date: Mon, 27 Jul 2009 12:28:53 GMT
Content-Type: text/html

<h1>foo</h1>
".as_bytes()).unwrap();
        self.socket.flush();
    }
}
