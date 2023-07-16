 rust
use std::comm::Chan;

fn main() {
    let (port, chan) = Chan::new();
    spawn(proc() {
        let port = port;
        loop {
            port.recv();
        }
    });
    loop {
        chan.send(~"test");
    }
}
