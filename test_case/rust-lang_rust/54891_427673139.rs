plain
[01:46:26] test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok
[01:46:26] 
[01:46:26] failures:
[01:46:26] 
[01:46:26] ---- net::tcp::tests::connect_timeout_unbound stdout ----
[01:46:26] thread 'net::tcp::tests::connect_timeout_unbound' panicked at 'called `Result::unwrap_err()` on an `Ok` value: TcpStream { addr: V4(127.0.0.1:41268), peer: V4(127.0.0.1:41268), fd: 3 }', libcore/result.rs:1009:5
[01:46:26] 
[01:46:26] failures:
[01:46:26]     net::tcp::tests::connect_timeout_unbound
[01:46:26] 
[01:46:26] 
[01:46:26] test result: FAILED. 768 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:46:26] 
[01:46:26] error: test failed, to rerun pass '--lib'
[01:46:26] 
[01:46:26] 
[01:46:26] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i686-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--"
[01:46:26] 
[01:46:26] 
[01:46:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:26] Build completed unsuccessfully in 0:35:01
[01:46:26] Build completed unsuccessfully in 0:35:01
[01:46:26] Makefile:58: recipe for target 'check' failed
[01:46:26] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:040df898
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08d9e407:start=1538934963627513755,finish=1538934963637061576,duration=9547821
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005e2c54
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2245c530
travis_time:start:2245c530
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d445d2a
$ dmesg | grep -i kill
