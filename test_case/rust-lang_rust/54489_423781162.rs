plain
tidy check
[00:05:19] * 555 error codes
[00:05:19] * highest error code: E0712
[00:05:20] * 231 features
[00:05:21] invalid source: "git+https://github.com/rust-lang-nursery/rust-clippy?rev=125907ad08853b92d35e86aecebcf0f784f348d5#125907ad08853b92d35e86aecebcf0f784f348d5"
[00:05:21] some tidy checks failed
[00:05:21] 
[00:05:21] 
[00:05:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:21] 
[00:05:21] 
[00:05:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:21] Build completed unsuccessfully in 0:00:50
[00:05:21] Build completed unsuccessfully in 0:00:50
[00:05:21] Makefile:79: recipe for target 'tidy' failed
[00:05:21] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24ba0670
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15924f72:start=1537654324288822613,finish=1537654324292900934,duration=4078321
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:337a2428
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b18f0fb
travis_time:start:2b18f0fb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e5bee26
$ dmesg | grep -i kill
