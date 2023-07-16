 rust
#![feature(panic_handler, recover, std_panic)]
use std::thread;
use std::panic::{self, recover};

fn main() {
    let mut build = Vec::new();
    for _ in 0..10 {
        build.push(thread::spawn(|| {
            let orig_handler = panic::take_handler();

            panic::set_handler(|_| ());

            let _res = recover(|| {
                panic!("fail");
            });


            panic::set_handler(move |info| { (*orig_handler)(info) });
        }));
    }

    let results: Vec<_> = build.into_iter().map(|jh| jh.join()).collect();
    let all_passed = results.into_iter().all(|r| r.is_ok());
    println!("{}", all_passed);
}
