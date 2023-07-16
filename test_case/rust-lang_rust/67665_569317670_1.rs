
error[E0277]: the size for values of type `T` cannot be known at compilation time
 --> src/lib.rs:5:18
  |
5 | fn foo2<T>(_: T) where T: ?Sized {
  |                  ^              - help: consider further restricting type parameter `T`: `, T: std::marker::Sized`
  |                  |
  |                  doesn't have a size known at compile-time
