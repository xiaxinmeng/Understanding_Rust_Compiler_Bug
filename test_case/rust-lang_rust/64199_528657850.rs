
error[E0599]: no method named `consume` found for type `Foo` in the current scope
 --> file.rs:9:7
  |
1 | struct Foo;
  | ----------- method `consume` not found for this
...
4 |     fn consume(self: Box<Self>) {}
  |        ------- the method is available for `std::boxed::Box<Foo>` here
...
9 |     x.consume();
  |       ^^^^^^^
  |
  = help: items from traits can only be used if the trait is implemented and in scope
  = note: the following trait defines an item `consume`, perhaps you need to implement it:
          candidate #1: `std::io::BufRead`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0601.
For more information about an error, try `rustc --explain E0599`.
