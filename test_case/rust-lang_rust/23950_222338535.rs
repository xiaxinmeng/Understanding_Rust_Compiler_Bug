 rust
use std::boxed::Box;
use std::error::Error;
use std::io;
use std::sync::mpsc::channel;

#[derive(Debug)]
struct A {
    b: Box<Error>
}

pub struct JoinGuard<'a, T: Send + 'a>(&'a T);
impl<'a, T: Send + 'a> JoinGuard<'a, T> {
    fn join(self) {}
}
pub fn scoped<'a, T, F>(_f: F) -> JoinGuard<'a, T> where T: Send + 'a, F: FnOnce() -> T, F: Send + 'a {
   unimplemented!()
}

fn main() {
    // Create a simple streaming channel
    let (tx, rx) = channel();
    let t = scoped(move|| {
        tx.send(A{ b: Box::new(io::Error::new(io::ErrorKind::Other, "other")) }).unwrap();
    });
    t.join();
    let v = rx.recv().unwrap();
    println!("{:?}",v);
}
