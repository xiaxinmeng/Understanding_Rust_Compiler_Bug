
error: `foo` does not live long enough
 --> src/main.rs:6:30
  |
6 |     let borrowed: &mut Foo = foo.borrow_mut();
  |                              ^^^ does not live long enough
7 | }
  | - borrowed value only lives until here
  |
  = note: borrowed value must be valid for the static lifetime...
