rust
#[derive(Clone)]
struct Foo<T> {
  data: Rc<T>
}
