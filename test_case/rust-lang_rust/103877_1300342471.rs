plain
ii.i..ii......i......i.ii...i.......i....F.i..i...................i..........i..i....... 88/112
........i...............
failures:

---- src/builtin.rs - builtin::LONG_RUNNING_CONST_EVAL (line 3283) stdout ----
error: expected `;`, found `}`
  |
8 | }
8 | }
  |  ^ help: add `;` here
9 | } _doctest_main_src_builtin_rs_3283_0() }
  | - unexpected token
error: aborting due to previous error

Couldn't compile the test.


failures:
    src/builtin.rs - builtin::LONG_RUNNING_CONST_EVAL (line 3283)

test result: FAILED. 94 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.95s

error: doctest failed, to rerun pass `-p rustc_lint_defs --doc`
