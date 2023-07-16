
error[E0277]: expected a `Fn<(i32, i32)>` closure, found `{integer}`
 --> <source>:2:12
  |
2 | fn g() { f(&0) }
  |            ^^ expected an `Fn<(i32, i32)>` closure, found `{integer}`
  |
  = help: the trait `Fn<(i32, i32)>` is not implemented for `{integer}`
  = note: required for the cast to the object type `dyn Fn(i32, i32) -> i32`

error: aborting due to previous error
