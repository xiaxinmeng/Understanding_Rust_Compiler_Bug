plain
travis_time:end:2335ba4a:start=1542431414712064425,finish=1542431468738479517,duration=54026415092
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:33] .................................................................................................... 100/5021
[00:50:36] .................................................................................................... 200/5021
[00:50:39] .............................ii............................................ii...................ii.. 300/5021
[00:50:42] ..............................................................................................iii... 400/5021
[00:50:44] .....iiiiiiii.iii............................iii...........................................i........ 500/5021
[00:50:51] .................................................................................................... 700/5021
[00:50:58] .................................................................................i...........i...... 800/5021
[00:51:01] .................................................................................................... 900/5021
[00:51:04] iiiii..................ii.iiii...................................................................... 1000/5021
---
[00:51:40] .................................................................................................... 2200/5021
[00:51:44] .................................................................................................... 2300/5021
[00:51:48] .................................................................................................... 2400/5021
[00:51:51] .................................................................................................... 2500/5021
[00:51:55] .................................................................................iiiiiiiii.......... 2600/5021
[00:52:02] ...............................................ii................................................... 2800/5021
[00:52:05] .................................................................................................... 2900/5021
[00:52:08] .................................................................................................... 3000/5021
[00:52:12] ..........................................i......................................................... 3100/5021
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:55] 
[01:05:55] running 116 tests
[01:05:58] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:05:59] i.i....iiii.....
[01:05:59] 
[01:05:59]  finished in 3.545
[01:05:59] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:13] 
[01:06:13] running 118 tests
[01:06:38] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:42] ......iii.i.....ii
[01:06:42] 
[01:06:42]  finished in 29.156
[01:06:42] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:42:55] 
[01:42:55] running 10 tests
[01:42:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:42:55] ..F.......
[01:42:55] 
[01:42:55] ---- [run-make] run-make/llvm-outputs stdout ----
[01:42:55] 
[01:42:55] error: make failed
[01:42:55] error: make failed
[01:42:55] status: exit code: 2
[01:42:55] command: "make"
[01:42:55] stdout:
[01:42:55] ------------------------------------------
[01:42:55] make[1]: Entering directory '/checkout/src/test/run-make/llvm-outputs'
[01:42:55] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-outputs/llvm-outputs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' - --out-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/llvm-outputs/llvm-outputs/random_directory_that_does_not_exist/ --emit=llvm-ir <<< 'fn main(){}'
[01:42:55] Makefile:4: recipe for target 'all' failed
[01:42:55] make[1]: Leaving director86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:42:55] 
[01:42:55] 
[01:42:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:55] Build completed unsuccessfully in 0:56:09
[01:42:55] Build completed unsuccessfully in 0:56:09
[01:42:55] make: *** [check] Error 1
[01:42:55] Makefile:58: recipe for target 'check' failed
149320 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
143616 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
138852 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
138848 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
138848 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
134668 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u
134664 ./obj/build/bootstrap/debug/incremental/bootstrap-zemjd6kcyh2u/s-f6qvawonnl-l6aneo-bouay2ep2m1v
123688 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
114332 ./obj/build/bootstrap/debug/incremental/bootstrap-3qpj2in0dvbju
114332 ./obj/build/bootstrap/debug/incremental/bootstrap-3qpj2in0dvbju
114328 ./obj/build/bootstrap/debug/incremental/bootstrap-3qpj2in0dvbju/s-f6qy1wz1c3-ym8unw-1xoxcfshv0xl8
109776 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
107892 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
99524 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
