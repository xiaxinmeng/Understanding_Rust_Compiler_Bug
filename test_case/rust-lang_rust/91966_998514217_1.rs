
error[E0308]: mismatched types
 --> src/main.rs:5:10
  |
5 |     func(&f);
  |          ^^ one type is more general than the other
  |
  = note: expected reference `&i32`
             found reference `&i32`
