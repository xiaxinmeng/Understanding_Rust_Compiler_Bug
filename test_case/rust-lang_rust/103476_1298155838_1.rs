text
error[[E0597]](https://doc.rust-lang.org/nightly/error-index.html#E0597): `n` does not live long enough
 --> src/lib.rs:9:33
  |
9 |     if let n = () && let _ = it(&n) {};
  |                              ---^^-  -- ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `Box<dyn Tr<'_>>`
  |                              |  |    |
  |                              |  |    `n` dropped here while still borrowed
  |                              |  borrowed value does not live long enough
  |                              a temporary with access to the borrow is created here ...
