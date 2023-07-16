plain
[00:59:49]    Compiling rand v0.4.2
[00:59:49]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:59:52] [RUSTC-TIMING] rand test:false 2.884
[01:00:10] [RUSTC-TIMING] core test:false 20.758
[01:00:10] error[E0523]: found two different crates with name `core` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[01:00:10]   --> libcore/../libcore/tests/lib.rs:49:1
[01:00:10] 49 | extern crate core;
[01:00:10]    | ^^^^^^^^^^^^^^^^^^
[01:00:10] 
[01:00:10] error: aborting due to previous error
---
[01:00:10] 
[01:00:10] To learn more, run the command again with --verbose.
[01:00:10] 
[01:00:10] 
[01:00:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
[01:00:10] 
[01:00:10] 
[01:00:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/run-pass src/test/compile-fail src/test/parse-fail src/test/mir-opt src/test/codegen-units src/libcore
[01:00:10] Build completed unsuccessfully in 0:57:29
---
travis_time:end:0943dc8b:start=1532272073164833523,finish=1532272073172195765,duration=7362242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08a8740f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12918420
travis_time:start:12918420
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0165f16b
$ dmesg | grep -i kill
