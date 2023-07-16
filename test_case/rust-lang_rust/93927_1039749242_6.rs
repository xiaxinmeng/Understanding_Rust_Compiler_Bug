
 --> test3.rs:4:7
  |
4 |     a + b
  |     - ^ - B
  |     |
  |     A
  |
help: consider restricting type parameter `A`
  |
3 | fn add<A: std::ops::Add<_>, B, C>(a: A, b: B) -> C {
  |         ++++++++++++++++++
