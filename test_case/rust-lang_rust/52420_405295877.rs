plain

[00:03:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:54] tidy error: /checkout/src/liballoc/string.rs:994: line longer than 100 chars
[00:03:54] tidy error: /checkout/src/liballoc/collections/linked_list.rs:1320: line longer than 100 chars
[00:03:54] tidy error: /checkout/src/liballoc/collections/vec_deque.rs:606: line longer than 100 chars
[00:03:54] tidy error: /checkout/src/liballoc/vec.rs:566: line longer than 100 chars
[00:03:55] some tidy checks failed
[00:03:55] 
[00:03:55] 
[00:03:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:55] 
[00:03:55] 
[00:03:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:55] Build completed unsuccessfully in 0:00:48
[00:03:55] Build completed unsuccessfully in 0:00:48
[00:03:55] Makefile:79: recipe for target 'tidy' failed
[00:03:55] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib
177640 ./obj/build/bootstrap/debug/deps
177640 ./obj/build/bootstrap/debug/deps
170388 ./obj/build/cache
170384 ./obj/build/cache/2018-07-13
158296 ./.git/modules
158292 ./.git/modules/src
149120 ./src/llvm-emscripten/test
145036 ./obj/build/bootstrap/debug/incremental
130516 ./obj/build/bootstrap/debug/incremental/bootstrap-3kaq1kqcanyi4
130512 ./obj/build/bootstrap/debug/incremental/bootstrap-3kaq1kqcanyi4/s-f2yn7t2c53-1dpl6b5-ee4a52izcif7
97528 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
76824 ./.git/modules/src/tools
71508 ./src/llvm/lib
65420 ./src/llvm-emscripten/test/CodeGen
---
32212 ./src/libcompiler_builtins/compiler-rt/test
31048 ./.git/modules/src/tools/lld
31024 ./src/llvm/test/tools
30672 ./.git/modules/src/tools/lld/objects
30664 ./.git/modules/src/t9820 ./src/llvm/test/CodeGen/AMDGPU
9176 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
8992 ./src/doc/book
8904 ./src/llvm/lib/CodeGen
8572 ./.git/modules/src/tools/rustfmt
---
travis_time:end:076ea8a5:start=1531756599015138620,finish=1531756599022816824,duration=7678204
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b87ee8c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07ce538e
travis_time:start:07ce538e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01e2c38b
$ dmesg | grep -i kill
