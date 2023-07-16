
error[E0401]: can't use type parameters from outer function
 --> src/lib.rs:3:16
  |
2 | fn a<T: Clone>(x: T) {
  |    - - type variable from outer function
  |    |
  |    try adding a local type parameter in this method instead
3 |     const foo: T = x;
  |                ^ use of type variable from outer function

error[E0434]: can't capture dynamic environment in a fn item
 --> src/lib.rs:3:20
  |
3 |     const foo: T = x;
  |                    ^
  |
  = help: use the `|| { ... }` closure form instead

error: aborting due to 2 previous errors
