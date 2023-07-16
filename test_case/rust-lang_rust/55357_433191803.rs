plain
[00:05:56] Caused by:
[00:05:56]   failed to download package from registry
[00:05:56] 
[00:05:56] Caused by:
[00:05:56]   failed to get 200 response from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`, got 403
[00:05:56] 
[00:05:56] 
[00:05:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[00:05:56] 
[00:05:56] 
[00:05:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:05:56] Build completed unsuccessfully in 0:01:58
---
travis_time:end:0415640d:start=1540498758606716883,finish=1540498758615311783,duration=8594900
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d376207
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b8063f4
travis_time:start:2b8063f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0582fa00
$ dmesg | grep -i kill
