rust
mod rc {
    impl Weak<T> {
        pub fn strong_count(&self) -> usize;
        pub fn weak_count(&self) -> Option<usize>;
    }
}

mod sync {
    impl Weak<T> {
        pub fn strong_count(&self) -> usize;
        pub fn weak_count(&self) -> Option<usize>;
    }
}
