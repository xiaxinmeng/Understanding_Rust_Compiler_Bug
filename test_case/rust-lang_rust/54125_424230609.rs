plain
[00:56:03] 
[00:56:06] error: unused variable: `b`
[00:56:06]    --> libstd/net/tcp.rs:732:34
[00:56:06]     |
[00:56:06] 732 |         self.0.accept().map(|(a, b)| (TcpStream(a), b))
[00:56:06]     |                                  ^ help: consider using `_b` instead
[00:56:06]     = note: `-D unused-variables` implied by `-D warnings`
[00:56:06] 
[00:56:06] error: aborting due to previous error
[00:56:06] 
[00:56:06] 
[00:56:06] [RUSTC-TIMING] std test:false 3.425
[00:56:06] error: Could not compile `std`.
[00:56:06] 
[00:56:06] To learn more, run the command again with --verbose.
[00:56:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:56:06] expected success, got: exit code: 101
[00:56:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:56:06] travis_fold:end:stage2-std

[00:56:06] travis_time:end:stage2-std:start=1537859025209186933,finish=1537859063745076578,duration=38535889645

---
travis_time:end:0702e1fc:start=1537859064616756368,finish=1537859064628312512,duration=11556144
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:35ed86f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0440d7ed
travis_time:start:0440d7ed
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16b0aaac
$ dmesg | grep -i kill
