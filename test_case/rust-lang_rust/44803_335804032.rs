Rust
    trait Trait { fn foo(self); }
    impl<T: Sync> Trait for (T, u32) {}
    impl<T> Trait for (T, u64) {}
    (|| { yield; }, Default::default()).foo();
    