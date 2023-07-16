rust
    /// A safe (though overly complex) identity function.
    fn overcomplicated_identity<T>(a: T) -> T {
        unsafe { std::mem::transmute(a) }
    }
    