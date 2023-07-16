
failures:

---- [run-fail] run-fail/qquote.rs stdout ----

error: compilation failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage1/bin/rustc /home/rprichard/work/rust-staging2/src/test/run-fail/qquote.rs -L x86_64-unknown-linux-gnu/test/run-fail/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-fail/qquote.stage1-x86_64-unknown-linux-gnu.run-fail.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/run-fail/qquote.stage1-x86_64-unknown-linux-gnu --cfg rtopt --cfg ndebug -C rpath -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/rprichard/work/rust-staging2/src/test/run-fail/qquote.rs:17:1: 17:21 error: can't find crate for `syntax`
/home/rprichard/work/rust-staging2/src/test/run-fail/qquote.rs:17 extern crate syntax;
                                                                  ^~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[run-fail] run-fail/qquote.rs' panicked at 'explicit panic', /home/rprichard/work/rust-staging2/src/compiletest/runtest.rs:1512



failures:
    [run-fail] run-fail/qquote.rs

test result: FAILED. 97 passed; 1 failed; 5 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /home/rprichard/work/rust-staging2/src/compiletest/compiletest.rs:256
make: *** [tmp/check-stage1-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-rfail.ok] Error 101
make: *** Waiting for unfinished jobs....
