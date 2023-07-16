 rust
#![feature(core, std_misc)]
use std::thread::Thread;

fn main() {
    let bad = {
        let x = 1;
        let y = &x;

        Thread::scoped(|| {
            let z = y;
            1 + *z
        })
    };

    println!("{}", bad.join().ok().unwrap());
}
