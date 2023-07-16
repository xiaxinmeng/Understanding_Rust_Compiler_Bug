rust
#[inline(never)]
fn from_elem_u16_std(n: u16) -> Vec<u16> {
    std::vec::from_elem(n, 100)
}
