rust
#![feature(never_type)]
fn magic<F: FnOnce() -> !>(f: F) -> F { f }

fn main() {
    let f2 = magic(|| loop {}) as fn() -> !;
}
