
error[E0308]: mismatched types
 --> test.rs:5:13
  |
5 |         r = 10;
  |             ^^ expected mutable reference, found integral variable
  |
  = note: expected type `&mut {integer}`
             found type `{integer}`
  = help: try with `&mut 10`
