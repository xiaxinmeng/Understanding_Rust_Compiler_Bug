
error[E0308]: mismatched types
  --> src/main.rs:19:9
   |
18 |     fn f<'a>(&'a self) -> <&'a Self as Iter>::Type where &'a Self: Iter {
   |                           ------------------------ expected `<&'a S as Iter>::Type` because of return type
19 |         0usize..10
   |         ^^^^^^^^^^ expected associated type, found struct `std::ops::Range`
   |
   = note: expected associated type `<&'a S as Iter>::Type`
                       found struct `std::ops::Range<usize>`
   = note: consider constraining the associated type `<&'a S as Iter>::Type` to `std::ops::Range<usize>` or calling a method that returns `<&'a S as Iter>::Type`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
