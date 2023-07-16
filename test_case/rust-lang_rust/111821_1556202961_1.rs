
error[[E0284]](https://doc.rust-lang.org/nightly/error_codes/E0284.html): type annotations needed
 --> src/lib.rs:7:11
  |
7 |     next: <Node<T> as Pointee>::Metadata,
  |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
  |
  = note: cannot satisfy `<Node<T> as Pointee>::Metadata == _`
