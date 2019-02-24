extern crate mqtt;

use std::io::{Result, Read, Write};
use std::net::{TcpStream, TcpListener};
use mqtt::*;

fn main() {
    let addr = "127.0.0.1:9002";
    if let Ok(listener) = TcpListener::bind(addr) {
        println!("Bound to {}", addr);
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                println!("Connection established");
            } else {
                println!("Error connecting to stream");
            }
        }
    } else {
        println!("Couldn't bind to {}", addr);
    }
}

fn response_bytes<'a>() -> &'a[u8] {
    "This is some MQTT or something".as_bytes()
}

fn handle_connection(mut stream: &TcpStream) -> Result<()> {
    let mut clone = stream.try_clone()?;
    if let Ok(msg) = mqtt::Message::de(&mut clone) {
        println!("GRAPE JOB");
    } else {
        println!("CHABOY FUCKED UP");
    }
    stream.write(response_bytes())?;
    stream.flush()
}
