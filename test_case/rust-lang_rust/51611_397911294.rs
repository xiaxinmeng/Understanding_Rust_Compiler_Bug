plain
[01:06:45] failures:
[01:06:45] 
[01:06:45] ---- [rustdoc] rustdoc/pub-use-extern-macros.rs stdout ----
[01:06:45] 
[01:06:45] error: htmldocck failed!
[01:06:45] status: exit code: 1
[01:06:45] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-use-extern-macros" "/checkout/src/test/rustdoc/pub-use-extern-macros.rs"
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] stderr:
[01:06:45] stderr:
[01:06:45] ------------------------------------------
[01:06:45] 26: @has check failed
[01:06:45]  File does not exist 'pub_use_extern_macros/macro.quux.html'
[01:06:45]  // @has pub_use_extern_macros/macro.quux.html
[01:06:45] Encountered 1 errors
[01:06:45] 
[01:06:45] ------------------------------------------
[01:06:45] 
[01:06:45] 
[01:06:45] thread '[rustdoc] rustdoc/pub-use-extern-macros.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:06:45] 
[01:06:45] 
[01:06:45] failures:
[01:06:45]     [rustdoc] rustdoc/pub-use-extern-macrosd to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:45] Build completed unsuccessfully in 0:23:39
[01:06:45] Makefile:58: recipe for target 'check' failed
[01:06:45] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0108d6d4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08c441f4:start=1529275148382355537,finish=1529275148389598456,duration=7242919
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01c7be57
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09a5ea1c
$ dmesg | grep -i kill
