
error[E0401]: can't use generic parameters for `const`s
 --> file.rs:4:17
  |
3 | fn bar<T: Foo>(n: T) {
  |        - type variable
4 |     const BASE: T = T::FOO;
  |                 ^ can't use generic parameter for a `const`

error[E0401]: can't use generic parameters for `const`s
 --> file.rs:4:21
  |
3 | fn bar<T: Foo>(n: T) {
  |        - type variable from outer function
4 |     const BASE: T = T::FOO;
  |                     ^^^^^^ can't use generic parameter for a `const`
