plain
[00:43:35]    Compiling compiletest v0.0.0 (file:///checkout/src/tools/compiletest)
[00:43:35] error[E0433]: failed to resolve. Use of undeclared type or module `Mode`
[00:43:35]    --> tools/compiletest/src/header.rs:401:17
[00:43:35]     |
[00:43:35] 401 |                 Mode::RunFail => 101,
[00:43:35]     |                 ^^^^ Use of undeclared type or module `Mode`
opts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libgetopts-94c49d5c7d7e9da4.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liblog-23b4fb6fa681b8b7.rlib --extern rustfix=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/librustfix-7c378e738615d1bc.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libserde-ccf345e7c4463972.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-ae49e551c47d9111.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-eb6f704b801bec35.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-b694aac2c244ef25/out` (exit code: 101)
[00:43:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--features" "" "--message-format" "json"
[00:43:36] expected success, got: exit code: 101
[00:43:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:43:36] Build completed unsuccessfully in 0:00:35
[00:43:36] make: *** [check] Error 1
[00:43:36] Makefile:58: recipe for target 'check' failed
travis_time:end:04763f4d:start=1532500078635travis_time:start:010d3ffd
