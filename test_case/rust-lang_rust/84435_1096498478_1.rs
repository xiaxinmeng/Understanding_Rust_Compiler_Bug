
error[E0283]: type annotations needed
 --> src/lib.rs:2:16
  |
2 | trait Foo<'a>: Trait<'a> + for<'b> Trait<'b> {}
  |                ^^^^^^^^^ cannot infer type for type parameter `Self`
  |
  = note: cannot satisfy `Self: Trait<'a>`
