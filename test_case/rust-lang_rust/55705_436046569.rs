plain
travis_time:end:00848b06:start=1541453930283672534,finish=1541453932994622969,duration=2710950435
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:56]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:05:01] error[E0547]: missing 'issue'
[00:05:01]     --> libcore/num/mod.rs:4774:1
[00:05:01]      |
[00:05:01] 4774 | / #[unstable(feature = "int_error_matching",
[00:05:01] 4775 | |            reason = "it can be useful to match errors when making error messages \
[00:05:01] 4776 | |                      for integer parsing")]
[00:05:01] 
[00:05:02]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:05:02]    Compiling cmake v0.1.33
[00:05:02]    Compiling std v0.0.0 (/checkout/src/libstd)
---
[00:05:13] 
[00:05:13] To learn more, run the command again with --verbose.
[00:05:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:13] expected success, got: exit code: 101
[00:05:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:05:13] travis_fold:end:stage0-std

[00:05:13] travis_time:end:stage0-std:start=1541454238612260335,finish=1541454255852601034,duration=17240340699


[00:05:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:13] Build completed unsuccessfully in 0:00:18
[00:05:13] make: *** [all] Error 1
[00:05:13] Makefile:28: recipe for target 'all' failed
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1719c4a6:start=1541454256521174131,finish=1541454256526206890,duration=5032759
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_fold:start:after_failure.4
travis_time:start:15059780
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:049e07ca
travis_time:start:049e07ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:28289880
$ dmesg | grep -i kil
