 rust
use std::rt::io::net::tcp::TcpListener;
use std::rt::io::{Reader, Writer, Listener, Acceptor};
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::str;
use std::cell::Cell;

fn main() {
    let addr = SocketAddr {
        ip: Ipv4Addr(0, 0, 0, 0),
        port: 9090
    };

    let mut acceptor = TcpListener::bind(addr).listen().unwrap();

    println("Listener is ready");

    loop {
        let stream = acceptor.accept().unwrap();
        let stream = Cell::new(stream);
        do spawn{
            let mut buf = [0,..512];
            let mut stream = stream.take();
            loop {
                match stream.read(buf) {
                    Some(count) => {
                        print("read "+count.to_str()+" bytes: "+str::from_utf8(buf));
                        stream.write(bytes!("Hello World\r\n"));
                        break; // close connection after read
                    }
                    None => { println("Finished"); break } // connection is closed by client
                }
            }

        }

    }
}
