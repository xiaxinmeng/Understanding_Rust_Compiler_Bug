
error[E0282]: type annotations needed
 --> t.rs:3:5
  |
2 |     let mut dirty_list = (0..5).collect();
  |         -------------- consider giving `dirty_list` a type
3 |     dirty_list.pop();
  |     ^^^^^^^^^^ cannot infer type for `_`

error: aborting due to previous error
