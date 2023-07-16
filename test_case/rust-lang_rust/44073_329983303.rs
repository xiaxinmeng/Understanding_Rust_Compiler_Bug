rust
union MaybeUninitialized<T> {
    value: T,
    dummy: ()
}
let uninit: MaybeUninitialized<RcBox<T>> = MaybeUninitialized { dummy: () }
