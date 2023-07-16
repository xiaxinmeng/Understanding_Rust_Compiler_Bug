 rust
#[link(name = "gcc_s")]
extern {
    fn __muldi3(u64, u64) -> u64;
}
