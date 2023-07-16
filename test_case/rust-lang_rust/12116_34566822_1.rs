
#[feature(globs)];
use std::io::*;
use std::io::File;
use std::io::net::tcp::TcpListener;
use std::io::net::tcp::TcpAcceptor;
use std::io::net::tcp::TcpStream;
use std::io::net::ip::SocketAddr;
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4000;


fn handle_connection(opt_stream: ~Option<net::tcp::TcpStream>) {

    let mut stream = opt_stream.unwrap();
    match(stream) {
        Some(ref mut s) => {
            match(s.peer_name()) {
                Some(pn) => {
                    println(format!("Received connection from: [{:s}]", pn.to_str()));
                }
                None => ()
            }
                           },
        None => ()
    }
    println!("Connection terminates.");
}

fn main() {
    let addr: SocketAddr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let listener: TcpListener = TcpListener::bind(addr);

    // bind the listener to the specified address
    let mut acceptor: TcpAcceptor = listener.listen();

    // output the server's address and port
    println(format!("Listening on [{:s}] ...", addr.to_str()));

    // accept connections and process them in a thread
    for stream in acceptor.incoming() {
        do spawn {
            handle_connection(stream);
        }
    }

    // close the server
    drop(acceptor);
}
