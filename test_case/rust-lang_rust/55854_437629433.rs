plain
[00:27:06] [RUSTC-TIMING] rustc_errors test:false 8.608
[00:27:20] error[E0080]: could not evaluate static initializer
[00:27:20]    --> libsyntax/ast.rs:929:1
[00:27:20]     |
[00:27:20] 929 | static_assert!(EXPR_IS_AT_MOST_88_BYTES: std::mem::size_of::<Expr>() <= 88);
[00:27:20]     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the len is 1 but the index is 1
[00:27:20]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:27:20] 
[00:27:20] error: aborting due to previous error
[00:27:20] 
---
[00:27:20] travis_time:end:stage1-rustc:start=1541891669942637492,finish=1541891754141493428,duration=84198855936

[00:27:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:27:20] expected success, got: exit code: 101
[00:27:20] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:27:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu
[00:27:20] Build completed unsuccessfully in 0:24:07
travis_time:end:02925d40:start=1541890113474424572,finish=1541891754404066686,duration=1640929642114
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:03574eed:start=1541891755090134330,finish=1541891755098202820,duration=8068490
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cdbba03
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:159e39b1
travis_time:start:159e39b1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cced766
$ dmesg | grep -i kill
