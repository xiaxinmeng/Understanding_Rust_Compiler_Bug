plain
travis_time:end:10909bdb:start=1543351689175491871,finish=1543351691414729242,duration=2239237371
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] Cleared directory 'src/tools/rls'
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
---
[00:50:14] .................................................................................................... 100/5064
[00:50:17] .................................................................................................... 200/5064
[00:50:20] .............................ii............................................ii...................ii.. 300/5064
[00:50:22] ..............................................................................................iii... 400/5064
[00:50:25] .....iiiiiiii.iii............................iii...........................................i........ 500/5064
[00:50:32] .................................................................................................... 700/5064
[00:50:37] ................................................................................................i... 800/5064
[00:50:41] ........i........................................................................................... 900/5064
[00:50:44] ...............iiiii..................ii.iiii....................................................... 1000/5064
---
[00:51:23] .................................................................................................... 2300/5064
[00:51:27] .................................................................................................... 2400/5064
[00:51:30] .................................................................................................... 2500/5064
[00:51:34] .................................................................................................... 2600/5064
[00:51:37] .......iiiiiiiii.................................................................................... 2700/5064
[00:51:43] .................................................................................................... 2900/5064
[00:51:47] .................................................................................................... 3000/5064
[00:51:50] ......................................................................i............................. 3100/5064
[00:51:53] .................................................................................................... 3200/5064
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:37] 
[01:05:37] running 117 tests
[01:05:40] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:05:40] i.i.....iiii.....
[01:05:40] 
[01:05:40]  finished in 3.394
[01:05:40] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:55] 
[01:05:55] running 118 tests
[01:06:19] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:23] ......iii.i.....ii
[01:06:23] 
[01:06:23]  finished in 28.999
[01:06:23] travis_fold:end:test_debuginfo

---
[01:35:27] travis_fold:end:stage0-linkchecker

[01:35:27] travis_time:end:stage0-linkchecker:start=1543357425135618272,finish=1543357427553407992,duration=2417789720

[01:35:28] index.html:103: broken link - embedded-book/index.html
[01:39:20] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:49:9
[01:39:20] 
[01:39:20] 
[01:39:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:39:20] expected success, got: exit code: 101
[01:39:20] expected success, got: exit code: 101
[01:39:20] 
[01:39:20] 
[01:39:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:20] Build completed unsuccessfully in 0:53:05
[01:39:20] make: *** [check] Error 1
[01:39:20] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d86b6fb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 27 22:27:40 UTC 2018
---
travis_time:end:003937d8:start=1543357666017388027,finish=1543357666227483637,duration=210095610
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c69a2b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0118cc5e
$ dmesg | grep -i kill
