
error[E0308]: mismatched types
  --> test.rs:14:25
   |
14 |     let vec2 = Baz(vec!(1, 2, 3));  // Offending line
   |                         ^ expected struct `Foo`, found integral variable
   |
   = note: expected type `Foo`
              found type `{integer}`

error: aborting due to previous error(s)
