 rust
#[deprecated]
fn _foo() {
    static FOO: &'static str = "abc";

    let _ = FOO; //~ WARN use of deprecated item
}
