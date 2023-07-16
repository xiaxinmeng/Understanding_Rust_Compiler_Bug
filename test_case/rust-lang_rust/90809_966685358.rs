rust
#![feature(io_safety)]
use std::os::unix::io::AsFd;
use std::os::unix::prelude::BorrowedFd;

fn main() {
    let stdout = std::io::stdout();
    let foo: BorrowedFd<'static> = Box::leak(Box::new(stdout)).as_fd();
    dbg!(foo);
}
