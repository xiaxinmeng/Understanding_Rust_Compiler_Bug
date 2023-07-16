plain
travis_time:end:01675622:start=1541711075457886333,finish=1541711161205082420,duration=85747196087
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:54] .................................................................................................... 100/4999
[00:52:57] .................................................................................................... 200/4999
[00:53:00] ........................................................................ii...................ii..... 300/4999
[00:53:03] ...........................................................................................iii...... 400/4999
[00:53:06] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4999
[00:53:13] .................................................................................................... 700/4999
[00:53:19] .....................................................................i...........i.................. 800/4999
[00:53:22] ........................................................................................iiiii....... 900/4999
[00:53:25] ...........ii.iiii.................................................................................. 1000/4999
---
[00:54:00] .................................................................................................... 2200/4999
[00:54:04] .................................................................................................... 2300/4999
[00:54:08] .................................................................................................... 2400/4999
[00:54:11] .................................................................................................... 2500/4999
[00:54:15] ...................................................................iiiiiiiii........................ 2600/4999
[00:54:22] ...............................ii................................................................... 2800/4999
[00:54:24] .................................................................................................... 2900/4999
[00:54:28] .................................................................................................... 3000/4999
[00:54:31] ..........................i......................................................................... 3100/4999
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:24] 
[01:08:24] running 115 tests
[01:08:27] i..ii...iii...iii....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:08:28] .i....iiii.....
[01:08:28] 
[01:08:28]  finished in 3.437
[01:08:28] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:42] 
[01:08:42] running 118 tests
[01:09:05] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:09:08] ......iii.i.....ii
[01:09:08] 
[01:09:08]  finished in 26.666
[01:09:08] travis_fold:end:test_debuginfo

---
[01:39:50] travis_fold:end:stage0-linkchecker

[01:39:50] travis_time:end:stage0-linkchecker:start=1541717159025219240,finish=1541717161396311668,duration=2371092428

[01:41:39] std/sync/atomic/struct.AtomicI64.html:356: broken link - std/std/primitive.i64.html
[01:41:39] std/sync/atomic/struct.AtomicU8.html:356: broken link - std/std/primitive.u8.html
[01:41:39] std/sync/atomic/struct.AtomicI32.html:356: broken link - std/std/primitive.i32.html
[01:41:39] std/sync/atomic/struct.AtomicU16.html:356: broken link - std/std/primitive.u16.html
[01:41:39] std/sync/atomic/struct.AtomicI16.html:356: broken link - std/std/primitive.i16.html
[01:41:39] std/sync/atomic/struct.AtomicI8.html:356: broken link - std/std/primitive.i8.html
[01:41:39] std/sync/atomic/struct.AtomicU64.html:356: broken link - std/std/primitive.u64.html
[01:41:39] std/sync/atomic/struct.AtomicU32.html:356: broken link - std/std/primitive.u32.html
[01:41:39] std/sync/atomic/struct.AtomicIsize.html:316: broken link - std/std/primitive.isize.html
[01:41:39] std/sync/atomic/struct.AtomicBool.html:237: broken link - std/std/primitive.bool.html
[01:41:39] std/sync/atomic/struct.AtomicUsize.html:316: broken link - std/std/primitive.usize.html
[01:41:45] std/convert/trait.From.html:199: broken link - std/convert/struct.AtomicBool.html
[01:41:45] std/convert/trait.From.html:222: broken link - std/convert/struct.AtomicI16.html
[01:41:45] std/convert/trait.From.html:226: broken link - std/convert/struct.AtomicI32.html
[01:41:45] std/convert/trait.From.html:228: broken link - std/convert/struct.AtomicI64.html
[01:41:45] std/convert/trait.From.html:236: broken link - std/convert/struct.AtomicI8.html
[01:41:45] std/convert/trait.From.html:237: broken link - std/convert/struct.AtomicIsize.html
[01:41:45] std/convert/trait.From.html:247: broken link - std/convert/struct.AtomicU16.html
[01:41:45] std/convert/trait.From.html:260: broken link - std/convert/struct.AtomicU32.html
[01:41:45] std/convert/trait.From.html:263: broken link - std/convert/struct.AtomicU64.html
[01:41:45] std/convert/trait.From.html:300: broken link - std/convert/struct.AtomicU8.html
[01:41:45] std/convert/trait.From.html:301: broken link - std/convert/struct.AtomicUsize.html
[01:42:07] core/sync/atomic/struct.AtomicI64.html:355: broken link - core/std/primitive.i64.html
[01:42:07] core/sync/atomic/struct.AtomicU8.html:355: broken link - core/std/primitive.u8.html
[01:42:07] core/sync/atomic/struct.AtomicI32.html:355: broken link - core/std/primitive.i32.html
[01:42:07] core/sync/atomic/struct.AtomicU16.html:355: broken link - core/std/primitive.u16.html
[01:42:07] core/sync/atomic/struct.AtomicI16.html:355: broken link - core/std/primitive.i16.html
[01:42:07] core/sync/atomic/struct.AtomicI8.html:355: broken link - core/std/primitive.i8.html
[01:42:07] core/sync/atomic/struct.AtomicU64.html:355: broken link - core/std/primitive.u64.html
[01:42:07] core/sync/atomic/struct.AtomicU32.html:355: broken link - core/std/primitive.u32.html
[01:42:07] core/sync/atomic/struct.AtomicIsize.html:315: broken link - core/std/primitive.isize.html
[01:42:07] core/sync/atomic/struct.AtomicBool.html:236: broken link - core/std/primitive.bool.html
[01:42:07] core/sync/atomic/struct.AtomicUsize.html:315: broken link - core/std/primitive.usize.html
[01:42:09] core/convert/trait.From.html:71: broken link - core/convert/struct.AtomicBool.html
[01:42:09] core/convert/trait.From.html:162: broken link - core/convert/struct.AtomicI16.html
[01:42:09] core/convert/trait.From.html:168: broken link - core/convert/struct.AtomicI32.html
[01:42:09] core/convert/trait.From.html:172: broken link - core/convert/struct.AtomicI64.html
[01:42:09] core/convert/trait.From.html:174: broken link - core/convert/struct.AtomicI8.html
[01:42:09] core/convert/trait.From.html:181: broken link - core/convert/struct.AtomicIsize.html
[01:42:09] core/convert/trait.From.html:209: broken link - core/convert/struct.AtomicU16.html
[01:42:09] core/convert/trait.From.html:218: broken link - core/convert/struct.AtomicU32.html
[01:42:09] core/convert/trait.From.html:224: broken link - core/convert/struct.AtomicU64.html
[01:42:09] core/convert/trait.From.html:227: broken link - core/convert/struct.AtomicU8.html
[01:42:09] core/convert/trait.From.html:238: broken link - core/convert/struct.AtomicUsize.html
[01:42:12] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:42:12] 
[01:42:12] 
[01:42:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:42:12] expected success, got: exit code: 101
[01:42:12] expected success, got: exit code: 101
[01:42:12] 
[01:42:12] 
[01:42:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:12] Build completed unsuccessfully in 0:53:01
[01:42:12] make: *** [check] Error 1
[01:42:12] Makefile:58: recipe for target 'check' failed
