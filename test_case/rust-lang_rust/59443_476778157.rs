plain
travis_time:end:09c5f8b0:start=1553617825696510208,finish=1553617826712589117,duration=1016078909
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:17] 
[01:19:17] running 9 tests
[01:19:17] iiiiiiiii
[01:19:17] 
[01:19:17]  finished in 0.160
[01:19:17] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:34] 
[01:19:34] running 120 tests
[01:20:02] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:20:07] .i......iii.i.....ii
[01:20:07] 
[01:20:07]  finished in 33.358
[01:20:07] travis_fold:end:test_debuginfo

---
[01:31:09] ....iiiii........................................................................................... 100/2298
[01:31:22] ......................................................................ii............................ 200/2298
[01:31:36] ..........................................................................................i......... 300/2298
[01:31:53] .................................................................................................... 400/2298
[01:32:05] ......................i..i....F..................................................................... 500/2298
[01:32:31] .................................................................................................... 700/2298
[01:32:44] .................................................................................................... 800/2298
[01:32:57] .................................................................................................... 900/2298
[01:33:10] .................................................................................................... 1000/2298
---
[01:36:11] 
[01:36:11] error: test failed, to rerun pass '--doc'
[01:36:11] 
[01:36:11] 
[01:36:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:36:11] 
[01:36:11] 
[01:36:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:11] Build completed unsuccessfully in 0:29:14
[01:36:11] Build completed unsuccessfully in 0:29:14
[01:36:11] make: *** [check] Error 1
[01:36:11] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:19c0f4b2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 18:06:49 UTC 2019
