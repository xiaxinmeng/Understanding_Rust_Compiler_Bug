
$ rustc --cfg A=B gcc/testsuite/rust/compile/cfg5.rs && echo OK
error: invalid `--cfg` argument: `A=B` (expected `key` or `key="value"`)
