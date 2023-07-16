 rust
fn main() {
    enum X { A, B(u64, u32) }
    enum Y { A, B(u32, u64) }
    assert_eq!(std::mem::size_of::<X>(), 24);
    assert_eq!(std::mem::size_of::<Y>(), 16);
}
