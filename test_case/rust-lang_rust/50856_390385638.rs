rust
// compile-pass, but shouldn't.
#![feature(never_type)]
fn main() {
    let _: ! = { 'a: while break 'a {}; };
}
