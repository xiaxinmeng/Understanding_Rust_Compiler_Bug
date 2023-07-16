rust
#[path = "../tests"]
mod tests {
    #[path = "support/mod.rs"]
    mod support; // expects "../tests/support/mod.rs" (relative to source file directory)

    // --- or just ---

    mod support; // expects "../tests/support/mod.rs" or "../tests/support.rs"
}
