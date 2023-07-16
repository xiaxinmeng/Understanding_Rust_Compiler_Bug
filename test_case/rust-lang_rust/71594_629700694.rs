
error[E0689]: can't call method `count_ones` on ambiguous numeric type `{integer}`
 --> src/main.rs:3:20
  |
3 |     let ones = int.count_ones();
  |                    ^^^^^^^^^^
  |
help: you must specify a type for this binding, like `i32`
  |
2 |     let int: i32 = 42;
  |         ^^^^^^^^
