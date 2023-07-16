 rust
use std::io::{IoResult};
use std::io::net::get_host_addresses;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::net::tcp::TcpStream;
use std::task;

static TARGET: &'static str = "localhost";

fn url_to_socket_addr(host: &str) -> IoResult<SocketAddr> {
    // Just grab the first IPv4 address
    let addrs = try!(get_host_addresses(host));
    let addr = addrs.into_iter().find(|&a| {
        match a {
            Ipv4Addr(..) => true,
            _ => false
        }
    });

    // TODO: Error handling
    let addr = addr.unwrap();

    let port = 8000;

    Ok(SocketAddr {
        ip: addr,
        port: port
    })
}

fn main() {
    for _ in range(0u32, 10u32) {
        task::spawn(proc() {
            let addr = url_to_socket_addr(TARGET).unwrap();
            let mut stream = TcpStream::connect(addr).unwrap();
            (write!(stream, "GET / HTTP/1.0\r\n")).unwrap();
            (write!(stream, "\r\n")).unwrap();
            stream.flush().unwrap();

            match stream.read_byte() {
                Ok(_) => {
                    stream.read_to_end().unwrap();
                    println!("success!");
                }
                Err(e) => println!("{}", e.desc),
            }
        });
    }
}
