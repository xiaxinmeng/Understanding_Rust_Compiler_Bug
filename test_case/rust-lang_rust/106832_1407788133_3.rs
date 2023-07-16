rust
error[[E0284]](https://doc.rust-lang.org/stable/error-index.html#E0284): type annotations needed: cannot satisfy `<T as Get>::T<'_> == T`
  --> src/lib.rs:13:11
   |
13 |     other.get()
   |           ^^^ cannot satisfy `<T as Get>::T<'_> == T`
