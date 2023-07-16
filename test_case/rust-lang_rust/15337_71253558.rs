
---- [pretty] run-pass/zero_sized_subslice_match.rs stdout ----

error: pretty-printing failed in round 1
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - -Zunstable-options --pretty normal --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass/zero_sized_subslice_match.stage2-x86_64-unknown-linux-gnulibaux --cfg rtopt -Z time-passes -Z time-llvm-passes -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
<anon>:1:1: 1:5 error: expected item, found `time`
<anon>:1 time: 0.000    parsing
         ^~~~

------------------------------------------

thread '[pretty] run-pass/zero_sized_subslice_match.rs' panicked at 'explicit panic', /home/emacs/build/rust/src/compiletest/runtest.rs:1458
