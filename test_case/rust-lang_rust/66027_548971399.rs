plain
2019-11-01T21:12:23.4194629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-01T21:12:23.4398193Z ##[command]git config gc.auto 0
2019-11-01T21:12:23.4493756Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-01T21:12:23.4555187Z ##[command]git config --get-all http.proxy
2019-11-01T21:12:23.4719861Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66027/merge:refs/remotes/pull/66027/merge
---
2019-11-01T22:14:22.6462190Z .................................................................................................... 1600/9262
2019-11-01T22:14:28.7978980Z .................................................................................................... 1700/9262
2019-11-01T22:14:41.8728172Z ..........................................................i...............i......................... 1800/9262
2019-11-01T22:14:50.2999963Z .................................................................................................... 1900/9262
2019-11-01T22:15:05.6981143Z ................................................iiiii............................................... 2000/9262
2019-11-01T22:15:17.0598915Z .................................................................................................... 2200/9262
2019-11-01T22:15:19.7877940Z .................................................................................................... 2300/9262
2019-11-01T22:15:23.7967601Z .................................................................................................... 2400/9262
2019-11-01T22:15:48.4011781Z .................................................................................................... 2500/9262
---
2019-11-01T22:18:49.6358748Z ................................................i...............i................................... 4800/9262
2019-11-01T22:18:59.0426106Z .................................................................................................... 4900/9262
2019-11-01T22:19:07.8060554Z .................................................................................................... 5000/9262
2019-11-01T22:19:14.7033554Z .................................................................................................... 5100/9262
2019-11-01T22:19:25.2563083Z .................................................ii.ii...........i.................................. 5200/9262
2019-11-01T22:19:35.9549226Z .............................................................................F...................... 5400/9262
2019-11-01T22:19:46.6311541Z .................................................................................................... 5500/9262
2019-11-01T22:19:54.5667595Z ......................i............................................................................. 5600/9262
2019-11-01T22:20:01.5044275Z .................................................................................................... 5700/9262
2019-11-01T22:20:01.5044275Z .................................................................................................... 5700/9262
2019-11-01T22:20:13.7933529Z .................................................................................................... 5800/9262
2019-11-01T22:20:26.7767528Z .......ii...i..ii...........i....................................................................... 5900/9262
2019-11-01T22:20:49.7882531Z .................................................................................................... 6100/9262
2019-11-01T22:20:58.3585548Z .................................................................................................... 6200/9262
2019-11-01T22:20:58.3585548Z .................................................................................................... 6200/9262
2019-11-01T22:21:13.3568030Z ..........................i..ii..................................................................... 6300/9262
2019-11-01T22:21:34.3821235Z ............................................................................................i....... 6500/9262
2019-11-01T22:21:36.8100580Z .................................................................................................... 6600/9262
2019-11-01T22:21:39.2360088Z ...................................................................i................................ 6700/9262
2019-11-01T22:21:42.3372266Z .................................................................................................... 6800/9262
---
2019-11-01T22:26:45.9894297Z -    |
2019-11-01T22:26:45.9894813Z - note: lint level defined here
2019-11-01T22:26:45.9895487Z -   --> $DIR/edition-extern-crate-allowed.rs:5:9
2019-11-01T22:26:45.9895970Z -    |
2019-11-01T22:26:45.9896455Z - LL | #![warn(rust_2018_idioms)]
2019-11-01T22:26:45.9896928Z -    |         ^^^^^^^^^^^^^^^^
2019-11-01T22:26:45.9897503Z -    = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`
2019-11-01T22:26:45.9898471Z - 
2019-11-01T22:26:45.9898718Z 
2019-11-01T22:26:45.9898919Z 
2019-11-01T22:26:45.9898919Z 
2019-11-01T22:26:45.9899509Z error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-extern-crate-allowed/edition-extern-crate-allowed.stderr`: No such file or directory (os error 2)
2019-11-01T22:26:45.9900241Z [ERROR compiletest::runtest] fatal error, panic: "failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/editions/edition-extern-crate-allowed/edition-extern-crate-allowed.stderr`: No such file or directory (os error 2)"
2019-11-01T22:26:45.9900854Z thread '[ui] ui/editions/edition-extern-crate-allowed.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2262:9
2019-11-01T22:26:45.9901377Z 
2019-11-01T22:26:45.9902453Z ---- [ui] ui/lint/lint-unused-extern-crate.rs stdout ----
2019-11-01T22:26:45.9903619Z 
2019-11-01T22:26:45.9903868Z error: ui test compiled successfully!
2019-11-01T22:26:45.9903868Z error: ui test compiled successfully!
2019-11-01T22:26:45.9904054Z status: exit code: 0
2019-11-01T22:26:45.9905241Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unused-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unused-extern-crate/auxiliary" "-A" "unused"
2019-11-01T22:26:45.9905855Z ------------------------------------------
2019-11-01T22:26:45.9906007Z 
2019-11-01T22:26:45.9906371Z ------------------------------------------
2019-11-01T22:26:45.9906537Z stderr:
---
2019-11-01T22:26:45.9914352Z 15    |
2019-11-01T22:26:45.9914475Z 
2019-11-01T22:26:45.9914596Z 
2019-11-01T22:26:45.9914740Z The actual stderr differed from the expected stderr.
2019-11-01T22:26:45.9915367Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-macro-use-attr/no-macro-use-attr.stderr
2019-11-01T22:26:45.9915775Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T22:26:45.9916272Z To only update this specific test, also pass `--test-args proc-macro/no-macro-use-attr.rs`
2019-11-01T22:26:45.9916589Z error: 1 errors occurred comparing output.
2019-11-01T22:26:45.9916753Z status: exit code: 1
2019-11-01T22:26:45.9916753Z status: exit code: 1
2019-11-01T22:26:45.9917815Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/no-macro-use-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-macro-use-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/no-macro-use-attr/auxiliary" "-A" "unused"
2019-11-01T22:26:45.9918427Z ------------------------------------------
2019-11-01T22:26:45.9918603Z 
2019-11-01T22:26:45.9918957Z ------------------------------------------
2019-11-01T22:26:45.9919139Z stderr:
---
2019-11-01T22:26:45.9929141Z 6    |
2019-11-01T22:26:45.9929413Z 7 note: lint level defined here
2019-11-01T22:26:45.9929875Z 8   --> $DIR/removing-extern-crate.rs:6:9
2019-11-01T22:26:45.9930069Z 
2019-11-01T22:26:45.9930215Z 10 LL | #![warn(rust_2018_idioms)]
2019-11-01T22:26:45.9930357Z 11    |         ^^^^^^^^^^^^^^^^
2019-11-01T22:26:45.9930527Z 12    = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`
2019-11-01T22:26:45.9931252Z - warning: unused extern crate
2019-11-01T22:26:45.9931946Z -   --> $DIR/removing-extern-crate.rs:10:1
2019-11-01T22:26:45.9932347Z -    |
2019-11-01T22:26:45.9932883Z - LL | extern crate core;
---
2019-11-01T22:26:45.9937234Z The actual stderr differed from the expected stderr.
2019-11-01T22:26:45.9937678Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/removing-extern-crate.stderr
2019-11-01T22:26:45.9937882Z diff of fixed:
2019-11-01T22:26:45.9938003Z 
2019-11-01T22:26:45.9938147Z 6 #![warn(rust_2018_idioms)]
2019-11-01T22:26:45.9938303Z 7 #![allow(unused_imports)]
2019-11-01T22:26:45.9938578Z + extern crate removing_extern_crate as foo;
2019-11-01T22:26:45.9947112Z 9 
2019-11-01T22:26:45.9949317Z 10 
2019-11-01T22:26:45.9949980Z - 
---
2019-11-01T22:26:45.9952740Z 16 
2019-11-01T22:26:45.9952769Z 
2019-11-01T22:26:45.9952814Z 
2019-11-01T22:26:45.9953058Z The actual fixed differed from the expected fixed.
2019-11-01T22:26:45.9953490Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/removing-extern-crate.fixed
2019-11-01T22:26:45.9953906Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T22:26:45.9961105Z To only update this specific test, also pass `--test-args removing-extern-crate.rs`
2019-11-01T22:26:45.9961209Z error: 2 errors occurred comparing output.
2019-11-01T22:26:45.9961256Z status: exit code: 0
2019-11-01T22:26:45.9961256Z status: exit code: 0
2019-11-01T22:26:45.9962500Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/removing-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/auxiliary" "-A" "unused"
2019-11-01T22:26:45.9962863Z ------------------------------------------
2019-11-01T22:26:45.9962896Z 
2019-11-01T22:26:45.9963117Z ------------------------------------------
2019-11-01T22:26:45.9963190Z stderr:
---
2019-11-01T22:26:45.9963918Z    |
2019-11-01T22:26:45.9963960Z note: lint level defined here
2019-11-01T22:26:45.9964202Z   --> /checkout/src/test/ui/removing-extern-crate.rs:6:9
2019-11-01T22:26:45.9964267Z    |
2019-11-01T22:26:45.9964309Z LL | #![warn(rust_2018_idioms)]
2019-11-01T22:26:45.9964353Z    |         ^^^^^^^^^^^^^^^^
2019-11-01T22:26:45.9964422Z    = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`
2019-11-01T22:26:45.9964500Z warning: unused extern crate
2019-11-01T22:26:45.9964760Z   --> /checkout/src/test/ui/removing-extern-crate.rs:14:5
2019-11-01T22:26:45.9964982Z    |
2019-11-01T22:26:45.9965037Z LL |     extern crate core;
---
2019-11-01T22:26:45.9967302Z 8   --> $DIR/extern-crate-idiomatic-in-2018.rs:9:9
2019-11-01T22:26:45.9967334Z 
2019-11-01T22:26:45.9967360Z 
2019-11-01T22:26:45.9967405Z The actual stderr differed from the expected stderr.
2019-11-01T22:26:45.9967767Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/extern-crate-idiomatic-in-2018.stderr
2019-11-01T22:26:45.9967856Z 
2019-11-01T22:26:45.9967856Z 
2019-11-01T22:26:45.9967899Z 9 #![deny(rust_2018_idioms)]
2019-11-01T22:26:45.9967959Z 10 #![allow(dead_code)]
2019-11-01T22:26:45.9968183Z - 
2019-11-01T22:26:45.9968243Z + use edition_lint_paths;
2019-11-01T22:26:45.9968243Z + use edition_lint_paths;
2019-11-01T22:26:45.9968289Z 13 //~^ ERROR unused extern crate
2019-11-01T22:26:45.9968330Z 14 
2019-11-01T22:26:45.9968697Z 15 // Shouldn't suggest changing to `use`, as `bar`
2019-11-01T22:26:45.9968761Z 
2019-11-01T22:26:45.9968809Z The actual fixed differed from the expected fixed.
2019-11-01T22:26:45.9968809Z The actual fixed differed from the expected fixed.
2019-11-01T22:26:45.9969204Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/extern-crate-idiomatic-in-2018.fixed
2019-11-01T22:26:45.9969482Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T22:26:45.9969803Z To only update this specific test, also pass `--test-args rust-2018/extern-crate-idiomatic-in-2018.rs`
2019-11-01T22:26:45.9969904Z error: 2 errors occurred comparing output.
2019-11-01T22:26:45.9969953Z status: exit code: 1
2019-11-01T22:26:45.9969953Z status: exit code: 1
2019-11-01T22:26:45.9970817Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/extern-crate-idiomatic-in-2018/auxiliary" "-A" "unused"
2019-11-01T22:26:45.9971203Z ------------------------------------------
2019-11-01T22:26:45.9971238Z 
2019-11-01T22:26:45.9972123Z ------------------------------------------
2019-11-01T22:26:45.9972176Z stderr:
---
2019-11-01T22:26:45.9973084Z    |
2019-11-01T22:26:45.9973125Z note: lint level defined here
2019-11-01T22:26:45.9973413Z   --> /checkout/src/test/ui/rust-2018/extern-crate-idiomatic-in-2018.rs:9:9
2019-11-01T22:26:45.9973479Z    |
2019-11-01T22:26:45.9973521Z LL | #![deny(rust_2018_idioms)]
2019-11-01T22:26:45.9973641Z    |         ^^^^^^^^^^^^^^^^
2019-11-01T22:26:45.9973691Z    = note: `#[deny(unused_extern_crates)]` implied by `#[deny(rust_2018_idioms)]`
2019-11-01T22:26:45.9973782Z error: aborting due to previous error
2019-11-01T22:26:45.9973811Z 
2019-11-01T22:26:45.9973837Z 
2019-11-01T22:26:45.9974093Z ------------------------------------------
2019-11-01T22:26:45.9974093Z ------------------------------------------
2019-11-01T22:26:45.9974124Z 
2019-11-01T22:26:45.9974149Z 
2019-11-01T22:26:45.9974405Z ---- [ui] ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs stdout ----
2019-11-01T22:26:45.9974455Z 
2019-11-01T22:26:45.9974506Z error: ui test compiled successfully!
2019-11-01T22:26:45.9974551Z status: exit code: 0
2019-11-01T22:26:45.9975455Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--cfg" "blandiloquence" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/auxiliary" "-A" "unused"
2019-11-01T22:26:45.9975808Z ------------------------------------------
2019-11-01T22:26:45.9975841Z 
2019-11-01T22:26:45.9976059Z ------------------------------------------
2019-11-01T22:26:45.9976128Z stderr:
---
2019-11-01T22:26:45.9978516Z 
2019-11-01T22:26:45.9978788Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-01T22:26:45.9978823Z 
2019-11-01T22:26:45.9978864Z 
2019-11-01T22:26:45.9980397Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-01T22:26:45.9980711Z 
2019-11-01T22:26:45.9980740Z 
2019-11-01T22:26:45.9980786Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-01T22:26:45.9980835Z Build completed unsuccessfully in 1:07:35
2019-11-01T22:26:45.9980835Z Build completed unsuccessfully in 1:07:35
2019-11-01T22:26:46.0014516Z == clock drift check ==
2019-11-01T22:26:46.0027698Z   local time: Fri Nov  1 22:26:46 UTC 2019
2019-11-01T22:26:46.2852056Z   network time: Fri, 01 Nov 2019 22:26:46 GMT
2019-11-01T22:26:46.2853469Z == end clock drift check ==
2019-11-01T22:26:47.6502267Z 
2019-11-01T22:26:47.6613483Z ##[error]Bash exited with code '1'.
2019-11-01T22:26:47.6674633Z ##[section]Starting: Checkout
2019-11-01T22:26:47.6676507Z ==============================================================================
2019-11-01T22:26:47.6676567Z Task         : Get sources
2019-11-01T22:26:47.6676631Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
