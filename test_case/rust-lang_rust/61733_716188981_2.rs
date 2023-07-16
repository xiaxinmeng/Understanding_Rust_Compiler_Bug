rust
fn foo() {
    { 0 }

    empty!(); //~ ERROR: mismatched types
}
