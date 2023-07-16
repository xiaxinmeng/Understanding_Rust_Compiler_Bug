 plain
rustc 1.14.0-nightly (cae6ab1c4 2016-11-05)
error[E0308]: mismatched types
  --> <anon>:14:25
   |
14 |     let vec2 = Baz(vec!(1, 2, 3));  // Offending line
   |                         ^ expected struct `Foo`, found integral variable
   |
   = note: expected type `_`
   = note:    found type `{integer}`

error[E0308]: mismatched types
  --> <anon>:14:20
   |
14 |     let vec2 = Baz(vec!(1, 2, 3));  // Offending line
   |                    ^^^^^^^^^^^^^ expected slice, found array of 3 elements
   |
   = note: expected type `Box<[Foo]>`
   = note:    found type `Box<[{integer}; 3]>`
   = note: this error originates in a macro outside of the current crate

error: aborting due to 2 previous errors
