rust
#[inline]
fn secure(mut x: f64) -> f64 {
    unsafe {
        // Won't compile without `# {0}` due to an "argument never used" error.
        asm!("# {0}", inout(xmm_reg) x, options(nomem, nostack));
    }
    x
}
