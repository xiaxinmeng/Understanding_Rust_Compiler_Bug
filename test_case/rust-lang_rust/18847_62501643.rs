 rust
use std::io::{TcpListener, Listener, Acceptor};

fn main() {
    let mut l = TcpListener::bind("127.0.0.1:8000").unwrap().listen().unwrap();
    for mut s in l.incoming() {
        let _ = s.read_exact(18);
        let _ = s.write([1]);
    }
}

