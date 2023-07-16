
error[E0423]: expected value, found struct `std::collections::HashMap`
 --> src/main.rs:5:37
  |
5 |     let uuid = Arc::new(RwLock::new(std::collections::HashMap<i32, i64>::new()));
  |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `std::collections::HashMap { base: val }`
 --> /rustc/ee0412d1ef81efcfabe7f66cd21476ca85d618b1/library/std/src/collections/hash/map.rs:214:1
  |
  = note: `std::collections::HashMap` defined here

error[E0423]: expected value, found builtin type `i32`
 --> src/main.rs:5:63
  |
5 |     let uuid = Arc::new(RwLock::new(std::collections::HashMap<i32, i64>::new()));
  |                                                               ^^^ not a value

error[[E0423]](https://doc.rust-lang.org/nightly/error-index.html#E0423): expected value, found builtin type `i64`
 --> src/main.rs:5:68
  |
5 |     let uuid = Arc::new(RwLock::new(std::collections::HashMap<i32, i64>::new()));
  |                                                                    ^^^ not a value

error[E0425]: cannot find external crate `new` in the crate root
 --> src/main.rs:5:74
  |
5 |     let uuid = Arc::new(RwLock::new(std::collections::HashMap<i32, i64>::new()));
  |                                                                          ^^^ not found in the crate root

error[E0061]: this function takes 1 argument but 2 arguments were supplied
 --> src/main.rs:5:25
  |
5 |     let uuid = Arc::new(RwLock::new(std::collections::HashMap<i32, i64>::new()));
  |                         ^^^^^^^^^^^                                ----------- argument of type `bool` unexpected
  |
note: associated function defined here
 --> /rustc/ee0412d1ef81efcfabe7f66cd21476ca85d618b1/library/std/src/sync/rwlock.rs:160:18
help: remove the extra argument
  |
5 |     let uuid = Arc::new(RwLock::new(std::collections::HashMap<i32));
  |                                    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
