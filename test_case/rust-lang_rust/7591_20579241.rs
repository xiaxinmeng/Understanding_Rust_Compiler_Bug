
error: compilation failed!
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt-vg/build/src/test/run-pass/test-ignore-cfg.rs -o x86_64-unknown-linux-gnu/test/run-pass/test-ignore-cfg.stage2-x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/run-pass -L x86_64-unknown-linux-gnu/test/run-pass/test-ignore-cfg.libaux -O --target=x86_64-unknown-linux-gnu --test --cfg ignorecfg
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt-vg/build/src/test/run-pass/test-ignore-cfg.rs:29:16: 29:29 error: unresolved name `__test::tests`.
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt-vg/build/src/test/run-pass/test-ignore-cfg.rs:29     let tests = __test::tests;
                                                                                                                           ^~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

rust: task failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt-vg/build/src/compiletest/runtest.rs:722
