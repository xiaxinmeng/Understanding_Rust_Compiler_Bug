plain
[00:48:16] .................................................................................................... 2200/4591
[00:48:20] .............i...................................................................................... 2300/4591
[00:48:24] .................................................................................................... 2400/4591
[00:48:27] .................................................................................................... 2500/4591
[00:48:31] ..........................iiiiiiiii................................................................. 2600/4591
[00:48:37] .................................................................................................... 2800/4591
[00:48:41] .................................................................................................... 2900/4591
[00:48:43] ..............................................i..................................................... 3000/4591
[00:48:46] .................................................................................................... 3100/4591
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:11] 
[01:01:11] running 112 tests
[01:01:14] i..ii...iii.......i...i..........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i. 100/112
[01:01:14] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:01:14] 
[01:01:14]  finished in 3.414
[01:01:14] travis_fold:end:test_codegen
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:29] 
[01:01:29] running 110 tests
[01:01:39] iiii.......i..i........i..i.i.............i..........iiii...........i....i.F........ii.i.i.......ii. 100/110
3025284 ./obj
2809240 ./obj/build
2176700 ./obj/build/x86_64-unknown-linux-gnu
1069824 ./src
---
151492 ./obj/build/bootstrap/debug/incremental
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
136476 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
136472 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unk\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:39bcc22c
travis_time:start:39bcc22c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d606feb
$ dmesg | grep -i kill
