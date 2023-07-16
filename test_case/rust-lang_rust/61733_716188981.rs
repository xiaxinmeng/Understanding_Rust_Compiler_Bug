rust
fn foo() -> u8 {
    { 0 }

    empty!(); //~ ERROR: mismatched types
}
