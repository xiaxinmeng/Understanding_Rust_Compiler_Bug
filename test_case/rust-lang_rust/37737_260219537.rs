 rust
for x in vec![1u32, 2, 3], iter() {
    // x: &u32;
    if x < 3 {
        return x + 1; // Under your proposal, `x` would be out of scope in this statement.
    }
}
