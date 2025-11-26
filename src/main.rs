#![allow(unused_imports)]
use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>), // raw bytes, may not be UTF-8
    Array(Vec<RespValue>),
}

struct Decoder {
    buf: [u8; 512],
    offset: usize,
}

impl Decoder {
    fn parse_value(&mut self) -> RespValue {
        match self.buf[self.offset] {
            // b'*' => self.parse_array(),
            // b'+' => self.parse_simple_string(),
            // b'$' => self.parse_bulk_string(),
            // b':' => self.parse_integer(),
            // b'-' => self.parse_error(),
            _ => panic!("unknown RESP type"),
        }
    }
    fn parse_array(&mut self) {}
}

fn handle_stream(stream: &mut TcpStream) {
    let mut buf: [u8; 512] = [0; 512];
    let (bytes) = stream.read(&mut buf);
}
fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_stream(&mut stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
