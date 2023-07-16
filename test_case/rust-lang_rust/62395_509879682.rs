rust
#![feature(const_generics)]
struct Checked<const F: fn(usize) -> bool>;

fn not_one(val: usize) -> bool { val != 1 }

fn new() {
    let a: Option<Checked<{not_one}>> = None;
}
