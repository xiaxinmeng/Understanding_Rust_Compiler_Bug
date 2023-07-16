rust
union Foo<T> {
    empty: (),
    value: ManuallyDrop<[T; 2]>
}
