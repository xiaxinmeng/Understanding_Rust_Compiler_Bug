
failures:

---- [compile-fail] compile-fail/issue-21146.rs stdout ----

error: no more error patterns to match against
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage1/bin/rustc /home/nham/code/other/rust2/src/test/compile-fail/issue-21146.rs -L x86_64-unknown-linux-gnu/test/compile-fail/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail/issue-21146.stage1-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/compile-fail/issue-21146.stage1-x86_64-unknown-linux-gnu --cfg rtopt -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/nham/code/other/rust2/src/test/compile-fail/../auxiliary/issue-21146-inc.rs:13:1: 13:12 error: expected item, found `parse_error`
/home/nham/code/other/rust2/src/test/compile-fail/../auxiliary/issue-21146-inc.rs:13 parse_error
                                                                                     ^~~~~~~~~~~

------------------------------------------

thread '[compile-fail] compile-fail/issue-21146.rs' panicked at 'explicit panic', /home/nham/code/other/rust2/src/compiletest/runtest.rs:1521
