 rust
extern {
    #[link_section=".data"]
    #[inline(never)]
    fn write_half_page(address: *mut u32, words: *const u32);
}
