rust
unsafe extern "msp430-interrupt" fn tim0() {
    P1OUT.write(0x00);
}
