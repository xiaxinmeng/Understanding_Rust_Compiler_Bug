plain
[00:05:36] Caused by:
[00:05:36]   failed to download package from registry
[00:05:36] 
[00:05:36] Caused by:
[00:05:36]   failed to get 200 response from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`, got 403
[00:05:36] 
[00:05:36] 
[00:05:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[00:05:36] 
[00:05:36] 
[00:05:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:05:36] Build completed unsuccessfully in 0:02:08
---
travis_time:end:044c5b6a:start=1540499353942303725,finish=1540499353951328642,duration=9024917
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19d56c28
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05e00bc0
travis_time:start:05e00bc0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07ccdffe
$ dmesg | grep -i kill
