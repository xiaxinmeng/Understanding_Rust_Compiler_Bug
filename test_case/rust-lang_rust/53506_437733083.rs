plain
travis_time:end:005a4730:start=1541983527774742137,finish=1541983584754637984,duration=56979895847
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:13] .................................................................................................... 100/5014
[00:52:16] .................................................................................................... 200/5014
[00:52:19] .............................ii............................................ii...................ii.. 300/5014
[00:52:22] ..............................................................................................iii... 400/5014
[00:52:25] .....iiiiiiii.iii............................iii...........................................i........ 500/5014
[00:52:32] .................................................................................................... 700/5014
[00:52:39] .............................................................................i...........i.......... 800/5014
[00:52:42] ................................................................................................iiii 900/5014
[00:52:46] i..................ii.iiii.......................................................................... 1000/5014
---
[00:53:23] .................................................................................................... 2200/5014
[00:53:27] .................................................................................................... 2300/5014
[00:53:31] .................................................................................................... 2400/5014
[00:53:35] .................................................................................................... 2500/5014
[00:53:38] .............................................................................iiiiiiiii.............. 2600/5014
[00:53:45] ..........................................ii........................................................ 2800/5014
[00:53:48] .................................................................................................... 2900/5014
[00:53:52] .................................................................................................... 3000/5014
[00:53:55] ....................................i............................................................... 3100/5014
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:51] 
[01:07:51] running 115 tests
[01:07:54] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii...............i..ii..ii 100/115
[01:07:54] .i....iiii.....
[01:07:54] 
[01:07:54]  finished in 3.552
[01:07:54] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:09] 
[01:08:09] running 118 tests
[01:08:36] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:08:41] ......iii.i.....ii
[01:08:41] 
[01:08:41]  finished in 31.325
[01:08:41] travis_fold:end:test_debuginfo

---
[01:38:32] travis_fold:end:stage0-linkchecker

[01:38:32] travis_time:end:stage0-linkchecker:start=1541989505347120390,finish=1541989507614615679,duration=2267495289

[01:41:35] std/convert/trait.From.html:199: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.bool.html
[01:41:35] std/convert/trait.From.html:199: broken link - std/convert/struct.AtomicBool.html
[01:41:35] std/convert/trait.From.html:222: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i16.html
[01:41:35] std/convert/trait.From.html:222: broken link - std/convert/struct.AtomicI16.html
[01:41:35] std/convert/trait.From.html:226: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i32.html
[01:41:35] std/convert/trait.From.html:226: broken link - std/convert/struct.AtomicI32.html
[01:41:35] std/convert/trait.From.html:228: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i64.html
[01:41:35] std/convert/trait.From.html:228: broken link - std/convert/struct.AtomicI64.html
[01:41:35] std/convert/trait.From.html:236: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i8.html
[01:41:35] std/convert/trait.From.html:236: broken link - std/convert/struct.AtomicI8.html
[01:41:35] std/convert/trait.From.html:237: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.isize.html
[01:41:35] std/convert/trait.From.html:237: broken link - std/convert/struct.AtomicIsize.html
[01:41:35] std/convert/trait.From.html:247: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u16.html
[01:41:35] std/convert/trait.From.html:247: broken link - std/convert/struct.AtomicU16.html
[01:41:35] std/convert/trait.From.html:260: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u32.html
[01:41:35] std/convert/trait.From.html:260: broken link - std/convert/struct.AtomicU32.html
[01:41:35] std/convert/trait.From.html:263: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u64.html
[01:41:35] std/convert/trait.From.html:263: broken link - std/convert/struct.AtomicU64.html
[01:41:35] std/convert/trait.From.html:300: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u8.html
[01:41:35] std/convert/trait.From.html:300: broken link - std/convert/struct.AtomicU8.html
[01:41:35] std/convert/trait.From.html:301: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.usize.html
[01:41:35] std/convert/trait.From.html:301: broken link - std/convert/struct.AtomicUsize.html
[01:42:03] core/convert/trait.From.html:71: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.bool.html
[01:42:03] core/convert/trait.From.html:71: broken link - core/convert/struct.AtomicBool.html
[01:42:03] core/convert/trait.From.html:162: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i16.html
[01:42:03] core/convert/trait.From.html:162: broken link - core/convert/struct.AtomicI16.html
[01:42:03] core/convert/trait.From.html:168: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i32.html
[01:42:03] core/convert/trait.From.html:168: broken link - core/convert/struct.AtomicI32.html
[01:42:03] core/convert/trait.From.html:172: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i64.html
[01:42:03] core/convert/trait.From.html:172: broken link - core/convert/struct.AtomicI64.html
[01:42:03] core/convert/trait.From.html:174: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i8.html
[01:42:03] core/convert/trait.From.html:174: broken link - core/convert/struct.AtomicI8.html
[01:42:03] core/convert/trait.From.html:181: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.isize.html
[01:42:03] core/convert/trait.From.html:181: broken link - core/convert/struct.AtomicIsize.html
[01:42:03] core/convert/trait.From.html:209: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u16.html
[01:42:03] core/convert/trait.From.html:209: broken link - core/convert/struct.AtomicU16.html
[01:42:03] core/convert/trait.From.html:218: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u32.html
[01:42:03] core/convert/trait.From.html:218: broken link - core/convert/struct.AtomicU32.html
[01:42:03] core/convert/trait.From.html:224: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u64.html
[01:42:03] core/convert/trait.From.html:224: broken link - core/convert/struct.AtomicU64.html
[01:42:03] core/convert/trait.From.html:227: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u8.html
[01:42:03] core/convert/trait.From.html:227: broken link - core/convert/struct.AtomicU8.html
[01:42:03] core/convert/trait.From.html:238: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.usize.html
[01:42:03] core/convert/trait.From.html:238: broken link - core/convert/struct.AtomicUsize.html
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
---
travis_time:end:03fb4cde:start=1541989726516928107,finish=1541989726525483044,duration=8554937
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:179d2409
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\
