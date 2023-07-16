
rustc 1.16.0-nightly (f0b420759 2017-01-19)
error[E0308]: mismatched types
 --> <anon>:2:13
  |
2 |     if true {}  
  |             ^^ expected i32, found ()
  |
  = note: expected type `i32`
  = note:    found type `()`

error[E0317]: if may be missing an else clause
 --> <anon>:2:5
  |
2 |     if true {}  
  |     ^^^^^^^^^^ expected (), found i32
  |
  = note: expected type `()`
  = note:    found type `i32`

error: aborting due to 2 previous errors
