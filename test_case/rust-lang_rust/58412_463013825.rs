plain
travis_time:end:2a1a5d5e:start=1550020206148454906,finish=1550020283561179260,duration=77412724354
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:00]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:00] error[E0432]: unresolved import `spec`
[00:06:00]  --> src/librustc_target/spec/armv6_unknown_freebsd.rs:1:5
[00:06:00]   |
[00:06:00] 1 | use spec::{LinkerFlavor, Target, TargetOptions, TargetResult};
[00:06:00]   |     ^^^^ did you mean `crate::spec`?
[00:06:00]   |
[00:06:00]   = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:06:00] error[E0432]: unresolved import `spec`
[00:06:00]  --> src/librustc_target/spec/armv7_unknown_freebsd.rs:1:5
[00:06:00]   |
[00:06:00]   |
[00:06:00] 1 | use spec::{LinkerFlavor, Target, TargetOptions, TargetResult};
[00:06:00]   |     ^^^^ did you mean `crate::spec`?
[00:06:00]   |
[00:06:00]   = note: `use` statements changed in Rust 2018; read more at <https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html>
[00:06:00]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:01] error: aborting due to 2 previous errors
[00:06:01] 
[00:06:01] For more information about this error, try `rustc --explain E0432`.
[00:06:01] For more information about this error, try `rustc --explain E0432`.
[00:06:01] error: Could not compile `rustc_target`.
[00:06:01] warning: build failed, waiting for other jobs to finish...
[00:06:04] error: build failed
[00:06:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:04] expected success, got: exit code: 101
[00:06:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:04] Build completed unsuccessfully in 0:02:11
[00:06:04] make: *** [all] Error 1
[00:06:04] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:10ef4510
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 01:17:37 UTC 2019
---
travis_time:end:0341da22:start=1550020657842733123,finish=1550020657847874710,duration=5141587
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003bc18f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1dd099e6
travis_time:start:1dd099e6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.ver
