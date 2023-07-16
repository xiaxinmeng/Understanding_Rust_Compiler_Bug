
error[E0277]: the trait bound `B: std::convert::AsRef<u32>` is not satisfied
  --> src/main.rs:14:5
   |
10 |     B: AsRef<A::Item>,
   |                       - help: consider further restricting type parameter `B`: `, B: std::convert::AsRef<u32>`
...
14 |     fn foo<T: AsRef<Self::Foo>>(&self, t: T) {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::AsRef<u32>` is not implemented for `B`
   |
   = note: required because of the requirements on the impl of `Example` for `(A, B)`

error[E0277]: the trait bound `T: std::convert::AsRef<()>` is not satisfied
  --> src/main.rs:14:5
   |
14 |     fn foo<T: AsRef<Self::Foo>>(&self, t: T) {}
   |     ^^^^^^^--^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |      |
   |     |      help: consider further restricting this bound: `T: std::convert::AsRef<()> +`
   |     the trait `std::convert::AsRef<()>` is not implemented for `T`
   |
   = note: required by `std::convert::AsRef`
