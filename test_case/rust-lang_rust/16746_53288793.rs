
extern crate debug;

use std::io::signal::{Listener, Interrupt};

fn main() {
    let mut sig_listener = Listener::new();
    let io_result = sig_listener.register(Interrupt);
    println!("{:?}", io_result);
    while match sig_listener.rx.recv() {
        Interrupt => {
            println!("Registered interrupt signal");
            false
        },
        _ => true,
    } {}
}
