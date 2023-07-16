rust
#[derive(Clone)]
enum Element {
    First( Cow<'static, [Element]>)
}
