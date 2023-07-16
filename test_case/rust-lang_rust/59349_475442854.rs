plain
travis_time:end:13dda8d2:start=1553204875517677854,finish=1553204876500656716,duration=982978862
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
[01:21:36] 
[01:21:36] running 9 tests
[01:21:36] iiiiiiiii
[01:21:36] 
[01:21:36]  finished in 0.160
[01:21:36] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:53] 
[01:21:53] running 120 tests
[01:22:19] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:22:23] .i......iii.i.....ii
[01:22:23] 
[01:22:23]  finished in 30.335
[01:22:23] travis_fold:end:test_debuginfo

---
[01:48:46] travis_fold:end:stage0-linkchecker

[01:48:46] travis_time:end:stage0-linkchecker:start=1553211411394143856,finish=1553211413371107825,duration=1976963969

[01:48:50] std/sync/atomic/struct.AtomicIsize.html:4: absolute path - /checkout/src/libcore/std/primitive.isize.html
[01:48:50] std/sync/atomic/struct.AtomicUsize.html:4: absolute path - /checkout/src/libcore/std/primitive.usize.html
[01:48:50] std/sync/atomic/struct.AtomicU32.html:4: absolute path - /checkout/src/libcore/std/primitive.u32.html
[01:48:50] std/sync/atomic/struct.AtomicI64.html:4: absolute path - /checkout/src/libcore/std/primitive.i64.html
[01:48:50] std/sync/atomic/struct.AtomicI32.html:4: absolute path - /checkout/src/libcore/std/primitive.i32.html
[01:48:50] std/sync/atomic/struct.AtomicU64.html:4: absolute path - /checkout/src/libcore/std/primitive.u64.html
[01:48:50] std/sync/atomic/struct.AtomicU8.html:4: absolute path - /checkout/src/libcore/std/primitive.u8.html
[01:48:50] std/sync/atomic/struct.AtomicU16.html:4: absolute path - /checkout/src/libcore/std/primitive.u16.html
[01:48:50] std/sync/atomic/struct.AtomicI16.html:4: absolute path - /checkout/src/libcore/std/primitive.i16.html
[01:48:50] std/sync/atomic/struct.AtomicI8.html:4: absolute path - /checkout/src/libcore/std/primitive.i8.html
[01:48:54] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:48:54] 
[01:48:54] 
[01:48:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:48:54] expected success, got: exit code: 101
[01:48:54] expected success, got: exit code: 101
[01:48:54] 
[01:48:54] 
[01:48:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:54] Build completed unsuccessfully in 0:39:05
[01:48:54] make: *** [check] Error 1
[01:48:54] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02ddf79a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 21 23:37:01 UTC 2019
