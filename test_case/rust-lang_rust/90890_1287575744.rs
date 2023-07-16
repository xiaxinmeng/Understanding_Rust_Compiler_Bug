
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
  --> src/lib.rs:2:3
   |
2  |   apply(x, id)
   |   ^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'r> FnOnce<(&'r (),)>`
              found trait `for<'b> FnOnce<(&'b (),)>`
note: the lifetime requirement is introduced here
  --> src/lib.rs:11:17
   |
11 |   F: Fn(&()) -> T,
   |
