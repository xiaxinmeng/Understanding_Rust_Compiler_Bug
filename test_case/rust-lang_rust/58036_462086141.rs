plain
travis_time:end:01095b10:start=1549751067371432220,finish=1549751071003108523,duration=3631676303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:16:44]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:16:45] error: unused import: `util`
[00:16:45]   --> src/librustc_lint/builtin.rs:29:19
[00:16:45]    |
[00:16:45] 29 | use rustc::{lint, util};
[00:16:45]    |
[00:16:45]    = note: `-D unused-imports` implied by `-D warnings`
[00:16:45] 
[00:16:46] error: aborting due to previous error
---
[00:18:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:25] expected success, got: exit code: 101
[00:18:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:25] Build completed unsuccessfully in 0:14:23
[00:18:25] Makefile:18: recipe for target 'all' failed
[00:18:25] make: *** [all] Error 1
/Logs/DiagnosticReports/': No such file or directory
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:02af1a2a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: ‘/home/travis/Library/Logs/DiagnosticReports’: No such file or directory
travis_time:end:02af1a2a:start=1549752187740713302,finish=1549752187745741649,duration=5028347
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14efe570
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:079cf278
travis_time:start:079cf278
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.ve
