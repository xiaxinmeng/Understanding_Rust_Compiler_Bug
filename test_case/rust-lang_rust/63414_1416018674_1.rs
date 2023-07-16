
error[E0529]: expected an array or slice, found `Vec<{integer}>`
 --> src/main.rs:4:14
  |
3 |     match Some(x) {
  |           ------- help: consider using `as_deref` here: `Some(x).as_deref()`
4 |         Some([..]) => {}
  |              ^^^^ pattern cannot match with input type `Vec<{integer}>`
