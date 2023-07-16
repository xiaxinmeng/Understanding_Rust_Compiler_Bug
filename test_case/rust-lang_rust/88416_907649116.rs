
error: `<` is interpreted as a start of generic arguments for `f64`, not a comparison
 --> <source>:2:17
  |
2 |     if 1 as f64 < 0.0 && 1 >= 0 {
  |        -------- ^ ------ interpreted as generic arguments
  |        |        |
  |        |        not interpreted as comparison
  |        help: try comparing the cast value: `(1 as f64)`

error: aborting due to previous error
