plain
travis_time:end:174e70cc:start=1550400773701676845,finish=1550400901763699795,duration=128062022950
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:40]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:02]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:04]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:04]    Compiling rustc-demangle v0.1.10
[00:04:04] error: unexpected `,` in pattern
[00:04:04]      |
[00:04:04] 2179 |         let mut iter, final_res;
[00:04:04] 2179 |         let mut iter, final_res;
[00:04:04]      |             --------^---------- help: try adding parentheses: `(mut iter, final_res)`
[00:04:04] 
[00:04:04] error: unexpected `,` in pattern
[00:04:04]      |
[00:04:04] 2223 |         let mut iter, final_res;
[00:04:04] 2223 |         let mut iter, final_res;
[00:04:04]      |             --------^---------- help: try adding parentheses: `(mut iter, final_res)`
[00:04:04]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:07] error: aborting due to 2 previous errors
[00:04:07] 
[00:04:07] error: Could not compile `alloc`.
[00:04:07] error: Could not compile `alloc`.
[00:04:07] 
[00:04:07] To learn more, run the command again with --verbose.
[00:04:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:07] expected success, got: exit code: 101
[00:04:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:07] Build completed unsuccessfully in 0:00:39
[00:04:07] Makefile:18: recipe for target 'all' failed
[00:04:07] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004dfb48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 10:59:18 UTC 2019
---
travis_time:end:014b84bb:start=1550401159794792019,finish=1550401159799654957,duration=4862938
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10b5b0c0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04af4e19
travis_time:start:04af4e19
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07cc803d
$ dmesg | grep -i kill
