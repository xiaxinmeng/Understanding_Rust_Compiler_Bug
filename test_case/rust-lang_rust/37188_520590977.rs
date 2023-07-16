
error[E0282]: type annotations needed for `Qux<B>`
  --> src/main.rs:24:13
   |
24 |     let q = Qux::new();
   |         -   ^^^^^^^^ cannot infer type for `B`
   |         |
   |         consider giving `q` the explicit type `Qux<B>`, where the type parameter `B` is specified
