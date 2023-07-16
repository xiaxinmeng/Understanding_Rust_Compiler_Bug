plain
travis_time:end:0ace0e88:start=1553557812434740059,finish=1553557813444816029,duration=1010075970
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
[01:18:55] 
[01:18:55] running 9 tests
[01:18:55] iiiiiiiii
[01:18:55] 
[01:18:55]  finished in 0.158
[01:18:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:12] 
[01:19:12] running 120 tests
[01:19:40] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:19:45] .i......iii.i.....ii
[01:19:45] 
[01:19:45]  finished in 33.181
[01:19:45] travis_fold:end:test_debuginfo

---
[01:45:20] travis_fold:end:stage0-linkchecker

[01:45:20] travis_time:end:stage0-linkchecker:start=1553564142482089225,finish=1553564144524837576,duration=2042748351

[01:45:26] std/sync/atomic/struct.AtomicIsize.html:3: absolute path - /checkout/src/libcore/std/primitive.isize.html
[01:45:26] std/sync/atomic/struct.AtomicUsize.html:3: absolute path - /checkout/src/libcore/std/primitive.usize.html
[01:45:26] std/sync/atomic/struct.AtomicU32.html:3: absolute path - /checkout/src/libcore/std/primitive.u32.html
[01:45:26] std/sync/atomic/struct.AtomicI64.html:3: absolute path - /checkout/src/libcore/std/primitive.i64.html
[01:45:26] std/sync/atomic/struct.AtomicI32.html:3: absolute path - /checkout/src/libcore/std/primitive.i32.html
[01:45:26] std/sync/atomic/struct.AtomicU64.html:3: absolute path - /checkout/src/libcore/std/primitive.u64.html
[01:45:26] std/sync/atomic/struct.AtomicU8.html:3: absolute path - /checkout/src/libcore/std/primitive.u8.html
[01:45:26] std/sync/atomic/struct.AtomicU16.html:3: absolute path - /checkout/src/libcore/std/primitive.u16.html
[01:45:26] std/sync/atomic/struct.AtomicI16.html:3: absolute path - /checkout/src/libcore/std/primitive.i16.html
[01:45:26] std/sync/atomic/struct.AtomicI8.html:3: absolute path - /checkout/src/libcore/std/primitive.i8.html
[01:45:32] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:45:32] 
[01:45:32] 
[01:45:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:45:32] expected success, got: exit code: 101
[01:45:32] expected success, got: exit code: 101
[01:45:32] 
[01:45:32] 
[01:45:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:32] Build completed unsuccessfully in 0:38:50
[01:45:32] Makefile:48: recipe for target 'check' failed
[01:45:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:038743dc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 01:35:56 UTC 2019
