plain
[00:24:17] warning:  ^
[00:24:17] warning: ../../libcompiler_builtins/compiler-rt/lib/builtins/mulsc3.c:21:1: warning: conflicting types for built-in function '__mulsc3'
[00:24:17] warning:  __mulsc3(float __a, float __b, float __c, float __d)
[00:24:17] warning:  ^
[00:24:51] LLVM ERROR: Do not know how to split the result of this operator!
[00:24:51] [RUSTC-TIMING] core test:false 48.585
[00:24:51] error: Could not compile `core`.
[00:24:51] 
[00:24:51] Caused by:
[00:24:51] Caused by:
[00:24:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=8e33bf35a8f951b6 -C extra-filename=-8e33bf35a8f951b6 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/powerpc-unknown-linux-gnu/release/deps --target powerpc-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/powerpc-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:24:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "powerpc-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:24:51] expected success, got: exit code: 101
[00:24:51] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:24:51] travis_fold:end:stage1-std

[00:24:51] travis_time:end:stage1-std:start=1532102401723431020,finish=1532102450753626300,duration=49030195280

---
travis_time:end:070a5b58:start=1532102451500736471,finish=1532102451511818803,duration=11082332
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11ed1900
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14b817e8
travis_time:start:14b817e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b749be4
$ dmesg | grep -i kill
