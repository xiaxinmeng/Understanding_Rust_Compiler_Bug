plain
[00:23:01] warning:  ^
[00:23:01] warning: ../../libcompiler_builtins/compiler-rt/lib/builtins/mulsc3.c:21:1: warning: conflicting types for built-in function '__mulsc3'
[00:23:01] warning:  __mulsc3(float __a, float __b, float __c, float __d)
[00:23:01] warning:  ^
[00:23:31] LLVM ERROR: Do not know how to split the result of this operator!
[00:23:31] [RUSTC-TIMING] core test:false 46.302
[00:23:31] error: Could not compile `core`.
[00:23:31] 
[00:23:31] Caused by:
[00:23:31] Caused by:
[00:23:31]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=34f41e8c087dbdeb -C extra-filename=-34f41e8c087dbdeb --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/powerpc-unknown-linux-gnu/release/deps --target powerpc-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/powerpc-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:23:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "powerpc-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:23:31] expected success, got: exit code: 101
[00:23:31] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:23:31] travis_fold:end:stage1-std

[00:23:31] travis_time:end:stage1-std:start=1531819401601672350,finish=1531819448299577851,duration=46697905501

---
travis_time:end:1d277870:start=1531819448974042536,finish=1531819448981045657,duration=7003121
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0845bb78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06415ec0
travis_time:start:06415ec0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:015b4c7a
$ dmesg | grep -i kill
