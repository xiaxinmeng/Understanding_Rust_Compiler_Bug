plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::map_into (line 3285) stdout ----
error[E0425]: cannot find value `a` in this scope
 --> src/iter/traits/iterator.rs:3288:22
  |
6 | let conv: Vec<u32> = a.iter().map_into().collect();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.

failures:
    src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::map_into (line 3285)

test result: FAILED. 3914 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 50.96s

error: doctest failed, to rerun pass `-p core --doc`
