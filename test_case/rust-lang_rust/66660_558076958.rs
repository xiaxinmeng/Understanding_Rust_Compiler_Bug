plain
2019-11-25T08:43:14.0542533Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T08:43:14.0729986Z ##[command]git config gc.auto 0
2019-11-25T08:43:14.0805184Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T08:43:15.0656540Z ##[command]git config --get-all http.proxy
2019-11-25T08:43:15.0660222Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66660/merge:refs/remotes/pull/66660/merge
---
2019-11-25T09:40:48.5713589Z .................................................................................................... 1600/9289
2019-11-25T09:40:53.1437670Z .................................................................................................... 1700/9289
2019-11-25T09:41:05.8236444Z .............................i...................................................................... 1800/9289
2019-11-25T09:41:12.3473980Z .................................................................................................... 1900/9289
2019-11-25T09:41:26.0805763Z ..............iiiii................................................................................. 2000/9289
2019-11-25T09:41:35.4149998Z .................................................................................................... 2200/9289
2019-11-25T09:41:37.7373774Z .................................................................................................... 2300/9289
2019-11-25T09:41:42.6271010Z .................................................................................................... 2400/9289
2019-11-25T09:42:03.3008931Z .................................................................................................... 2500/9289
---
2019-11-25T09:44:37.2553417Z ..............i...............i..................................................................... 4800/9289
2019-11-25T09:44:47.2637876Z .................................................................................................... 4900/9289
2019-11-25T09:44:52.7136249Z .................................................................................................... 5000/9289
2019-11-25T09:45:01.9296263Z .................................................................................................... 5100/9289
2019-11-25T09:45:07.7358250Z ...................ii.ii...........i................................................................ 5200/9289
2019-11-25T09:45:16.2259673Z .................................................................................................... 5400/9289
2019-11-25T09:45:26.7156953Z .................................................................................................... 5500/9289
2019-11-25T09:45:34.0203573Z ..i................................................................................................. 5600/9289
2019-11-25T09:45:39.2731770Z .................................................................................................... 5700/9289
2019-11-25T09:45:39.2731770Z .................................................................................................... 5700/9289
2019-11-25T09:45:49.1801548Z ........................................................................................ii...i..ii.. 5800/9289
2019-11-25T09:46:11.0803051Z .................................................................................................... 6000/9289
2019-11-25T09:46:18.8484454Z .................................................................................................... 6100/9289
2019-11-25T09:46:22.6361483Z .................................................................................................... 6200/9289
2019-11-25T09:46:22.6361483Z .................................................................................................... 6200/9289
2019-11-25T09:46:35.8296984Z ...........i..ii.................................................................................... 6300/9289
2019-11-25T09:46:54.1650379Z ...............................................................................i.................... 6500/9289
2019-11-25T09:46:56.3560945Z .................................................................................................... 6600/9289
2019-11-25T09:46:58.5302074Z ......................................................................i............................. 6700/9289
2019-11-25T09:47:01.5010435Z .................................................................................................... 6800/9289
---
2019-11-25T09:51:25.2883964Z .............................................................................................i...... 9200/9289
2019-11-25T09:51:37.0161564Z .........................................................................................
2019-11-25T09:51:37.0162488Z failures:
2019-11-25T09:51:37.0198106Z 
2019-11-25T09:51:37.0198832Z ---- [ui] ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs stdout ----
2019-11-25T09:51:37.0199055Z normalized stderr:
2019-11-25T09:51:37.0199241Z error: structure field `lowerCamelCaseName` should have a snake case name
2019-11-25T09:51:37.0199676Z   --> $DIR/issue66362-no-snake-case-warning-for-field-puns.rs:7:9
2019-11-25T09:51:37.0199869Z    |
2019-11-25T09:51:37.0200017Z LL |         lowerCamelCaseName: bool,
2019-11-25T09:51:37.0200199Z    |         ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `lower_camel_case_name`
2019-11-25T09:51:37.0200523Z note: lint level defined here
2019-11-25T09:51:37.0200523Z note: lint level defined here
2019-11-25T09:51:37.0201026Z   --> $DIR/issue66362-no-snake-case-warning-for-field-puns.rs:1:9
2019-11-25T09:51:37.0201290Z LL | #![deny(non_snake_case)]
2019-11-25T09:51:37.0201398Z    |         ^^^^^^^^^^^^^^
2019-11-25T09:51:37.0201490Z 
2019-11-25T09:51:37.0201490Z 
2019-11-25T09:51:37.0201616Z error: variable `lowerCamelCaseName` should have a snake case name
2019-11-25T09:51:37.0201907Z   --> $DIR/issue66362-no-snake-case-warning-for-field-puns.rs:19:20
2019-11-25T09:51:37.0202040Z    |
2019-11-25T09:51:37.0202177Z LL |         Foo::Bad { lowerCamelCaseName } => {}
2019-11-25T09:51:37.0202297Z    |                    ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `lower_camel_case_name`
2019-11-25T09:51:37.0202402Z 
2019-11-25T09:51:37.0202529Z error: variable `lowerCamelCaseBinding` should have a snake case name
2019-11-25T09:51:37.0202819Z   --> $DIR/issue66362-no-snake-case-warning-for-field-puns.rs:20:38
2019-11-25T09:51:37.0202953Z    |
2019-11-25T09:51:37.0203086Z LL |         Foo::Good { snake_case_name: lowerCamelCaseBinding } => { }
2019-11-25T09:51:37.0203205Z    |                                      ^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `lower_camel_case_binding`
2019-11-25T09:51:37.0203324Z 
2019-11-25T09:51:37.0203441Z error: variable `anotherLowerCamelCaseBinding` should have a snake case name
2019-11-25T09:51:37.0203733Z   --> $DIR/issue66362-no-snake-case-warning-for-field-puns.rs:24:41
2019-11-25T09:51:37.0203886Z    |
2019-11-25T09:51:37.0204148Z LL |     if let Foo::Good { snake_case_name: anotherLowerCamelCaseBinding } = b { }
2019-11-25T09:51:37.0204334Z    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `another_lower_camel_case_binding`
2019-11-25T09:51:37.0204476Z 
2019-11-25T09:51:37.0204595Z error: variable `yetAnotherLowerCamelCaseBinding` should have a snake case name
2019-11-25T09:51:37.0204948Z   --> $DIR/issue66362-no-snake-case-warning-for-field-puns.rs:27:43
2019-11-25T09:51:37.0205114Z    |
2019-11-25T09:51:37.0205231Z LL |     if let Foo::Bad { lowerCamelCaseName: yetAnotherLowerCamelCaseBinding } = b { }
2019-11-25T09:51:37.0205376Z    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `yet_another_lower_camel_case_binding`
2019-11-25T09:51:37.0205598Z error: aborting due to 5 previous errors
2019-11-25T09:51:37.0205696Z 
2019-11-25T09:51:37.0205808Z 
2019-11-25T09:51:37.0205904Z 
2019-11-25T09:51:37.0205904Z 
2019-11-25T09:51:37.0206017Z 
2019-11-25T09:51:37.0206140Z The actual stderr differed from the expected stderr.
2019-11-25T09:51:37.0206573Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns/issue66362-no-snake-case-warning-for-field-puns.stderr
2019-11-25T09:51:37.0207052Z To update references, rerun the tests and pass the `--bless` flag
2019-11-25T09:51:37.0207945Z To only update this specific test, also pass `--test-args lint/issue66362-no-snake-case-warning-for-field-puns.rs`
2019-11-25T09:51:37.0208316Z error: 1 errors occurred comparing output.
2019-11-25T09:51:37.0208467Z status: exit code: 1
2019-11-25T09:51:37.0208467Z status: exit code: 1
2019-11-25T09:51:37.0209477Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns/auxiliary" "-A" "unused"
2019-11-25T09:51:37.0210113Z ------------------------------------------
2019-11-25T09:51:37.0210272Z 
2019-11-25T09:51:37.0210788Z ------------------------------------------
2019-11-25T09:51:37.0211119Z stderr:
2019-11-25T09:51:37.0211119Z stderr:
2019-11-25T09:51:37.0211379Z ------------------------------------------
2019-11-25T09:51:37.0211518Z error: structure field `lowerCamelCaseName` should have a snake case name
2019-11-25T09:51:37.0211834Z   --> /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:7:9
2019-11-25T09:51:37.0211971Z    |
2019-11-25T09:51:37.0212080Z LL |         lowerCamelCaseName: bool,
2019-11-25T09:51:37.0212220Z    |         ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `lower_camel_case_name`
2019-11-25T09:51:37.0212467Z note: lint level defined here
2019-11-25T09:51:37.0212467Z note: lint level defined here
2019-11-25T09:51:37.0213046Z   --> /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:1:9
2019-11-25T09:51:37.0213316Z LL | #![deny(non_snake_case)]
2019-11-25T09:51:37.0213426Z    |         ^^^^^^^^^^^^^^
2019-11-25T09:51:37.0213516Z 
2019-11-25T09:51:37.0213516Z 
2019-11-25T09:51:37.0213643Z error: variable `lowerCamelCaseName` should have a snake case name
2019-11-25T09:51:37.0213950Z   --> /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:19:20
2019-11-25T09:51:37.0214085Z    |
2019-11-25T09:51:37.0214213Z LL |         Foo::Bad { lowerCamelCaseName } => {}
2019-11-25T09:51:37.0214396Z    |                    ^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `lower_camel_case_name`
2019-11-25T09:51:37.0214501Z 
2019-11-25T09:51:37.0214726Z error: variable `lowerCamelCaseBinding` should have a snake case name
2019-11-25T09:51:37.0215115Z   --> /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:20:38
2019-11-25T09:51:37.0215268Z    |
2019-11-25T09:51:37.0215401Z LL |         Foo::Good { snake_case_name: lowerCamelCaseBinding } => { }
2019-11-25T09:51:37.0215519Z    |                                      ^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `lower_camel_case_binding`
2019-11-25T09:51:37.0215618Z 
2019-11-25T09:51:37.0215751Z error: variable `anotherLowerCamelCaseBinding` should have a snake case name
2019-11-25T09:51:37.0216060Z   --> /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:24:41
2019-11-25T09:51:37.0216212Z    |
2019-11-25T09:51:37.0216326Z LL |     if let Foo::Good { snake_case_name: anotherLowerCamelCaseBinding } = b { }
2019-11-25T09:51:37.0216465Z    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `another_lower_camel_case_binding`
2019-11-25T09:51:37.0216591Z 
2019-11-25T09:51:37.0216708Z error: variable `yetAnotherLowerCamelCaseBinding` should have a snake case name
2019-11-25T09:51:37.0217613Z   --> /checkout/src/test/ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs:27:43
2019-11-25T09:51:37.0217862Z    |
2019-11-25T09:51:37.0218016Z LL |     if let Foo::Bad { lowerCamelCaseName: yetAnotherLowerCamelCaseBinding } = b { }
2019-11-25T09:51:37.0218200Z    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `yet_another_lower_camel_case_binding`
2019-11-25T09:51:37.0218486Z error: aborting due to 5 previous errors
2019-11-25T09:51:37.0218610Z 
2019-11-25T09:51:37.0218749Z 
2019-11-25T09:51:37.0219109Z ------------------------------------------
2019-11-25T09:51:37.0219109Z ------------------------------------------
2019-11-25T09:51:37.0219268Z 
2019-11-25T09:51:37.0219390Z 
2019-11-25T09:51:37.0219537Z 
2019-11-25T09:51:37.0219678Z failures:
2019-11-25T09:51:37.0220065Z     [ui] ui/lint/issue66362-no-snake-case-warning-for-field-puns.rs
2019-11-25T09:51:37.0221649Z test result: FAILED. 9245 passed; 1 failed; 43 ignored; 0 measured; 0 filtered out
2019-11-25T09:51:37.0221691Z 
2019-11-25T09:51:37.0225566Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-25T09:51:37.0225641Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-25T09:51:37.0225641Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-25T09:51:37.0244799Z 
2019-11-25T09:51:37.0244859Z 
2019-11-25T09:51:37.0246170Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-25T09:51:37.0246363Z 
2019-11-25T09:51:37.0246402Z 
2019-11-25T09:51:37.0248875Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-25T09:51:37.0248967Z Build completed unsuccessfully in 1:02:27
2019-11-25T09:51:37.0248967Z Build completed unsuccessfully in 1:02:27
2019-11-25T09:51:37.0299350Z == clock drift check ==
2019-11-25T09:51:37.6126112Z   local time: Mon Nov 25 09:51:37 UTC 2019
2019-11-25T09:51:37.6136058Z   network time: Mon, 25 Nov 2019 09:51:37 GMT
2019-11-25T09:51:37.6136354Z == end clock drift check ==
2019-11-25T09:51:38.1294587Z 
2019-11-25T09:51:38.1424654Z ##[error]Bash exited with code '1'.
2019-11-25T09:51:38.1462177Z ##[section]Starting: Checkout
2019-11-25T09:51:38.1463934Z ==============================================================================
2019-11-25T09:51:38.1463995Z Task         : Get sources
2019-11-25T09:51:38.1464044Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
