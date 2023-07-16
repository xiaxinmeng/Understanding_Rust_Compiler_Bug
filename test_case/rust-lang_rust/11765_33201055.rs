 rust
use std::comm::{Port, Chan};

struct BiChan<T> {
    port: Port<T>,
    chan: Chan<T>,
}

impl<T: Send> BiChan<T> {
    fn new() -> BiChan<T> {
        let (port, chan) = Chan::new();
        BiChan { port: port, chan: chan }
    }

    fn send(&self, thing: T) {
        self.chan.send(thing);
    }

    fn recv(&self) -> T {
        self.port.recv()
    }
}

fn main() {
    let chan = BiChan::new();
    chan.send(1);
    println!("{}", chan.recv());
}
