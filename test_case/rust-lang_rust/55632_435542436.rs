plain
travis_time:end:0de321d0:start=1541197078263157112,finish=1541197137001122189,duration=58737965077
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:51:22] .................................................................................................... 100/4983
[00:51:25] .................................................................................................... 200/4983
[00:51:28] ...........................................................................................ii....... 300/4983
[00:51:31] .........................................................................................iii........ 400/4983
[00:51:34] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:51:41] .................................................................................................... 700/4983
[00:51:48] ..................................................................i...........i..................... 800/4983
[00:51:51] ....................................................................................iiiii........... 900/4983
[00:51:55] .................................................................................................... 1000/4983
---
[00:52:34] .................................................................................................... 2200/4983
[00:52:39] .................................................................................................... 2300/4983
[00:52:42] .................................................................................................... 2400/4983
[00:52:46] .................................................................................................... 2500/4983
[00:52:50] ..................................................................iiiiiiiii......................... 2600/4983
[00:52:57] ..................ii................................................................................ 2800/4983
[00:53:00] .................................................................................................... 2900/4983
[00:53:04] .................................................................................................... 3000/4983
[00:53:07] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:55] 
[01:06:55] running 115 tests
[01:06:58] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:06:58] .i....iiii.....
[01:06:58] 
[01:06:58]  finished in 3.577
[01:06:58] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:13] 
[01:07:13] running 118 tests
[01:07:38] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:07:42] ......iii.i.....ii
[01:07:42] 
[01:07:42]  finished in 28.620
[01:07:42] travis_fold:end:test_debuginfo

---
[01:42:30] 
[01:42:30] failures:
[01:42:30] 
[01:42:30] ---- /checkout/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::overflowing_literals (line 293) stdout ----
[01:42:30] error: literal out of range for u8
[01:42:30]   |
[01:42:30]   |
[01:42:30] 3 | let x: u8 = 1000;
[01:42:30]   |
[01:42:30]   = note: #[deny(overflowing_literals)] on by default
[01:42:30] 
[01:42:30] thread '/checkout/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::overflowing_literals (line 293)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
---
[01:42:30] 
[01:42:30] 
[01:42:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:30] Build completed unsuccessfully in 0:54:59
[01:42:30] Makefile:58: recipe for target 'check' failed
[01:42:30] make: *** [check] Error 1
3552496 ./obj
3406820 ./obj/build
2767728 ./obj/build/x86_64-unknown-linux-gnu
1195164 ./.git
---
163176 ./.git/modules/src/tools/lldb/objects
163168 ./.git/modules/src/tools/lldb/objects/pack
151412 ./src/tools/clang
150348 ./obj/build/bootstrap/debug/incremental
149112 ./src/llvm-emscripte"$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
