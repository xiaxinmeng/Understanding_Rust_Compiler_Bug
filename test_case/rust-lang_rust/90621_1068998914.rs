rust
fn main() {
    unsafe {
        let _ = core::arch::aarch64::vmovq_n_u8(0);
    }
}
