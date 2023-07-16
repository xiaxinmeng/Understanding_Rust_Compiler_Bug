plain
travis_time:end:0263f7ab:start=1544535081973781003,finish=1544535203572342663,duration=121598561660
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
[00:54:35] 
[00:54:35] running 120 tests
[00:54:38] i..ii...iii..iiii.....i...i..........i...iii............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:54:38] ..ii.i.....iiii.....
[00:54:38] 
[00:54:38]  finished in 3.345
[00:54:38] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:52] 
[00:54:52] running 118 tests
[00:55:14] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:55:18] ......iii.i.....ii
[00:55:18] 
[00:55:18]  finished in 26.444
[00:55:18] travis_fold:end:test_debuginfo

---
[01:19:26] travis_fold:end:stage0-linkchecker

[01:19:26] travis_time:end:stage0-linkchecker:start=1544539976208694202,finish=1544539978550606666,duration=2341912464

[01:19:26] embedded-book/print.html:377: broken link - embedded-book/install/linux.html
[01:19:26] embedded-book/print.html:378: broken link - embedded-book/install/windows.html
[01:19:26] embedded-book/print.html:379: broken link - embedded-book/install/macos.html
[01:19:26] embedded-book/print.html:482: broken link - embedded-book/verify.html
[01:19:26] embedded-book/print.html:494: broken link - embedded-book/verify.html
[01:19:26] embedded-book/print.html:520: broken link - embedded-book/verify.html
[01:19:26] embedded-book/print.html:555: broken link - hardware.html
[01:19:26] embedded-book/print.html:562: broken link - hardware.html
[01:19:26] embedded-book/print.html:565: broken link - embedded-book/linux.html
[01:19:26] embedded-book/print.html:631: broken link - embedded-book/hardware.html
[01:19:26] embedded-book/print.html:1110: broken link - embedded-book/qemu.html
[01:19:26] embedded-book/print.html:1154: broken link - intro/install/verify.html
[01:19:26] embedded-book/print.html:1176: broken link - intro/install/verify.html
[01:19:26] embedded-book/print.html:1332: broken link - portability/index.html
[01:19:26] embedded-book/print.html:1355: broken link - peripherals/index.html
[01:19:31] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:49:9
[01:19:31] 
[01:19:31] 
[01:19:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:19:31] expected success, got: exit code: 101
[01:19:31] expected success, got: exit code: 101
[01:19:31] 
[01:19:31] 
[01:19:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:31] Build completed unsuccessfully in 0:35:09
[01:19:31] Makefile:58: recipe for target 'check' failed
[01:19:31] make: *** [check] Error 1
3128556 ./obj
3113272 ./obj/build
2458616 ./obj/build/x86_64-unknown-linux-gnu
1168224 ./src
