plain
2020-03-22T19:38:30.8102304Z ========================== Starting Command Output ===========================
2020-03-22T19:38:30.8107206Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8479b2be-3bc5-4ce5-af78-9c42d459a85d.sh
2020-03-22T19:38:30.8107667Z 
2020-03-22T19:38:30.8112273Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T19:38:30.8130640Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-22T19:38:30.8134122Z Task         : Get sources
2020-03-22T19:38:30.8134412Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T19:38:30.8134709Z Version      : 1.0.0
2020-03-22T19:38:30.8134902Z Author       : Microsoft
---
2020-03-22T19:38:31.8014114Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T19:38:31.8019361Z ##[command]git config gc.auto 0
2020-03-22T19:38:31.8023161Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T19:38:31.8026589Z ##[command]git config --get-all http.proxy
2020-03-22T19:38:31.8032708Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69740/merge:refs/remotes/pull/69740/merge
---
2020-03-22T20:35:04.3417223Z .................................................................................................... 1700/9821
2020-03-22T20:35:08.5111049Z .................................................................................................... 1800/9821
2020-03-22T20:35:18.2986828Z ................................................................................i................... 1900/9821
2020-03-22T20:35:25.4459879Z .................................................................................................... 2000/9821
2020-03-22T20:35:32.0511024Z ......................................................................iiiii......................... 2100/9821
2020-03-22T20:35:51.6051525Z .................................................................................................... 2300/9821
2020-03-22T20:35:53.7296552Z .................................................................................................... 2400/9821
2020-03-22T20:35:56.2391453Z .................................................................................................... 2500/9821
2020-03-22T20:36:09.7226668Z .................................................................................................... 2600/9821
---
2020-03-22T20:38:48.4312004Z ............................................i...............i....................................... 5000/9821
2020-03-22T20:38:57.3646027Z .................................................................................................... 5100/9821
2020-03-22T20:39:04.0569724Z ........................................................................................i........... 5200/9821
2020-03-22T20:39:09.7553573Z .................................................................................................... 5300/9821
2020-03-22T20:39:19.8071757Z .......................................................................ii.ii........i...i........... 5400/9821
2020-03-22T20:39:27.6529916Z ...........i........................................................................................ 5600/9821
2020-03-22T20:39:36.2075901Z ................i................................................................................... 5700/9821
2020-03-22T20:39:42.5022484Z .................................ii...................................i............................. 5800/9821
2020-03-22T20:39:49.1596422Z .................................................................................................... 5900/9821
2020-03-22T20:39:49.1596422Z .................................................................................................... 5900/9821
2020-03-22T20:39:55.2939723Z .................................................................................................... 6000/9821
2020-03-22T20:40:04.1158295Z ................................................................ii...i..ii...........i.............. 6100/9821
2020-03-22T20:40:23.0203428Z .................................................................................................... 6300/9821
2020-03-22T20:40:29.8818036Z .................................................................................................... 6400/9821
2020-03-22T20:40:29.8818036Z .................................................................................................... 6400/9821
2020-03-22T20:40:36.6995293Z ..............................................................................................i..ii. 6500/9821
2020-03-22T20:41:04.6294189Z .................................................................................................... 6700/9821
2020-03-22T20:41:14.8537838Z .............................................................................................i...... 6800/9821
2020-03-22T20:41:16.8449060Z .................................................................................................... 6900/9821
2020-03-22T20:41:18.9490170Z .................................................................................................... 7000/9821
---
2020-03-22T20:42:58.5741361Z .................................................................................................... 7800/9821
2020-03-22T20:43:03.0257372Z .................................................................................................... 7900/9821
2020-03-22T20:43:08.8015614Z ...................................................................................i................ 8000/9821
2020-03-22T20:43:17.0465974Z .................................................................................................... 8100/9821
2020-03-22T20:43:23.7283743Z ................................iiiiiiiiii.i........................................................ 8200/9821
2020-03-22T20:43:37.2802045Z .................................................................................................... 8400/9821
2020-03-22T20:43:42.2697708Z .................................................................................................... 8500/9821
2020-03-22T20:43:56.5897815Z .................................................................................................... 8600/9821
2020-03-22T20:44:03.9908586Z .................................................................................................... 8700/9821
---
2020-03-22T20:46:18.1559694Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-22T20:46:18.1731439Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T20:46:18.3758300Z 
2020-03-22T20:46:18.3761493Z running 183 tests
2020-03-22T20:46:21.0333473Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-22T20:46:23.6035440Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-22T20:46:23.6038294Z 
2020-03-22T20:46:23.6043378Z  finished in 5.431
2020-03-22T20:46:23.6049985Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-22T20:46:23.6224799Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-22T20:46:25.5548171Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-22T20:46:25.5736906Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T20:46:25.7254500Z 
2020-03-22T20:46:25.7254834Z running 9 tests
2020-03-22T20:46:25.7256410Z iiiiiiiii
2020-03-22T20:46:25.7257459Z 
2020-03-22T20:46:25.7257656Z  finished in 0.152
2020-03-22T20:46:25.7263928Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-22T20:46:25.7440860Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-22T20:46:44.7222264Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-22T20:46:44.7426843Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T20:46:44.9217029Z 
2020-03-22T20:46:44.9217814Z running 115 tests
2020-03-22T20:46:57.8469133Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-22T20:46:59.6138602Z ...iiii.....ii.
2020-03-22T20:46:59.6139950Z 
2020-03-22T20:46:59.6143007Z  finished in 14.871
2020-03-22T20:46:59.6147435Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-22T20:46:59.6151297Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-22T20:59:32.8282568Z 
2020-03-22T20:59:32.8283644Z    Doc-tests core
2020-03-22T20:59:37.4353739Z 
2020-03-22T20:59:37.4362782Z running 2484 tests
2020-03-22T20:59:46.0377552Z ......iiiii......................................................................................... 100/2484
2020-03-22T20:59:54.4397141Z .....................................................................................ii............. 200/2484
2020-03-22T21:00:15.4378047Z ....................i............................................................................... 400/2484
2020-03-22T21:00:15.4378047Z ....................i............................................................................... 400/2484
2020-03-22T21:00:24.8313892Z .........................................................................i..i..................iiii. 500/2484
2020-03-22T21:00:40.3190872Z .................................................................................................... 700/2484
2020-03-22T21:00:48.2519019Z .................................................................................................... 800/2484
2020-03-22T21:00:56.1738635Z .................................................................................................... 900/2484
2020-03-22T21:01:04.0363543Z .................................................................................................... 1000/2484
---
2020-03-22T21:04:14.6476290Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-03-22T21:04:14.6485277Z .. 300/760
2020-03-22T21:04:14.7149917Z .................................................................................................... 400/760
2020-03-22T21:04:16.7806414Z .................................................................................................... 500/760
2020-03-22T21:04:16.8037864Z ..................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-03-22T21:04:16.8052012Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-03-22T21:04:16.8057809Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-03-22T21:04:16.8075687Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-03-22T21:04:17.7897933Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-03-22T21:04:17.7899476Z ....thread '<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError.', src/libstd/sync/mpsc/mod.rs:2034:21
2020-03-22T21:04:17.7901237Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-03-22T21:04:19.1354788Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-03-22T21:04:19.1363135Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-03-22T21:04:19.1371392Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-03-22T21:04:19.1379199Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-03-22T21:04:28.7933140Z 
2020-03-22T21:04:28.7933375Z running 1012 tests
2020-03-22T21:04:45.0467472Z i................................................................................................... 100/1012
2020-03-22T21:04:54.5905531Z .................................................................................................... 200/1012
2020-03-22T21:05:01.2354307Z ..................iii......i......i...i......i...................................................... 300/1012
2020-03-22T21:05:12.3456750Z ............................................i....i......................................ii.......... 500/1012
2020-03-22T21:05:19.1354314Z .................................................................................................... 600/1012
2020-03-22T21:05:24.1066257Z .................................................................................................... 700/1012
2020-03-22T21:05:24.1066257Z .................................................................................................... 700/1012
2020-03-22T21:05:30.5206078Z ......................................iiii.......................................................... 800/1012
2020-03-22T21:05:43.8855362Z .................................................................................................... 900/1012
2020-03-22T21:05:50.2596214Z ............................................................iiii.................................... 1000/1012
2020-03-22T21:05:50.6941814Z test result: ok. 992 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-22T21:05:50.6942119Z 
2020-03-22T21:05:50.6989596Z  finished in 159.488
2020-03-22T21:05:50.6994577Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-22T21:24:32.5075904Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-03-22T21:24:32.5371971Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-22T21:24:32.7585157Z 
2020-03-22T21:24:32.7585996Z running 210 tests
2020-03-22T21:25:03.4850681Z ......................i...ii.......................................................................i 100/210
2020-03-22T21:25:34.1329379Z ........................................iiiiii......i..............iii.............................. 200/210
2020-03-22T21:25:39.8962787Z test result: ok. 195 passed; 0 failed; 15 ignored; 0 measured; 0 filtered out
2020-03-22T21:25:39.8963105Z 
2020-03-22T21:25:39.8965834Z  finished in 67.359
2020-03-22T21:25:39.8979444Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
2020-03-22T21:26:15.5386055Z - error: missing documentation for crate
2020-03-22T21:26:15.5386332Z + error: missing documentation for the crate
2020-03-22T21:26:15.5386823Z 2   --> $DIR/deny-missing-docs-crate.rs:1:1
2020-03-22T21:26:15.5387060Z 3    |
2020-03-22T21:26:15.5387246Z 4 LL | / #![deny(missing_docs)]
2020-03-22T21:26:15.5387497Z 
2020-03-22T21:26:15.5443080Z The actual stderr differed from the expected stderr.
2020-03-22T21:26:15.5444217Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/deny-missing-docs-crate.stderr
2020-03-22T21:26:15.5444217Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/deny-missing-docs-crate.stderr
2020-03-22T21:26:15.5445077Z To update references, rerun the tests and pass the `--bless` flag
2020-03-22T21:26:15.5445842Z To only update this specific test, also pass `--test-args deny-missing-docs-crate.rs`
2020-03-22T21:26:15.5446408Z error: 1 errors occurred comparing output.
2020-03-22T21:26:15.5446723Z status: exit code: 1
2020-03-22T21:26:15.5446723Z status: exit code: 1
2020-03-22T21:26:15.5449119Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/auxiliary"
2020-03-22T21:26:15.5451126Z ------------------------------------------
2020-03-22T21:26:15.5451349Z 
2020-03-22T21:26:15.5451802Z ------------------------------------------
2020-03-22T21:26:15.5452055Z stderr:
2020-03-22T21:26:15.5452055Z stderr:
2020-03-22T21:26:15.5452690Z ------------------------------------------
2020-03-22T21:26:15.5453059Z error: missing documentation for the crate
2020-03-22T21:26:15.5453698Z   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1:1
2020-03-22T21:26:15.5454023Z    |
2020-03-22T21:26:15.5454286Z LL | / #![deny(missing_docs)] //~ ERROR
2020-03-22T21:26:15.5454763Z LL | | pub struct Foo; //~ ERROR
2020-03-22T21:26:15.5455029Z    | |_______________^
2020-03-22T21:26:15.5455244Z    |
2020-03-22T21:26:15.5455475Z note: the lint level is defined here
2020-03-22T21:26:15.5455475Z note: the lint level is defined here
2020-03-22T21:26:15.5456108Z   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1:9
2020-03-22T21:26:15.5456445Z    |
2020-03-22T21:26:15.5456677Z LL | #![deny(missing_docs)] //~ ERROR
2020-03-22T21:26:15.5457153Z 
2020-03-22T21:26:15.5457389Z error: missing documentation for a struct
2020-03-22T21:26:15.5458021Z   --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:3:1
2020-03-22T21:26:15.5458343Z    |
---
2020-03-22T21:26:15.5462249Z test result: FAILED. 39 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-22T21:26:15.5462605Z 
2020-03-22T21:26:15.5462796Z 
2020-03-22T21:26:15.5462934Z 
2020-03-22T21:26:15.5467843Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-22T21:26:15.5472421Z 
2020-03-22T21:26:15.5472522Z 
2020-03-22T21:26:15.5473140Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-22T21:26:15.5473551Z Build completed unsuccessfully in 1:43:40
2020-03-22T21:26:15.5473551Z Build completed unsuccessfully in 1:43:40
2020-03-22T21:26:15.5473803Z == clock drift check ==
2020-03-22T21:26:15.5501792Z   local time: Sun Mar 22 21:26:15 UTC 2020
2020-03-22T21:26:15.8469539Z   network time: Sun, 22 Mar 2020 21:26:15 GMT
2020-03-22T21:26:15.8470911Z == end clock drift check ==
2020-03-22T21:26:16.5210966Z 
2020-03-22T21:26:16.5289835Z ##[error]Bash exited with code '1'.
2020-03-22T21:26:16.5304430Z ##[section]Finishing: Run build
2020-03-22T21:26:16.5356691Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-22T21:26:16.5362344Z Task         : Get sources
2020-03-22T21:26:16.5362692Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T21:26:16.5363029Z Version      : 1.0.0
2020-03-22T21:26:16.5363254Z Author       : Microsoft
2020-03-22T21:26:16.5363254Z Author       : Microsoft
2020-03-22T21:26:16.5363622Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T21:26:16.5364053Z ==============================================================================
2020-03-22T21:26:16.8682443Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T21:26:16.8730434Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69740/merge to s
2020-03-22T21:26:16.8819709Z Cleaning up task key
2020-03-22T21:26:16.8821119Z Start cleaning up orphan processes.
2020-03-22T21:26:16.9006013Z Terminate orphan process: pid (3873) (python)
2020-03-22T21:26:16.9299894Z ##[section]Finishing: Finalize Job
