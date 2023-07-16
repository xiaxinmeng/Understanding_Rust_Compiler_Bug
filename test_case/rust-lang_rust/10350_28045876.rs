 rust
pub fn bar(x: || -> int) -> int {
    x()
}
