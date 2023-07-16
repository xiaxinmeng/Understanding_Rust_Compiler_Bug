
error[[E0277]](https://doc.rust-lang.org/stable/error-index.html#E0277): the trait bound `Foo: Qux` is not satisfied
  --> src/main.rs:12:15
   |
12 |     needs_bar(Some(Foo));
   |     --------- ^^^^^^^^^ the trait `Qux` is not implemented for `Foo`
   |     |
   |     required by a bound introduced by this call
   |
   = help: the trait `Bar` is implemented for `Option<T>`
note: required because of the requirements on the impl of `Bar` for `Option<Foo>`
  --> src/main.rs:7:9
   |
7  | impl<T> Bar for Option<T> where T: Qux {}
   |         ^^^     ^^^^^^^^^
note: required by a bound in `needs_bar`
  --> src/main.rs:9:22
   |
9  | fn needs_bar(t: impl Bar) {}
   |                      ^^^ required by this bound in `needs_bar`
