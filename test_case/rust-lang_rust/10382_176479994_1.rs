 rust
struct VtableForTrait {
    dtor: unsafe fn(*mut u8),
    size: usize,
    align: usize,
    method: unsafe fn(*const u8) -> char,
    // The above fields are present in current Rust, only CONST below is new:
    CONST: char
}
