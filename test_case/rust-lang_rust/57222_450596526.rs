plain
travis_time:end:096672ea:start=1546215378515812814,finish=1546215379652737040,duration=1136924226
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:25]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:30] error[E0308]: mismatched types
[00:03:30]    --> src/libcore/str/mod.rs:649:52
[00:03:30]     |
[00:03:30] 649 |             (Some(t), Some(o)) if t.eq_ignore_case(o) => (),
[00:03:30]     |                                                    |
[00:03:30]     |                                                    |
[00:03:30]     |                                                    expected &char, found char
[00:03:30]     |                                                    help: consider borrowing here: `&o`
[00:03:30]     = note: expected type `&char`
[00:03:30]                found type `char`
[00:03:30] 
[00:03:33] error: aborting due to previous error
---
[00:03:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:33] expected success, got: exit code: 101
[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:33] Build completed unsuccessfully in 0:00:20
[00:03:33] make: *** [all] Error 1
[00:03:33] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01d33ce2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 31 00:20:01 UTC 2018
---
travis_time:end:041d1407:start=1546215601617528289,finish=1546215601623241920,duration=5713631
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:322a6290
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ba07281
travis_time:start:0ba07281
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ebd4210
$ dmesg | grep -i kill
