 rust
use std::net::UdpSocket;
use std::thread;
use std::os::unix::prelude::*;
fn main() {
    let b = UdpSocket::bind("0.0.0.0:3944").unwrap();

    let _t = thread::scoped(|| {
        let mut b = &b;
        println!("a");
        assert!(b.recv_from(&mut []).is_err());
        println!("here");
    });

    let fd = b.as_raw_fd();
    extern {
        fn shutdown(socket: i32, how: i32) -> i32;
        fn sleep(seconds: u32) -> u32;
    }

    unsafe {
        sleep(1);
        println!("shutting down");
        println!("{}", shutdown(fd as i32, 2));
        println!("shut down");
    }
}
