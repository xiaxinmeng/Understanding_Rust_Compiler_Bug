 rust
#[deprecated]
mod nope {
    fn foo() {}

    fn bar() {
        foo() //~ WARN use of deprecated item
    }
}
