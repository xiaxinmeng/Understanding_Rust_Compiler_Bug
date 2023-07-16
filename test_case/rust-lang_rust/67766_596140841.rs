plain
2020-03-07T20:38:36.7925294Z ========================== Starting Command Output ===========================
2020-03-07T20:38:36.7928442Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4cb0032d-ddb5-48dc-a118-eb823bbdf2d2.sh
2020-03-07T20:38:36.7928744Z 
2020-03-07T20:38:36.7932985Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-07T20:38:36.7955867Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-03-07T20:38:36.7962544Z Task         : Get sources
2020-03-07T20:38:36.7962873Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T20:38:36.7963217Z Version      : 1.0.0
2020-03-07T20:38:36.7963431Z Author       : Microsoft
---
2020-03-07T20:38:37.8135004Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-07T20:38:37.8141143Z ##[command]git config gc.auto 0
2020-03-07T20:38:37.8145315Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-07T20:38:37.8149548Z ##[command]git config --get-all http.proxy
2020-03-07T20:38:37.8156637Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67766/merge:refs/remotes/pull/67766/merge
---
2020-03-07T21:37:07.1712116Z .................................................................................................... 1700/9744
2020-03-07T21:37:11.4050863Z .................................................................................................... 1800/9744
2020-03-07T21:37:22.2774140Z ...........................................................i........................................ 1900/9744
2020-03-07T21:37:29.3952434Z .................................................................................................... 2000/9744
2020-03-07T21:37:43.0181866Z .................................................iiiii.............................................. 2100/9744
2020-03-07T21:37:53.0393999Z .................................................................................................... 2300/9744
2020-03-07T21:37:55.2353811Z .................................................................................................... 2400/9744
2020-03-07T21:37:58.5066425Z .................................................................................................... 2500/9744
2020-03-07T21:38:19.6646798Z .................................................................................................... 2600/9744
---
2020-03-07T21:40:55.1602954Z ..........i...............i......................................................................... 5000/9744
2020-03-07T21:41:04.3454681Z .................................................................................................... 5100/9744
2020-03-07T21:41:09.0366783Z .....................................................i.............................................. 5200/9744
2020-03-07T21:41:17.1260548Z .................................................................................................... 5300/9744
2020-03-07T21:41:24.4937556Z ..................................ii.ii........i...i................................................ 5400/9744
2020-03-07T21:41:32.6693561Z .....F.............................................................................................. 5600/9744
2020-03-07T21:41:41.7380590Z .................................................................................................... 5700/9744
2020-03-07T21:41:48.5244548Z ..........................i......................................................................... 5800/9744
2020-03-07T21:41:53.8875928Z .................................................................................................... 5900/9744
2020-03-07T21:41:53.8875928Z .................................................................................................... 5900/9744
2020-03-07T21:42:04.5182036Z .................................................................................................... 6000/9744
2020-03-07T21:42:14.0083753Z ..................ii...i..ii...........i............................................................ 6100/9744
2020-03-07T21:42:29.4686124Z .................................................................................................... 6300/9744
2020-03-07T21:42:36.0458946Z .................................................................................................... 6400/9744
2020-03-07T21:42:36.0458946Z .................................................................................................... 6400/9744
2020-03-07T21:42:51.4413067Z .................................................i..ii.............................................. 6500/9744
2020-03-07T21:43:14.5370499Z .................................................................................................... 6700/9744
2020-03-07T21:43:16.5923913Z ...........................................i........................................................ 6800/9744
2020-03-07T21:43:18.4497014Z .................................................................................................... 6900/9744
2020-03-07T21:43:20.5139864Z ..........................................................................i......................... 7000/9744
---
2020-03-07T21:44:54.9799803Z .................................................................................................... 7700/9744
2020-03-07T21:44:59.1888630Z .................................................................................................... 7800/9744
2020-03-07T21:45:04.0488522Z .................................................................................................... 7900/9744
2020-03-07T21:45:11.5369140Z .........................i.......................................................................... 8000/9744
2020-03-07T21:45:19.2935297Z ..........................................................................iiiiiiiii.i............... 8100/9744
2020-03-07T21:45:34.3242588Z .................i......i........................................................................... 8300/9744
2020-03-07T21:45:39.1845478Z .................................................................................................... 8400/9744
2020-03-07T21:45:51.2884240Z .................................................................................................... 8500/9744
2020-03-07T21:46:00.4584589Z .................................................................................................... 8600/9744
---
2020-03-07T21:47:51.1462734Z 
2020-03-07T21:47:51.1462891Z 17     use MyEnum::*;
2020-03-07T21:47:51.1463068Z 18 
2020-03-07T21:47:51.1463210Z 19     match x {
2020-03-07T21:47:51.1463699Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-07T21:47:51.1464075Z +         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-03-07T21:47:51.1464545Z 22         }
2020-03-07T21:47:51.1464704Z 23     }
2020-03-07T21:47:51.1464810Z 
2020-03-07T21:47:51.1464964Z 27     use MyEnum::*;
2020-03-07T21:47:51.1464964Z 27     use MyEnum::*;
2020-03-07T21:47:51.1465244Z 28 
2020-03-07T21:47:51.1465407Z 29     match x {
2020-03-07T21:47:51.1465890Z -         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-07T21:47:51.1466273Z +         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-03-07T21:47:51.1466783Z 32         }
2020-03-07T21:47:51.1466929Z 33     }
2020-03-07T21:47:51.1467038Z 
2020-03-07T21:47:51.1467214Z 37     use MyEnum::*;
2020-03-07T21:47:51.1467214Z 37     use MyEnum::*;
2020-03-07T21:47:51.1467376Z 38 
2020-03-07T21:47:51.1467521Z 39     match x {
2020-03-07T21:47:51.1468029Z -         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1469206Z +         Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1469726Z 42         }
2020-03-07T21:47:51.1469862Z 43 
2020-03-07T21:47:51.1469963Z 
2020-03-07T21:47:51.1470121Z 49     use MyEnum::*;
2020-03-07T21:47:51.1470121Z 49     use MyEnum::*;
2020-03-07T21:47:51.1470308Z 50 
2020-03-07T21:47:51.1470453Z 51     match x {
2020-03-07T21:47:51.1471011Z -         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1471433Z +         Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1471925Z 54         }
2020-03-07T21:47:51.1472079Z 55 
2020-03-07T21:47:51.1472180Z 
2020-03-07T21:47:51.1472294Z 59 
2020-03-07T21:47:51.1472294Z 59 
2020-03-07T21:47:51.1472488Z 60 pub fn mixed_no_ref(x: MixedEnum) {
2020-03-07T21:47:51.1472725Z 61     match x {
2020-03-07T21:47:51.1473166Z -         MixedEnum::A { i: _ } | MixedEnum::B(_) => {
2020-03-07T21:47:51.1473552Z +         MixedEnum::A { i } | MixedEnum::B(i) => { //~ ERROR unused variable
2020-03-07T21:47:51.1473890Z 63             println!("match");
2020-03-07T21:47:51.1474393Z 65     }
2020-03-07T21:47:51.1474515Z 
2020-03-07T21:47:51.1475118Z 67 
2020-03-07T21:47:51.1475118Z 67 
2020-03-07T21:47:51.1475844Z 68 pub fn mixed_with_ref(x: MixedEnum) {
2020-03-07T21:47:51.1476072Z 69     match x {
2020-03-07T21:47:51.1476634Z -         MixedEnum::A { i: _ } | MixedEnum::B(_) => {
2020-03-07T21:47:51.1477041Z +         MixedEnum::A { ref i } | MixedEnum::B(ref i) => { //~ ERROR unused variable
2020-03-07T21:47:51.1477379Z 71             println!("match");
2020-03-07T21:47:51.1477730Z 73     }
2020-03-07T21:47:51.1477841Z 
2020-03-07T21:47:51.1477936Z 
2020-03-07T21:47:51.1478150Z The actual fixed differed from the expected fixed.
2020-03-07T21:47:51.1478150Z The actual fixed differed from the expected fixed.
2020-03-07T21:47:51.1478914Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/issue-67691-unused-field-in-or-pattern.fixed
2020-03-07T21:47:51.1479611Z To update references, rerun the tests and pass the `--bless` flag
2020-03-07T21:47:51.1480269Z To only update this specific test, also pass `--test-args lint/issue-67691-unused-field-in-or-pattern.rs`
2020-03-07T21:47:51.1480855Z error: 1 errors occurred comparing output.
2020-03-07T21:47:51.1481112Z status: exit code: 1
2020-03-07T21:47:51.1481112Z status: exit code: 1
2020-03-07T21:47:51.1483246Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-67691-unused-field-in-or-pattern/auxiliary"
2020-03-07T21:47:51.1484993Z ------------------------------------------
2020-03-07T21:47:51.1485173Z 
2020-03-07T21:47:51.1485552Z ------------------------------------------
2020-03-07T21:47:51.1485762Z stderr:
2020-03-07T21:47:51.1485762Z stderr:
2020-03-07T21:47:51.1486132Z ------------------------------------------
2020-03-07T21:47:51.1486404Z error: unused variable: `j`
2020-03-07T21:47:51.1486959Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:20:16
2020-03-07T21:47:51.1487251Z    |
2020-03-07T21:47:51.1487517Z LL |         A { i, j } | B { i, j } => { //~ ERROR unused variable
2020-03-07T21:47:51.1487999Z    |
2020-03-07T21:47:51.1488203Z note: the lint level is defined here
2020-03-07T21:47:51.1488760Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:4:9
2020-03-07T21:47:51.1489047Z    |
2020-03-07T21:47:51.1489047Z    |
2020-03-07T21:47:51.1489226Z LL | #![deny(unused)]
2020-03-07T21:47:51.1489416Z    |         ^^^^^^
2020-03-07T21:47:51.1489698Z    = note: `#[deny(unused_variables)]` implied by `#[deny(unused)]`
2020-03-07T21:47:51.1490167Z help: try ignoring the field
2020-03-07T21:47:51.1490341Z    |
2020-03-07T21:47:51.1490607Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-07T21:47:51.1491117Z 
2020-03-07T21:47:51.1491285Z error: unused variable: `j`
2020-03-07T21:47:51.1491837Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:30:16
2020-03-07T21:47:51.1492134Z    |
2020-03-07T21:47:51.1492134Z    |
2020-03-07T21:47:51.1492385Z LL |         A { i, ref j } | B { i, ref j } => { //~ ERROR unused variable
2020-03-07T21:47:51.1492903Z    |
2020-03-07T21:47:51.1493071Z help: try ignoring the field
2020-03-07T21:47:51.1493239Z    |
2020-03-07T21:47:51.1493239Z    |
2020-03-07T21:47:51.1493518Z LL |         A { i, j: _ } | B { i, j: _ } => { //~ ERROR unused variable
2020-03-07T21:47:51.1494120Z 
2020-03-07T21:47:51.1494301Z error: unused variable: `j`
2020-03-07T21:47:51.1494896Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:40:21
2020-03-07T21:47:51.1495260Z    |
2020-03-07T21:47:51.1495260Z    |
2020-03-07T21:47:51.1495507Z LL |         Some(A { i, j } | B { i, j }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1496015Z    |
2020-03-07T21:47:51.1496183Z help: try ignoring the field
2020-03-07T21:47:51.1496369Z    |
2020-03-07T21:47:51.1496369Z    |
2020-03-07T21:47:51.1496647Z LL |         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1497170Z 
2020-03-07T21:47:51.1497355Z error: unused variable: `j`
2020-03-07T21:47:51.1497918Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:52:21
2020-03-07T21:47:51.1498198Z    |
2020-03-07T21:47:51.1498198Z    |
2020-03-07T21:47:51.1498482Z LL |         Some(A { i, ref j } | B { i, ref j }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1499015Z    |
2020-03-07T21:47:51.1499205Z help: try ignoring the field
2020-03-07T21:47:51.1499372Z    |
2020-03-07T21:47:51.1499372Z    |
2020-03-07T21:47:51.1499648Z LL |         Some(A { i, j: _ } | B { i, j: _ }) => { //~ ERROR unused variable
2020-03-07T21:47:51.1500186Z 
2020-03-07T21:47:51.1500353Z error: unused variable: `i`
2020-03-07T21:47:51.1500893Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:62:24
2020-03-07T21:47:51.1501189Z    |
2020-03-07T21:47:51.1501189Z    |
2020-03-07T21:47:51.1501468Z LL |         MixedEnum::A { i } | MixedEnum::B(i) => { //~ ERROR unused variable
2020-03-07T21:47:51.1502038Z    |
2020-03-07T21:47:51.1502207Z help: try ignoring the field
2020-03-07T21:47:51.1502378Z    |
2020-03-07T21:47:51.1502378Z    |
2020-03-07T21:47:51.1502684Z LL |         MixedEnum::A { i: _ } | MixedEnum::B(_) => { //~ ERROR unused variable
2020-03-07T21:47:51.1503240Z 
2020-03-07T21:47:51.1503424Z error: unused variable: `i`
2020-03-07T21:47:51.1503966Z   --> /checkout/src/test/ui/lint/issue-67691-unused-field-in-or-pattern.rs:70:24
2020-03-07T21:47:51.1504244Z    |
2020-03-07T21:47:51.1504244Z    |
2020-03-07T21:47:51.1504540Z LL |         MixedEnum::A { ref i } | MixedEnum::B(ref i) => { //~ ERROR unused variable
2020-03-07T21:47:51.1505157Z    |
2020-03-07T21:47:51.1505326Z help: try ignoring the field
2020-03-07T21:47:51.1505512Z    |
2020-03-07T21:47:51.1505512Z    |
2020-03-07T21:47:51.1505798Z LL |         MixedEnum::A { i: _ } | MixedEnum::B(_) => { //~ ERROR unused variable
2020-03-07T21:47:51.1506364Z 
2020-03-07T21:47:51.1506546Z error: aborting due to 6 previous errors
2020-03-07T21:47:51.1506818Z 
2020-03-07T21:47:51.1506912Z 
---
2020-03-07T21:47:51.1522667Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-07T21:47:51.1523076Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-07T21:47:51.1540075Z 
2020-03-07T21:47:51.1540286Z 
2020-03-07T21:47:51.1545342Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-07T21:47:51.1552546Z 
2020-03-07T21:47:51.1552679Z 
2020-03-07T21:47:51.1553409Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-07T21:47:51.1553815Z Build completed unsuccessfully in 1:03:32
2020-03-07T21:47:51.1553815Z Build completed unsuccessfully in 1:03:32
2020-03-07T21:47:51.1599327Z == clock drift check ==
2020-03-07T21:47:51.1616278Z   local time: Sat Mar  7 21:47:51 UTC 2020
2020-03-07T21:47:51.7040443Z   network time: Sat, 07 Mar 2020 21:47:51 GMT
2020-03-07T21:47:51.7046573Z == end clock drift check ==
2020-03-07T21:47:52.1466872Z 
2020-03-07T21:47:52.1565756Z ##[error]Bash exited with code '1'.
2020-03-07T21:47:52.1581910Z ##[section]Finishing: Run build
2020-03-07T21:47:52.1641804Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-03-07T21:47:52.1648147Z Task         : Get sources
2020-03-07T21:47:52.1648733Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T21:47:52.1649097Z Version      : 1.0.0
2020-03-07T21:47:52.1649365Z Author       : Microsoft
2020-03-07T21:47:52.1649365Z Author       : Microsoft
2020-03-07T21:47:52.1649801Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-07T21:47:52.1650268Z ==============================================================================
2020-03-07T21:47:52.5187661Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-07T21:47:52.5235901Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67766/merge to s
2020-03-07T21:47:52.5334708Z Cleaning up task key
2020-03-07T21:47:52.5337035Z Start cleaning up orphan processes.
2020-03-07T21:47:52.5534173Z Terminate orphan process: pid (3967) (python)
2020-03-07T21:47:52.5801922Z ##[section]Finishing: Finalize Job
