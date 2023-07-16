rust
union MaybeInitialized<T> {
    init: T,
    uninit: ()
}
