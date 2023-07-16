 rust
#![feature(core)]
#![feature(std_misc)]

use std::thread::{Thread, JoinGuard};

fn callback(v: usize) -> usize { v * 2 }

trait CB: Fn(usize) -> usize + Send {}

struct S<T>
where T: CB {
    cb: T,
}

fn threaded<'a, T>(s: S<T>) -> JoinGuard<'a, usize>
where T: CB {
    Thread::scoped( move || {
        (s.cb)(1337)
    })
}

fn main() {
    if let Ok(r) = threaded( S { cb: callback } ).join() {
        println!("{}", r);
    }
}
