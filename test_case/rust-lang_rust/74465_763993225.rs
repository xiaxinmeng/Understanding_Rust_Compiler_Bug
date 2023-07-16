rust
mod core {
    pub mod cell {
        pub struct OnceCell<T> {}
        pub struct LazyCell<T, F = fn() -> T> {}
    }
}

mod std {
    pub mod lazy {
        pub struct Once<T> {}
        pub struct Lazy<T, F = fn() -> T> {}
    }
}
