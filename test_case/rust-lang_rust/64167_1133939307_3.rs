
error[[E0507]](https://doc.rust-lang.org/nightly/error-index.html#E0507): cannot move out of `*f` which is behind a shared reference
   --> src/lib.rs:20:5
    |
20  |     f.neg()
    |     ^^-----
    |     | |
    |     | `*f` moved due to this method call
    |     move occurs because `*f` has type `Foo<T>`, which does not implement the `Copy` trait
    |
note: this function takes ownership of the receiver `self`, which moves `*f`

For more information about this error, try `rustc --explain E0507`.
