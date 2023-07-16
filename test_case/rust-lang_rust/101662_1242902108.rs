
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
 --> /home/gh-compiler-errors/test.rs:9:22
  |
9 |     fn uwu() -> impl OnlySized<[u8]>;
  |                      ^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `[u8]`
note: required by a bound in `OnlySized`
 --> /home/gh-compiler-errors/test.rs:4:17
  |
4 | trait OnlySized<T> {}
  |                 ^ required by this bound in `OnlySized`
help: consider relaxing the implicit `Sized` restriction
  |
4 | trait OnlySized<T: ?Sized> {}
  |                  ++++++++
