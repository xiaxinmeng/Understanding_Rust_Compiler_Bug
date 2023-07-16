
error[E0308]: mismatched types
  --> src/main.rs:31:9
   |
29 | fn f1_with_named_generic<F: FnMut(u32) -> u32, T: Iterator<Item=u32>>(a: u32) -> impl FnOnce(F) -> T {
   |                                                - this type parameter
30 |     move |f: F| {
31 |         (0..a).map(f)
   |         ^^^^^^^^^^^^^ expected type parameter `T`, found struct `std::iter::Map`
   |
   = note: expected type parameter `T`
                      found struct `std::iter::Map<std::ops::Range<u32>, F>`

