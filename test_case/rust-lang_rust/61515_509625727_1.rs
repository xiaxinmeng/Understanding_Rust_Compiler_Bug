rust
    struct SomeError<T>(Box<T>);

    impl SomeError {
        fn into_original(self) -> Box<T> { /* … */ }
    }
    