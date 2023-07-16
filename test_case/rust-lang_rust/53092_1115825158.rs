rust
type Foo<T> = impl Default;
fn foo<T: Debug>(t: T) -> Foo<T> {
    Some(t)
}
struct NotDefault;
fn main() {
    let x = Foo::<NotDefault>::default();
}
