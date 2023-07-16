
error[E0401]: can't use generic parameters from outer function
 --> src/lib.rs:2:46
  |
1 | fn foo<T>() {
  |        - type parameter from outer function
2 |     const SIZE: usize = core::mem::size_of::<T>();
  |     -----                                    ^ use of generic parameter from outer function
  |     | 
  |     this is a separate item from `fn`
