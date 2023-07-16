plain
travis_time:end:01c7d174:start=1555246532610861516,finish=1555246533383886425,duration=773024909
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:27] 
[01:19:27] running 9 tests
[01:19:27] iiiiiiiii
[01:19:27] 
[01:19:27]  finished in 0.189
[01:19:27] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:46] 
[01:19:46] running 121 tests
[01:20:16] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:20:22] i.i......iii.i.....ii
[01:20:22] 
[01:20:22]  finished in 35.779
[01:20:22] travis_fold:end:test_debuginfo

---
[01:50:12] travis_fold:end:stage0-linkchecker

[01:50:12] travis_time:end:stage0-linkchecker:start=1555253155310759135,finish=1555253157582462053,duration=2271702918

[01:50:15] std/primitive.str.html:1142: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.char.html
[01:50:15] std/primitive.str.html:1165: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.char.html
[01:50:15] std/primitive.str.html:1186: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.char.html
[01:50:24] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:50:24] 
[01:50:24] 
[01:50:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:50:24] expected success, got: exit code: 101
[01:50:24] expected success, got: exit code: 101
[01:50:24] 
[01:50:24] 
[01:50:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:50:24] Build completed unsuccessfully in 0:44:20
[01:50:24] Makefile:48: recipe for target 'check' failed
[01:50:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d138dc7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 14 14:46:10 UTC 2019
---
travis_time:end:17bc1c18:start=1555253173151116224,finish=1555253173231373112,duration=80256888
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1907094f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:070eb53c
$ dmesg | grep -i kill
