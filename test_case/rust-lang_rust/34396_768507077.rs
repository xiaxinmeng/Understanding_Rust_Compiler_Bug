rust
#![allow(stable_features)]
#![feature(min_const_generics)]

fn foo<F>(x: i32) -> i32 { x }
fn bar<'a>(x: i32) -> i32 { x }
fn baz<const TOTO: i32>(x: i32) -> i32 { x }
fn reality_check(x: i32, y: i64) -> i32 { x }

fn main() {
    bar(1);
    foo::<f64>(3);
    baz::<5>(4);
    reality_check(99, -1);
}
