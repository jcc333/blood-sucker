#![feature(try_from)]
extern crate tokio;
extern crate futures;
extern crate byteorder;

use futures::Future;
use std::collections::BTreeMap;
use tokio::prelude::*;
use tokio::io;
use tokio::net::tcp;
use tokio::net::TcpListener;
use tokio::io::AsyncWrite;
use tokio::net::TcpStream;
use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:9002".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let sessions = mqtt::Sessions::new();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = listener
        .incoming()
        .map_err(|err| {
            println!("listener error = {:?}", err);
        })
        .for_each(|socket| {
            let (reader, writer) = socket.split();
            let bytes_copied = tokio::io::copy(reader, writer);
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("I/O error {:?}", err)
            });
            tokio::spawn(handle_conn);
            Ok(())
        });

    println!("server running on localhost:9002");

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);
}
