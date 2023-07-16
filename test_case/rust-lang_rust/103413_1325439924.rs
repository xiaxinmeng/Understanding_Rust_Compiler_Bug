rust
struct MyBox<T>(NonNull<T>);

impl<T> Drop for MyBox<T> { … }
