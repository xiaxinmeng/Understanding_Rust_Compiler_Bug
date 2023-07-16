
warning: any use of this value will cause an error
  --> src/main.rs:26:9
   |
9  | /     const CHECK: () = if std::mem::size_of::<Self>() == 0 {
10 | |         // would like to use the length of `IS_ZST` + the length of the
11 | |         // return value of `type_name::<Self>()` here, but that won't work because you can't use
12 | |         // generic parameters in array lengths, not even in repeat expression lengths.
...  |
26 | |         panic!(x)
   | |         ^^^^^^^^^ the evaluated program panicked at '() is zst', src/main.rs:26:9
27 | |     };
   | |______-
   |
note: the lint level is defined here
  --> src/main.rs:8:12
   |
8  |     #[warn(const_err)]
   |            ^^^^^^^^^
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: erroneous constant encountered
  --> src/main.rs:35:9
   |
35 |         T::CHECK;
   |         ^^^^^^^^

error: aborting due to previous error; 1 warning emitted
