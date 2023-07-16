 rust
use std::libc;

fn main() {
    let context = zmq::Context::new();
    let responder = match context.socket(zmq::REP).get();

    let x = responder.bind("tcp://*:5555");
    error!("%? %?", responder, x);
    let x = x.unwrap();

    let mut buf = [0, ..10];
    loop {
        unsafe { responder.recv_into(buf, 0) };
        println("Received Hello");
        responder.send(bytes!("Hello"), 0);
        unsafe { libc::sleep(1) };
    }
}
