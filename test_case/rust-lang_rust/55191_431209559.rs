plain
[00:48:00] .................................................................................................... 2200/4625
[00:48:04] ..............................i..................................................................... 2300/4625
[00:48:07] .................................................................................................... 2400/4625
[00:48:11] .................................................................................................... 2500/4625
[00:48:14] ............................................iiiiiiiii............................................... 2600/4625
[00:48:21] .................................................................................................... 2800/4625
[00:48:24] .................................................................................................... 2900/4625
[00:48:27] ........................................................................i........................... 3000/4625
[00:48:30] .................................................................................................... 3100/4625
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:55] 
[01:00:55] running 111 tests
[01:00:58] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii..i. 100/111
[01:00:58] ..iiii.....
[01:00:58] 
[01:00:58]  finished in 3.380
[01:00:58] travis_fold:end:test_codegen

---
[01:07:19] 
[01:07:19] running 268 tests
[01:08:21] .......................i............................................................................ 100/268
[01:09:14] ..............................i..................................................................... 200/268
[01:09:46] .....................................F..............................
[01:09:46] 
[01:09:46] ---- [rustdoc] rustdoc/structfields.rs stdout ----
[01:09:46] 
[01:09:46] 
[01:09:46] error: htmldocck failed!
[01:09:46] status: exit code: 1
[01:09:46] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/structfields" "/checkout/src/test/rustdoc/structfields.rs"
[01:09:46] ------------------------------------------
[01:09:46] 
[01:09:46] ------------------------------------------
[01:09:46] stderr:
[01:09:46] stderr:
[01:09:46] ------------------------------------------
[01:09:46] Traceback (most recent call last):
[01:09:46]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[01:09:46]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:09:46]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[01:09:46]     check_command(c, cache)
[01:09:46]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[01:09:46]     tree = cache.get_tree(c.args[0])
[01:09:46]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[01:09:46]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[01:09:46] RuntimeError: Cannot parse an HTML file 'structfields/enum.Qux.html': pop from empty stack
[01:09:46] ------------------------------------------
[01:09:46] 
[01:09:46] thread '[rustdoc] rustdoc/structfields.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:09:46] 
---
[01:09:46] 
[01:09:46] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:09:46] 
[01:09:46] 
[01:09:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:46] 
[01:09:46] 
[01:09:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:46] Build completed unsuccessfully in 0:26:19
[01:09:46] Build completed unsuccessfully in 0:26:19
[01:09:46] make: *** [check] Error 1
[01:09:46] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13788b46
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
