rust
fn foo() {
    {
        use std::ops::Deref;
        <&i32>::Target::default();
    }
}
