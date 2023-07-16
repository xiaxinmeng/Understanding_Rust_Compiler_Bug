
error[E0271]: type mismatch resolving `<usize as SliceIndex<[A]>>::Output == String`
  --> test2.rs:11:21
   |
11 |     let b: String = widths[a];
   |                     ^^^^^^^^^ expected struct `A`, found struct `String`

