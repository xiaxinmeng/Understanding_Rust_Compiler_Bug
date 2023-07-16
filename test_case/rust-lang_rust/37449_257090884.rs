
failures:

---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----

error: compilation failed!
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /buildslave/rust-buildbot/slave/auto-linux-64-opt/build/src/test/codegen-units/partitioning/local-inlining.rs -L x86_64-unknown-linux-gnu/test/codegen-units/ --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining.stage2-x86_64-unknown-linux-gnu.codegen-units.libaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/codegen-units/partitioning/local-inlining.stage2-x86_64-unknown-linux-gnu --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt -Zprint-trans-items=lazy -Zincremental=tmp/partitioning-tests/local-inlining
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: Could not create incremental compilation crate directory `tmp/partitioning-tests/local-inlining/local_inlining-4pp8dn3374zr`: No such file or directory (os error 2)

error: aborting due to previous error


------------------------------------------

thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', /buildslave/rust-buildbot/slave/auto-linux-64-opt/build/src/tools/compiletest/src/runtest.rs:2377
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [codegen-units] codegen-units/partitioning/local-inlining.rs

test result: FAILED. 31 passed; 1 failed; 3 ignored; 0 measured

thread 'main' panicked at 'Some tests failed', /buildslave/rust-buildbot/slave/auto-linux-64-opt/build/src/tools/compiletest/src/main.rs:298
/buildslave/rust-buildbot/slave/auto-linux-64-opt/build/mk/tests.mk:771: recipe for target 'tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-codegen-units.ok' failed
make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-codegen-units.ok] Error 101
