plain
[00:11:30] Caused by:
[00:11:30]   failed to download package from registry
[00:11:30] 
[00:11:30] Caused by:
[00:11:30]   failed to get 200 response from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`, got 403
[00:11:30] 
[00:11:30] 
[00:11:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[00:11:30] 
[00:11:30] 
[00:11:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:11:30] Build completed unsuccessfully in 0:08:22
---
travis_time:end:23ad8215:start=1540497514133708537,finish=1540497514144670069,duration=10961532
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2306c493
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12cb9f30
travis_time:start:12cb9f30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2c5de38f
$ dmesg | grep -i kill
