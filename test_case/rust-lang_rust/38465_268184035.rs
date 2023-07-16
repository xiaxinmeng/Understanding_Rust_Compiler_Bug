rust
unsafe extern "msp430-interrupt" fn tim0() {
    not_inlined_foo();
}
