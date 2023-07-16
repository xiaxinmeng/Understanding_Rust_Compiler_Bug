plain
[00:52:06] 
[00:52:09] error: unused variable: `b`
[00:52:09]    --> libstd/net/tcp.rs:732:34
[00:52:09]     |
[00:52:09] 732 |         self.0.accept().map(|(a, b)| (TcpStream(a), b))
[00:52:09]     |                                  ^ help: consider using `_b` instead
[00:52:09]     = note: `-D unused-variables` implied by `-D warnings`
[00:52:09] 
[00:52:09] error: aborting due to previous error
[00:52:09] 
[00:52:09] 
[00:52:09] [RUSTC-TIMING] std test:false 3.072
[00:52:09] error: Could not compile `std`.
[00:52:09] 
[00:52:09] To learn more, run the command again with --verbose.
[00:52:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:52:09] expected success, got: exit code: 101
[00:52:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:52:09] travis_fold:end:stage2-std

[00:52:09] travis_time:end:stage2-std:start=1537880104164660775,finish=1537880137298812239,duration=33134151464

---
travis_time:end:0a18beb0:start=1537880138664168818,finish=1537880138669007886,duration=4839068
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0208f450
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d798bba
travis_time:start:1d798bba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:025be8e0
$ dmesg | grep -i kill
