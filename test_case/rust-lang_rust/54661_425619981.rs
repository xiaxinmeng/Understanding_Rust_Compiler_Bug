plain
[00:07:58]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:08:29] error[E0308]: mismatched types
[00:08:29]    --> librustc/ty/relate.rs:705:66
[00:08:29]     |
[00:08:29] 705 |             (UnpackedKind::Lifetime(a), b) | (UnpackedKind::Type(a), b) => {
[00:08:29]     |                                                                  ^ expected enum `ty::sty::RegionKind`, found struct `ty::TyS`
[00:08:29]     |
[00:08:29]     = note: expected type `&ty::sty::RegionKind`
[00:08:29]                found type `&ty::TyS<'_>`
[00:08:31] error: aborting due to previous error
[00:08:31] 
[00:08:31] For more information about this error, try `rustc --explain E0308`.
[00:08:31] error: Could not compile `rustc`.
[00:08:31] error: Could not compile `rustc`.
[00:08:31] 
[00:08:31] To learn more, run the command again with --verbose.
[00:08:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:31] expected success, got: exit code: 101
[00:08:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:08:31] travis_fold:end:stage0-rustc

[00:08:31] travis_time:end:stage0-rustc:start=1538201806045186247,finish=1538201988538998808,duration=182493812561


[00:08:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:31] Build completed unsuccessfully in 0:04:02
[00:08:31] Makefile:28: recipe for target 'all' failed
[00:08:31] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f0fef88
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:29c1f58b:start=1538201989168133185,finish=1538201989172166309,duration=4033124
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0aacf08c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:031190c9
travis_time:start:031190c9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:089dd640
$ dmesg | grep -i kill
