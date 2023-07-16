rust
error[[E0284]](https://doc.rust-lang.org/stable/error-index.html#E0284): type annotations needed: cannot satisfy `<T as Get>::T<'b> == T`
  --> src/lib.rs:13:5
   |
13 |     <T as Get>::get::<'b>(other)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot satisfy `<T as Get>::T<'b> == T`
