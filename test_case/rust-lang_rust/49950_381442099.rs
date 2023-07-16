plain
[00:01:04] configure: rust.quiet-tests     := True
---
[00:43:17] .................................................................................i..................
[00:43:22] ........................i..F........................................................................
[00:43:26] ....................................................................................................
[00:43:30] ....................................................................................................
[00:43:33] ....................................................................................................
[00:43:37] ....................................................................................................
[00:43:43] ...............F...........................................................F........................
[00:43:49] .....F..............................................................................................
[00:43:55] ....................................................................................................
[00:44:02] .......................i...........................................................................i
[00:44:07] ............................................................................F.......................
[00:44:13] .............ii.....................................................................................
[00:44:21] ....................................................................................................
est/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait-supertrait-indirect.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait-supertrait-indirect.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:25] unexpected errors (from JSON output): [
[00:44:25]     Error {
[00:44:25]         line_num: 17,
[00:44:25]         kind: Some(
[00:44:25]             Error
[00:44:25]         ),
[00:44:25]         msg: "17:1: 17:11: cycle detected when computing the supertraits of `B` [E0391]"
---
[00:44:25]         msg: "cyclic dependency detected"
[00:44:25]     },
[00:44:25]     Error {
[00:44:25]         line_num: 20,
[00:44:25]         kind: None,
[00:44:25]         msg: "cyclic reference"
---
[00:44:25] error: /checkout/src/test/ui/impl-trait/auto-trait-leak.rs:50: unexpected er,
[00:44:25]         msg: "cyclic dependency detected"
[00:44:25]     },
[00:44:25]     Error {
[00:44:25]         line_num: 42,
[00:44:25]         kind: None,
[00:44:25]         msg: "cyclic reference"
---
[00:44:25] error: /checkout/src/test/ui/issue-12511.rs:11: unexpected error: '11:1: 11:14: cycle detected when computing the supertraits of `t1` [E0391]'
[00:44:25]
[00:44:25] error: /checkout/src/test/ui/issue-12511.rs:14: expected error not found: cyclic dependency detected
[00:44:25]
[00:44:25] error: /checkout/src/test/ui/issue-12511.rs:14: expected message not found: cyclic reference
[00:44:25]
[00:44:25] error: 1 unexpected errors found, 2 expected errors not found
[00:44:25] status: exit code: 101
[00:44:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-12511.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-12511.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:25] unexpected errors (from JSON outpun: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:44:25] Build completed unsuccessfully in 0:01:43
[00:44:25] make: *** [check] Error 1
[00:44:25] Makefile:58: recipe for target 'check' failed
ei3hhuv/s-f05gsdk5tn-yjlezp-3pbjjk2ek2e4p
109932 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
109928 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
106036 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102508 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
102504 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f05hu981eo-mpsbit-bcs1sclq83s
---
70496 ./obj/build/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:2cad7824:start=1523830250697241214,finish=1523830250703896676,duration=6655462
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:022cb37c
$ dmesg | grep -i kill
[   10.992445] init: failsafe main process (1097) killed by TERM signal
