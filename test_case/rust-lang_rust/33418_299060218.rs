
$ rustc first-example.rs
error: expected one of `(`, `?`, `for`, `where`, `{`, lifetime, or path, found `!`
 --> test.rs:1:13
  |
1 | trait Qqq : !Sized {
  |            -^ unexpected token
  |            |
  |            expected one of 7 possible tokens here

error: aborting due to previous error

$ rustc second-example.rs
error: `?Trait` is not permitted in supertraits
 --> test.rs:1:14
  |
1 | trait Qqq : ?Sized {
  |              ^^^^^
  |
  = note: traits are `?Sized` by default

error: main function not found

error: aborting due to 2 previous errors
