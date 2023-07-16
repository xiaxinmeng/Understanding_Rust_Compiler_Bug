 rust
fn foo() -> Iterator<uint> {
    range(0u, 10).map(|i| i*i) as Iterator<uint>
}
