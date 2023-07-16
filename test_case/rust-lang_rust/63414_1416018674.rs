
error[E0529]: expected an array or slice, found `Vec<{integer}>`
 --> src/main.rs:4:9
  |
3 |     match x {
  |           - help: consider slicing here: `x[..]`
4 |         [..] => {}
  |         ^^^^ pattern cannot match with input type `Vec<{integer}>`
