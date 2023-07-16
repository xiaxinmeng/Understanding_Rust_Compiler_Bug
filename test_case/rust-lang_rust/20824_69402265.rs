
failures:

---- [pretty] run-pass/matches-macro.rs stdout ----

error: pretty-printed source (expanded) does not typecheck
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc - --no-trans --crate-type=lib --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass -L x86_64-unknown-linux-gnu/test/run-pass/matches-macro.stage2-x86_64-unknown-linux-gnulibaux --cfg rtopt --cfg debug -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: --no-trans is deprecated in favor of -Z no-trans
<anon>:24:69: 24:71 error: expected one of `,`, `.`, or `}`, found `&&`
<anon>:24             match bar.char_at(0) { '+' | '-' => true, _ => false, } &&
                                                                              ^~

------------------------------------------

thread '[pretty] run-pass/matches-macro.rs' panicked at 'explicit panic', /home/simon/projects/rust/src/compiletest/runtest.rs:1453



failures:
    [pretty] run-pass/matches-macro.rs

test result: FAILED. 1779 passed; 1 failed; 75 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /home/simon/projects/rust/src/compiletest/compiletest.rs:269
/home/simon/projects/rust/mk/tests.mk:778: recipe for target 'tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-pretty-rpass.ok' failed
make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-pretty-rpass.ok] Error 101
