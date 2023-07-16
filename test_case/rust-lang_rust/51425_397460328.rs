plain
[01:04:20] failures:
[01:04:20] 
[01:04:20] ---- [rustdoc] rustdoc/pub-use-extern-macros.rs stdout ----
[01:04:20] 
[01:04:20] error: htmldocck failed!
[01:04:20] status: exit code: 1
[01:04:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/pub-use-extern-macros" "/checkout/src/test/rustdoc/pub-use-extern-macros.rs"
[01:04:20] ------------------------------------------
[01:04:20] 
[01:04:20] ------------------------------------------
[01:04:20] stderr:
[01:04:20] stderr:
[01:04:20] ------------------------------------------
[01:04:20] 18: @!has check failed
[01:04:20]  `XPATH PATTERN` did not match
[01:04:20]  // @!has pub_use_extern_macros/index.html '//code' 'pub use macros::bar;'
[01:04:20] 22: @!has check failed
[01:04:20]  `XPATH PATTERN` did not match
[01:04:20]  // @!has pub_use_extern_macros/index.html '//code' 'pub use macros::baz;'
[01:04:20] Encountered 2 errors
[01:04:20] 
[01:04:20] ------------------------------------------
[01:04:20] 
[01:04:20] 
[01:04:20] thread '[rustdoc] rustdoc/pub-use-extern-macros.rs' panicked at 'explicit panic', too" "--android-cross-path" "" "--color" "always"
[01:04:20] 
[01:04:20] 
[01:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:20] Build completed unsuccessfully in 0:23:13
[01:04:20] Build completed unsuccessfully in 0:23:13
[01:04:20] Makefile:58: recipe for target 'check' failed
[01:04:20] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f9317b2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0c8c789d:start=1529016211635082647,finish=1529016211642102837,duration=7020190
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0db6743a
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08466530
$ dmesg | grep -i kill
