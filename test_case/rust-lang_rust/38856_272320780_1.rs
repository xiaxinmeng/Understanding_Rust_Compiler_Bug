
error[E0507]: cannot move out of borrowed content
 --> test.rs:8:27
  |
8 |     for (ref k, ref v) in *x {
  |                           ^^ cannot move out of borrowed content
