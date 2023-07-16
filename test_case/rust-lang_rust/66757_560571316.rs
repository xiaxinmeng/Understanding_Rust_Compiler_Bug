rust
#![allow(unreachable_code)]

// Uncomment and you will get an error:
// #![feature(never_type_fallback)]

fn foo(never: !) {
    let x: _ = never;
    bar(x);
}

fn bar<T: Bar>(t: T) { }
trait Bar { }
impl Bar for u32 { }
impl Bar for () { }

fn main() { }
