plain
[02:09:50] test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok
[02:09:50] 
[02:09:50] failures:
[02:09:50] 
[02:09:50] ---- net::tcp::tests::connect_timeout_unbound stdout ----
[02:09:50] thread 'net::tcp::tests::connect_timeout_unbound' panicked at 'called `Result::unwrap_err()` on an `Ok` value: TcpStream { addr: V4(127.0.0.1:37428), peer: V4(127.0.0.1:37428), fd: 6 }', src/libcore/result.rs:1009:5
[02:09:50] 
[02:09:50] failures:
[02:09:50]     net::tcp::tests::connect_timeout_unbound
[02:09:50] 
[02:09:50] 
[02:09:50] test result: FAILED. 769 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[02:09:50] 
[02:09:50] error: test failed, to rerun pass '--lib'
[02:09:50] 
[02:09:50] 
[02:09:50] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i686-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--"
[02:09:50] 
[02:09:50] 
[02:09:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[02:09:50] Build completed unsuccessfully in 2:07:29
---
travis_time:end:163b9d02:start=1543049193915500450,finish=1543049193925339139,duration=9838689
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0270198b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2105f9d6
travis_time:start:2105f9d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16246520
$ dmesg | grep -i kill
