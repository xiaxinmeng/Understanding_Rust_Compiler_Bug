rust
union MaybeInitialized<T> {
    initialized: T,
    uninitialized: (),
}
