plain
########################################                                  56.0%
######################################################################    97.7%
######################################################################## 100.0%
[00:03:24] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:03:24]     Updating git repository `https://github.com/Xanewok/rls-data`
[00:03:50]  Downloading getopts v0.2.17
[00:03:50]  Downloading time v0.1.40
[00:03:50]  Downloading lazy_static v0.2.11
[00:03:51]  Downloading num_cpus v1.8.0
---
tidy check
[00:06:02] * 556 error codes
[00:06:02] * highest error code: E0713
[00:06:02] * 231 features
[00:06:03] invalid source: "git+https://github.com/Xanewok/rls-data?branch=compilation-options#5d2c63f02db25a7530ddb52fea9d05cac0401355"
[00:06:03] some tidy checks failed
[00:06:03] 
[00:06:03] 
[00:06:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:06:03] 
[00:06:03] 
[00:06:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:06:03] Build completed unsuccessfully in 0:00:49
[00:06:03] Build completed unsuccessfully in 0:00:49
[00:06:03] Makefile:79: recipe for target 'tidy' failed
[00:06:03] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:24ddf5ad
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01ab35fc:start=1537805268067661134,finish=1537805268072267080,duration=4605946
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12b7da6a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d6f1ba4
travis_time:start:0d6f1ba4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:335b41e4
$ dmesg | grep -i kill
