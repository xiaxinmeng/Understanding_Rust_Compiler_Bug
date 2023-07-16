
error[E0271]: type mismatch resolving `<std::slice::Iter<'_, {integer}> as std::iter::Iterator>::Item == i32`
 --> src/main.rs:4:5
  |
1 | fn consume<I: Iterator<Item = i32>>(_: I) {}
  |    -------             ---------- required by this bound in `consume`
...
4 |     consume([1, 2, 3].into_iter());
  |     ^^^^^^^ expected `i32`, found reference
  |
  = note:   expected type `i32`
          found reference `&{integer}`
