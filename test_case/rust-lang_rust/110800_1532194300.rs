plain

   Doc-tests rustc_middle

running 104 tests
iiiiiiiiiiiiiiii.i.i.....i..ii.iiii...i...i...i...i...i...i...i.iiiiiiii...iiii...ii.iii  88/104
iF....F.........
failures:

---- src/ty/typeck_results.rs - ty::typeck_results::TypeckResults::closure_fake_reads (line 169) stdout ----
---- src/ty/typeck_results.rs - ty::typeck_results::TypeckResults::closure_fake_reads (line 169) stdout ----
error[E0381]: used binding `x` isn't initialized
  |
3 | let x: u8;
  |     - binding declared here but left uninitialized
  |     - binding declared here but left uninitialized
4 | let c = || match x { _ => () };
  |                  ^ `x` used here but it isn't initialized
help: consider assigning a value
  |
3 | let x: u8 = 0;
  |           +++
  |           +++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0381`.
Couldn't compile the test.
---- src/ty/typeck_results.rs - ty::typeck_results::TypeckResults::closure_fake_reads (line 177) stdout ----
error: expected `;`, found `}`
  |
5 | }
5 | }
  |  ^ help: add `;` here
6 | } _doctest_main_src_ty_typeck_results_rs_177_0() }
  | - unexpected token
error[E0425]: cannot find value `t` in this scope
 --> src/ty/typeck_results.rs:179:20
  |
4 |     let (t1, t2) = t;
---
    src/ty/typeck_results.rs - ty::typeck_results::TypeckResults::closure_fake_reads (line 177)

test result: FAILED. 52 passed; 2 failed; 50 ignored; 0 measured; 0 filtered out; finished in 809.78ms

error: doctest failed, to rerun pass `-p rustc_middle --doc`
