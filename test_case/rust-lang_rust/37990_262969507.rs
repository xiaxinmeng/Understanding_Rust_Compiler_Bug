rust
use std::net::{TcpListener, TcpStream, SocketAddr};

struct NonBlockingAcceptor(::std::net::TcpListener);

impl Iterator for NonBlockingAcceptor {
    type Item = ::std::io::Result<(TcpStream, SocketAddr)>;
    fn next(&mut self) -> Option<Self::Item> {
        let accept = self.0.accept();
        match accept {
            Err(ref e) if e.kind() == ::std::io::ErrorKind::WouldBlock => None,
            v => Some(v)
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    listener.set_nonblocking(true);
    let mut iter = NonBlockingAcceptor(listener).flat_map(Result::ok);
    loop {
    // epoll(listener);
    while let Some((client, peeraddr)) = iter.next() {
        println!("got client {:?}", peeraddr);
        client.shutdown(::std::net::Shutdown::Both);
    }
    }
}
