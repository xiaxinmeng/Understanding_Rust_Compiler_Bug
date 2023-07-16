rust
#![feature(closure_to_fn_coercion)]

fn foo(f: fn(Vec<u32>) -> usize) { }

fn main() {
    foo(|x| x.len())
}
