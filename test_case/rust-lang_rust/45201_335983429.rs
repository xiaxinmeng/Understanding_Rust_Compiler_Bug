rust
#[target_feature = "+avx"]
unsafe fn foo() {
    let a = i16x16::new(...);
}
