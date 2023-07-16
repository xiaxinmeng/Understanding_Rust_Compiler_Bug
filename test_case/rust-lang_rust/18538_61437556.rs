
---- [run-pass] run-pass/match-range.rs stdout ----

        error: compilation failed!
        status: exit code: 101
        command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/alex/code/rust4/src/test/run-pass/match-range.rs -L x86_64-unknown-linux-gnu/test/run-pass --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/match-range.stage2-x86_64-unknown-linux-gnulibaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/run-pass/match-range.stage2-x86_64-unknown-linux-gnu --cfg rtopt --cfg debug -C rpath -L x86_64-unknown-linux-gnu/rt
        stdout:
        ------------------------------------------

        ------------------------------------------
        stderr:
        ------------------------------------------
        /home/alex/code/rust4/src/test/run-pass/match-range.rs:32:7: 32:12 error: lower range bound must be less than upper [E0030]
        /home/alex/code/rust4/src/test/run-pass/match-range.rs:32       -7...5 => {}
                                                                        ^~~~~
        error: aborting due to previous error

        ------------------------------------------

        task '[run-pass] run-pass/match-range.rs' panicked at 'explicit panic', /home/alex/code/rust4/src/compiletest/runtest.rs:1485



