rust
#![feature(const_generics)]

fn main() {
    takes_closure_of_array(|_| {});
}

fn takes_closure_of_array<F>(f: F)
where
    F: FnOnce([i32; 0]),
{
    f([]);
}
