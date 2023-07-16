 rustc
% rustc src/test/compile-fail/issue-31109.rs
error: internal compiler error: ../src/librustc_const_eval/eval.rs:1273: could not evaluate float literal (see issue #31407)
  --> src/test/compile-fail/issue-31109.rs:14:18
   |
14 |     let _: f64 = 1234567890123456789012345678901234567890e-340; //~ ERROR could not evaluate float
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.
