
use std::io::net::tcp;

fn main() {
  println!("{}", tcp::TcpStream::connect("google.ca", 80).err());
}
