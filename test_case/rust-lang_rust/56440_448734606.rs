plain
travis_time:end:09eb2ffc:start=1545250379263888141,finish=1545250478781955081,duration=99518066940
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:19:03]    Compiling ena v0.11.0
[00:19:04]    Compiling smallvec v0.6.7
[00:19:04]    Compiling lock_api v0.1.3
[00:19:04]    Compiling crossbeam-epoch v0.3.1
[00:19:04] error[E0720]: unions may not contain fields that need dropping
[00:19:04]     |
[00:19:04]     |
[00:19:04] 272 | / union SmallVecData<A: Array> {
[00:19:04] 273 | |     inline: A,
[00:19:04] 274 | |     heap: (*mut A::Item, usize),
[00:19:04] 275 | | }
[00:19:04]     | |_^ ManuallyDrop can be used to wrap such fields
[00:19:04] error: aborting due to previous error
[00:19:04] 
[00:19:04] For more information about this error, try `rustc --explain E0720`.
[00:19:04] error: Could not compile `smallvec`.
[00:19:04] error: Could not compile `smallvec`.
[00:19:04] warning: build failed, waiting for other jobs to finish...
[00:19:05] error: build failed
[00:19:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:05] expected success, got: exit code: 101
[00:19:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:05] Build completed unsuccessfully in 0:16:02
[00:19:05] Makefile:28: recipe for target 'all' failed
[00:19:05] make: *** [all] Error 1
travis_time:end:0079be01:start=1545251632995024654,finish=1545251633007078065,duration=12053411
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0b37c5c0
travis_time:start:0b37c5c0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0b37c5c0:start=1545251633011693449,finish=1545251633016395192,duration=4701743
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0976b9a2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1423dc24
travis_time:start:1423dc24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No s
