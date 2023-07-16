
RUST ➜  rust2 git:(prefer-non-err-candidates) ✗ rustc +rust2 ~/test.rs
error[E0412]: cannot find type `Type` in this scope
 --> /home/ubuntu/test.rs:6:14
  |
6 | impl Generic<Type> for S {}
  |     -        ^^^^ not found in this scope
  |     |
  |     help: you might be missing a type parameter: `<Type>`

error: aborting due to previous error
