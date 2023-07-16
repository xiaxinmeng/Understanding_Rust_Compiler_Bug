plain
travis_time:end:1de05380:start=1545408094994088324,finish=1545408098199588097,duration=3205499773
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
[01:07:46] 
[01:07:46] running 118 tests
[01:08:11] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:08:15] ......iii.i.....ii
[01:08:15] 
[01:08:15]  finished in 29.084
[01:08:15] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:22] 
[01:13:22] running 284 tests
[01:14:27] ..........................i...............F......................................................... 100/284
[01:16:08] ....................................................................................
[01:16:08] failures:
[01:16:08] 
[01:16:08] ---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----
[01:16:08] ---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----
[01:16:08] 
[01:16:08] error: htmldocck failed!
[01:16:08] status: exit code: 1
[01:16:08] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
[01:16:08] ------------------------------------------
[01:16:08] 
[01:16:08] ------------------------------------------
[01:16:08] stderr:
[01:16:08] stderr:
[01:16:08] ------------------------------------------
[01:16:08] 40: @"-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:08] 
[01:16:08] 
[01:16:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:08] Build completed unsuccessfully in 0:19:20
[01:16:08] Build completed unsuccessfully in 0:19:20
[01:16:08] Makefile:58: recipe for target 'check' failed
[01:16:08] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07c52ee8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 17:17:58 UTC 2018
