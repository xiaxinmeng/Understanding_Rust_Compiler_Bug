
error[E0308]: if and else have incompatible types
  --> src/main.rs:14:9
   |
11 | /     if condition {
12 | |         mask(iter::once(23))
   | |         -------------------- expected because of this
13 | |     } else {
14 | |         mask(1..23)
   | |         ^^^^^^^^^^^ expected struct `std::iter::Once`, found struct `std::ops::Range`
15 | |     }
   | |_____- if and else have incompatible types
   |
   = note: expected type `impl std::iter::Iterator` (struct `std::iter::Once`)
              found type `impl std::iter::Iterator` (struct `std::ops::Range`)

error[E0308]: mismatched types
  --> src/main.rs:24:10
   |
24 |     mask(1..23)
   |          ^^^^^ expected struct `std::iter::Once`, found struct `std::ops::Range`
   |
   = note: expected type `std::iter::Once<i32>`
              found type `std::ops::Range<{integer}>`

error: aborting due to 2 previous errors
