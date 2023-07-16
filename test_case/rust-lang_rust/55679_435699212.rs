plain
travis_time:end:06cab522:start=1541354273525415759,finish=1541354330566736164,duration=57041320405
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:33] .................................................................................................... 100/4988
[00:52:36] .................................................................................................... 200/4988
[00:52:39] ...........................................................................................ii....... 300/4988
[00:52:42] .........................................................................................iii........ 400/4988
[00:52:44] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4988
[00:52:52] .................................................................................................... 700/4988
[00:52:58] ..................................................................i...........i..................... 800/4988
[00:53:01] .....................................................................................iiiii.......... 900/4988
[00:53:04] .................................................................................................... 1000/4988
---
[00:53:41] .................................................................................................... 2200/4988
[00:53:45] .................................................................................................... 2300/4988
[00:53:49] .................................................................................................... 2400/4988
[00:53:53] .................................................................................................... 2500/4988
[00:53:57] ......................................................................iiiiiiiii..................... 2600/4988
[00:54:04] .....................ii............................................................................. 2800/4988
[00:54:06] .................................................................................................... 2900/4988
[00:54:10] .................................................................................................... 3000/4988
[00:54:13] ................i................................................................................... 3100/4988
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:01] 
[01:08:01] running 115 tests
[01:08:04] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:08:04] .i....iiii.....
[01:08:04] 
[01:08:04]  finished in 3.506
[01:08:04] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:19] 
[01:08:19] running 118 tests
[01:08:44] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:08:48] ......iii.i.....ii
[01:08:48] 
[01:08:48]  finished in 29.405
[01:08:48] travis_fold:end:test_debuginfo

---
[01:38:45] travis_fold:end:stage0-linkchecker

[01:38:45] travis_time:end:stage0-linkchecker:start=1541360263285010366,finish=1541360265642009548,duration=2356999182

[01:39:02] std/primitive.str.html:89: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/str/fn.from_utf8.html
[01:39:02] std/primitive.str.html:98: broken link - /checkout/obj/build/x86_64-unknown-linux-gnu/std/str/fn.from_utf8_mut.html
[01:42:22] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:42:22] 
[01:42:22] 
[01:42:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:42:22] expected success, got: exit code: 101
[01:42:22] expected success, got: exit code: 101
[01:42:22] 
[01:42:22] 
[01:42:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:22] Build completed unsuccessfully in 0:53:34
[01:42:22] make: *** [check] Error 1
[01:42:22] Makefile:58: recipe for target 'check' failed
