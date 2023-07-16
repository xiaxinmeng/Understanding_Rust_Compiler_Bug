plain
travis_time:end:0973e462:start=1540993844087269090,finish=1540993897605643361,duration=53518374271
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:34]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:35]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:04:35]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:40]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:41] error[E0425]: cannot find function `unreachable` in module `hint`
[00:04:41]     |
[00:04:41] 286 |             None => hint::unreachable(),
[00:04:41]     |                           ^^^^^^^^^^^ not found in `hint`
[00:04:41] help: possible candidate is found in another module, you can import it into scope
---
[00:04:44] 
[00:04:44] To learn more, run the command again with --verbose.
[00:04:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:44] expected success, got: exit code: 101
[00:04:44] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:04:44] travis_fold:end:stage0-std

[00:04:44] travis_time:end:stage0-std:start=1540994152659334940,finish=1540994192963284602,duration=40303949662


[00:04:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:44] Build completed unsuccessfully in 0:00:41
[00:04:44] Makefile:28: recipe for target 'all' failed
[00:04:44] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05cfb831
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:117972d8:start=1540994193575495334,finish=1540994193580007948,duration=4512614
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09861eb1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:30ba3b40
travis_time:start:30ba3b40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07436812
$ dmesg | grep -i kill
