plain
[00:54:20] ....................................................................................................
[00:54:23] ....................................................i...............................................
[00:54:26] ....................................................................................................
[00:54:29] ....................................................................................................
[00:54:32] iiiiiiiii...........................................................................................
[00:54:39] ....................................................................................................
[00:54:42] ....................................................................................i...............
[00:54:45] ....................................................................................................
[00:54:48] .......................................i.i..ii......................................................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:58] 
[01:08:58] running 264 tests
[01:09:54] ...F...................i............................................................................
5438736 .
3177408 ./obj
2961440 ./obj/build
2341548 ./obj/build/x86_64-unknown-linux-gnu
---
151412 ./src/tools/clang
149120 ./src/llvm-emscripten/test
148628 ./obj/build/bootstrap/debug/incremental
134176 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134172 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f51g3c5y9a-jtdi7f-2g4vvhe5tuul3
131352 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
129776 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
129772 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
128436 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
---
76548 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
76544 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
74192 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
74188 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
74184 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/n printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00624020
travis_time:start:00624020
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01886056
$ dmesg | grep -i kill
