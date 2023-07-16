
error[E0271]: type mismatch resolving `for<'a> <<BoolVec as Iterable>::Iter<'a> as std::iter::Iterator>::Item == <BoolVec as Iterable>::Item<'a>`
  --> src/main.rs:14:3
   |
13 | impl Iterable for BoolVec {
   | ------------------------- in this `impl` item
14 |   type Item<'a> = &'a bool;
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&bool`, found associated type
   |
   = note:    expected reference `&bool`
           found associated type `<BoolVec as Iterable>::Item<'_>`
   = note: consider constraining the associated type `<BoolVec as Iterable>::Item<'_>` to `&bool`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

error[E0271]: type mismatch resolving `for<'a> <<BoolVec as Iterable>::Iter<'a> as std::iter::Iterator>::Item == <BoolVec as Iterable>::Item<'a>`
  --> src/main.rs:17:28
   |
4  | trait Iterable {
   | -------------- required by `Iterable`
...
17 |   fn iter<'a>(&'a self) -> Self::Iter<'a> {
   |                            ^^^^^^^^^^^^^^ expected associated type, found `&bool`
   |
   = note: expected associated type `<BoolVec as Iterable>::Item<'_>`
                    found reference `&bool`
   = note: consider constraining the associated type `<BoolVec as Iterable>::Item<'_>` to `&bool` or calling a method that returns `<BoolVec as Iterable>::Item<'_>`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
