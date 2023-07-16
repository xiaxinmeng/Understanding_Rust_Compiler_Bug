rust
#[target_feature = "+avx2"] unsafe fn foo(a: u8x32) {}
#[target_feature = "+avx2"]
unsafe fn bar() {
    FOO = foo;
}
