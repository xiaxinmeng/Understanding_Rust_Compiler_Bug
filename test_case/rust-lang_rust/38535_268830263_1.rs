rust
error[E0282]: unable to infer enough type information about `T`
  --> /home/simon/projects/servo2/components/style/font_face.rs:66:13
   |
66 |             Err(err) => return Err(err),
   |             ^^^^^^^^ cannot infer type for `T`
   |
   = note: type annotations or generic parameter binding required

error: aborting due to previous error
