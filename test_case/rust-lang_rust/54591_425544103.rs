plain
151412 ./src/tools/clang
149124 ./src/llvm-emscripten/test
148656 ./obj/build/bootstrap/debug/incremental
134216 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q
134212 ./obj/build/bootstrap/debug/incremental/bootstrap-2zc4gzhr0d54q/s-f58dzdre87-19xvc4g-abll1u66rzrn
104700 ./src/tools/lldb
98940 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
72532 ./src/llvm/lib
---
travis_time:end:0b686af4:start=1538163417717083938,finish=1538163417721589856,duration=4505918
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:116a135c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:160b8cb4
travis_time:start:160b8cb4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3d4eac7c
$ dmesg | grep -i kill
