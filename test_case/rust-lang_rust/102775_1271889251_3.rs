
error[E0282]: type annotations needed
  --> src/main.rs:10:9
   |
10 |     let u = a[m.into()];
   |         ^
   |
help: consider giving `u` an explicit type
   |
10 |     let u: _ = a[m.into()];
   |          +++
