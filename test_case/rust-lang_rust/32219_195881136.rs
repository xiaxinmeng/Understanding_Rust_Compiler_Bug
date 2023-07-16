
---- [run-pass] run-pass/ifmt.rs stdout ----

error: compilation failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/travis/build/rust-lang/rust/src/test/run-pass/ifmt.rs -L x86_64-unknown-linux-gnu/test/run-pass/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/ifmt.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/run-pass/ifmt.stage2-x86_64-unknown-linux-gnu --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/travis/build/rust-lang/rust/src/test/run-pass/ifmt.rs:15:10: 15:26 error: lint unknown_features has been renamed to unused_features
/home/travis/build/rust-lang/rust/src/test/run-pass/ifmt.rs:15 #![allow(unknown_features)]
                                                                        ^~~~~~~~~~~~~~~~
/home/travis/build/rust-lang/rust/src/test/run-pass/ifmt.rs:13:9: 13:17 note: lint level defined here
/home/travis/build/rust-lang/rust/src/test/run-pass/ifmt.rs:13 #![deny(warnings)]
                                                                       ^~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[run-pass] run-pass/ifmt.rs' panicked at 'explicit panic', /home/travis/build/rust-lang/rust/src/compiletest/runtest.rs:1651
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/ifmt.rs
