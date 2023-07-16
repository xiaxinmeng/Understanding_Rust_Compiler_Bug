rs
use std::env;
use std::process::Command;

#[inline(never)]
fn double() {
    struct Double;

    impl Drop for Double {
        fn drop(&mut self) { panic!("twice") }
    }

    let _d = Double;

    panic!("once");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "fail" {
    } else if args.len() >= 2 && args[1] == "double-fail" {
        double();
    } else {
        Command::new(&args[0]);
    }
}
