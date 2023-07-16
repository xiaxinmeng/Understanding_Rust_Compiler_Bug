


---- [compile-fail] compile-fail/derive-assoc-type-not-impl.rs stdout ----

error: compile-fail test compiled successfully!
status: exit code: 0
command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/compile-fail/derive-assoc-type-not-impl.rs -L x86_64-apple-darwin/test/compile-fail/ --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/compile-fail/derive-assoc-type-not-impl.stage2-x86_64-apple-darwinlibaux -C prefer-dynamic -o x86_64-apple-darwin/test/compile-fail/derive-assoc-type-not-impl.stage2-x86_64-apple-darwin --cfg rtopt --cfg debug -O -L x86_64-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[compile-fail] compile-fail/derive-assoc-type-not-impl.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1482



failures:
    [compile-fail] compile-fail/derive-assoc-type-not-impl.rs
