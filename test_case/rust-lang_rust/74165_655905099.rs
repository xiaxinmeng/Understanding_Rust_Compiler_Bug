
error[E0392]: parameter `'a` is never used
 --> src/main.rs:3:12
  |
3 | struct Foo<'a>(for<'r> fn(&'r Foo<'a>));
  |            ^^ unused parameter
  |
  = help: consider removing `'a`, referring to it in a field, or using a marker such as `std::marker::PhantomData`
