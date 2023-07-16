plain
travis_time:end:07c31680:start=1548913633872644963,finish=1548913636070937308,duration=2198292345
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:19]    Compiling cc v1.0.28
[00:04:19]    Compiling libc v0.2.46
[00:04:19]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:04:19]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:19] error: expected one of `,` or `}`, found `.`
[00:04:19]    --> src/libcore/iter/adapters/flatten.rs:236:18
[00:04:19]     |
[00:04:19] 236 |                 f.checked_add(b)?.checked_add(m.checked_mul(max)?)?
[00:04:19]     |                  ^ expected one of `,` or `}` here
[00:04:19]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:24] error[E0574]: expected struct, variant or union type, found macro `try`
[00:04:24]    --> src/libcore/iter/adapters/flatten.rs:235:55
[00:04:24]     |
[00:04:24]     |
[00:04:24] 235 |             (Some(f), Some(b), Some(m), Some(max)) => try {
[00:04:24]     |                                                       ^^^ did you mean `try!(...)`?
[00:04:25]    Compiling compiler_builtins v0.1.5
[00:04:25]    Compiling cmake v0.1.33
[00:04:25]    Compiling backtrace-sys v0.1.27
[00:04:28]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:04:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:37] expected success, got: exit code: 101
[00:04:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:37] Build completed unsuccessfully in 0:00:21
[00:04:37] make: *** [all] Error 1
[00:04:37] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27fc9b7c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 05:52:04 UTC 2019
---
travis_time:end:03b3c9d4:start=1548913925852079980,finish=1548913925856721395,duration=4641415
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cffa42e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1067c9d6
travis_time:start:1067c9d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18762618
$ dmesg | grep -i kill
