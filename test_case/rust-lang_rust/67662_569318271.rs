
------------------------------------------
stderr:
------------------------------------------
warning: due to multiple output types requested, the explicitly specified output file name will be adapted for each output type

warning: struct is never constructed: `PrintName`
  --> /usr/local/google/home/adamperry/c/rust/src/test/mir-opt/retain-never-const.rs:10:8
   |
10 | struct PrintName<T>(T);
   |        ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: associated const is never used: `VOID`
  --> /usr/local/google/home/adamperry/c/rust/src/test/mir-opt/retain-never-const.rs:13:5
   |
13 |     const VOID: ! = panic!();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function is never used: `no_codegen`
  --> /usr/local/google/home/adamperry/c/rust/src/test/mir-opt/retain-never-const.rs:16:4
   |
16 | fn no_codegen<T>() {
   |    ^^^^^^^^^^

error: any use of this value will cause an error
  --> /usr/local/google/home/adamperry/c/rust/src/test/mir-opt/retain-never-const.rs:13:21
   |
13 |     const VOID: ! = panic!();
   |     ----------------^^^^^^^^-
   |                     |
   |                     the evaluated program panicked at 'explicit panic', /usr/local/google/home/adamperry/c/rust/src/test/mir-opt/retain-never-const.rs:13:21
   |
   = note: `#[deny(const_err)]` on by default
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0080]: erroneous constant used
  --> /usr/local/google/home/adamperry/c/rust/src/test/mir-opt/retain-never-const.rs:17:13
   |
17 |     let _ = PrintName::<T>::VOID;
   |             ^^^^^^^^^^^^^^^^^^^^ referenced constant has errors

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.

------------------------------------------

failures:
    [mir-opt] mir-opt/retain-never-const.rs

test result: FAILED. 73 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
