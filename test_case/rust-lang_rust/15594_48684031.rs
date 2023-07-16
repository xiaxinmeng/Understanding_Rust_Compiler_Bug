 rust
struct Span {
    start: u32,
    is_huge: u1,
    len_or_hugelen_index: u7,
    expn_info_index: u24
}
