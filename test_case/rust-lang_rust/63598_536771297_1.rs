
error[E0637]: `&` without an explicit lifetime name cannot be used here
 --> src/lib.rs:2:9
  |
2 | fn f<T: Tr>() {}
  |         ^^ explicit lifetime name needed here
