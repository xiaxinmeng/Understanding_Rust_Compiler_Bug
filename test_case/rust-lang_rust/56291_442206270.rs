plain
travis_time:end:1480349b:start=1543344821282012911,finish=1543344822353429122,duration=1071416211
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
[00:50:01] .................................................................................................... 100/5064
[00:50:04] .................................................................................................... 200/5064
[00:50:07] .............................ii............................................ii...................ii.. 300/5064
[00:50:09] ..............................................................................................iii... 400/5064
[00:50:12] .....iiiiiiii.iii............................iii...........................................i........ 500/5064
[00:50:19] .................................................................................................... 700/5064
[00:50:24] ................................................................................................i... 800/5064
[00:50:28] ........i........................................................................................... 900/5064
[00:50:31] ...............iiiii..................ii.iiii....................................................... 1000/5064
---
[00:51:10] .................................................................................................... 2300/5064
[00:51:14] .................................................................................................... 2400/5064
[00:51:18] .................................................................................................... 2500/5064
[00:51:21] .................................................................................................... 2600/5064
[00:51:25] .......iiiiiiiii.................................................................................... 2700/5064
[00:51:31] .................................................................................................... 2900/5064
[00:51:34] .................................................................................................... 3000/5064
[00:51:38] ......................................................................i............................. 3100/5064
[00:51:41] .................................................................................................... 3200/5064
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:22] 
[01:05:22] running 117 tests
[01:05:25] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:05:25] i.i.....iiii.....
[01:05:25] 
[01:05:25]  finished in 3.424
[01:05:25] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:39] 
[01:05:39] running 118 tests
[01:06:03] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:07] ......iii.i.....ii
[01:06:07] 
[01:06:07]  finished in 27.857
[01:06:07] travis_fold:end:test_debuginfo

---
[01:34:15] travis_fold:end:stage0-linkchecker

[01:34:15] travis_time:end:stage0-linkchecker:start=1543350484457610554,finish=1543350486851739619,duration=2394129065

[01:34:15] index.html:103: broken link - embedded-book/index.html
none            4.0K     0  4.0K   0% /sys/fs/cgroup
none            5.0M     0  5.0M   0% /run/lock
none            7.4G     0  7.4G   0% /run/shm
none            100M     0  100M   0% /run/user
none            100M     0  100M   0% /run/user
none            768M     0  768M   0% /var/ramfs
60 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134656 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f72ivlhz8s-d8n60a-18zp7vy2v2s65
129168 ./.git/modules/src
123696 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
115352 ./src/llvm/test/CodeGen
---
36560 ./.git/modules/src/libcompiler_builtins/modules
36044 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt
35640 ./src/tools/clang/lib
35580 ./.git/modules/src/libcompiler_builtins/modules/compiler-rt/objects
35572 ./.git/modules/src/libcompiler_builtins/modules/compileores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
