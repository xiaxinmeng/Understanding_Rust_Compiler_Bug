
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:14:9
   |
5  |   fn mask<T: Iterator<Item = i32>>(iter: T) -> impl Iterator<Item = i32> {
   |                                                ------------------------- the found opaque type
...
11 | /     if condition {
12 | |         mask(iter::once(23))
   | |         -------------------- expected because of this
13 | |     } else {
14 | |         mask(1..23)
   | |         ^^^^^^^^^^^ expected struct `std::iter::Once`, found struct `std::ops::Range`
15 | |     }
   | |_____- `if` and `else` have incompatible types
   |
   = note:     expected type `impl Iterator` (struct `std::iter::Once`)
           found opaque type `impl Iterator` (struct `std::ops::Range`)
help: you could change the return type to be a boxed trait object
   |
10 | fn test_clear_error_message(condition: bool) -> Box<dyn Iterator<Item = i32>> {
   |                                                 ~~~~~~~                     +
help: if you change the return type to expect trait objects, box the returned expressions
   |
12 ~         Box::new(mask(iter::once(23)))
13 |     } else {
14 ~         Box::new(mask(1..23))
   |

error[E0308]: mismatched types
  --> src/main.rs:24:10
   |
24 |     mask(1..23)
   |          ^^^^^ expected struct `std::iter::Once`, found struct `std::ops::Range`
   |
   = note: expected struct `std::iter::Once<i32>`
              found struct `std::ops::Range<{integer}>`
