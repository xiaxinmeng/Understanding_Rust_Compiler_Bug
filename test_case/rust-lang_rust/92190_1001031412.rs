text
error[E0277]: the trait bound `MyEnum: Hash` is not satisfied
  --> src/main.rs:11:18
   |
11 |     let _hash_map = HashMap::from([
   |                     ^^^^^^^^^^^^^ the trait `Hash` is not implemented for `MyEnum`
   |
   = note: required because of the requirements on the impl of `From<[(MyEnum, {integer}); 3]>` for `HashMap<MyEnum, {integer}>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
