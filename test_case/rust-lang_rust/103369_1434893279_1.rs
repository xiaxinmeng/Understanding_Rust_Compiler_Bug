
error[[E0080]](https://doc.rust-lang.org/stable/error_codes/E0080.html): evaluation of constant value failed
  --> src/lib.rs:11:9
   |
11 |         panic!("Some error occurred")
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'Some error occurred', src/lib.rs:11:9
   |
note: inside `my_fn`
  --> src/lib.rs:11:9
   |
11 |         panic!("Some error occurred")
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `<() as ConstGenericTrait<{my_fn(2)}>>::{constant#0}`
  --> src/lib.rs:7:25
   |
7  | impl ConstGenericTrait<{my_fn(2)}> for () {}
   |                         ^^^^^^^^
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[[E0119]](https://doc.rust-lang.org/stable/error_codes/E0119.html): conflicting implementations of trait `ConstGenericTrait<3>` for type `()`
 --> src/lib.rs:7:1
  |
3 | impl ConstGenericTrait<{my_fn(3)}> for () {}
  | ----------------------------------------- first implementation here
...
7 | impl ConstGenericTrait<{my_fn(2)}> for () {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `()`
