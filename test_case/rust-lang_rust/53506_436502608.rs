plain
travis_time:end:03875da0:start=1541559760217855409,finish=1541559815657922282,duration=55440066873
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:49:39] .................................................................................................... 100/4997
[00:49:41] .................................................................................................... 200/4997
[00:49:44] ........................................................................ii...................ii..... 300/4997
[00:49:47] ...........................................................................................iii...... 400/4997
[00:49:50] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4997
[00:49:57] .................................................................................................... 700/4997
[00:50:03] .....................................................................i...........i.................. 800/4997
[00:50:06] ........................................................................................iiiii....... 900/4997
[00:50:10] ...........ii.iiii.................................................................................. 1000/4997
---
[00:50:44] .................................................................................................... 2200/4997
[00:50:48] .................................................................................................... 2300/4997
[00:50:52] .................................................................................................... 2400/4997
[00:50:56] .................................................................................................... 2500/4997
[00:50:59] ..............................................................................iiiiiiiii............. 2600/4997
[00:51:06] .............................ii..................................................................... 2800/4997
[00:51:08] .................................................................................................... 2900/4997
[00:51:12] .................................................................................................... 3000/4997
[00:51:15] ........................i........................................................................... 3100/4997
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:33] 
[01:04:33] running 115 tests
[01:04:35] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:04:36] .i....iiii.....
[01:04:36] 
[01:04:36]  finished in 3.393
[01:04:36] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:50] 
[01:04:50] running 118 tests
[01:05:14] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:17] ......iii.i.....ii
[01:05:17] 
[01:05:17]  finished in 27.525
[01:05:17] travis_fold:end:test_debuginfo

---
[01:33:05] travis_fold:end:stage0-linkchecker

[01:33:05] travis_time:end:stage0-linkchecker:start=1541565408321354955,finish=1541565410576930935,duration=2255575980

[01:35:37] std/convert/trait.From.html:199: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.bool.html
[01:35:37] std/convert/trait.From.html:199: broken link - std/convert/struct.AtomicBool.html
[01:35:37] std/convert/trait.From.html:222: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i16.html
[01:35:37] std/convert/trait.From.html:222: broken link - std/convert/struct.AtomicI16.html
[01:35:37] std/convert/trait.From.html:226: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i32.html
[01:35:37] std/convert/trait.From.html:226: broken link - std/convert/struct.AtomicI32.html
[01:35:37] std/convert/trait.From.html:228: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i64.html
[01:35:37] std/convert/trait.From.html:228: broken link - std/convert/struct.AtomicI64.html
[01:35:37] std/convert/trait.From.html:236: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i8.html
[01:35:37] std/convert/trait.From.html:236: broken link - std/convert/struct.AtomicI8.html
[01:35:37] std/convert/trait.From.html:237: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.isize.html
[01:35:37] std/convert/trait.From.html:237: broken link - std/convert/struct.AtomicIsize.html
[01:35:37] std/convert/trait.From.html:247: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u16.html
[01:35:37] std/convert/trait.From.html:247: broken link - std/convert/struct.AtomicU16.html
[01:35:37] std/convert/trait.From.html:260: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u32.html
[01:35:37] std/convert/trait.From.html:260: broken link - std/convert/struct.AtomicU32.html
[01:35:37] std/convert/trait.From.html:263: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u64.html
[01:35:37] std/convert/trait.From.html:263: broken link - std/convert/struct.AtomicU64.html
[01:35:37] std/convert/trait.From.html:300: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u8.html
[01:35:37] std/convert/trait.From.html:300: broken link - std/convert/struct.AtomicU8.html
[01:35:37] std/convert/trait.From.html:301: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.usize.html
[01:35:37] std/convert/trait.From.html:301: broken link - std/convert/struct.AtomicUsize.html
[01:35:58] core/convert/trait.From.html:71: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.bool.html
[01:35:58] core/convert/trait.From.html:71: broken link - core/convert/struct.AtomicBool.html
[01:35:58] core/convert/trait.From.html:162: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i16.html
[01:35:58] core/convert/trait.From.html:162: broken link - core/convert/struct.AtomicI16.html
[01:35:58] core/convert/trait.From.html:168: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i32.html
[01:35:58] core/convert/trait.From.html:168: broken link - core/convert/struct.AtomicI32.html
[01:35:58] core/convert/trait.From.html:172: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i64.html
[01:35:58] core/convert/trait.From.html:172: broken link - core/convert/struct.AtomicI64.html
[01:35:58] core/convert/trait.From.html:174: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.i8.html
[01:35:58] core/convert/trait.From.html:174: broken link - core/convert/struct.AtomicI8.html
[01:35:58] core/convert/trait.From.html:181: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.isize.html
[01:35:58] core/convert/trait.From.html:181: broken link - core/convert/struct.AtomicIsize.html
[01:35:58] core/convert/trait.From.html:209: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u16.html
[01:35:58] core/convert/trait.From.html:209: broken link - core/convert/struct.AtomicU16.html
[01:35:58] core/convert/trait.From.html:218: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u32.html
[01:35:58] core/convert/trait.From.html:218: broken link - core/convert/struct.AtomicU32.html
[01:35:58] core/convert/trait.From.html:224: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u64.html
[01:35:58] core/convert/trait.From.html:224: broken link - core/convert/struct.AtomicU64.html
[01:35:58] core/convert/trait.From.html:227: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.u8.html
[01:35:58] core/convert/trait.From.html:227: broken link - core/convert/struct.AtomicU8.html
[01:35:58] core/convert/trait.From.html:238: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/primitive.usize.html
[01:35:58] core/convert/trait.From.html:238: broken link - core/convert/struct.AtomicUsize.html
