plain

[00:04:38] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:39] tidy error: /checkout/src/librustc/middle/resolve_lifetime.rs:1595: line longer than 100 chars
[00:04:44] some tidy checks failed
[00:04:44] 
[00:04:44] 
[00:04:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:44] 
[00:04:44] 
[00:04:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:44] Build completed unsuccessfully in 0:01:14
---
travis_time:end:0ce616d0:start=1560493917882537935,finish=1560493917888206705,duration=5668770
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e1e86ed
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:181b93a0
travis_time:start:181b93a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:166925a6
$ dmesg | grep -i kill
