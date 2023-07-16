rust
use std::ops::{Deref, DerefMut};
use std::net::UdpSocket;

pub trait BufItem: Clone {
    fn empty() -> Self;
}

impl BufItem for u8 { fn empty() -> Self { 0 } }
impl BufItem for u16 { fn empty() -> Self { 0 } }
impl BufItem for u32 { fn empty() -> Self { 0 } }
// etc...

pub struct Buf<T: BufItem> {
    data: Vec<T>, // It doesn't need to use a Vec internally, of course
}

impl<T: BufItem> Buf<T> {
    pub fn with_length(len: usize) -> Buf<T> {
        Buf {
            data: vec![T::empty(); len],
        }
    }
}

impl<T: BufItem> Deref for Buf<T> {
    type Target = [T];
    fn deref<'a>(&'a self) -> &'a [T] {
        &self.data
    }
}

impl<T: BufItem> DerefMut for Buf<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [T] {
        &mut self.data
    }
}




fn main() {
    let sock = UdpSocket::bind("127.0.0.1: 5900").unwrap();

    let mut buf = Buf::with_length(4096);
    sock.recv(&mut buf);
}
