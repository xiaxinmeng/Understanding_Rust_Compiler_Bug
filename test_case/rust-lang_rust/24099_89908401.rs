 rust
#![feature(set_stdio)]

use std::thread;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc,Mutex};

fn main() {
    struct Sink(Arc<Mutex<Vec<u8>>>);
    impl Write for Sink {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            Write::write(&mut *self.0.lock().unwrap(), data)
        }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let data = Arc::new(Mutex::new(Vec::new()));
    let err = Sink(data.clone());

    let r = thread::spawn(|| {
        io::set_panic(Box::new(err));
        panic!();
    }).join();
    println!("{:?}", r.is_err());
}
