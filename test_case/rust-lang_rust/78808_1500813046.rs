
error[[E0507]](https://doc.rust-lang.org/stable/error_codes/E0507.html): cannot move out of `other.__` which is behind a shared reference
 --> src/main.rs:9:5
  |
7 | #[derive(PartialEq)]
  |          --------- in this derive macro expansion
8 | struct Bar {
9 |     __: ::std::rc::Rc<dyn Foo>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ move occurs because `other.__` has type `Rc<dyn Foo>`, which does not implement the `Copy` trait
  