 rust
struct X {
    #[aligned(16)]
    x: u8,
    #[aligned(align_of(libc::c_double))]
    y: u8,
}
