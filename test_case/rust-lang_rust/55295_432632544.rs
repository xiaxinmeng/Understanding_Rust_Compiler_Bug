plain
[00:52:37] 
[00:52:40] error: unused variable: `b`
[00:52:40]    --> libstd/net/tcp.rs:732:34
[00:52:40]     |
[00:52:40] 732 |         self.0.accept().map(|(a, b)| (TcpStream(a), b))
[00:52:40]     |                                  ^ help: consider using `_b` instead
[00:52:40]     = note: `-D unused-variables` implied by `-D warnings`
[00:52:40] 
[00:52:41] error: aborting due to previous error
[00:52:41] 
[00:52:41] 
[00:52:41] [RUSTC-TIMING] std test:false 3.534
[00:52:41] error: Could not compile `std`.
[00:52:41] 
[00:52:41] To learn more, run the command again with --verbose.
[00:52:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:52:41] expected success, got: exit code: 101
[00:52:41] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1115:9
[00:52:41] travis_fold:end:stage2-std

[00:52:41] travis_time:end:stage2-std:start=1540383542989991370,finish=1540383578616051155,duration=35626059785

---
travis_time:end:0758bd1a:start=1540383579776048203,finish=1540383579783902111,duration=7853908
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22697e9b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:123b0c50
travis_time:start:123b0c50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1204ca38
$ dmesg | grep -i kill
