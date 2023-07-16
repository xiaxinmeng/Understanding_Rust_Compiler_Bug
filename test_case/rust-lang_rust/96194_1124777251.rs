rust
fn assert_unique_assoc<Assoc>(_: impl for<'a> Tr<'a, Assoc = Assoc>) {}
fn main() { assert_unique_assoc(f()); }
