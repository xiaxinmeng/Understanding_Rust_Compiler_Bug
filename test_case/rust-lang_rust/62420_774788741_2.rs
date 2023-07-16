rust
match get_kernel_end() {
    v if v as usize & 0xFFF == 0 => v,
    v => (v as usize & !0xFFF + 0x1000) as *const usize,
}
