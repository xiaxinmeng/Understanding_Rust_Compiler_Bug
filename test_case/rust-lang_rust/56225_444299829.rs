plain
travis_time:end:0a113a1c:start=1543963281710329877,finish=1543963284781592430,duration=3071262553
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 1.4MB 896kB/s 
Collecting botocore==1.12.59 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/21/cf/71dfc14692883aaf709bc1098a56770173a760a14b0b1cb74471609181be/botocore-1.12.59-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 47.6MB/s eta 0:00:01
    0% |▏                               | 20kB 34.5MB/s eta 0:00:01
    0% |▏                               | 30kB 40.8MB/s eta 0:00:01
    0% |▎                               | 40kB 21.1MB/s eta 0:00:01
---
[00:46:45] .................................................................................................... 1100/5107
[00:46:47] .................................................................................................... 1200/5107
[00:46:50] .................................................................................................... 1300/5107
[00:46:53] .................................................................................................... 1400/5107
[00:46:55] ...............................................................................................F.... 1500/5107
[00:47:01] .................................................................................................... 1700/5107
[00:47:05] .................................................................................................... 1800/5107
[00:47:08] .................................................................................................... 1900/5107
[00:47:12] ..................................i................................................................. 2000/5107
---
[00:48:36] .................................................................................................... 4400/5107
[00:48:39] .................................................................................................... 4500/5107
[00:48:42] .................................................................................................... 4600/5107
[00:48:46] .....................................................................i.............................. 4700/5107
[00:48:49] .................................................................................F.................. 4800/5107
[00:48:56] .................................................................................................... 5000/5107
[00:48:56] .................................................................................................... 5000/5107
=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_alias_enum_variants/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_alias_enum_variants/auxiliary" "-A" "unused"
[00:48:59]     Error {
[00:48:59]         line_num: 19,
[00:48:59]         kind: Some(
[00:48:59]             Error
[00:48:59]             Error
[00:48:59]         ),
[00:48:59]         msg: "19:13: 19:23: type alias enum variants are not yet allowed"
[00:48:59]     Error {
[00:48:59]         line_num: 19,
[00:48:59]         kind: Some(
[00:48:59]             Error
[00:48:59]             Error
[00:48:59]         ),
[00:48:59]         msg: "19:20: 19:23: no variant named `Bar` found for type `Foo` in the current scope [E0599]"
[00:48:59]     Error {
[00:48:59]         line_num: 20,
[00:48:59]         kind: Some(
[00:48:59]             Error
[00:48:59]             Error
[00:48:59]         ),
[00:48:59]         msg: "20:13: 20:23: type alias enum variants are not yet allowed"
[00:48:59]     Error {
[00:48:59]         line_num: 20,
[00:48:59]         kind: Some(
[00:48:59]             Error
---
[00:48:59]         line_num: 22,
[00:48:59]         kind: Some(
[00:48:59]             Error
[00:48:59]         ),
[00:48:59]         msg: "22:9: 22:23: type alias enum variants are not yet allowed"
[00:48:59]     Error {
[00:48:59]         line_num: 22,
[00:48:59]         kind: Some(
[00:48:59]             Error
[00:48:59]             Error
[00:48:59]         ),
[00:48:59]         msg: "22:16: 22:19: no variant named `Bar` found for type `Foo` in the current scope [E0599]"
[00:48:59]     Error {
[00:48:59]         line_num: 23,
[00:48:59]         kind: Some(
[00:48:59]             Error
[00:48:59]             Error
[00:48:59]         ),
[00:48:59]         msg: "23:9: 23:19: type alias enum variants are not yet allowed"
[00:48:59]     Error {
[00:48:59]         line_num: 23,
[00:48:59]         kind: Some(
[00:48:59]             Error
---
[00:48:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:59] 
[00:48:59] ---- [ui] ui/type-alias-enum-variants.rs stdout ----
[00:48:59] 
[00:48:59] error: /checkout/src/test/ui/type-alias-enum-variants.rs:9: unexpected error: '9:27: 9:29: type parameters are not allowed on this type [E0109]'
[00:48:59] error: 1 unexpected errors found, 0 expected errors not found
[00:48:59] status: exit code: 1
[00:48:59] status: exit code: 1
[00:48:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants.rs" "--target=x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:59] 
[00:48:59] 
[00:48:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:59] Build completed unsuccessfully in 0:04:02
[00:48:59] Build completed unsuccessfully in 0:04:02
[00:48:59] make: *** [check] Error 1
[00:48:59] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0aa1036f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec  4 23:30:35 UTC 2018
---
travis_time:end:02522bec:start=1543966236811470653,finish=1543966236820138319,duration=8667666
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:154e1518
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e682418
$ dmesg | grep -i kill
