rust
    fn x() -> impl std::fmt::Debug { 42 }
    fn send_it<T: Send>(_: T) {}
    send_it(x());
    