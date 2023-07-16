
error[E0310]: the parameter type `T` may not live long enough
 --> src/lib.rs:4:5
  |
3 | impl<T> Foo<T> {
  |      - help: consider adding an explicit lifetime bound...: `T: 'static`
4 |     const FUNC: &'static fn()->T = &||loop{};
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the reference type `&'static fn() -> T` does not outlive the data it points at
