plain
travis_time:end:159d8f90:start=1543343823544017395,finish=1543343825315236869,duration=1771219474
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

[00:02:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:02:57] tidy error: /checkout/src/doc/embedded-book/ci/install.sh:4: line longer than 100 chars
[00:02:59] some tidy checks failed
[00:02:59] 
[00:02:59] 
[00:02:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:02:59] 
[00:02:59] 
[00:02:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:02:59] Build completed unsuccessfully in 0:00:55
[00:02:59] Build completed unsuccessfully in 0:00:55
[00:02:59] Makefile:79: recipe for target 'tidy' failed
[00:02:59] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:011c118c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 27 18:40:13 UTC 2018
---
travis_time:end:122f4312:start=1543344013496758017,finish=1543344013502017681,duration=5259664
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ee2938
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b3f5d0
travis_time:start:01b3f5d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a578208
$ dmesg | grep -i kill
