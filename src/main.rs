#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::utils::format_resp;

mod utils;

enum RespValue {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespValue>),
}

struct Encoder {
    buf: Vec<u8>,
}

struct Decoder {
    buf: [u8; 512],
    offset: usize,
}

impl Decoder {
    // fn parse_value(&mut self) -> Vec<RespValue> {
    //     match self.buf[self.offset] {
    //         // b'*' => self.parse_array(),
    //         // b'+' => self.parse_simple_string(),
    //         // b'$' => self.parse_bulk_string(),
    //         // b':' => self.parse_integer(),
    //         // b'-' => self.parse_error(),
    //         _ => panic!("unknown RESP type"),
    //     }
    // }
    // fn parse_array(&mut self) {
    //     self.offset += 1;
    //     match String::from_utf8(self.buf[self.offset..].to_vec()) {
    //         Ok(parsed) => return,
    //     }

    //     print!("{}", parsed);
    // }
}

impl Encoder {
    fn encode_string(&mut self, string: &str) {
        let string_bytes = string.as_bytes();
        format_resp(&mut self.buf, b'+', string_bytes);
    }
}

fn handle_stream(stream: &mut TcpStream) {
    let mut buf: [u8; 512] = [0; 512];
    let (_) = stream.read(&mut buf);
    let mut encoder = Encoder { buf: Vec::new() };
    encoder.encode_string("PONG");

    if let Err(e) = stream.write(&encoder.buf) {
        print!("{}", e);
    }
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
