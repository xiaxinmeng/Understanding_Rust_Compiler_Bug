plain
2019-08-20T19:17:10.9322297Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T19:17:10.9505458Z ##[command]git config gc.auto 0
2019-08-20T19:17:10.9590177Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T19:17:10.9646735Z ##[command]git config --get-all http.proxy
2019-08-20T19:17:10.9776026Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61845/merge:refs/remotes/pull/61845/merge
---
2019-08-20T19:17:45.0874095Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T19:17:45.0874143Z 
2019-08-20T19:17:45.0874349Z   git checkout -b <new-branch-name>
2019-08-20T19:17:45.0874379Z 
2019-08-20T19:17:45.0874443Z HEAD is now at 30dd50369 Merge 92e65d020012d49ae8e5645650f4e071785a869e into 5a56e05abd34e1936df74625c1f40cb6fee0cd4a
2019-08-20T19:17:45.1049880Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T19:17:45.1052703Z ==============================================================================
2019-08-20T19:17:45.1052778Z Task         : Bash
2019-08-20T19:17:45.1052823Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T20:21:26.6200843Z .................................................................................................... 1500/8943
2019-08-20T20:21:32.3092467Z .................................................................................................... 1600/8943
2019-08-20T20:21:45.5922109Z .......................................i...............i............................................ 1700/8943
2019-08-20T20:21:53.9337292Z .................................................................................................... 1800/8943
2019-08-20T20:22:08.6248851Z ...............................iiiii................................................................ 1900/8943
2019-08-20T20:22:19.5497359Z .................................................................................................... 2100/8943
2019-08-20T20:22:22.2125451Z .................................................................................................... 2200/8943
2019-08-20T20:22:27.0158447Z .................................................................................................... 2300/8943
2019-08-20T20:22:34.2099726Z .................................................................................................... 2400/8943
---
2019-08-20T20:25:37.6496204Z ...................i...............i................................................................ 4700/8943
2019-08-20T20:25:49.4777826Z .................................................................................................... 4800/8943
2019-08-20T20:25:55.9101756Z .................................................................................................... 4900/8943
2019-08-20T20:26:07.2032812Z .................................................................................................... 5000/8943
2019-08-20T20:26:12.4576608Z ii.ii............................................................................................... 5100/8943
2019-08-20T20:26:27.3633338Z .................................................................................................... 5300/8943
2019-08-20T20:26:34.4323112Z ........................................................i........................................... 5400/8943
2019-08-20T20:26:41.5657976Z .................................................................................................... 5500/8943
2019-08-20T20:26:51.2874115Z .................................................................................................... 5600/8943
2019-08-20T20:26:51.2874115Z .................................................................................................... 5600/8943
2019-08-20T20:27:00.7943801Z .................................................ii...i..ii...........i............................. 5700/8943
2019-08-20T20:27:23.6956546Z .................................................................................................... 5900/8943
2019-08-20T20:27:28.6945815Z .................................................................................................... 6000/8943
2019-08-20T20:27:28.6945815Z .................................................................................................... 6000/8943
2019-08-20T20:27:39.9151671Z ..................................................i..ii............................................. 6100/8943
2019-08-20T20:28:03.8439469Z ................................................................................................i... 6300/8943
2019-08-20T20:28:06.1258736Z .................................................................................................... 6400/8943
2019-08-20T20:28:08.4279931Z ....................................................................i............................... 6500/8943
2019-08-20T20:28:11.3911743Z .................................................................................................... 6600/8943
---
2019-08-20T20:32:58.4142441Z  finished in 21.839
2019-08-20T20:32:58.4346199Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T20:32:58.6033549Z 
2019-08-20T20:32:58.6033857Z running 148 tests
2019-08-20T20:33:01.9319396Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/148
2019-08-20T20:33:03.9228603Z .iiii..............i.........iii.i......ii......
2019-08-20T20:33:03.9232234Z 
2019-08-20T20:33:03.9233178Z  finished in 5.488
2019-08-20T20:33:03.9412115Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T20:33:04.1042599Z 
---
2019-08-20T20:33:06.2015285Z  finished in 2.259
2019-08-20T20:33:06.2208077Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T20:33:06.3813765Z 
2019-08-20T20:33:06.3813960Z running 9 tests
2019-08-20T20:33:06.3815195Z iiiiiiiii
2019-08-20T20:33:06.3815555Z 
2019-08-20T20:33:06.3815596Z  finished in 0.160
2019-08-20T20:33:06.3993351Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-20T20:33:06.5653941Z 
2019-08-20T20:33:06.5653941Z 
2019-08-20T20:33:06.5654185Z running 104 tests
2019-08-20T20:33:23.7044268Z .....F.........F....F.FFFFFFFFFFFFFFFFFFFFFFFFF...F.......F.F..........F.F...F....................F. 100/104
2019-08-20T20:33:24.2992183Z F.FF
2019-08-20T20:33:24.3002891Z 
2019-08-20T20:33:24.3003389Z ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
2019-08-20T20:33:24.3003451Z 
2019-08-20T20:33:24.3003451Z 
2019-08-20T20:33:24.3003532Z error in revision `rpass3`: compilation failed!
2019-08-20T20:33:24.3003580Z status: exit code: 101
2019-08-20T20:33:24.3004576Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
2019-08-20T20:33:24.3005193Z ------------------------------------------
2019-08-20T20:33:24.3005241Z 
2019-08-20T20:33:24.3005459Z ------------------------------------------
2019-08-20T20:33:24.3005524Z stderr:
2019-08-20T20:33:24.3005524Z stderr:
2019-08-20T20:33:24.3006102Z ------------------------------------------
2019-08-20T20:33:24.3006426Z thread 'rustc' panicked at 'index out of bounds: the len is 5660 but the index is 5660', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3006507Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T20:33:24.3006812Z thread 'rustc' panicked at 'index out of bounds: the len is 5660 but the index is 5661', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3006912Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3006941Z 
2019-08-20T20:33:24.3006985Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3007015Z 
2019-08-20T20:33:24.3007015Z 
2019-08-20T20:33:24.3007423Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3007703Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3007736Z 
2019-08-20T20:33:24.3007736Z 
2019-08-20T20:33:24.3008080Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3008150Z 
2019-08-20T20:33:24.3008560Z ------------------------------------------
2019-08-20T20:33:24.3008592Z 
2019-08-20T20:33:24.3008616Z 
2019-08-20T20:33:24.3008616Z 
2019-08-20T20:33:24.3008841Z ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
2019-08-20T20:33:24.3008874Z 
2019-08-20T20:33:24.3008935Z error in revision `rpass3`: compilation failed!
2019-08-20T20:33:24.3008980Z status: exit code: 101
2019-08-20T20:33:24.3010596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
2019-08-20T20:33:24.3010977Z ------------------------------------------
2019-08-20T20:33:24.3011010Z 
2019-08-20T20:33:24.3011211Z ------------------------------------------
2019-08-20T20:33:24.3011271Z stderr:
2019-08-20T20:33:24.3011271Z stderr:
2019-08-20T20:33:24.3011469Z ------------------------------------------
2019-08-20T20:33:24.3011763Z thread 'rustc' panicked at 'index out of bounds: the len is 7111 but the index is 7111', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3011874Z 
2019-08-20T20:33:24.3011916Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3011944Z 
2019-08-20T20:33:24.3012005Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3012005Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3012034Z 
2019-08-20T20:33:24.3012905Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3013219Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3013254Z 
2019-08-20T20:33:24.3013254Z 
2019-08-20T20:33:24.3013615Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3013708Z 
2019-08-20T20:33:24.3013923Z ------------------------------------------
2019-08-20T20:33:24.3013956Z 
2019-08-20T20:33:24.3014009Z 
2019-08-20T20:33:24.3014009Z 
2019-08-20T20:33:24.3014254Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2019-08-20T20:33:24.3014290Z 
2019-08-20T20:33:24.3014535Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3014603Z status: exit code: 101
2019-08-20T20:33:24.3015589Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2019-08-20T20:33:24.3015951Z ------------------------------------------
2019-08-20T20:33:24.3016165Z 
2019-08-20T20:33:24.3016367Z ------------------------------------------
2019-08-20T20:33:24.3016411Z stderr:
2019-08-20T20:33:24.3016411Z stderr:
2019-08-20T20:33:24.3016617Z ------------------------------------------
2019-08-20T20:33:24.3016924Z thread 'rustc' panicked at 'index out of bounds: the len is 2861 but the index is 2910', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3017017Z 
2019-08-20T20:33:24.3017075Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3017102Z 
2019-08-20T20:33:24.3017143Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3017143Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3017172Z 
2019-08-20T20:33:24.3017577Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3017883Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3017934Z 
2019-08-20T20:33:24.3017934Z 
2019-08-20T20:33:24.3018300Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3018398Z 
2019-08-20T20:33:24.3018603Z ------------------------------------------
2019-08-20T20:33:24.3018633Z 
2019-08-20T20:33:24.3018657Z 
2019-08-20T20:33:24.3018657Z 
2019-08-20T20:33:24.3018880Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2019-08-20T20:33:24.3018932Z 
2019-08-20T20:33:24.3019161Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3019208Z status: exit code: 101
2019-08-20T20:33:24.3020339Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2019-08-20T20:33:24.3020979Z ------------------------------------------
2019-08-20T20:33:24.3021014Z 
2019-08-20T20:33:24.3021228Z ------------------------------------------
2019-08-20T20:33:24.3021303Z stderr:
2019-08-20T20:33:24.3021303Z stderr:
2019-08-20T20:33:24.3021519Z ------------------------------------------
2019-08-20T20:33:24.3022448Z thread 'rustc' panicked at 'index out of bounds: the len is 2576 but the index is 2630', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3022944Z 
2019-08-20T20:33:24.3022991Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3023038Z 
2019-08-20T20:33:24.3023085Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3023085Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3023115Z 
2019-08-20T20:33:24.3023491Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3023772Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3023806Z 
2019-08-20T20:33:24.3023806Z 
2019-08-20T20:33:24.3024218Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3024296Z 
2019-08-20T20:33:24.3024514Z ------------------------------------------
2019-08-20T20:33:24.3024564Z 
2019-08-20T20:33:24.3024591Z 
2019-08-20T20:33:24.3024591Z 
2019-08-20T20:33:24.3024820Z ---- [incremental] incremental/hashes/consts.rs stdout ----
2019-08-20T20:33:24.3024864Z 
2019-08-20T20:33:24.3025128Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3025178Z status: exit code: 101
2019-08-20T20:33:24.3026368Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/consts.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/auxiliary"
2019-08-20T20:33:24.3026771Z ------------------------------------------
2019-08-20T20:33:24.3026807Z 
2019-08-20T20:33:24.3027017Z ------------------------------------------
2019-08-20T20:33:24.3027062Z stderr:
2019-08-20T20:33:24.3027062Z stderr:
2019-08-20T20:33:24.3027282Z ------------------------------------------
2019-08-20T20:33:24.3027579Z thread 'rustc' panicked at 'index out of bounds: the len is 3493 but the index is 3493', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3027693Z 
2019-08-20T20:33:24.3027744Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3027773Z 
2019-08-20T20:33:24.3027816Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3027816Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3027863Z 
2019-08-20T20:33:24.3028168Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3028452Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3028566Z 
2019-08-20T20:33:24.3028566Z 
2019-08-20T20:33:24.3028965Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3029098Z 
2019-08-20T20:33:24.3029312Z ------------------------------------------
2019-08-20T20:33:24.3029343Z 
2019-08-20T20:33:24.3029367Z 
2019-08-20T20:33:24.3029367Z 
2019-08-20T20:33:24.3029607Z ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
2019-08-20T20:33:24.3029640Z 
2019-08-20T20:33:24.3029881Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3029931Z status: exit code: 101
2019-08-20T20:33:24.3030857Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
2019-08-20T20:33:24.3031210Z ------------------------------------------
2019-08-20T20:33:24.3031245Z 
2019-08-20T20:33:24.3031471Z ------------------------------------------
2019-08-20T20:33:24.3031515Z stderr:
2019-08-20T20:33:24.3031515Z stderr:
2019-08-20T20:33:24.3031718Z ------------------------------------------
2019-08-20T20:33:24.3032028Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6288', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3032462Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T20:33:24.3032818Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6293', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3035249Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6298', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3035585Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6517', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3036187Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6522', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3036555Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6527', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3036849Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6532', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3037172Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6537', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3037465Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6542', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3037756Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6547', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3038071Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6552', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3038365Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6553', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3038656Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6554', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3039072Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6563', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3039366Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6564', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3039675Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6565', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3039974Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6570', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3040268Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6575', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3040578Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6580', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3040877Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6581', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3041167Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6582', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3041477Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6588', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3041766Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6589', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3042446Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6590', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3042821Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6595', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3043131Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6596', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3043478Z thread 'rustc' panicked at 'index out of bounds: the len is 6284 but the index is 6597', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3043567Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3043597Z 
2019-08-20T20:33:24.3043660Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3043692Z 
2019-08-20T20:33:24.3043692Z 
2019-08-20T20:33:24.3044025Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3044442Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3044487Z 
2019-08-20T20:33:24.3044487Z 
2019-08-20T20:33:24.3044904Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3045016Z 
2019-08-20T20:33:24.3045237Z ------------------------------------------
2019-08-20T20:33:24.3045288Z 
2019-08-20T20:33:24.3045315Z 
2019-08-20T20:33:24.3045315Z 
2019-08-20T20:33:24.3045557Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2019-08-20T20:33:24.3045593Z 
2019-08-20T20:33:24.3046608Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3046667Z status: exit code: 101
2019-08-20T20:33:24.3047596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2019-08-20T20:33:24.3048092Z ------------------------------------------
2019-08-20T20:33:24.3048127Z 
2019-08-20T20:33:24.3048329Z ------------------------------------------
2019-08-20T20:33:24.3048372Z stderr:
2019-08-20T20:33:24.3048372Z stderr:
2019-08-20T20:33:24.3048583Z ------------------------------------------
2019-08-20T20:33:24.3048880Z thread 'rustc' panicked at 'index out of bounds: the len is 4807 but the index is 4807', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3048989Z 
2019-08-20T20:33:24.3049029Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3049065Z 
2019-08-20T20:33:24.3049107Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3049107Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3049152Z 
2019-08-20T20:33:24.3049455Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3049733Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3049767Z 
2019-08-20T20:33:24.3049767Z 
2019-08-20T20:33:24.3050127Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3050226Z 
2019-08-20T20:33:24.3050428Z ------------------------------------------
2019-08-20T20:33:24.3050458Z 
2019-08-20T20:33:24.3050483Z 
2019-08-20T20:33:24.3050483Z 
2019-08-20T20:33:24.3050716Z ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
2019-08-20T20:33:24.3050749Z 
2019-08-20T20:33:24.3050973Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3051029Z status: exit code: 101
2019-08-20T20:33:24.3052060Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
2019-08-20T20:33:24.3053068Z ------------------------------------------
2019-08-20T20:33:24.3053123Z 
2019-08-20T20:33:24.3053366Z ------------------------------------------
2019-08-20T20:33:24.3053414Z stderr:
2019-08-20T20:33:24.3053414Z stderr:
2019-08-20T20:33:24.3053625Z ------------------------------------------
2019-08-20T20:33:24.3053950Z thread 'rustc' panicked at 'index out of bounds: the len is 900 but the index is 900', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3054050Z 
2019-08-20T20:33:24.3054095Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3054145Z 
2019-08-20T20:33:24.3054197Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3054197Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3054228Z 
2019-08-20T20:33:24.3054566Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3054847Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3054881Z 
2019-08-20T20:33:24.3054881Z 
2019-08-20T20:33:24.3055283Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3055498Z 
2019-08-20T20:33:24.3055766Z ------------------------------------------
2019-08-20T20:33:24.3055799Z 
2019-08-20T20:33:24.3055825Z 
2019-08-20T20:33:24.3055825Z 
2019-08-20T20:33:24.3056063Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2019-08-20T20:33:24.3056098Z 
2019-08-20T20:33:24.3056366Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3056461Z status: exit code: 101
2019-08-20T20:33:24.3057741Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2019-08-20T20:33:24.3058070Z ------------------------------------------
2019-08-20T20:33:24.3058110Z 
2019-08-20T20:33:24.3058310Z ------------------------------------------
2019-08-20T20:33:24.3058370Z stderr:
2019-08-20T20:33:24.3058370Z stderr:
2019-08-20T20:33:24.3058566Z ------------------------------------------
2019-08-20T20:33:24.3058851Z thread 'rustc' panicked at 'index out of bounds: the len is 1655 but the index is 1655', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3058966Z 
2019-08-20T20:33:24.3059007Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3059034Z 
2019-08-20T20:33:24.3059093Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3059093Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3059121Z 
2019-08-20T20:33:24.3059413Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3059689Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3059720Z 
2019-08-20T20:33:24.3059720Z 
2019-08-20T20:33:24.3060155Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3060256Z 
2019-08-20T20:33:24.3060485Z ------------------------------------------
2019-08-20T20:33:24.3060516Z 
2019-08-20T20:33:24.3060565Z 
2019-08-20T20:33:24.3060565Z 
2019-08-20T20:33:24.3060793Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-08-20T20:33:24.3060825Z 
2019-08-20T20:33:24.3061053Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3061117Z status: exit code: 101
2019-08-20T20:33:24.3062603Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-08-20T20:33:24.3063177Z ------------------------------------------
2019-08-20T20:33:24.3063233Z 
2019-08-20T20:33:24.3063450Z ------------------------------------------
2019-08-20T20:33:24.3063497Z stderr:
2019-08-20T20:33:24.3063497Z stderr:
2019-08-20T20:33:24.3063726Z ------------------------------------------
2019-08-20T20:33:24.3064045Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 8026', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3064109Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T20:33:24.3064433Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 8236', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3064744Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 8241', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3065063Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9558', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3065392Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9562', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3065700Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9567', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3066194Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9571', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3066500Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9575', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3066801Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9579', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3067129Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9583', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3067430Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9588', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3067729Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9589', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3073215Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9594', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3073748Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9595', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3074141Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9601', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3074459Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9606', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3074783Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9607', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3075114Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9608', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3075424Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9613', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3075908Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9614', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3076220Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9615', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3076511Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9619', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3076928Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9624', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3077222Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9629', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3077512Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9630', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3077830Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9631', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3078121Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9638', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3078410Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9639', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3078729Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9640', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3079021Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9644', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3079328Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9645', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3079620Z thread 'rustc' panicked at 'index out of bounds: the len is 7411 but the index is 9646', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3079715Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3079761Z 
2019-08-20T20:33:24.3079805Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3079833Z 
2019-08-20T20:33:24.3079833Z 
2019-08-20T20:33:24.3080179Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3080457Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3080489Z 
2019-08-20T20:33:24.3080489Z 
2019-08-20T20:33:24.3080863Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3080937Z 
2019-08-20T20:33:24.3081155Z ------------------------------------------
2019-08-20T20:33:24.3081185Z 
2019-08-20T20:33:24.3081210Z 
2019-08-20T20:33:24.3081210Z 
2019-08-20T20:33:24.3081509Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2019-08-20T20:33:24.3081550Z 
2019-08-20T20:33:24.3081823Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3081873Z status: exit code: 101
2019-08-20T20:33:24.3083440Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2019-08-20T20:33:24.3083824Z ------------------------------------------
2019-08-20T20:33:24.3083859Z 
2019-08-20T20:33:24.3084074Z ------------------------------------------
2019-08-20T20:33:24.3084121Z stderr:
2019-08-20T20:33:24.3084121Z stderr:
2019-08-20T20:33:24.3084353Z ------------------------------------------
2019-08-20T20:33:24.3084662Z thread 'rustc' panicked at 'index out of bounds: the len is 10198 but the index is 10205', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3084897Z 
2019-08-20T20:33:24.3084941Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3084971Z 
2019-08-20T20:33:24.3085034Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3085034Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3085065Z 
2019-08-20T20:33:24.3085421Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3086142Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3086181Z 
2019-08-20T20:33:24.3086181Z 
2019-08-20T20:33:24.3086584Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3086685Z 
2019-08-20T20:33:24.3086895Z ------------------------------------------
2019-08-20T20:33:24.3086926Z 
2019-08-20T20:33:24.3086968Z 
2019-08-20T20:33:24.3086968Z 
2019-08-20T20:33:24.3087203Z ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
2019-08-20T20:33:24.3087236Z 
2019-08-20T20:33:24.3087467Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3087531Z status: exit code: 101
2019-08-20T20:33:24.3088493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
2019-08-20T20:33:24.3088843Z ------------------------------------------
2019-08-20T20:33:24.3088894Z 
2019-08-20T20:33:24.3089103Z ------------------------------------------
2019-08-20T20:33:24.3089147Z stderr:
2019-08-20T20:33:24.3089147Z stderr:
2019-08-20T20:33:24.3089348Z ------------------------------------------
2019-08-20T20:33:24.3089768Z thread 'rustc' panicked at 'index out of bounds: the len is 2493 but the index is 2527', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3089876Z 
2019-08-20T20:33:24.3089937Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3089966Z 
2019-08-20T20:33:24.3090017Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3090017Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3090046Z 
2019-08-20T20:33:24.3090400Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3090669Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3090719Z 
2019-08-20T20:33:24.3090719Z 
2019-08-20T20:33:24.3091090Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3091190Z 
2019-08-20T20:33:24.3091399Z ------------------------------------------
2019-08-20T20:33:24.3091430Z 
2019-08-20T20:33:24.3091455Z 
2019-08-20T20:33:24.3091455Z 
2019-08-20T20:33:24.3091681Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2019-08-20T20:33:24.3091733Z 
2019-08-20T20:33:24.3092138Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3092702Z status: exit code: 101
2019-08-20T20:33:24.3093758Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2019-08-20T20:33:24.3094108Z ------------------------------------------
2019-08-20T20:33:24.3094152Z 
2019-08-20T20:33:24.3094369Z ------------------------------------------
2019-08-20T20:33:24.3094433Z stderr:
2019-08-20T20:33:24.3094433Z stderr:
2019-08-20T20:33:24.3094644Z ------------------------------------------
2019-08-20T20:33:24.3094947Z thread 'rustc' panicked at 'index out of bounds: the len is 2509 but the index is 2882', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3095063Z 
2019-08-20T20:33:24.3095108Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3095160Z 
2019-08-20T20:33:24.3095213Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3095213Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3095244Z 
2019-08-20T20:33:24.3095564Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3096029Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3096060Z 
2019-08-20T20:33:24.3096060Z 
2019-08-20T20:33:24.3096443Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3096515Z 
2019-08-20T20:33:24.3096710Z ------------------------------------------
2019-08-20T20:33:24.3096756Z 
2019-08-20T20:33:24.3096780Z 
2019-08-20T20:33:24.3096780Z 
2019-08-20T20:33:24.3096994Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2019-08-20T20:33:24.3097026Z 
2019-08-20T20:33:24.3097352Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3097409Z status: exit code: 101
2019-08-20T20:33:24.3111776Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/chethread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-20T20:33:24.3112694Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-20T20:33:24.3114063Z ckout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2019-08-20T20:33:24.3114711Z ------------------------------------------
2019-08-20T20:33:24.3114872Z 
2019-08-20T20:33:24.3115255Z ------------------------------------------
2019-08-20T20:33:24.3115426Z stderr:
2019-08-20T20:33:24.3115426Z stderr:
2019-08-20T20:33:24.3115923Z ------------------------------------------
2019-08-20T20:33:24.3116609Z thread 'rustc' panicked at 'index out of bounds: the len is 1838 but the index is 1838', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3116917Z 
2019-08-20T20:33:24.3117069Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3117194Z 
2019-08-20T20:33:24.3117326Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3117326Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3117456Z 
2019-08-20T20:33:24.3117923Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3118486Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3118633Z 
2019-08-20T20:33:24.3118633Z 
2019-08-20T20:33:24.3119117Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3119422Z 
2019-08-20T20:33:24.3119746Z ------------------------------------------
2019-08-20T20:33:24.3119912Z 
2019-08-20T20:33:24.3120021Z 
2019-08-20T20:33:24.3120021Z 
2019-08-20T20:33:24.3120364Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2019-08-20T20:33:24.3120532Z 
2019-08-20T20:33:24.3120892Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3121056Z status: exit code: 101
2019-08-20T20:33:24.3122113Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2019-08-20T20:33:24.3123218Z ------------------------------------------
2019-08-20T20:33:24.3123383Z 
2019-08-20T20:33:24.3123749Z ------------------------------------------
2019-08-20T20:33:24.3123924Z stderr:
2019-08-20T20:33:24.3123924Z stderr:
2019-08-20T20:33:24.3124410Z ------------------------------------------
2019-08-20T20:33:24.3125009Z thread 'rustc' panicked at 'index out of bounds: the len is 6259 but the index is 7827', /checkout/src/libcore/slice/mod.rs:2715:10
2019-08-20T20:33:24.3125340Z 
2019-08-20T20:33:24.3125502Z error: internal compiler error: unexpected panic
2019-08-20T20:33:24.3125631Z 
2019-08-20T20:33:24.3126203Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3126203Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-20T20:33:24.3126358Z 
2019-08-20T20:33:24.3126839Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-20T20:33:24.3127388Z note: rustc 1.39.0-dev running on x86_64-unknown-linux-gnu
2019-08-20T20:33:24.3127534Z 
2019-08-20T20:33:24.3127534Z 
2019-08-20T20:33:24.3128031Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-20T20:33:24.3128336Z 
2019-08-20T20:33:24.3128657Z ------------------------------------------
2019-08-20T20:33:24.3128820Z 
2019-08-20T20:33:24.3128928Z 
2019-08-20T20:33:24.3128928Z 
2019-08-20T20:33:24.3129272Z ---- [incremental] incremental/hashes/let_expressions.rs stdout ----
2019-08-20T20:33:24.3129419Z 
2019-08-20T20:33:24.3129799Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-08-20T20:33:24.3130124Z status: exit code: 101
2019-08-20T20:33:24.3131311Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
---
2019-08-20T20:33:24.3448713Z test result: FAILED. 66 passed; 38 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-20T20:33:24.3448759Z 
2019-08-20T20:33:24.3448795Z 
2019-08-20T20:33:24.3448821Z 
2019-08-20T20:33:24.3450616Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-20T20:33:24.3452393Z 
2019-08-20T20:33:24.3452426Z 
2019-08-20T20:33:24.3452748Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-20T20:33:24.3452994Z Build completed unsuccessfully in 1:08:50
2019-08-20T20:33:24.3452994Z Build completed unsuccessfully in 1:08:50
2019-08-20T20:33:24.3453168Z == clock drift check ==
2019-08-20T20:33:24.3453234Z   local time: Tue Aug 20 20:33:24 UTC 2019
2019-08-20T20:33:24.4713546Z   network time: Tue, 20 Aug 2019 20:33:24 GMT
2019-08-20T20:33:24.4724279Z == end clock drift check ==
2019-08-20T20:33:28.2954119Z ##[error]Bash exited with code '1'.
2019-08-20T20:33:28.2994368Z ##[section]Starting: Checkout
2019-08-20T20:33:28.2996453Z ==============================================================================
2019-08-20T20:33:28.2996523Z Task         : Get sources
2019-08-20T20:33:28.2996570Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
