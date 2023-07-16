plain
travis_time:end:03773c7c:start=1546954811781750889,finish=1546954815037847677,duration=3256096788
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:02:33] .................................................................................................... 3100/5299
[01:02:37] ..................................................................................i................. 3200/5299
[01:02:40] .................................................................................................... 3300/5299
[01:02:43] .............................................ii...i..ii............................................. 3400/5299
[01:02:47] ..............................................................................................F..... 3500/5299
[01:02:54] .....................................ii............................................................. 3700/5299
[01:02:56] .......................................................i............................................ 3800/5299
[01:02:58] .................................................................................................... 3900/5299
[01:03:00] ...........i........................................................................................ 4000/5299
---
[01:03:40] .................................................................................................... 4900/5299
[01:03:44] .................................................................................................... 5000/5299
[01:03:47] .................................................................................................... 5100/5299
[01:03:50] .................................................................................................... 5200/5299
nux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:52] 
[01:03:52] 
[01:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:52] Build completed unsuccessfully in 0:04:06
[01:03:52] Build completed unsuccessfully in 0:04:06
[01:03:52] make: *** [check] Error 1
[01:03:52] Makefile:48: recipe for target 'check' failed
