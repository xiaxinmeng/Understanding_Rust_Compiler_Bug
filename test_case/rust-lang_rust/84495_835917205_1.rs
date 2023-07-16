
   Compiling playground v0.0.1 (/playground)
error[E0599]: no method named `f1` found for type `i32` in the current scope
  --> src/lib.rs:15:10
   |
15 |     1i32.f1();
   |          ^^ method not found in `i32`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Trait` defines an item `f1`, perhaps you need to implement it
  --> src/lib.rs:1:1
   |
1  | trait Trait {
   | ^^^^^^^^^^^

error[E0599]: the method `f2` exists for type `i32`, but its trait bounds were not satisfied
  --> src/lib.rs:16:10
   |
16 |     1i32.f2();
   |          ^^ method cannot be called on `i32` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `i32: Trait`
           which is required by `&i32: Trait`
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Trait` defines an item `f2`, perhaps you need to implement it
  --> src/lib.rs:1:1
   |
1  | trait Trait {
   | ^^^^^^^^^^^

error[E0599]: no method named `f3` found for type `i32` in the current scope
  --> src/lib.rs:17:10
   |
17 |     1i32.f3();
   |          ^^ method not found in `i32`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Trait` defines an item `f3`, perhaps you need to implement it
  --> src/lib.rs:1:1
   |
1  | trait Trait {
   | ^^^^^^^^^^^

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
