
error[E0282]: type annotations needed
 --> src/main.rs:5:13
  |
4 |         for tile in row {
  |                     --- the element type for this iterator is not specified
5 |             *tile = 0;
  |             ^^^^^ cannot infer type for `_`
