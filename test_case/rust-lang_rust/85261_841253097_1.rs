rust
#![feature(panic_always_abort)]

use std::panic;

fn run(do_panic: &dyn Fn()) {
    panic::always_abort();
    do_panic();
}

fn main() {
    run(&|| panic!());
}
