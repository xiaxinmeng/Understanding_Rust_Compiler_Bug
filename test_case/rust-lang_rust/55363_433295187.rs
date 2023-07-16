plain
[00:05:24] 
[00:05:24] Caused by:
[00:05:24]   feature `edition` is required
[00:05:24] 
[00:05:24] this Cargo does not support nightly features, but if you
[00:05:24] switch to nightly channel you can add
[00:05:24] `cargo-features = ["edition"]` to enable this feature
[00:05:24] 
[00:05:24] 
[00:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "vendor"
[00:05:24] 
[00:05:24] 
[00:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:05:24] Build completed unsuccessfully in 0:02:16
---
travis_time:end:0a7faa1d:start=1540533255226169726,finish=1540533255233755734,duration=7586008
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a85b042
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ef7194a
travis_time:start:0ef7194a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:017a7a79
$ dmesg | grep -i kill
