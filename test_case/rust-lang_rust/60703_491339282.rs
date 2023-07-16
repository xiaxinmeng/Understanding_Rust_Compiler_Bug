plain
travis_time:end:17a8bd44:start=1557503377594146373,finish=1557503519271010477,duration=141676864104
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:48] tidy error: /checkout/src/liballoc/collections/linked_list.rs:1350: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/collections/vec_deque.rs:597: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/vec.rs:559: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/raw_vec.rs:386: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/raw_vec.rs:397: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/raw_vec.rs:621: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/raw_vec.rs:624: line longer than 100 chars
[00:03:48] tidy error: /checkout/src/liballoc/string.rs:995: line longer than 100 chars
[00:03:52] some tidy checks failed
[00:03:52] 
[00:03:52] 
[00:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:52] 
[00:03:52] 
[00:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:52] Build completed unsuccessfully in 0:01:07
[00:03:52] Build completed unsuccessfully in 0:01:07
[00:03:52] Makefile:67: recipe for target 'tidy' failed
[00:03:52] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:078cd32d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 10 15:56:00 UTC 2019
---
travis_time:end:06975eb6:start=1557503761483950234,finish=1557503761488138239,duration=4188005
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2fe89a44
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27661260
travis_time:start:27661260
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01b2e9aa
$ dmesg | grep -i kill
