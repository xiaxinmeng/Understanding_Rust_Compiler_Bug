
failures:

---- [run-pass] run-pass/out-of-stack-no-split.rs stdout ----

        error: test run failed!
        status: exit code: 101
        command: x86_64-unknown-linux-gnu/test/run-pass/out-of-stack-no-split.stage2-x86_64-unknown-linux-gnu 
        stdout:
        ------------------------------------------

        ------------------------------------------
        stderr:
        ------------------------------------------
        task '<main>' failed at 'assertion failed: error.as_slice().contains("has overflowed its stack")', /home/alex/code/rust4/src/test/run-pass/out-of-stack-no-split.rs:43

        ------------------------------------------

        task '[run-pass] run-pass/out-of-stack-no-split.rs' failed at 'explicit failure', /home/alex/code/rust4/src/compiletest/runtest.rs:1436



failures:
    [run-pass] run-pass/out-of-stack-no-split.rs

test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured

task '<main>' failed at 'Some tests failed', /home/alex/code/rust4/src/compiletest/compiletest.rs:257
make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-rpass.ok] Error 101
