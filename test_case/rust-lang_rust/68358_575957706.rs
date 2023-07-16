plain
2020-01-19T00:53:40.5236094Z ========================== Starting Command Output ===========================
2020-01-19T00:53:40.5240323Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/89fdb62c-760e-4425-a48e-2d0e9e6101eb.sh
2020-01-19T00:53:40.5680857Z 
2020-01-19T00:53:40.5762224Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-19T00:53:40.5769366Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68358/merge to s
2020-01-19T00:53:40.5771186Z Task         : Get sources
2020-01-19T00:53:40.5771269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T00:53:40.5771306Z Version      : 1.0.0
2020-01-19T00:53:40.5771339Z Author       : Microsoft
---
2020-01-19T00:53:46.2007720Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-19T00:53:46.2469771Z ##[command]git config gc.auto 0
2020-01-19T00:53:46.2539010Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-19T00:53:46.2595619Z ##[command]git config --get-all http.proxy
2020-01-19T00:53:46.2757410Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68358/merge:refs/remotes/pull/68358/merge
---
2020-01-19T01:49:35.5149253Z .................................................................................................... 1700/9540
2020-01-19T01:49:42.4357499Z .................................................................................................... 1800/9540
2020-01-19T01:49:54.5865159Z ...................i................................................................................ 1900/9540
2020-01-19T01:50:01.9738941Z .................................................................................................... 2000/9540
2020-01-19T01:50:18.5317962Z .........iiiii...................................................................................... 2100/9540
2020-01-19T01:50:28.5163754Z .................................................................................................... 2300/9540
2020-01-19T01:50:30.9164233Z .................................................................................................... 2400/9540
2020-01-19T01:50:36.3936343Z .................................................................................................... 2500/9540
2020-01-19T01:50:57.8380704Z .................................................................................................... 2600/9540
---
2020-01-19T01:53:45.2492812Z .....................................................i...............i.............................. 4900/9540
2020-01-19T01:53:53.4996428Z .................................................................................................... 5000/9540
2020-01-19T01:54:01.5284469Z ................................................................................................i... 5100/9540
2020-01-19T01:54:06.8571910Z .................................................................................................... 5200/9540
2020-01-19T01:54:17.6304368Z ....................................................................ii.ii...........i............... 5300/9540
2020-01-19T01:54:26.5402163Z .....i.............................................................................................. 5500/9540
2020-01-19T01:54:36.4480560Z .................................................................................................... 5600/9540
2020-01-19T01:54:42.7806978Z ......................................................i............................................. 5700/9540
2020-01-19T01:54:49.5331266Z .................................................................................................... 5800/9540
2020-01-19T01:54:49.5331266Z .................................................................................................... 5800/9540
2020-01-19T01:54:59.4308700Z .................................................................................................... 5900/9540
2020-01-19T01:55:06.0772745Z ............................................ii...i..ii...........i.................................. 6000/9540
2020-01-19T01:55:27.2775928Z .................................................................................................... 6200/9540
2020-01-19T01:55:35.1912033Z .................................................................................................... 6300/9540
2020-01-19T01:55:35.1912033Z .................................................................................................... 6300/9540
2020-01-19T01:55:45.0614625Z ........................................................................i..ii....................... 6400/9540
2020-01-19T01:56:14.0288459Z .................................................................................................... 6600/9540
2020-01-19T01:56:16.9047381Z ................................................i................................................... 6700/9540
2020-01-19T01:56:18.9875178Z .................................................................................................... 6800/9540
2020-01-19T01:56:21.1827391Z ...............................................i.................................................... 6900/9540
---
2020-01-19T01:57:55.7721471Z .................................................................................................... 7500/9540
2020-01-19T01:58:00.2044904Z .................................................................................................... 7600/9540
2020-01-19T01:58:05.6208917Z .................................................................................................... 7700/9540
2020-01-19T01:58:11.9524722Z .................................................................................................... 7800/9540
2020-01-19T01:58:23.1610544Z ..................................................................................................ii 7900/9540
2020-01-19T01:58:29.8130513Z iiiii............................................................................................... 8000/9540
2020-01-19T01:58:44.7911472Z ..........................................................................................FF........ 8200/9540
2020-01-19T01:58:56.1954706Z .................................................................................................... 8300/9540
2020-01-19T01:59:08.3299847Z .................................................................................................... 8400/9540
2020-01-19T01:59:13.9401110Z .................................................................................................... 8500/9540
---
2020-01-19T02:01:06.1189990Z ---- [ui] ui/specialization/soundness/partial_eq_range_inclusive.rs stdout ----
2020-01-19T02:01:06.1190208Z 
2020-01-19T02:01:06.1190563Z error: test compilation failed although it shouldn't!
2020-01-19T02:01:06.1190766Z status: exit code: 1
2020-01-19T02:01:06.1191619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/soundness/partial_eq_range_inclusive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/soundness/partial_eq_range_inclusive/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/soundness/partial_eq_range_inclusive/auxiliary"
2020-01-19T02:01:06.1192197Z ------------------------------------------
2020-01-19T02:01:06.1192350Z 
2020-01-19T02:01:06.1194191Z ------------------------------------------
2020-01-19T02:01:06.1194508Z stderr:
2020-01-19T02:01:06.1194508Z stderr:
2020-01-19T02:01:06.1195080Z ------------------------------------------
2020-01-19T02:01:06.1195634Z error[E0369]: binary operation `==` cannot be applied to type `std::cell::Ref<'_, std::vec::Vec<&str>>`
2020-01-19T02:01:06.1196186Z   --> /checkout/src/test/ui/specialization/soundness/partial_eq_range_inclusive.rs:34:5
2020-01-19T02:01:06.1196729Z LL |     assert_eq!(values.borrow(), Vec::new());
2020-01-19T02:01:06.1196873Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-19T02:01:06.1197000Z    |     |
2020-01-19T02:01:06.1197335Z    |     std::cell::Ref<'_, std::vec::Vec<&str>>
2020-01-19T02:01:06.1197335Z    |     std::cell::Ref<'_, std::vec::Vec<&str>>
2020-01-19T02:01:06.1197537Z    |     std::vec::Vec<_>
2020-01-19T02:01:06.1197682Z    |
2020-01-19T02:01:06.1198088Z    = note: an implementation of `std::cmp::PartialEq` might be missing for `std::cell::Ref<'_, std::vec::Vec<&str>>`
2020-01-19T02:01:06.1198993Z 
2020-01-19T02:01:06.1199194Z error: aborting due to previous error
2020-01-19T02:01:06.1199314Z 
2020-01-19T02:01:06.1199755Z For more information about this error, try `rustc --explain E0369`.
---
2020-01-19T02:01:06.1205600Z 
2020-01-19T02:01:06.1205811Z warning: unused variable: `other`
2020-01-19T02:01:06.1206262Z   --> $DIR/partial_ord_slice.rs:25:19
2020-01-19T02:01:06.1206698Z    |
2020-01-19T02:01:06.1207256Z LL |     fn cmp(&self, other: &Self) -> Ordering {
2020-01-19T02:01:06.1207464Z    |                   ^^^^^ help: consider prefixing with an underscore: `_other`
2020-01-19T02:01:06.1207739Z warning: unused comparison that must be used
2020-01-19T02:01:06.1208247Z   --> $DIR/partial_ord_slice.rs:37:9
2020-01-19T02:01:06.1208420Z    |
2020-01-19T02:01:06.1208420Z    |
2020-01-19T02:01:06.1208547Z LL |         &y[..] <= &y[..];
2020-01-19T02:01:06.1209035Z    |
2020-01-19T02:01:06.1209198Z    = note: `#[warn(unused_must_use)]` on by default
2020-01-19T02:01:06.1209315Z 
2020-01-19T02:01:06.1209427Z 
2020-01-19T02:01:06.1209427Z 
2020-01-19T02:01:06.1209539Z 
2020-01-19T02:01:06.1209651Z 
2020-01-19T02:01:06.1209784Z The actual stderr differed from the expected stderr.
2020-01-19T02:01:06.1210457Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/soundness/partial_ord_slice/partial_ord_slice.stderr
2020-01-19T02:01:06.1210897Z To update references, rerun the tests and pass the `--bless` flag
2020-01-19T02:01:06.1211361Z To only update this specific test, also pass `--test-args specialization/soundness/partial_ord_slice.rs`
2020-01-19T02:01:06.1211684Z error: 1 errors occurred comparing output.
2020-01-19T02:01:06.1211854Z status: exit code: 0
2020-01-19T02:01:06.1211854Z status: exit code: 0
2020-01-19T02:01:06.1213119Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/soundness/partial_ord_slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/soundness/partial_ord_slice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/soundness/partial_ord_slice/auxiliary"
2020-01-19T02:01:06.1227330Z ------------------------------------------
2020-01-19T02:01:06.1227725Z 
2020-01-19T02:01:06.1228097Z ------------------------------------------
2020-01-19T02:01:06.1228277Z stderr:
---
2020-01-19T02:01:06.1247439Z 
2020-01-19T02:01:06.1247569Z warning: unused variable: `other`
2020-01-19T02:01:06.1248021Z   --> /checkout/src/test/ui/specialization/soundness/partial_ord_slice.rs:25:19
2020-01-19T02:01:06.1248987Z    |
2020-01-19T02:01:06.1249494Z LL |     fn cmp(&self, other: &Self) -> Ordering {
2020-01-19T02:01:06.1249747Z    |                   ^^^^^ help: consider prefixing with an underscore: `_other`
2020-01-19T02:01:06.1249998Z warning: unused comparison that must be used
2020-01-19T02:01:06.1250396Z   --> /checkout/src/test/ui/specialization/soundness/partial_ord_slice.rs:37:9
2020-01-19T02:01:06.1250591Z    |
2020-01-19T02:01:06.1250591Z    |
2020-01-19T02:01:06.1250718Z LL |         &y[..] <= &y[..];
2020-01-19T02:01:06.1251010Z    |
2020-01-19T02:01:06.1251138Z    = note: `#[warn(unused_must_use)]` on by default
2020-01-19T02:01:06.1251245Z 
2020-01-19T02:01:06.1251351Z 
---
2020-01-19T02:01:06.1254814Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-19T02:01:06.1255102Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-19T02:01:06.1255260Z 
2020-01-19T02:01:06.1255428Z 
2020-01-19T02:01:06.1257249Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-19T02:01:06.1257855Z 
2020-01-19T02:01:06.1257963Z 
2020-01-19T02:01:06.1261144Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-19T02:01:06.1261336Z Build completed unsuccessfully in 1:01:09
2020-01-19T02:01:06.1261336Z Build completed unsuccessfully in 1:01:09
2020-01-19T02:01:06.1321845Z == clock drift check ==
2020-01-19T02:01:06.1334280Z   local time: Sun Jan 19 02:01:06 UTC 2020
2020-01-19T02:01:06.6878784Z   network time: Sun, 19 Jan 2020 02:01:06 GMT
2020-01-19T02:01:06.6879509Z == end clock drift check ==
2020-01-19T02:01:07.1742311Z 
2020-01-19T02:01:07.1835227Z ##[error]Bash exited with code '1'.
2020-01-19T02:01:07.1848073Z ##[section]Finishing: Run build
2020-01-19T02:01:07.1869478Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68358/merge to s
2020-01-19T02:01:07.1871441Z Task         : Get sources
2020-01-19T02:01:07.1871479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T02:01:07.1871536Z Version      : 1.0.0
2020-01-19T02:01:07.1871588Z Author       : Microsoft
2020-01-19T02:01:07.1871588Z Author       : Microsoft
2020-01-19T02:01:07.1871628Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-19T02:01:07.1871670Z ==============================================================================
2020-01-19T02:01:07.5846374Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-19T02:01:07.5887631Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68358/merge to s
2020-01-19T02:01:07.6000979Z Cleaning up task key
2020-01-19T02:01:07.6001748Z Start cleaning up orphan processes.
2020-01-19T02:01:07.6107823Z Terminate orphan process: pid (3794) (python)
2020-01-19T02:01:07.6952760Z ##[section]Finishing: Finalize Job
