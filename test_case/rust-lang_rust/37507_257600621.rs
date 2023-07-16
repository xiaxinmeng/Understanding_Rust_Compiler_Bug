 rust
  #![feature(alloc_system)]
  extern crate alloc_system;
  
  use std::net::TcpStream;
  
  fn main() {
    TcpStream::connect("google.com:80").unwrap();
  }
  