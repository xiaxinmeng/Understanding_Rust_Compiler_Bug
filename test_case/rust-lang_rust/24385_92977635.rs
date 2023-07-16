 rust
fn foo() {
    finally(|| {
        // code to run now
    }, || {
        // code to run afterwards, even on a panic
    }
}
