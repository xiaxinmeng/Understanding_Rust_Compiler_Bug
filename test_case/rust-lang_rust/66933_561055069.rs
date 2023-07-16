plain
2019-12-03T07:22:14.3663137Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T07:22:14.3873475Z ##[command]git config gc.auto 0
2019-12-03T07:22:14.3954831Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T07:22:14.3990374Z ##[command]git config --get-all http.proxy
2019-12-03T07:22:14.4163069Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66933/merge:refs/remotes/pull/66933/merge
---
2019-12-03T08:16:40.7317750Z .................................................................................................... 1600/9320
2019-12-03T08:16:45.0268794Z .................................................................................................... 1700/9320
2019-12-03T08:16:56.8223071Z ........................................i........................................................... 1800/9320
2019-12-03T08:17:04.0446688Z .................................................................................................... 1900/9320
2019-12-03T08:17:16.5821982Z .........................iiiii...................................................................... 2000/9320
2019-12-03T08:17:26.1344992Z .................................................................................................... 2200/9320
2019-12-03T08:17:28.4985744Z .................................................................................................... 2300/9320
2019-12-03T08:17:32.7561689Z .................................................................................................... 2400/9320
2019-12-03T08:17:52.8957538Z .................................................................................................... 2500/9320
---
2019-12-03T08:20:23.7615803Z ..........................i...............i......................................................... 4800/9320
2019-12-03T08:20:33.2341657Z .................................................................................................... 4900/9320
2019-12-03T08:20:38.6138331Z .................................................................................................... 5000/9320
2019-12-03T08:20:45.9046769Z ...............................F.................................................................... 5100/9320
2019-12-03T08:20:52.7716309Z .................................ii.ii...........i.................................................. 5200/9320
2019-12-03T08:21:01.3320206Z .................................................................................................... 5400/9320
2019-12-03T08:21:10.2881854Z .................................................................................................... 5500/9320
2019-12-03T08:21:16.9077499Z ...............i.................................................................................... 5600/9320
2019-12-03T08:21:22.3460832Z .................................................................................................... 5700/9320
2019-12-03T08:21:22.3460832Z .................................................................................................... 5700/9320
2019-12-03T08:21:33.0187663Z .................................................................................................... 5800/9320
2019-12-03T08:21:44.3520515Z .ii...i..ii...........i............................................................................. 5900/9320
2019-12-03T08:22:01.4604676Z .................................................................................................... 6100/9320
2019-12-03T08:22:07.6404715Z .................................................................................................... 6200/9320
2019-12-03T08:22:07.6404715Z .................................................................................................... 6200/9320
2019-12-03T08:22:20.2034722Z ........................i..ii....................................................................... 6300/9320
2019-12-03T08:22:38.6393829Z ...............................................................................................i.... 6500/9320
2019-12-03T08:22:40.8250631Z .................................................................................................... 6600/9320
2019-12-03T08:22:42.9001651Z ......................................................................................i............. 6700/9320
2019-12-03T08:22:45.5033974Z .................................................................................................... 6800/9320
---
2019-12-03T08:24:16.4516278Z .................................................................................................... 7300/9320
2019-12-03T08:24:21.3744032Z .................................................................................................... 7400/9320
2019-12-03T08:24:27.0137932Z .................................................................................................... 7500/9320
2019-12-03T08:24:33.3574124Z .................................................................................................... 7600/9320
2019-12-03T08:24:44.8182986Z ...................................................................................................i 7700/9320
2019-12-03T08:24:51.5751092Z iii................................................................................................. 7800/9320
2019-12-03T08:25:00.2740372Z ............................ii......i............................................................... 7900/9320
2019-12-03T08:25:16.4243284Z .................................................................................................... 8100/9320
2019-12-03T08:25:27.9192093Z .................................................................................................... 8200/9320
2019-12-03T08:25:34.0638391Z .................................................................................................... 8300/9320
2019-12-03T08:25:57.6266559Z .................................................................................................... 8400/9320
---
2019-12-03T08:27:20.6340636Z 
2019-12-03T08:27:20.6341530Z ---- [ui] ui/issues/issue-66923-show-error-for-correct-call.rs stdout ----
2019-12-03T08:27:20.6341859Z diff of stderr:
2019-12-03T08:27:20.6342031Z 
2019-12-03T08:27:20.6342235Z 1 error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T08:27:20.6342705Z -   --> $DIR/issue-66923.rs:6:39
2019-12-03T08:27:20.6343171Z +   --> $DIR/issue-66923-show-error-for-correct-call.rs:8:39
2019-12-03T08:27:20.6343414Z 3    |
2019-12-03T08:27:20.6343626Z 4 LL |     let x2: Vec<f64> = x1.into_iter().collect();
2019-12-03T08:27:20.6343999Z 5    |                                       ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T08:27:20.6344172Z 
2019-12-03T08:27:20.6344395Z 7    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-03T08:27:20.6344586Z 8 
2019-12-03T08:27:20.6344798Z 9 error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T08:27:20.6345201Z -   --> $DIR/issue-66923.rs:10:29
2019-12-03T08:27:20.6345843Z +   --> $DIR/issue-66923-show-error-for-correct-call.rs:12:29
2019-12-03T08:27:20.6346110Z 11    |
2019-12-03T08:27:20.6346309Z 12 LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
2019-12-03T08:27:20.6346632Z 13    |                             ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T08:27:20.6347165Z 
2019-12-03T08:27:20.6347373Z The actual stderr differed from the expected stderr.
2019-12-03T08:27:20.6347373Z The actual stderr differed from the expected stderr.
2019-12-03T08:27:20.6347892Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/issue-66923-show-error-for-correct-call.stderr
2019-12-03T08:27:20.6348382Z To update references, rerun the tests and pass the `--bless` flag
2019-12-03T08:27:20.6348942Z To only update this specific test, also pass `--test-args issues/issue-66923-show-error-for-correct-call.rs`
2019-12-03T08:27:20.6349416Z error: 1 errors occurred comparing output.
2019-12-03T08:27:20.6349614Z status: exit code: 1
2019-12-03T08:27:20.6349614Z status: exit code: 1
2019-12-03T08:27:20.6350637Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/auxiliary" "-A" "unused"
2019-12-03T08:27:20.6351391Z ------------------------------------------
2019-12-03T08:27:20.6351830Z 
2019-12-03T08:27:20.6352256Z ------------------------------------------
2019-12-03T08:27:20.6352511Z stderr:
2019-12-03T08:27:20.6352511Z stderr:
2019-12-03T08:27:20.6352897Z ------------------------------------------
2019-12-03T08:27:20.6353141Z error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T08:27:20.6353869Z   --> /checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs:8:39
2019-12-03T08:27:20.6354222Z    |
2019-12-03T08:27:20.6354459Z LL |     let x2: Vec<f64> = x1.into_iter().collect();
2019-12-03T08:27:20.6354681Z    |                                       ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T08:27:20.6354870Z    |
2019-12-03T08:27:20.6355082Z    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-03T08:27:20.6355252Z 
2019-12-03T08:27:20.6355441Z error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T08:27:20.6355913Z   --> /checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs:12:29
2019-12-03T08:27:20.6356150Z    |
2019-12-03T08:27:20.6356336Z LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
2019-12-03T08:27:20.6356565Z    |                             ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T08:27:20.6356755Z    |
2019-12-03T08:27:20.6356957Z    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-03T08:27:20.6357304Z error: aborting due to 2 previous errors
2019-12-03T08:27:20.6357462Z 
2019-12-03T08:27:20.6357890Z For more information about this error, try `rustc --explain E0277`.
2019-12-03T08:27:20.6358103Z 
---
2019-12-03T08:27:20.6380540Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-03T08:27:20.6380936Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-03T08:27:20.6396966Z 
2019-12-03T08:27:20.6401880Z 
2019-12-03T08:27:20.6403713Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-03T08:27:20.6404525Z 
2019-12-03T08:27:20.6404635Z 
2019-12-03T08:27:20.6407093Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-03T08:27:20.6407273Z Build completed unsuccessfully in 0:59:30
2019-12-03T08:27:20.6407273Z Build completed unsuccessfully in 0:59:30
2019-12-03T08:27:20.6452735Z == clock drift check ==
2019-12-03T08:27:20.6466057Z   local time: Tue Dec  3 08:27:20 UTC 2019
2019-12-03T08:27:20.9246731Z   network time: Tue, 03 Dec 2019 08:27:20 GMT
2019-12-03T08:27:20.9248217Z == end clock drift check ==
2019-12-03T08:27:21.7463341Z 
2019-12-03T08:27:21.7556587Z ##[error]Bash exited with code '1'.
2019-12-03T08:27:21.7590643Z ##[section]Starting: Checkout
2019-12-03T08:27:21.7592423Z ==============================================================================
2019-12-03T08:27:21.7592495Z Task         : Get sources
2019-12-03T08:27:21.7592540Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
