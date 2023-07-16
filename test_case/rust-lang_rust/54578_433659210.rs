
error[E0308]: try expression alternatives have incompatible types
 --> segrif.rs:5:9
  |
5 |     foo(bar()?);
  |         ^^^^^^ expected &i32, found i32
  |
  = note: expected type `&i32`
             found type `i32`

error: aborting due to previous error
