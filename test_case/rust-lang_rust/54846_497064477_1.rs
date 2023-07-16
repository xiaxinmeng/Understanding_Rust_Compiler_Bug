rust
// "Borrowed value does not live long enough" pointing to the slice
static CONTAINER: StaticContainer = StaticContainer(&[Value::new()]);
