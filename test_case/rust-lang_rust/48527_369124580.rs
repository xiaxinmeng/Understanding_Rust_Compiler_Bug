
zmd@ReflectiveCoherence:~/Code/rust$ rustc +stage1 recover.rs 
error: unexpected `,` in pattern
 --> recover.rs:2:10
  |
2 |     let a, b = (1, 2);
  |         -^-- help: try adding parentheses: `(a, b)`

error[E0277]: cannot divide `{integer}` by `{float}`
 --> recover.rs:3:15
  |
3 |     let c = a / 1.5;
  |               ^ no implementation for `{integer} / {float}`
  |
  = help: the trait `std::ops::Div<{float}>` is not implemented for `{integer}`

error: aborting due to 2 previous errors
