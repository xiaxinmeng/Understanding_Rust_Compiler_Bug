rust
fn mul(a: Avx, b: Avx) -> Avx {
    unsafe { Avx(avx_mul(a.0, b.0)) }
}
