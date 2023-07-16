plain
travis_time:end:2970619c:start=1561126628956220622,finish=1561126767323113393,duration=138366892771
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:56:24] .................................................................................................... 4900/5691
[00:56:29] .................................................................................................... 5000/5691
[00:56:32] .................................................................................................... 5100/5691
[00:56:36] .................................................................................................... 5200/5691
[00:56:40] ..............................................F..............................F.FF................... 5300/5691
[00:56:43] .........F.......................................................................................... 5400/5691
[00:56:49] .................................................................................................... 5600/5691
[00:56:51] .............................i.............................................................
[00:56:51] failures:
[00:56:51] 
---
[00:56:51] error: /checkout/src/test/ui/traits/trait-alias/trait-alias-only-maybe-bound.rs:19: expected error not found: at least one non-builtin trait is required for an object type [E0224]
[00:56:51] 
[00:56:51] error: 2 unexpected errors found, 2 expected errors not found
[00:56:51] status: exit code: 1
[00:56:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias/trait-alias-only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-only-maybe-bound" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias-only-maybe-bound/auxiliary" "-A" "unused"
[00:56:51]     Error {
[00:56:51]         line_num: 13,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "13:12: 13:18: at least one trait is required for an object type [E0224]",
[00:56:51]     Error {
[00:56:51]         line_num: 19,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "19:12: 19:18: at least one trait is required for an object type [E0224]",
[00:56:51] ]
[00:56:51] 
[00:56:51] not found errors (from test file): [
[00:56:51]     Error {
---
[00:56:51] error: /checkout/src/test/ui/traits/trait-object-macro-matcher.rs:11: expected error not found: at least one non-builtin trait is required for an object type
[00:56:51] 
[00:56:51] error: 1 unexpected errors found, 1 expected errors not found
[00:56:51] status: exit code: 1
[00:56:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-macro-matcher.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-macro-matcher/auxiliary" "-A" "unused"
[00:56:51]     Error {
[00:56:51]         line_num: 11,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "11:8: 11:21: at least one trait is required for an object type [E0224]",
[00:56:51] ]
[00:56:51] 
[00:56:51] not found errors (from test file): [
[00:56:51]     Error {
---
[00:56:51] error: /checkout/src/test/ui/traits/trait-object-vs-lifetime-2.rs:7: expected error not found: at least one non-builtin trait is required for an object type
[00:56:51] 
[00:56:51] error: 1 unexpected errors found, 1 expected errors not found
[00:56:51] status: exit code: 1
[00:56:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-vs-lifetime-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime-2/auxiliary" "-A" "unused"
[00:56:51]     Error {
[00:56:51]         line_num: 7,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "7:5: 7:18: at least one trait is required for an object type [E0224]",
[00:56:51] ]
[00:56:51] 
[00:56:51] not found errors (from test file): [
[00:56:51]     Error {
---
[00:56:51] error: /checkout/src/test/ui/traits/trait-object-vs-lifetime.rs:14: expected error not found: at least one non-builtin trait is required for an object type
[00:56:51] 
[00:56:51] error: 2 unexpected errors found, 2 expected errors not found
[00:56:51] status: exit code: 1
[00:56:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-object-vs-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-object-vs-lifetime/auxiliary" "-A" "unused"
[00:56:51]     Error {
[00:56:51]         line_num: 9,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "9:23: 9:36: at least one trait is required for an object type [E0224]",
[00:56:51]     Error {
[00:56:51]         line_num: 14,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "14:14: 14:27: at least one trait is required for an object type [E0224]",
[00:56:51] ]
[00:56:51] 
[00:56:51] not found errors (from test file): [
[00:56:51]     Error {
---
[00:56:51] error: /checkout/src/test/ui/traits/wf-trait-object-only-maybe-bound.rs:3: expected error not found: at least one non-builtin trait is required for an object type [E0224]
[00:56:51] 
[00:56:51] error: 1 unexpected errors found, 1 expected errors not found
[00:56:51] status: exit code: 1
[00:56:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/wf-trait-object-only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-trait-object-only-maybe-bound/auxiliary" "-A" "unused"
[00:56:51]     Error {
[00:56:51]         line_num: 3,
[00:56:51]         kind: Some(
[00:56:51]             Error,
[00:56:51]             Error,
[00:56:51]         ),
[00:56:51]         msg: "3:11: 3:21: at least one trait is required for an object type [E0224]",
[00:56:51] ]
[00:56:51] 
[00:56:51] not found errors (from test file): [
[00:56:51]     Error {
---
[00:56:51] 
[00:56:51] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:56:51] 
[00:56:51] 
[00:56:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:51] 
[00:56:51] 
[00:56:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:51] Build completed unsuccessfully in 0:52:51
---
travis_time:end:02fc2bf4:start=1561130191246752963,finish=1561130191251623883,duration=4870920
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:193ece20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:cra
