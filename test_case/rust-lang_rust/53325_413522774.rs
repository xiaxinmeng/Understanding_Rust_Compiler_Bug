
union Foo<T> {
    t: T,
    u: (),
}
unsafe const fn uninitialized<T>() -> T {
    Foo { u: () }.t
}
