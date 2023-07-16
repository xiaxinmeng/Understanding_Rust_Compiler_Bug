rust
#[no_mangle]
#[link_section = "__interrupt_vector_10"]
pub static TIM0_VECTOR: unsafe extern "msp430-interrupt" fn() = tim0;

unsafe extern "msp430-interrupt" fn tim0() {
  P1OUT.write(0x00);
}
