
---- [run-pass] run-pass/core-run-destroy.rs stdout ----

    error: compilation failed!
    status: exit code: 101
    command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/core-run-destroy.rs -L x86_64-apple-darwin/test/run-pass --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/run-pass/core-run-destroy.stage2-x86_64-apple-darwin.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/run-pass/core-run-destroy.stage2-x86_64-apple-darwin --cfg rtopt --cfg debug -O -L x86_64-apple-darwin/rt --test
    stdout:
    ------------------------------------------

    ------------------------------------------
    stderr:
    ------------------------------------------
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/core-run-destroy.rs:58:50: 58:62 error: failed to resolve. Use of undeclared module `__test`
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/core-run-destroy.rs:58     green::start(argc, argv, rustuv::event_loop, __test::main)
                                                                                                                                                             ^~~~~~~~~~~~
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/core-run-destroy.rs:58:50: 58:62 error: unresolved name `__test::main`.
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/core-run-destroy.rs:58     green::start(argc, argv, rustuv::event_loop, __test::main)
                                                                                                                                                             ^~~~~~~~~~~~
    error: aborting due to 2 previous errors

    ------------------------------------------

    task '[run-pass] run-pass/core-run-destroy.rs' failed at 'explicit failure', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1321


---- [run-pass] run-pass/tcp-connect-timeouts.rs stdout ----

    error: compilation failed!
    status: exit code: 101
    command: x86_64-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/tcp-connect-timeouts.rs -L x86_64-apple-darwin/test/run-pass --target=x86_64-apple-darwin -L x86_64-apple-darwin/test/run-pass/tcp-connect-timeouts.stage2-x86_64-apple-darwin.libaux -C prefer-dynamic -o x86_64-apple-darwin/test/run-pass/tcp-connect-timeouts.stage2-x86_64-apple-darwin --cfg rtopt --cfg debug -O -L x86_64-apple-darwin/rt --test
    stdout:
    ------------------------------------------

    ------------------------------------------
    stderr:
    ------------------------------------------
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/tcp-connect-timeouts.rs:28:50: 28:62 error: failed to resolve. Use of undeclared module `__test`
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/tcp-connect-timeouts.rs:28     green::start(argc, argv, rustuv::event_loop, __test::main)
                                                                                                                                                                 ^~~~~~~~~~~~
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/tcp-connect-timeouts.rs:28:50: 28:62 error: unresolved name `__test::main`.
    /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/test/run-pass/tcp-connect-timeouts.rs:28     green::start(argc, argv, rustuv::event_loop, __test::main)
                                                                                                                                                                 ^~~~~~~~~~~~
    error: aborting due to 2 previous errors

    ------------------------------------------

    task '[run-pass] run-pass/tcp-connect-timeouts.rs' failed at 'explicit failure', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/compiletest/runtest.rs:1321
