plain
travis_time:end:0f9ed180:start=1542848564734308315,finish=1542848565870466954,duration=1136158639
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/2ce92beabb912d417a7314d6da83ac9b50dc2afb.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm-emscripten.tar.gz &&         curl -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/7f23313edff8beccb3fe44b815714269c5124c15.tar.gz
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/doc/rustc-guide &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/doc/rustc-guide
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/doc/rustc-guide'
[00:00:00] Cleared directory 'src/libbacktrace'
---
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/doc/rustc-guide' (https://github.com/rust-lang-nursery/rustc-guide.git) registered for path 'src/doc/rustc-guide'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
---
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/doc/rustc-guide'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/stdsimd'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
---
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lldb'...
[00:00:53] Submodule path 'src/dlmalloc': checked out 'c99638dc2ecfc750cc1656f6edb2bd062c1e0981'
[00:00:53] Submodule path 'src/doc/nomicon': checked out 'f8a4e96feb2e5a6ed1ef170ad40e3509a7755cb4'
[00:00:53] Submodule path 'src/doc/reference': checked out '60077efda319c95a89fe39609803c5433567adbf'
[00:00:53] Submodule path 'src/doc/rustc-guide': checked out 'fe35892c7878418ba08f6c2c6bd2e45d604f39ea'
[00:00:53] Submodule path 'src/libcompiler_builtins': checked out '939cbca6e9d829265d6cf006d3532142a4061cd3'
[00:00:53] Submodule 'compiler-rt' (https://github.com/rust-lang/compiler-rt) registered for path 'src/libcompiler_builtins/compiler-rt'
[00:00:53] Submodule 'libm' (https://github.com/rust-lang-nursery/libm) registered for path 'src/libcompiler_builtins/libm'
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
---

[00:03:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:50] tidy error: /checkout/src/doc/rustc-guide/ci/check_line_lengths.sh:33: line longer than 100 chars
[00:03:51] some tidy checks failed
[00:03:51] 
[00:03:51] 
[00:03:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:51] 
[00:03:51] 
[00:03:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:51] Build completed unsuccessfully in 0:00:55
[00:03:51] Build completed unsuccessfully in 0:00:55
[00:03:51] make: *** [tidy] Error 1
[00:03:51] Makefile:79: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:106616be
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 22 01:06:46 UTC 2018
---
travis_time:end:0a1b5e5b:start=1542848807291175862,finish=1542848807295186031,duration=4010169
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21d72815
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aafcc64
travis_time:start:0aafcc64
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0544a40f
$ dmesg | grep -i kill
