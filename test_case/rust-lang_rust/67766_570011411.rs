plain
2020-01-01T00:01:57.1586422Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-01T00:01:57.1601387Z ##[command]git config gc.auto 0
2020-01-01T00:01:57.1603944Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-01T00:01:57.1605616Z ##[command]git config --get-all http.proxy
2020-01-01T00:01:57.1608480Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67766/merge:refs/remotes/pull/67766/merge
---
2020-01-01T00:04:40.5336765Z Looks like docker image is the same as before, not uploading
2020-01-01T00:04:55.8327474Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-01-01T00:04:56.0523209Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-01-01T00:04:56.0565226Z == clock drift check ==
2020-01-01T00:04:56.0611047Z   local time: Wed Jan  1 00:04:56 UTC 2020
2020-01-01T00:04:56.1355041Z   network time: Wed, 01 Jan 2020 00:04:56 GMT
2020-01-01T00:04:56.1379044Z Starting sccache server...
2020-01-01T00:04:56.2100994Z configure: processing command line
2020-01-01T00:04:56.2102555Z configure: 
2020-01-01T00:04:56.2104050Z configure: rust.dist-src        := False
---
2020-01-01T00:54:01.8561281Z .................................................................................................... 1500/9469
2020-01-01T00:54:06.8152428Z .................................................................................................... 1600/9469
2020-01-01T00:54:10.9185876Z .................................................................................................... 1700/9469
2020-01-01T00:54:18.9972955Z .................................................................................................... 1800/9469
2020-01-01T00:54:26.0576182Z i................................................................................................... 1900/9469
2020-01-01T00:54:31.6948647Z ......................................................................................iiiii......... 2000/9469
2020-01-01T00:54:50.3008541Z .................................................................................................... 2200/9469
2020-01-01T00:54:52.3483787Z .................................................................................................... 2300/9469
2020-01-01T00:54:54.5350172Z .................................................................................................... 2400/9469
2020-01-01T00:54:59.7357543Z .................................................................................................... 2500/9469
---
2020-01-01T00:57:38.3604546Z .................i...............i.................................................................. 4900/9469
2020-01-01T00:57:47.3735273Z .................................................................................................... 5000/9469
2020-01-01T00:57:52.5556249Z ..............................................................i..................................... 5100/9469
2020-01-01T00:57:59.8118345Z .................................................................................................... 5200/9469
2020-01-01T00:58:06.5287801Z .............................ii.ii...........i...................................................... 5300/9469
2020-01-01T00:58:10.1481576Z ..................................................................i..............................FF. 5400/9469
2020-01-01T00:58:24.0382743Z .................................................................................................... 5600/9469
2020-01-01T00:58:30.4258550Z ..............i..................................................................................... 5700/9469
2020-01-01T00:58:35.7925512Z .................................................................................................... 5800/9469
2020-01-01T00:58:45.3245625Z .................................................................................................... 5900/9469
2020-01-01T00:58:45.3245625Z .................................................................................................... 5900/9469
2020-01-01T00:58:55.8161013Z ..ii...i..ii...........i............................................................................ 6000/9469
2020-01-01T00:59:10.1017986Z .................................................................................................... 6200/9469
2020-01-01T00:59:16.6342626Z .................................................................................................... 6300/9469
2020-01-01T00:59:16.6342626Z .................................................................................................... 6300/9469
2020-01-01T00:59:29.1227380Z .............................i..ii.................................................................. 6400/9469
2020-01-01T00:59:45.9174936Z .................................................................................................... 6600/9469
2020-01-01T00:59:47.5927483Z ....i............................................................................................... 6700/9469
2020-01-01T00:59:49.4647682Z .................................................................................................... 6800/9469
2020-01-01T00:59:51.5205016Z ....i............................................................................................... 6900/9469
---
2020-01-01T01:01:13.3978443Z .................................................................................................... 7500/9469
2020-01-01T01:01:17.2382212Z .................................................................................................... 7600/9469
2020-01-01T01:01:21.2996039Z .................................................................................................... 7700/9469
2020-01-01T01:01:30.2361074Z .................................................................................................... 7800/9469
2020-01-01T01:01:36.5325454Z ......................................iiii.......................................................... 7900/9469
2020-01-01T01:01:48.8103583Z .................................................................................................... 8100/9469
2020-01-01T01:01:56.2134876Z .................................................................................................... 8200/9469
2020-01-01T01:02:08.0306616Z .................................................................................................... 8300/9469
2020-01-01T01:02:14.4404475Z .................................................................................................... 8400/9469
---
2020-01-01T01:03:51.7082889Z 
2020-01-01T01:03:51.7083725Z ---- [ui] ui/lint/issue-67691-unused-field-in-or-pattern.rs stdout ----
2020-01-01T01:03:51.7083958Z diff of fixed:
2020-01-01T01:03:51.7084027Z 
2020-01-01T01:03:51.7084069Z 11     use MyEnum::*;
2020-01-01T01:03:51.7084195Z 13     match x {
2020-01-01T01:03:51.7084195Z 13     match x {
2020-01-01T01:03:51.7084576Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-01-01T01:03:51.7084650Z +         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-01-01T01:03:51.7085046Z 16         }
2020-01-01T01:03:51.7085123Z 17     }
2020-01-01T01:03:51.7085148Z 
2020-01-01T01:03:51.7085170Z 
2020-01-01T01:03:51.7085170Z 
2020-01-01T01:03:51.7085208Z The actual fixed differed from the expected fixed.
2020-01-01T01:03:51.7085857Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/issue-67691-unused-field-in-or-pattern.fixed
2020-01-01T01:03:51.7086215Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T01:03:51.7086661Z To only update this specific test, also pass `--test-args lint/issue-67691-unused-field-in-or-pattern.rs`
2020-01-01T01:03:51.7086896Z error: 1 errors occurred comparing output.
2020-01-01T01:03:51.7087127Z status: exit code: 1
2020-01-01T01:03:51.7087127Z status: exit code: 1
2020-01-01T01:03:51.7088259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/auxiliary" "-A" "unused"
2020-01-01T01:03:51.7088779Z ------------------------------------------
2020-01-01T01:03:51.7088813Z 
2020-01-01T01:03:51.7089047Z ------------------------------------------
2020-01-01T01:03:51.7089214Z stderr:
2020-01-01T01:03:51.7089214Z stderr:
2020-01-01T01:03:51.7089628Z ------------------------------------------
2020-01-01T01:03:51.7089691Z error: unused variable: `j`
2020-01-01T01:03:51.7090363Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:14:16
2020-01-01T01:03:51.7090562Z    |
2020-01-01T01:03:51.7090647Z LL |         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-01-01T01:03:51.7090746Z    |
2020-01-01T01:03:51.7090813Z note: lint level defined here
2020-01-01T01:03:51.7091107Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:3:9
2020-01-01T01:03:51.7091288Z    |
2020-01-01T01:03:51.7091288Z    |
2020-01-01T01:03:51.7091353Z LL | #![deny(unused)]
2020-01-01T01:03:51.7091411Z    |         ^^^^^^
2020-01-01T01:03:51.7091658Z    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
2020-01-01T01:03:51.7091829Z help: try ignoring the field
2020-01-01T01:03:51.7091894Z    |
2020-01-01T01:03:51.7091956Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-01-01T01:03:51.7092146Z 
2020-01-01T01:03:51.7092231Z error: aborting due to previous error
2020-01-01T01:03:51.7092257Z 
2020-01-01T01:03:51.7092280Z 
2020-01-01T01:03:51.7092280Z 
2020-01-01T01:03:51.7092541Z ------------------------------------------
2020-01-01T01:03:51.7092713Z 
2020-01-01T01:03:51.7092799Z 
2020-01-01T01:03:51.7093068Z ---- [ui] ui/lint/issue-67691-unused-ref-field-in-or-pattern.rs stdout ----
2020-01-01T01:03:51.7093234Z diff of fixed:
2020-01-01T01:03:51.7093311Z 
2020-01-01T01:03:51.7093350Z 11     use MyEnum::*;
2020-01-01T01:03:51.7093536Z 13     match x {
2020-01-01T01:03:51.7093536Z 13     match x {
2020-01-01T01:03:51.7093831Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-01-01T01:03:51.7094120Z +         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-01-01T01:03:51.7094274Z 16         }
2020-01-01T01:03:51.7094310Z 17     }
2020-01-01T01:03:51.7094441Z 
2020-01-01T01:03:51.7094520Z 
2020-01-01T01:03:51.7094520Z 
2020-01-01T01:03:51.7094562Z The actual fixed differed from the expected fixed.
2020-01-01T01:03:51.7094986Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-ref-field-in-or-pattern/issue-67691-unused-ref-field-in-or-pattern.fixed
2020-01-01T01:03:51.7095373Z To update references, rerun the tests and pass the `--bless` flag
2020-01-01T01:03:51.7095971Z To only update this specific test, also pass `--test-args lint/issue-67691-unused-ref-field-in-or-pattern.rs`
2020-01-01T01:03:51.7096302Z error: 1 errors occurred comparing output.
2020-01-01T01:03:51.7096366Z status: exit code: 1
2020-01-01T01:03:51.7096366Z status: exit code: 1
2020-01-01T01:03:51.7097308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-67691-unused-ref-field-in-or-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-ref-field-in-or-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-ref-field-in-or-pattern/auxiliary" "-A" "unused"
2020-01-01T01:03:51.7097789Z ------------------------------------------
2020-01-01T01:03:51.7097978Z 
2020-01-01T01:03:51.7098298Z ------------------------------------------
2020-01-01T01:03:51.7098466Z stderr:
2020-01-01T01:03:51.7098466Z stderr:
2020-01-01T01:03:51.7098723Z ------------------------------------------
2020-01-01T01:03:51.7098909Z error: unused variable: `j`
2020-01-01T01:03:51.7099291Z   --> /checkout/src/test/ui/lint/issue-67691-unused-ref-field-in-or-pattern.rs:14:20
2020-01-01T01:03:51.7099657Z    |
2020-01-01T01:03:51.7099738Z LL |         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-01-01T01:03:51.7099834Z    |
2020-01-01T01:03:51.7099872Z note: lint level defined here
2020-01-01T01:03:51.7100253Z   --> /checkout/src/test/ui/lint/issue-67691-unused-ref-field-in-or-pattern.rs:3:9
2020-01-01T01:03:51.7100434Z    |
2020-01-01T01:03:51.7100434Z    |
2020-01-01T01:03:51.7100499Z LL | #![deny(unused)]
2020-01-01T01:03:51.7100538Z    |         ^^^^^^
2020-01-01T01:03:51.7100722Z    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
2020-01-01T01:03:51.7100792Z help: try ignoring the field
2020-01-01T01:03:51.7100851Z    |
2020-01-01T01:03:51.7101005Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-01-01T01:03:51.7101235Z 
2020-01-01T01:03:51.7101291Z error: aborting due to previous error
2020-01-01T01:03:51.7101317Z 
2020-01-01T01:03:51.7101430Z 
---
2020-01-01T01:03:51.7113249Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-01T01:03:51.7113333Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-01T01:03:51.7126292Z 
2020-01-01T01:03:51.7126567Z 
2020-01-01T01:03:51.7130919Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-01T01:03:51.7131472Z 
2020-01-01T01:03:51.7131611Z 
2020-01-01T01:03:51.7142078Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-01T01:03:51.7142323Z Build completed unsuccessfully in 0:55:55
2020-01-01T01:03:51.7142323Z Build completed unsuccessfully in 0:55:55
2020-01-01T01:03:51.7196508Z == clock drift check ==
2020-01-01T01:03:51.7211129Z   local time: Wed Jan  1 01:03:51 UTC 2020
2020-01-01T01:03:51.9986981Z   network time: Wed, 01 Jan 2020 01:03:51 GMT
2020-01-01T01:03:53.0266328Z 
2020-01-01T01:03:53.0266328Z 
2020-01-01T01:03:53.0349324Z ##[error]Bash exited with code '1'.
2020-01-01T01:03:53.0383003Z ##[section]Starting: Checkout
2020-01-01T01:03:53.0384514Z ==============================================================================
2020-01-01T01:03:53.0384560Z Task         : Get sources
2020-01-01T01:03:53.0384613Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
