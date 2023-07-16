plain
2020-01-27T18:23:36.9978987Z ========================== Starting Command Output ===========================
2020-01-27T18:23:36.9994889Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c146be9e-590f-43bc-b47f-0de7f9295722.sh
2020-01-27T18:23:37.0192984Z 
2020-01-27T18:23:37.0242471Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T18:23:37.0246777Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68577/merge to s
2020-01-27T18:23:37.0248013Z Task         : Get sources
2020-01-27T18:23:37.0248039Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T18:23:37.0248064Z Version      : 1.0.0
2020-01-27T18:23:37.0248121Z Author       : Microsoft
---
2020-01-27T18:23:37.7767970Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T18:23:37.7950615Z ##[command]git config gc.auto 0
2020-01-27T18:23:37.7957576Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T18:23:37.8016941Z ##[command]git config --get-all http.proxy
2020-01-27T18:23:37.8155803Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68577/merge:refs/remotes/pull/68577/merge
---
2020-01-27T19:10:05.6422628Z .................................................................................................... 1700/9556
2020-01-27T19:10:10.1821211Z .................................................................................................... 1800/9556
2020-01-27T19:10:20.7723135Z ........................i........................................................................... 1900/9556
2020-01-27T19:10:26.7273554Z .................................................................................................... 2000/9556
2020-01-27T19:10:38.9070239Z ..............iiiii................................................................................. 2100/9556
2020-01-27T19:10:47.1601032Z .................................................................................................... 2300/9556
2020-01-27T19:10:49.1411939Z .................................................................................................... 2400/9556
2020-01-27T19:10:53.5893908Z .................................................................................................... 2500/9556
2020-01-27T19:11:10.7271272Z .................................................................................................... 2600/9556
---
2020-01-27T19:13:20.7635055Z .................................................................................................... 4800/9556
2020-01-27T19:13:25.0870217Z ..........................................................i...............i......................... 4900/9556
2020-01-27T19:13:31.8567383Z .................................................................................................... 5000/9556
2020-01-27T19:13:38.5296318Z .................................................................................................... 5100/9556
2020-01-27T19:13:42.6196557Z .i.................................................................................................. 5200/9556
2020-01-27T19:13:52.2218506Z ..........................................................................ii.ii........i...i........ 5300/9556
2020-01-27T19:13:59.7883654Z ............i....................................................................................... 5500/9556
2020-01-27T19:14:08.4364591Z .................................................................................................... 5600/9556
2020-01-27T19:14:14.1355703Z .............................................................i...................................... 5700/9556
2020-01-27T19:14:20.4520067Z .................................................................................................... 5800/9556
2020-01-27T19:14:20.4520067Z .................................................................................................... 5800/9556
2020-01-27T19:14:27.5435932Z .................................................................................................... 5900/9556
2020-01-27T19:14:35.0439818Z ....................................................ii...i..ii...........i.......................... 6000/9556
2020-01-27T19:14:52.5241429Z .................................................................................................... 6200/9556
2020-01-27T19:14:55.9257699Z .................................................................................................... 6300/9556
2020-01-27T19:14:55.9257699Z .................................................................................................... 6300/9556
2020-01-27T19:14:59.7812232Z ................................................................................i..ii............... 6400/9556
2020-01-27T19:15:22.3266543Z .................................................................................................... 6600/9556
2020-01-27T19:15:26.8949995Z ........................................................i........................................... 6700/9556
2020-01-27T19:15:28.7606503Z .................................................................................................... 6800/9556
2020-01-27T19:15:30.6867841Z .......................................................i............................................ 6900/9556
---
2020-01-27T19:16:54.4786680Z .................................................................................................... 7600/9556
2020-01-27T19:16:59.2055263Z .................................................................................................... 7700/9556
2020-01-27T19:17:04.7810900Z .................................................................................................... 7800/9556
2020-01-27T19:17:13.7028620Z .................................................................................................... 7900/9556
2020-01-27T19:17:18.8555392Z ...........iiiiiii.................................................................................. 8000/9556
2020-01-27T19:17:30.9183786Z .................................................................................................... 8200/9556
2020-01-27T19:17:40.1394193Z .................................................................................................... 8300/9556
2020-01-27T19:17:50.9866720Z .................................................................................................... 8400/9556
2020-01-27T19:17:56.5375288Z .................................................................................................... 8500/9556
---
2020-01-27T19:19:55.2526064Z  finished in 6.478
2020-01-27T19:19:55.2705464Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-27T19:19:55.4157218Z 
2020-01-27T19:19:55.4158680Z running 169 tests
2020-01-27T19:19:57.9546881Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-27T19:19:59.8378868Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-27T19:19:59.8379965Z 
2020-01-27T19:19:59.8384768Z  finished in 4.568
2020-01-27T19:19:59.8555010Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-27T19:19:59.9898983Z 
---
2020-01-27T19:20:01.6603589Z  finished in 1.804
2020-01-27T19:20:01.6783139Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-27T19:20:01.8146978Z 
2020-01-27T19:20:01.8148764Z running 9 tests
2020-01-27T19:20:01.8150602Z iiiiiiiii
2020-01-27T19:20:01.8152084Z 
2020-01-27T19:20:01.8152641Z  finished in 0.136
2020-01-27T19:20:01.8318176Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-27T19:20:01.9831017Z 
2020-01-27T19:20:01.9831017Z 
2020-01-27T19:20:01.9832425Z running 114 tests
2020-01-27T19:20:16.8481984Z ...................................................F.F.............................................. 100/114
2020-01-27T19:20:18.6867639Z ..............
2020-01-27T19:20:18.6867754Z failures:
2020-01-27T19:20:18.6870498Z 
2020-01-27T19:20:18.6874848Z ---- [incremental] incremental/hashes/while_let_loops.rs stdout ----
2020-01-27T19:20:18.6874892Z 
2020-01-27T19:20:18.6875159Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-01-27T19:20:18.6875213Z status: exit code: 1
2020-01-27T19:20:18.6876207Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary"
2020-01-27T19:20:18.6876579Z ------------------------------------------
2020-01-27T19:20:18.6876612Z 
2020-01-27T19:20:18.6876818Z ------------------------------------------
2020-01-27T19:20:18.6876872Z stderr:
2020-01-27T19:20:18.6876872Z stderr:
2020-01-27T19:20:18.6877089Z ------------------------------------------
2020-01-27T19:20:18.6877148Z error: `typeck_tables_of(change_break_label)` should be clean but is not
2020-01-27T19:20:18.6877450Z    |
2020-01-27T19:20:18.6877450Z    |
2020-01-27T19:20:18.6877493Z LL | / pub fn change_break_label() {
2020-01-27T19:20:18.6877540Z LL | |     let mut _x = 0;
2020-01-27T19:20:18.6877770Z LL | |     'outer: while let Some(0u32) = None {
2020-01-27T19:20:18.6877992Z LL | |         'inner: while let Some(0u32) = None {
2020-01-27T19:20:18.6878094Z LL | |     }
2020-01-27T19:20:18.6878136Z LL | | }
2020-01-27T19:20:18.6878176Z    | |_^
2020-01-27T19:20:18.6878202Z 
2020-01-27T19:20:18.6878202Z 
2020-01-27T19:20:18.6878263Z error: `typeck_tables_of(change_continue_label)` should be clean but is not
2020-01-27T19:20:18.6878545Z    |
2020-01-27T19:20:18.6878545Z    |
2020-01-27T19:20:18.6878594Z LL | / pub fn change_continue_label() {
2020-01-27T19:20:18.6878816Z LL | |     let mut _x = 0;
2020-01-27T19:20:18.6879043Z LL | |     'outer: while let Some(0u32) = None {
2020-01-27T19:20:18.6879258Z LL | |         'inner: while let Some(0u32) = None {
2020-01-27T19:20:18.6879359Z LL | |     }
2020-01-27T19:20:18.6879400Z LL | | }
2020-01-27T19:20:18.6879452Z    | |_^
2020-01-27T19:20:18.6879478Z 
---
2020-01-27T19:20:18.6879818Z 
2020-01-27T19:20:18.6879843Z 
2020-01-27T19:20:18.6880064Z ---- [incremental] incremental/hashes/while_loops.rs stdout ----
2020-01-27T19:20:18.6880111Z 
2020-01-27T19:20:18.6880345Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-01-27T19:20:18.6880393Z status: exit code: 1
2020-01-27T19:20:18.6881445Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/while_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_loops/auxiliary"
2020-01-27T19:20:18.6881773Z ------------------------------------------
2020-01-27T19:20:18.6881805Z 
2020-01-27T19:20:18.6882008Z ------------------------------------------
2020-01-27T19:20:18.6882074Z stderr:
2020-01-27T19:20:18.6882074Z stderr:
2020-01-27T19:20:18.6882283Z ------------------------------------------
2020-01-27T19:20:18.6882333Z error: `typeck_tables_of(change_break_label)` should be clean but is not
2020-01-27T19:20:18.6882629Z    |
2020-01-27T19:20:18.6882629Z    |
2020-01-27T19:20:18.6882671Z LL | / pub fn change_break_label() {
2020-01-27T19:20:18.6882730Z LL | |     let mut _x = 0;
2020-01-27T19:20:18.6883126Z LL | |         'inner: while true {
2020-01-27T19:20:18.6883184Z ...  |
2020-01-27T19:20:18.6883225Z LL | |     }
2020-01-27T19:20:18.6883265Z LL | | }
2020-01-27T19:20:18.6883265Z LL | | }
2020-01-27T19:20:18.6883304Z    | |_^
2020-01-27T19:20:18.6883344Z 
2020-01-27T19:20:18.6883391Z error: `typeck_tables_of(change_continue_label)` should be clean but is not
2020-01-27T19:20:18.6883688Z    |
2020-01-27T19:20:18.6883688Z    |
2020-01-27T19:20:18.6883730Z LL | / pub fn change_continue_label() {
2020-01-27T19:20:18.6883781Z LL | |     let mut _x = 0;
2020-01-27T19:20:18.6884188Z LL | |         'inner: while true {
2020-01-27T19:20:18.6884231Z ...  |
2020-01-27T19:20:18.6884270Z LL | |     }
2020-01-27T19:20:18.6884324Z LL | | }
---
2020-01-27T19:20:18.6889406Z test result: FAILED. 112 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-27T19:20:18.6889440Z 
2020-01-27T19:20:18.6889465Z 
2020-01-27T19:20:18.6889489Z 
2020-01-27T19:20:18.6890997Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-27T19:20:18.6891291Z 
2020-01-27T19:20:18.6891332Z 
2020-01-27T19:20:18.6893540Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-27T19:20:18.6893612Z Build completed unsuccessfully in 0:51:43
2020-01-27T19:20:18.6893612Z Build completed unsuccessfully in 0:51:43
2020-01-27T19:20:18.6945252Z == clock drift check ==
2020-01-27T19:20:18.6962139Z   local time: Mon Jan 27 19:20:18 UTC 2020
2020-01-27T19:20:18.8663129Z   network time: Mon, 27 Jan 2020 19:20:18 GMT
2020-01-27T19:20:18.8664532Z == end clock drift check ==
2020-01-27T19:20:21.7163030Z 
2020-01-27T19:20:21.7248620Z ##[error]Bash exited with code '1'.
2020-01-27T19:20:21.7260090Z ##[section]Finishing: Run build
2020-01-27T19:20:21.7280822Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68577/merge to s
2020-01-27T19:20:21.7282793Z Task         : Get sources
2020-01-27T19:20:21.7282858Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T19:20:21.7282908Z Version      : 1.0.0
2020-01-27T19:20:21.7282953Z Author       : Microsoft
2020-01-27T19:20:21.7282953Z Author       : Microsoft
2020-01-27T19:20:21.7283017Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T19:20:21.7283086Z ==============================================================================
2020-01-27T19:20:22.0974848Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T19:20:22.1019888Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68577/merge to s
2020-01-27T19:20:22.1123365Z Cleaning up task key
2020-01-27T19:20:22.1124109Z Start cleaning up orphan processes.
2020-01-27T19:20:22.1239204Z Terminate orphan process: pid (3663) (python)
2020-01-27T19:20:22.1519675Z ##[section]Finishing: Finalize Job
