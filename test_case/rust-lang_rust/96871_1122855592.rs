
error[E0063]: missing fields `n1` and `n2` in initializer of `A`
  --> src/main.rs:11:13
   |
11 |     let x = A {
   |             ^ missing `n1` and `n2`
   |
help: to set the remaining fields from `x`, separate the last named field with a comma
   |
13 |         field2: String::from("yo"),..x,
   |                                   +
