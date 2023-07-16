rust
type Foo<T> = impl Default;
fn foo<T: Debug>(t: T) -> Foo<T> {
    t // T does not implement `Default`
}
