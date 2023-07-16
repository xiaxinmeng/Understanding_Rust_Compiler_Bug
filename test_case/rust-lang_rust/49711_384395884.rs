rust
struct Foo<T: Iterator> {
  data: <T as Iterator>::Item
}
