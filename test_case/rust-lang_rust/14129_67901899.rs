 rust
fn foo() {
    #[start]
    fn bar(_: int, _: *const *const u8) -> int { foo(); 0 }
}
