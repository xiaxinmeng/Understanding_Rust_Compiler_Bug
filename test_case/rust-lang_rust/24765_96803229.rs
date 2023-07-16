

failures:

---- [run-pass] run-pass/issue-868.rs stdout ----

error: compilation failed!
status: signal: 4
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-pass/issue-868.rs -L x86_64-unknown-linux-gnu/test/run-pass/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/issue-868.stage2-x86_64-unknown-linux-gnu.run-pass.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/run-pass/issue-868.stage2-x86_64-unknown-linux-gnu --cfg rtopt --cfg ndebug -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-pass/issue-868.rs:25:13: 25:19 warning: unnecessary parentheses around assigned value, #[warn(unused_parens)] on by default
/home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/run-pass/issue-868.rs:25     let _ = (||{});
                                                                                                              ^~~~~~

------------------------------------------

thread '[run-pass] run-pass/issue-868.rs' panicked at 'explicit panic', /home/ubuntu/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/compiletest/runtest.rs:1512



failures:
    [run-pass] run-pass/issue-868.rs

test result: FAILED. 2039 passed; 1 failed; 25 ignored; 0 measured
