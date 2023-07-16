
error[E0308]: mismatched types
  --> src/main.rs:12:7
   |
12 |     x(2i32);
   |       ^^^^ expected associated type, found `i32`
   |
   = note: expected associated type `<S as T<'_>>::A`
                         found type `i32`
   = note: consider constraining the associated type `<S as T<'_>>::A` to `i32` or calling a method that returns `<S as T<'_>>::A`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
