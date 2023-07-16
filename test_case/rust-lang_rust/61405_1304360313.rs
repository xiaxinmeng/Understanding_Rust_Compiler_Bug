rust
#![allow(unused_variables)]
fn foo() -> i32 { 22 }
fn bar(x: &i32) -> &i32 { x }
fn main() {
    let p = bar(&foo());
    let q = *p;
}
