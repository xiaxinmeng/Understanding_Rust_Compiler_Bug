plain
#######################################                                   54.7%
###################################################                       71.9%
######################################################################## 100.0%
[00:02:01] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:01]     Updating git repository `https://github.com/Xanewok/rls-data`
[00:02:36]  Downloading serde v1.0.75
[00:02:37]  Downloading num_cpus v1.8.0
[00:02:37]  Downloading libc v0.2.43
[00:02:37]  Downloading time v0.1.40
---
tidy check
[00:04:44] * 555 error codes
[00:04:44] * highest error code: E0712
[00:04:44] * 231 features
[00:04:45] invalid source: "git+https://github.com/Xanewok/rls-data?branch=compilation-options#0b9dbe5bf766a20009def9acac2f997194823373"
[00:04:45] some tidy checks failed
[00:04:45] 
[00:04:45] 
[00:04:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:45] 
[00:04:45] 
[00:04:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:45] Build completed unsuccessfully in 0:00:49
[00:04:45] Build completed unsuccessfully in 0:00:49
[00:04:45] Makefile:79: recipe for target 'tidy' failed
[00:04:45] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11d44f9d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:080b6960:start=1537651696283338024,finish=1537651696287764768,duration=4426744
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06871760
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f276a50
travis_time:start:0f276a50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0795d281
$ dmesg | grep -i kill
