plain
travis_time:end:02cd6382:start=1540813194435776745,finish=1540813249675946409,duration=55240169664
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:51:40] .................................................................................................... 2200/4977
[00:51:44] .................................................................................................... 2300/4977
[00:51:48] .................................................................................................... 2400/4977
[00:51:52] .................................................................................................... 2500/4977
[00:51:55] .............................................................iiiiiiiii.............................. 2600/4977
[00:52:02] ............ii...................................................................................... 2800/4977
[00:52:05] .................................................................................................... 2900/4977
[00:52:09] .................................................................................................... 3000/4977
[00:52:11] .......i............................................................................................ 3100/4977
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:24] 
[01:05:24] running 112 tests
[01:05:27] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:05:28] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:28] 
[01:05:28]  finished in 3.551
[01:05:28] travis_fold:end:test_codegen
---
[01:16:01] error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.
[01:16:01] 
[01:16:01] 
[01:16:01] running 407 tests
[01:16:22] .....................F.............................................................................. 100/407
[01:16:55] .................................................................................................... 300/407
[01:17:11] .................................................................................................... 400/407
rustlib
72800 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
---
39608 ./obj/build/x86_64-unknown-linux-gnu/doc/book
39016 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps
38040 ./obj/build/x86_64-unknown-linux-gnu/test/ui/issues
37756 ./src/tools/lldb/www
37532 . travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a242b7c
travis_time:start:1a242b7c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01ea6fe6
$ dmesg | grep -i kill
