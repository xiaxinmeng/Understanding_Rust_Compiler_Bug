
error[E0505]: cannot move out of `v` because it is borrowed
 --> src/main.rs:6:10
  |
5 |     let _foo = foo(&v);
  |                    -- borrow of `v` occurs here
6 |     drop(v); // do something with v
  |          ^ move out of `v` occurs here
7 | }
  | - borrow might be used here, when `_foo` is dropped and runs the destructor for type `impl Iterator<Item = String> + '_`

For more information about this error, try `rustc --explain E0505`.
error: could not compile `lf` due to previous error
