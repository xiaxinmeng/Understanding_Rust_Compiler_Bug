plain
[00:03:50]  Downloading lazycell v0.6.0
[00:03:50] error: unable to get packages from source
[00:03:50] 
[00:03:50] Caused by:
[00:03:50]   [18] Transferred a partial file (transfer closed with outstanding read data remaining)
[00:03:50] thread 'main' panicked at 'command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "metadata" "--format-version" "1" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml"
[00:03:50] expected success, got: exit code: 101', build_helper/lib.rs:122:9
[00:03:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:03:50] Build completed unsuccessfully in 0:02:18
travis_time:end:1444d154:start=1530557188789423760,finish=1530557419939705166,duration=231150281406

---
travis_time:end:1283bc83:start=1530557420286968071,finish=1530557420298180853,duration=11212782
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:051f0388
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers��� for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f1619bf
$ dmesg | grep -i kill
