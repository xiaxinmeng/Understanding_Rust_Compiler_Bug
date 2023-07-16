rust
#![feature(slice_patterns)]

fn foo(s: &[i32]) {
    let &[ref _xs..] = s;
}

fn main() {}
