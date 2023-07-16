plain
travis_time:end:083b0e13:start=1550308833977580611,finish=1550308836029960069,duration=2052379458
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:06:50]     Checking parking_lot_core v0.3.0
[00:06:51]     Checking parking_lot v0.6.4
[00:06:52]     Checking tempfile v3.0.5
[00:06:52]     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:06:52] thread 'rustc' panicked at 'src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)', src/librustc/util/bug.rs:37:26
[00:06:52] 
[00:06:52] error: internal compiler error: unexpected panic
[00:06:52] 
[00:06:52] note: the compiler unexpectedly panicked. this is a bug.
---
[00:06:52] 
[00:06:52] error: Could not compile `rustdoc`.
[00:06:52] 
[00:06:52] To learn more, run the command again with --verbose.
[00:06:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:06:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:06:52] Build completed unsuccessfully in 0:04:36
travis_time:end:06b8928f:start=1550308847760608905,finish=1550309261174853862,duration=413414244957
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:116e8ac0:start=1550309262188442379,finish=1550309262193985809,duration=5543430
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0eddab68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03fc2d20
travis_time:start:03fc2d20
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:034c9464
$ dmesg | grep -i kill
