none
error[E0308]: mismatched types
  --> src/main.rs:42:42
   |
42 |     fn new(data: [i32; 8]) -> Self { Bar(Foo::new_foo(data)) } // !!!
   |                                          ^^^^^^^^^^^^^^^^^^ expected type parameter, found struct `TagA`
   |
   = note: expected type `Foo<T>`
              found type `Foo<TagA>`
