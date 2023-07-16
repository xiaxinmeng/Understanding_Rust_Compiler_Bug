 rust
union MaybeUninitialized<R> {
    data: R,
    _uninitialized: (),
}

let r: MaybeUninitialized<R> = mem::uninitialized();
