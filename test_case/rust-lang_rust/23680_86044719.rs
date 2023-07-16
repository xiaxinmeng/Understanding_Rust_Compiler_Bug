

thread '<main>' panicked at 'Some tests failed', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/compiletest.rs:258
---- [run-pass] bench/bench-io.rs stdout ----

error: compilation failed!
status: exit code: 101
command: i686-apple-darwin/stage2/bin/rustc /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/test/bench/bench-io.rs -L i686-apple-darwin/test/bench/ --target=i686-apple-darwin -L i686-apple-darwin/test/bench/bench-io.stage2-i686-apple-darwinlibaux -C prefer-dynamic -o i686-apple-darwin/test/bench/bench-io.stage2-i686-apple-darwin --cfg rtopt --cfg debug -O -L i686-apple-darwin/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: crate names soon cannot contain hyphens: bench-io
error: main function not found
error: aborting due to previous error

------------------------------------------

thread '[run-pass] bench/bench-io.rs' panicked at 'explicit panic', /Users/rustbuild/src/rust-buildbot/slave/auto-mac-32-opt/build/src/compiletest/runtest.rs:1482



failures:
    [run-pass] bench/bench-io.rs

