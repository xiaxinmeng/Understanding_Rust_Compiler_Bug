rust
impl Iterator for ! {
    type Item = !;
    fn next(&mut self) -> Option<!> { ::core::intrinsics::unreachable() }
}
