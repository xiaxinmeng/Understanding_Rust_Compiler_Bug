
failures:

---- [compile-fail] compile-fail/dropck_vec_cycle_checked.rs stdout ----

error: compile-fail test compiled successfully!
status: exit code: 0
command: x86_64-apple-darwin/stage2/bin/rustc /Users/ABeingessner/dev/rust/src/test/compile-fail/dropck_vec_cycle_checked.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/dropck_vec_cycle_checked.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/dropck_vec_cycle_checked.stage2-x86_64-apple-darwin --cfg rtopt -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/ABeingessner/dev/rust/src/test/compile-fail/dropck_vec_cycle_checked.rs:21:14: 21:22 warning: unknown `allow` attribute: `unstable`, #[warn(unknown_lints)] on by default
/Users/ABeingessner/dev/rust/src/test/compile-fail/dropck_vec_cycle_checked.rs:21     #![allow(unstable)]
                                                                                               ^~~~~~~~

------------------------------------------

thread '[compile-fail] compile-fail/dropck_vec_cycle_checked.rs' panicked at 'explicit panic', /Users/ABeingessner/dev/rust/src/compiletest/runtest.rs:1497


---- [compile-fail] compile-fail/issue-25199.rs stdout ----

error: compile-fail test compiled successfully!
status: exit code: 0
command: x86_64-apple-darwin/stage2/bin/rustc /Users/ABeingessner/dev/rust/src/test/compile-fail/issue-25199.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/issue-25199.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/issue-25199.stage2-x86_64-apple-darwin --cfg rtopt -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[compile-fail] compile-fail/issue-25199.rs' panicked at 'explicit panic', /Users/ABeingessner/dev/rust/src/compiletest/runtest.rs:1497


---- [compile-fail] compile-fail/vec-must-not-hide-type-from-dropck.rs stdout ----

error: compile-fail test compiled successfully!
status: exit code: 0
command: x86_64-apple-darwin/stage2/bin/rustc /Users/ABeingessner/dev/rust/src/test/compile-fail/vec-must-not-hide-type-from-dropck.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/vec-must-not-hide-type-from-dropck.stage2-x86_64-apple-darwin.compile-fail.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/vec-must-not-hide-type-from-dropck.stage2-x86_64-apple-darwin --cfg rtopt -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/ABeingessner/dev/rust/src/test/compile-fail/vec-must-not-hide-type-from-dropck.rs:32:14: 32:22 warning: unknown `allow` attribute: `unstable`, #[warn(unknown_lints)] on by default
/Users/ABeingessner/dev/rust/src/test/compile-fail/vec-must-not-hide-type-from-dropck.rs:32     #![allow(unstable)]
                                                                                                         ^~~~~~~~

------------------------------------------

thread '[compile-fail] compile-fail/vec-must-not-hide-type-from-dropck.rs' panicked at 'explicit panic', /Users/ABeingessner/dev/rust/src/compiletest/runtest.rs:1497



failures:
    [compile-fail] compile-fail/dropck_vec_cycle_checked.rs
    [compile-fail] compile-fail/issue-25199.rs
    [compile-fail] compile-fail/vec-must-not-hide-type-from-dropck.rs

test result: FAILED. 1764 passed; 3 failed; 12 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /Users/ABeingessner/dev/rust/src/compiletest/compiletest.rs:252
make: *** [tmp/check-stage2-T-x86_64-apple-darwin-H-x86_64-apple-darwin-cfail.ok] Error 101
