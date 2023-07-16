plain
2020-02-11T13:40:52.7996724Z ========================== Starting Command Output ===========================
2020-02-11T13:40:52.8014851Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bcda59bf-e854-404b-a6d1-0b08d4eed5ba.sh
2020-02-11T13:40:52.8193277Z 
2020-02-11T13:40:52.8261238Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T13:40:52.8266866Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69015/merge to s
2020-02-11T13:40:52.8268466Z Task         : Get sources
2020-02-11T13:40:52.8268503Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T13:40:52.8268553Z Version      : 1.0.0
2020-02-11T13:40:52.8268589Z Author       : Microsoft
---
2020-02-11T13:40:53.6688301Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T13:40:53.6810127Z ##[command]git config gc.auto 0
2020-02-11T13:40:53.6857283Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T13:40:53.6905581Z ##[command]git config --get-all http.proxy
2020-02-11T13:40:53.7038133Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69015/merge:refs/remotes/pull/69015/merge
---
2020-02-11T14:34:09.7058120Z .................................................................................................... 1700/9627
2020-02-11T14:34:14.0496946Z .................................................................................................... 1800/9627
2020-02-11T14:34:24.6849563Z ..............................i..................................................................... 1900/9627
2020-02-11T14:34:31.5257538Z .................................................................................................... 2000/9627
2020-02-11T14:34:43.9855783Z ....................iiiii........................................................................... 2100/9627
2020-02-11T14:34:52.3334273Z .................................................................................................... 2300/9627
2020-02-11T14:34:54.3414118Z .................................................................................................... 2400/9627
2020-02-11T14:34:58.4935243Z .................................................................................................... 2500/9627
2020-02-11T14:35:15.3883240Z .................................................................................................... 2600/9627
---
2020-02-11T14:37:39.6134443Z .......................................................................i...............i............ 4900/9627
2020-02-11T14:37:47.5359742Z .................................................................................................... 5000/9627
2020-02-11T14:37:55.0189266Z .................................................................................................... 5100/9627
2020-02-11T14:37:59.4197554Z .............i...................................................................................... 5200/9627
2020-02-11T14:38:09.5925604Z .......................................................................................ii.ii........ 5300/9627
2020-02-11T14:38:13.1595251Z i...i............................................................................................... 5400/9627
2020-02-11T14:38:23.7272106Z .................................................................................................... 5600/9627
2020-02-11T14:38:31.2129772Z ...........................................................................i........................ 5700/9627
2020-02-11T14:38:37.7445695Z .................................................................................................... 5800/9627
2020-02-11T14:38:43.4593201Z .................................................................................................... 5900/9627
2020-02-11T14:38:43.4593201Z .................................................................................................... 5900/9627
2020-02-11T14:38:52.5787985Z ...................................................................ii...i..ii...........i........... 6000/9627
2020-02-11T14:39:11.8704444Z .................................................................................................... 6200/9627
2020-02-11T14:39:18.7477959Z .................................................................................................... 6300/9627
2020-02-11T14:39:26.4509804Z ...............................................................................................i..ii 6400/9627
2020-02-11T14:39:38.8564600Z .................................................................................................... 6500/9627
---
2020-02-11T14:41:33.2749855Z .................................................................................................... 7600/9627
2020-02-11T14:41:37.2546510Z .................................................................................................... 7700/9627
2020-02-11T14:41:42.8256856Z .................................................................................................... 7800/9627
2020-02-11T14:41:51.2655102Z .................................................................................................... 7900/9627
2020-02-11T14:41:59.4934110Z .....................................................................iiiiiii.i...................... 8000/9627
2020-02-11T14:42:13.6021094Z .........i......i................................................................................... 8200/9627
2020-02-11T14:42:19.0970606Z .................................................................................................... 8300/9627
2020-02-11T14:42:33.2351742Z .................................................................................................... 8400/9627
2020-02-11T14:42:42.5861344Z .................................................................................................... 8500/9627
---
2020-02-11T14:45:02.0119018Z  finished in 7.080
2020-02-11T14:45:02.0331564Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T14:45:02.2515205Z 
2020-02-11T14:45:02.2516078Z running 178 tests
2020-02-11T14:45:05.3997207Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-11T14:45:07.8888701Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-11T14:45:07.8889319Z 
2020-02-11T14:45:07.8893018Z  finished in 5.856
2020-02-11T14:45:07.9110339Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T14:45:08.0877885Z 
---
2020-02-11T14:45:10.1617934Z  finished in 2.249
2020-02-11T14:45:10.1819236Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T14:45:10.3513464Z 
2020-02-11T14:45:10.3513689Z running 9 tests
2020-02-11T14:45:10.3514489Z iiiiiiiii
2020-02-11T14:45:10.3515126Z 
2020-02-11T14:45:10.3515174Z  finished in 0.168
2020-02-11T14:45:10.3718863Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T14:45:10.5602787Z 
2020-02-11T14:45:10.5602787Z 
2020-02-11T14:45:10.5603391Z running 115 tests
2020-02-11T14:45:29.2321878Z .......................................................F............................................ 100/115
2020-02-11T14:45:31.4475076Z ...............
2020-02-11T14:45:31.4476016Z failures:
2020-02-11T14:45:31.4476074Z 
2020-02-11T14:45:31.4476671Z ---- [incremental] incremental/ich_nested_items.rs stdout ----
2020-02-11T14:45:31.4476746Z 
2020-02-11T14:45:31.4477042Z error in revision `cfail2`: test compilation failed although it shouldn't!
2020-02-11T14:45:31.4477095Z status: exit code: 1
2020-02-11T14:45:31.4478161Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_nested_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items/ich_nested_items.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items/auxiliary"
2020-02-11T14:45:31.4478505Z ------------------------------------------
2020-02-11T14:45:31.4478544Z 
2020-02-11T14:45:31.4478748Z ------------------------------------------
2020-02-11T14:45:31.4478807Z stderr:
2020-02-11T14:45:31.4478807Z stderr:
2020-02-11T14:45:31.4479005Z ------------------------------------------
2020-02-11T14:45:31.4479049Z warning: function is never used: `bar`
2020-02-11T14:45:31.4479285Z   --> /checkout/src/test/incremental/ich_nested_items.rs:18:12
2020-02-11T14:45:31.4479339Z    |
2020-02-11T14:45:31.4479553Z LL |     pub fn bar() { } // but that doesn't matter.
2020-02-11T14:45:31.4479650Z    |
2020-02-11T14:45:31.4479688Z    = note: `#[warn(dead_code)]` on by default
2020-02-11T14:45:31.4479715Z 
2020-02-11T14:45:31.4479769Z warning: function is never used: `baz`
---
2020-02-11T14:45:31.4480150Z 
2020-02-11T14:45:31.4480379Z warning: function is never used: `bap`
2020-02-11T14:45:31.4480679Z   --> /checkout/src/test/incremental/ich_nested_items.rs:23:12
2020-02-11T14:45:31.4480723Z    |
2020-02-11T14:45:31.4480762Z LL |     pub fn bap() { } // neither does adding a new item
2020-02-11T14:45:31.4480960Z 
2020-02-11T14:45:31.4480960Z 
2020-02-11T14:45:31.4481000Z error: `Hir(foo::bar)` should be clean but is not
2020-02-11T14:45:31.4481311Z    |
2020-02-11T14:45:31.4481311Z    |
2020-02-11T14:45:31.4481518Z LL |     pub fn bar() { } // but that doesn't matter.
2020-02-11T14:45:31.4481604Z 
2020-02-11T14:45:31.4481641Z error: aborting due to previous error
2020-02-11T14:45:31.4481666Z 
2020-02-11T14:45:31.4481689Z 
---
2020-02-11T14:45:31.4482712Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-11T14:45:31.4482764Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-11T14:45:31.4484190Z 
2020-02-11T14:45:31.4484333Z 
2020-02-11T14:45:31.4485974Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-11T14:45:31.4486416Z 
2020-02-11T14:45:31.4486442Z 
2020-02-11T14:45:31.4496582Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-11T14:45:31.4496663Z Build completed unsuccessfully in 0:58:18
2020-02-11T14:45:31.4496663Z Build completed unsuccessfully in 0:58:18
2020-02-11T14:45:31.4551929Z == clock drift check ==
2020-02-11T14:45:31.4576325Z   local time: Tue Feb 11 14:45:31 UTC 2020
2020-02-11T14:45:31.7505459Z   network time: Tue, 11 Feb 2020 14:45:31 GMT
2020-02-11T14:45:31.7508898Z == end clock drift check ==
2020-02-11T14:45:34.3547129Z 
2020-02-11T14:45:34.3627690Z ##[error]Bash exited with code '1'.
2020-02-11T14:45:34.3638701Z ##[section]Finishing: Run build
2020-02-11T14:45:34.3660847Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69015/merge to s
2020-02-11T14:45:34.3662410Z Task         : Get sources
2020-02-11T14:45:34.3662449Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T14:45:34.3662486Z Version      : 1.0.0
2020-02-11T14:45:34.3662537Z Author       : Microsoft
2020-02-11T14:45:34.3662537Z Author       : Microsoft
2020-02-11T14:45:34.3662575Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T14:45:34.3662615Z ==============================================================================
2020-02-11T14:45:34.7546390Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T14:45:34.7583099Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69015/merge to s
2020-02-11T14:45:34.7679494Z Cleaning up task key
2020-02-11T14:45:34.7680109Z Start cleaning up orphan processes.
2020-02-11T14:45:34.7773411Z Terminate orphan process: pid (3397) (python)
2020-02-11T14:45:34.7982965Z ##[section]Finishing: Finalize Job
