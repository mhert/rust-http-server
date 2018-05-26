use std::io::Read;
use mio::net::TcpStream;
use std::io::BufReader;
use std::io;
use std::str;
use std::io::Write;
use std::io::BufRead;
use request::RawHttpRequest;
use std::collections::HashMap;

use memchr::memchr;

enum ParserState {
    Start,
    ParseMethod,
    ParseRequestUri,
    ParseHttpVersion,
    End,
}

const BUF_SIZE: usize =  8 * 1024;

pub struct Parser {
    state: ParserState,
    request: RawHttpRequest
}

impl Parser {
    pub fn new() -> Self {
        Parser{
            state: ParserState::Start,
            request: RawHttpRequest{
                version: Option::None,
                method: Option::None,
                request_uri: Option::None,
            }
        }
    }

    pub fn parse(&mut self, stream: &mut Read) {
        loop {
            let mut buf = [0; BUF_SIZE];

            match stream.read (&mut buf) {
                Ok(0) => {
                    break;
                }
                Ok(len) => {
                    self.do_parse(&buf);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                e => panic!("err={:?}", e), // Unexpected error
            }
        }

        println!("{:?}", str::from_utf8(&self.request.version.take().unwrap()).unwrap());
        println!("{:?}", str::from_utf8(&self.request.method.take().unwrap()).unwrap());
        println!("{:?}", str::from_utf8(&self.request.request_uri.take().unwrap()).unwrap());
    }

    pub fn do_parse(&mut self, buf: &[u8]) {
        let mut cursor = 0;

        loop {
            match self.state {
                ParserState::Start => {
                    self.state = ParserState::ParseMethod;
                }
                ParserState::ParseMethod => {
                   match memchr(b' ', &buf[cursor..]) {
                       Some(pos) => {
                           cursor = pos;
                           self.request.method = self.parse_token(&buf[cursor..], pos);
                           self.state = ParserState::ParseRequestUri;
                       }
                       None => {
                           // TODO implement
                       }
                   }
                }
                ParserState::ParseRequestUri => {
                    match memchr(b' ', &buf[cursor..]) {
                        Some(pos) => {
                            cursor = pos;
                            self.request.request_uri = self.parse_token(&buf[cursor..], pos);
                            self.state = ParserState::ParseHttpVersion;
                        }
                        None => {
                            // TODO implement
                        }
                    }
                }
                ParserState::ParseHttpVersion => {
                    match memchr(b' ', &buf[cursor..]) {
                        Some(pos) => {
                            cursor = pos;
                            self.request.version = self.parse_token(&buf[cursor..], pos);
                            self.state = ParserState::End;
                        }
                        None => {
                            // TODO implement
                        }
                    }
                }
                ParserState::End => {
                    break;
                }
            }
        }
    }

    fn parse_token(&mut self, buf: &[u8], pos: usize) -> Option<Box<[u8]>> {
        let mut val: Vec<u8> = vec![0; pos];
        val.clone_from_slice(&buf[0..pos]);
        return Option::Some(val.into_boxed_slice());
    }

    pub fn done()-> bool {
        true
    }
}