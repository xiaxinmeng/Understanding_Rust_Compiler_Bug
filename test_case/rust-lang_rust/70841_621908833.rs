rust
struct MyBox<T>(NonNull<T>);
// Mirror `Box` API, including `Drop`.
