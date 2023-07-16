
#![feature(never_type)]
fn foo(x: Option<u32>) { }
fn bar() {
    let x: Option<!> = None;
    foo(x);
}
