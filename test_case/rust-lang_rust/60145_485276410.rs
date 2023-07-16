plain
travis_time:end:062f709c:start=1555869029121791978,finish=1555869113280110912,duration=84158318934
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
[01:09:33] 
[01:09:33] running 9 tests
[01:09:33] iiiiiiiii
[01:09:33] 
[01:09:33]  finished in 0.142
[01:09:33] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:09:48] 
[01:09:48] running 121 tests
[01:10:12] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:10:16] i.i......iii.i.....ii
[01:10:16] 
[01:10:16]  finished in 28.010
[01:10:16] travis_fold:end:test_debuginfo

---
[01:35:58] travis_fold:end:stage0-linkchecker

[01:35:58] travis_time:end:stage0-linkchecker:start=1555874877649971373,finish=1555874879491024694,duration=1841053321

[01:35:59] std/net/struct.Ipv4Addr.html:111: broken link fragment `#method.is_reserved()` pointing to `std/net/struct.Ipv4Addr.html`
[01:36:06] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:36:06] 
[01:36:06] 
[01:36:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:36:06] expected success, got: exit code: 101
[01:36:06] expected success, got: exit code: 101
[01:36:06] 
[01:36:06] 
[01:36:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:06] Build completed unsuccessfully in 0:37:27
[01:36:06] make: *** [check] Error 1
[01:36:06] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e9293fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 19:28:07 UTC 2019
