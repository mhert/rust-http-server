
use std::io;
use std::str;
use mio::Token;
use std::io::Read;

pub struct Client {
    token: Token,
    socket: Read,
}

const BUF_SIZE: usize = 1;

impl Client {
    pub fn new(token: Token, socket: Stream) -> Self {

        Client{
            token,
            socket,
        }
    }

    pub fn process(&mut self) -> () {
        let mut buf = [0; BUF_SIZE];

        loop {
            match self.socket.read(&mut buf) {
                Ok(0) => {
                    println!("Connection: {:?} - Disconnect", &self.token);
                    //sockets.remove(&token);
                    break;
                }
                Ok(len) => {
                    println!("Connection: {:?} - Got something", &self.token);
                    println!("len: {:?}", len);
                    println!("Buf: {:?}", str::from_utf8(&buf).unwrap());
                    if (len < BUF_SIZE) {
                        break;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    println!("Connection: {:?} - WouldBlock", &self.token);
                    //sockets.remove(&token);
                    break;
                }
                e => panic!("err={:?}", e), // Unexpected error
            }
        }
    }
}
