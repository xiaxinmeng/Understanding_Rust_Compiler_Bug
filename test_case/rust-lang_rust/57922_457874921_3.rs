ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:03:12] 
[00:03:14] some tidy checks failed
[00:03:14] 
[00:03:14] 
[00:03:14] 
[00:03:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:14] 
[00:03:14] 
[00:03:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:14] Build completed unsuccessfully in 0:00:46
[00:03:14] Build completed unsuccessfully in 0:00:46
[00:03:14] make: *** [tidy] Error 1
[00:03:14] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3bec96b9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 22:53:52 UTC 2019
---
travis_time:end:06199a00:start=1548543233630159938,finish=1548543233634719973,duration=4560035
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:151eda36
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13488200
travis_time:start:13488200
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09d5050d
$ dmesg | grep -i kill
