plain
[00:47:44] ....................................................................................................
[00:47:48] ....................................................................................................
[00:47:50] ...........................i........................................................................
[00:47:53] ....................................................................................................
[00:47:56] .............................................................................iiiiiiiii..............
[00:48:01] ....................................................................................................
[00:48:05] ....................................................................................................
[00:48:07] .........................................................i..........................................
[00:48:10] ....................................................................................................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:58] 
[00:59:58] running 258 tests
[01:00:24] .......................i................................................................F...........
stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
74020 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib
72532 ./src/llvm/lib
71772 ./obj/build/x86_64-unknown-linux-gnu/doc/std
---
38124 ./src/test/ui
38112 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
37752 ./src/tools/lldb/www
36984 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
36960 ./otf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fd39067
travis_time:start:0fd39067
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0cc56efb
$ dmesg | grep -i kill
