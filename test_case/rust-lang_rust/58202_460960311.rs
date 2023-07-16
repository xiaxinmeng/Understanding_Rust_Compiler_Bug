plain
travis_time:end:0b0455fe:start=1549441708890022009,finish=1549441785011391743,duration=76121369734
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:17] 
[01:07:17] running 119 tests
[01:07:43] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:07:48] i......iii.i.....ii
[01:07:48] 
[01:07:48]  finished in 31.503
[01:07:48] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:27] 
[01:11:27] running 293 tests
[01:12:33] .........................F.iFF...............................................................F...... 100/293
[01:14:20] .............................................................................................
[01:14:20] failures:
[01:14:20] 
[01:14:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:14:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:14:20] ---- [rustdoc] rustdoc/deprecated-future.rs stdout ----
[01:14:20] 
[01:14:20] error: htmldocck failed!
[01:14:20] status: exit code: 1
[01:14:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-future" "/checkout/src/test/rustdoc/deprecated-future.rs"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] 3: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]  // @has deprecated_future/struct.S.html '//*[@class="stab deprecated"]' 'Deprecating in 99.99.99: effectively never'
[01:14:20] Encountered 1 errors
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] 
[01:14:20] thread '[rustdoc] rustdoc/deprecated-future.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:14:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:20] 
[01:14:20] ---- [rustdoc] rustdoc/deprecated-impls.rs stdout ----
[01:14:20] 
[01:14:20] error: htmldocck failed!
[01:14:20] status: exit code: 1
[01:14:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-impls" "/checkout/src/test/rustdoc/deprecated-impls.rs"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] 7: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.1: fn_with_doc'
[01:14:20] 16: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.2: fn_without_doc'
[01:14:20] 54: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.3: fn_empty_with_doc'
[01:14:20] 62: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.4: fn_empty_without_doc'
[01:14:20] 65: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.5: fn_def_with_doc'
[01:14:20] 73: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.6: fn_def_without_doc'
[01:14:20] 76: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.7: fn_def_def_with_doc'
[01:14:20] 80: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.8: fn_def_def_without_doc'
[01:14:20] 87: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.3: fn_empty_with_doc'
[01:14:20] 92: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.4: fn_empty_without_doc'
[01:14:20] 100: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.5: fn_def_with_doc'
[01:14:20] 105: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.6: fn_def_without_doc'
[01:14:20] 113: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.7: fn_def_def_with_doc'
[01:14:20] 117: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.0.8: fn_def_def_without_doc'
[01:14:20] Encountered 14 errors
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] 
[01:14:20] thread '[rustdoc] rustdoc/deprecated-impls.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:14:20] 
[01:14:20] ---- [rustdoc] rustdoc/deprecated.rs stdout ----
[01:14:20] 
[01:14:20] error: htmldocck failed!
[01:14:20] status: exit code: 1
[01:14:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated" "/checkout/src/test/rustdoc/deprecated.rs"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] 5: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]  // @has deprecated/struct.S.html '//*[@class="stab deprecated"]' 'Deprecated since 1.0.0: text'
[01:14:20] 15: @matches check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]  // @matches deprecated/struct.U.html '//*[@class="stab deprecated"]' 'Deprecated since 1.0.0$'
[01:14:20] Encountered 2 errors
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] 
[01:14:20] thread '[rustdoc] rustdoc/deprecated.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:14:20] 
[01:14:20] ---- [rustdoc] rustdoc/inline_local/macro_by_example.rs stdout ----
[01:14:20] 
[01:14:20] error: htmldocck failed!
[01:14:20] status: exit code: 1
[01:14:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/macro_by_example" "/checkout/src/test/rustdoc/inline_local/macro_by_example.rs"
[01:14:20] ------------------------------------------
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] stderr:
[01:14:20] stderr:
[01:14:20] ------------------------------------------
[01:14:20] 13: @has check failed
[01:14:20]  `XPATH PATTERN` did not match
[01:14:20]      // @has - '//*[@class="stab deprecated"]' 'Deprecated since 1.2.3: text'
[01:14:20] Encountered 1 errors
[01:14:20] 
[01:14:20] ------------------------------------------
[01:14:20] 
---
[01:14:20] test result: FAILED. 287 passed; 4 failed; 2 ignored; 0 measured; 0 filtered out
[01:14:20] 
[01:14:20] 
[01:14:20] 
[01:14:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:20] 
[01:14:20] 
[01:14:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:20] Build completed unsuccessfully in 0:18:45
[01:14:20] Build completed unsuccessfully in 0:18:45
[01:14:20] Makefile:48: recipe for target 'check' failed
[01:14:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04078140
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb  6 09:44:15 UTC 2019
