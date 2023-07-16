 rust
// compile-flags:--test
// rustc-env:RUSTC_BOOTSTRAP_KEY=

#![cfg(feature="does-not-exist")]
#![feature(iter_arith_traits)]

// nothing below actually matters, but I added it anyway
#[test]
fn dummy() {
    let () = "this should not reach type-checking";
    panic!("this should not run");
}
