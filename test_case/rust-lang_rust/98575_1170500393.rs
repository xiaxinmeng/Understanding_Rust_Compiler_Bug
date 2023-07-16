
error[[E0433]](https://doc.rust-lang.org/nightly/error-index.html#E0433): failed to resolve: use of undeclared type `X`
  --> src/lib.rs:10:33
   |
10 | fn bar<T: foo::X>() where <T as X>::Y: Clone {
   |                                 ^ use of undeclared type `X`

error[[E0282]](https://doc.rust-lang.org/nightly/error-index.html#E0282): type annotations needed
  --> src/lib.rs:10:8
   |
10 | fn bar<T: foo::X>() where <T as X>::Y: Clone {
   |        ^ cannot infer type for type parameter `T`
