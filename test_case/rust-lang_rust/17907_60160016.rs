 rust
#![feature(unboxed_closures)]
fn doit<T, F: Fn(T)>(val: T, f: F) { f.call((val,)); }
fn doit_boxed<T>(val: T, f: |T|) { f(val); }
pub fn main() {
    doit_boxed(0i, |x| { x.to_int(); });
    doit(0i, |&: x/*: int*/| { x.to_int(); });
}
