rust
#![feature(never_type)]

fn banana<F: FnOnce() -> !>(f: F) -> ! { f(); loop {} }

fn main() {
    banana(move || -> ! { loop {} })
}
