plain
travis_time:end:03ce1320:start=1544468232109732892,finish=1544468233350606338,duration=1240873446
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lld.tar.gz &&         curl -sSL -o download-src-tools-lld.tar.gz https://github.com/rust-lang/lld/archive/1928c5eeb613a4c6d232fc47ae91914bbfd92a79.tar.gz
[00:00:00] rm 'src/tools/lldb'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lldb.tar.gz &&         curl -sSL -o download-src-tools-lldb.tar.gz https://github.com/rust-lang-nursery/lldb/archive/8ad0817ce45b0eef9d374691b23f2bd69c164254.tar.gz
[00:00:00] rm 'src/tools/clang'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide src/doc/edition-guide src/rust-sgx src/doc/embedded-book &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/libbacktrace src/doc/rustc-guide src/doc/edition-guide src/rust-sgx src/doc/embedded-book
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/edition-guide'
[00:00:00] Cleared directory 'src/doc/embedded-book'
[00:00:00] Cleared directory 'src/doc/nomicon'
---
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/edition-guide' (https://github.com/rust-lang-nursery/edition-guide) registered for path 'src/doc/edition-guide'
[00:00:00] Submodule 'src/doc/embedded-book' (https://github.com/rust-embedded/book.git) registered for path 'src/doc/embedded-book'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:35] 
[00:55:35] running 120 tests
[00:55:38] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:38] ..ii.i.....iiii.....
[00:55:38] 
[00:55:38]  finished in 3.450
[00:55:38] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:53] 
[00:55:53] running 118 tests
[00:56:16] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:20] ......iii.i.....ii
[00:56:20] 
[00:56:20]  finished in 27.178
[00:56:20] travis_fold:end:test_debuginfo

---
[01:21:06] travis_fold:end:stage0-linkchecker

[01:21:06] travis_time:end:stage0-linkchecker:start=1544473105736719694,finish=1544473108036872422,duration=2300152728

[01:21:06] index.html:106: broken link - embedded-book/index.html
[01:21:12] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:49:9
[01:21:12] 
[01:21:12] 
[01:21:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:21:12] expected success, got: exit code: 101
[01:21:12] expected success, got: exit code: 101
[01:21:12] 
[01:21:12] 
[01:21:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:12] Build completed unsuccessfully in 0:36:08
[01:21:12] make: *** [check] Error 1
[01:21:12] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d52d498
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 10 20:18:34 UTC 2018
---
travis_time:end:25ccfe1e:start=1544473116457021205,finish=1544473116547228790,duration=90207585
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:087ce538
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:047a9a46
$ dmesg | grep -i kill
