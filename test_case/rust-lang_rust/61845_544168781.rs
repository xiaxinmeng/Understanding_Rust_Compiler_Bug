plain
2019-10-19T15:11:25.3057587Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T15:11:25.3276482Z ##[command]git config gc.auto 0
2019-10-19T15:11:25.3375403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T15:11:25.3457500Z ##[command]git config --get-all http.proxy
2019-10-19T15:11:25.3622563Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61845/merge:refs/remotes/pull/61845/merge
---
2019-10-19T16:17:15.1795416Z .................................................................................................... 1600/9200
2019-10-19T16:17:20.9716938Z .................................................................................................... 1700/9200
2019-10-19T16:17:34.9086250Z ...............................i...............i.................................................... 1800/9200
2019-10-19T16:17:42.8324983Z .................................................................................................... 1900/9200
2019-10-19T16:17:57.7744625Z .....................iiiii.......................................................................... 2000/9200
2019-10-19T16:18:08.8825033Z .................................................................................................... 2200/9200
2019-10-19T16:18:11.6236000Z .................................................................................................... 2300/9200
2019-10-19T16:18:17.1030022Z .................................................................................................... 2400/9200
2019-10-19T16:18:40.3201938Z .................................................................................................... 2500/9200
---
2019-10-19T16:21:46.9940352Z ........................i...............i........................................................... 4800/9200
2019-10-19T16:21:59.6725172Z .................................................................................................... 4900/9200
2019-10-19T16:22:06.2653358Z .................................................................................................... 5000/9200
2019-10-19T16:22:16.1773971Z .................................................................................................... 5100/9200
2019-10-19T16:22:23.8645397Z ........................ii.ii....................................................................... 5200/9200
2019-10-19T16:22:34.7206992Z .................................................................................................... 5400/9200
2019-10-19T16:22:45.3365828Z ..........................................................................................i......... 5500/9200
2019-10-19T16:22:53.9966177Z .................................................................................................... 5600/9200
2019-10-19T16:22:59.1920830Z .................................................................................................... 5700/9200
2019-10-19T16:22:59.1920830Z .................................................................................................... 5700/9200
2019-10-19T16:23:10.5161910Z .......................................................................................ii...i..ii... 5800/9200
2019-10-19T16:23:38.0312322Z .................................................................................................... 6000/9200
2019-10-19T16:23:47.9547921Z .................................................................................................... 6100/9200
2019-10-19T16:23:55.0875367Z .................................................................................................... 6200/9200
2019-10-19T16:23:55.0875367Z .................................................................................................... 6200/9200
2019-10-19T16:24:09.5864565Z .........i..ii...................................................................................... 6300/9200
2019-10-19T16:24:30.5954637Z .....................................................................i.............................. 6500/9200
2019-10-19T16:24:32.9408714Z .................................................................................................... 6600/9200
2019-10-19T16:24:35.6204130Z ............................................i....................................................... 6700/9200
2019-10-19T16:24:39.5199814Z .................................................................................................... 6800/9200
---
2019-10-19T16:29:25.6374075Z  finished in 6.190
2019-10-19T16:29:25.6606702Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T16:29:25.8346053Z 
2019-10-19T16:29:25.8346295Z running 153 tests
2019-10-19T16:29:29.1717073Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-19T16:29:31.6757527Z i..iiii..............i.........iii.i.........ii......
2019-10-19T16:29:31.6766360Z 
2019-10-19T16:29:31.6766409Z  finished in 5.581
2019-10-19T16:29:31.6766749Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T16:29:31.6766789Z 
---
2019-10-19T16:29:33.6069710Z  finished in 2.345
2019-10-19T16:29:33.6293601Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T16:29:34.6766158Z 
2019-10-19T16:29:34.6766420Z running 9 tests
2019-10-19T16:29:34.6767343Z iiiiiiiii
2019-10-19T16:29:34.6767901Z 
2019-10-19T16:29:34.6768482Z  finished in 0.170
2019-10-19T16:29:34.6769646Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-19T16:29:34.6770203Z 
2019-10-19T16:29:34.6770203Z 
2019-10-19T16:29:34.6770642Z running 104 tests
2019-10-19T16:29:51.7093696Z .....F.........F.....FFFFFFFFFFFFFFFFFFFFFFFFFF...F.......F.F..........F.F...F....................F. 100/104
2019-10-19T16:29:52.3481991Z F.FF
2019-10-19T16:29:52.3509294Z 
2019-10-19T16:29:52.3509868Z ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
2019-10-19T16:29:52.3509966Z 
2019-10-19T16:29:52.3509966Z 
2019-10-19T16:29:52.3510017Z error in revision `rpass3`: compilation failed!
2019-10-19T16:29:52.3510077Z status: exit code: 101
2019-10-19T16:29:52.3511067Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
2019-10-19T16:29:52.3511430Z ------------------------------------------
2019-10-19T16:29:52.3511464Z 
2019-10-19T16:29:52.3512015Z ------------------------------------------
2019-10-19T16:29:52.3512095Z stderr:
2019-10-19T16:29:52.3512095Z stderr:
2019-10-19T16:29:52.3512345Z ------------------------------------------
2019-10-19T16:29:52.3512658Z thread 'rustc' panicked at 'index out of bounds: the len is 5658 but the index is 5658', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3512781Z 
2019-10-19T16:29:52.3512825Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3512856Z 
2019-10-19T16:29:52.3512919Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3512919Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3512950Z 
2019-10-19T16:29:52.3513373Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3513690Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3513725Z 
2019-10-19T16:29:52.3513725Z 
2019-10-19T16:29:52.3514079Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3514145Z 
2019-10-19T16:29:52.3514455Z thread 'rustc' panicked at 'index out of bounds: the len is 5658 but the index is 5661', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3514557Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3514587Z 
2019-10-19T16:29:52.3514634Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3514664Z 
2019-10-19T16:29:52.3514664Z 
2019-10-19T16:29:52.3515142Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3516450Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3516708Z 
2019-10-19T16:29:52.3516708Z 
2019-10-19T16:29:52.3517101Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3517283Z 
2019-10-19T16:29:52.3517542Z ------------------------------------------
2019-10-19T16:29:52.3517575Z 
2019-10-19T16:29:52.3517601Z 
2019-10-19T16:29:52.3517601Z 
2019-10-19T16:29:52.3517835Z ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
2019-10-19T16:29:52.3517870Z 
2019-10-19T16:29:52.3517932Z error in revision `rpass3`: compilation failed!
2019-10-19T16:29:52.3517979Z status: exit code: 101
2019-10-19T16:29:52.3518930Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
2019-10-19T16:29:52.3519286Z ------------------------------------------
2019-10-19T16:29:52.3519320Z 
2019-10-19T16:29:52.3519536Z ------------------------------------------
2019-10-19T16:29:52.3519583Z stderr:
2019-10-19T16:29:52.3519583Z stderr:
2019-10-19T16:29:52.3519811Z ------------------------------------------
2019-10-19T16:29:52.3520118Z thread 'rustc' panicked at 'index out of bounds: the len is 7143 but the index is 7143', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3520243Z 
2019-10-19T16:29:52.3520288Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3520319Z 
2019-10-19T16:29:52.3520388Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3520388Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3520420Z 
2019-10-19T16:29:52.3520750Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3521048Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3521081Z 
2019-10-19T16:29:52.3521081Z 
2019-10-19T16:29:52.3521436Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3521527Z 
2019-10-19T16:29:52.3521995Z ------------------------------------------
2019-10-19T16:29:52.3522033Z 
2019-10-19T16:29:52.3522059Z 
2019-10-19T16:29:52.3522059Z 
2019-10-19T16:29:52.3522319Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2019-10-19T16:29:52.3522366Z 
2019-10-19T16:29:52.3522610Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3522678Z status: exit code: 101
2019-10-19T16:29:52.3523659Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2019-10-19T16:29:52.3524151Z ------------------------------------------
2019-10-19T16:29:52.3524185Z 
2019-10-19T16:29:52.3524510Z ------------------------------------------
2019-10-19T16:29:52.3524564Z stderr:
2019-10-19T16:29:52.3524564Z stderr:
2019-10-19T16:29:52.3524802Z ------------------------------------------
2019-10-19T16:29:52.3525126Z thread 'rustc' panicked at 'index out of bounds: the len is 2818 but the index is 2818', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3525227Z 
2019-10-19T16:29:52.3525289Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3525320Z 
2019-10-19T16:29:52.3525365Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3525365Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3525396Z 
2019-10-19T16:29:52.3525736Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3526026Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3526077Z 
2019-10-19T16:29:52.3526077Z 
2019-10-19T16:29:52.3526918Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3526989Z 
2019-10-19T16:29:52.3527607Z ------------------------------------------
2019-10-19T16:29:52.3527634Z 
2019-10-19T16:29:52.3527657Z 
2019-10-19T16:29:52.3527657Z 
2019-10-19T16:29:52.3527871Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2019-10-19T16:29:52.3527917Z 
2019-10-19T16:29:52.3528461Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3528509Z status: exit code: 101
2019-10-19T16:29:52.3529584Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2019-10-19T16:29:52.3530068Z ------------------------------------------
2019-10-19T16:29:52.3530097Z 
2019-10-19T16:29:52.3530283Z ------------------------------------------
2019-10-19T16:29:52.3530575Z stderr:
2019-10-19T16:29:52.3530575Z stderr:
2019-10-19T16:29:52.3530934Z ------------------------------------------
2019-10-19T16:29:52.3531607Z thread 'rustc' panicked at 'index out of bounds: the len is 2559 but the index is 2617', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3531890Z 
2019-10-19T16:29:52.3532126Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3532157Z 
2019-10-19T16:29:52.3532221Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3532221Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3532251Z 
2019-10-19T16:29:52.3532608Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3532907Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3532940Z 
2019-10-19T16:29:52.3532940Z 
2019-10-19T16:29:52.3533319Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3533542Z 
2019-10-19T16:29:52.3533793Z ------------------------------------------
2019-10-19T16:29:52.3533842Z 
2019-10-19T16:29:52.3533868Z 
2019-10-19T16:29:52.3533868Z 
2019-10-19T16:29:52.3534175Z ---- [incremental] incremental/hashes/consts.rs stdout ----
2019-10-19T16:29:52.3534216Z 
2019-10-19T16:29:52.3534504Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3534555Z status: exit code: 101
2019-10-19T16:29:52.3535623Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/consts.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/auxiliary"
2019-10-19T16:29:52.3535984Z ------------------------------------------
2019-10-19T16:29:52.3536012Z 
2019-10-19T16:29:52.3536197Z ------------------------------------------
2019-10-19T16:29:52.3536255Z stderr:
2019-10-19T16:29:52.3536255Z stderr:
2019-10-19T16:29:52.3536439Z ------------------------------------------
2019-10-19T16:29:52.3536704Z thread 'rustc' panicked at 'index out of bounds: the len is 3514 but the index is 3514', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3536808Z 
2019-10-19T16:29:52.3536846Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3536873Z 
2019-10-19T16:29:52.3536932Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3536932Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3536966Z 
2019-10-19T16:29:52.3537242Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3537509Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3537538Z 
2019-10-19T16:29:52.3537538Z 
2019-10-19T16:29:52.3537869Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3537956Z 
2019-10-19T16:29:52.3538141Z ------------------------------------------
2019-10-19T16:29:52.3538189Z 
2019-10-19T16:29:52.3538212Z 
2019-10-19T16:29:52.3538212Z 
2019-10-19T16:29:52.3538418Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2019-10-19T16:29:52.3538449Z 
2019-10-19T16:29:52.3538657Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3538730Z status: exit code: 101
2019-10-19T16:29:52.3539583Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2019-10-19T16:29:52.3539886Z ------------------------------------------
2019-10-19T16:29:52.3540020Z 
2019-10-19T16:29:52.3540239Z ------------------------------------------
2019-10-19T16:29:52.3540280Z stderr:
2019-10-19T16:29:52.3540280Z stderr:
2019-10-19T16:29:52.3540482Z ------------------------------------------
2019-10-19T16:29:52.3540994Z thread 'rustc' panicked at 'index out of bounds: the len is 4786 but the index is 4786', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3541110Z 
2019-10-19T16:29:52.3541150Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3541176Z 
2019-10-19T16:29:52.3541218Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3541218Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3541263Z 
2019-10-19T16:29:52.3542129Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3542637Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3542703Z 
2019-10-19T16:29:52.3542703Z 
2019-10-19T16:29:52.3543116Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3543232Z 
2019-10-19T16:29:52.3543449Z ------------------------------------------
2019-10-19T16:29:52.3543480Z 
2019-10-19T16:29:52.3543507Z 
2019-10-19T16:29:52.3543507Z 
2019-10-19T16:29:52.3543755Z ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
2019-10-19T16:29:52.3543789Z 
2019-10-19T16:29:52.3544031Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3544081Z status: exit code: 101
2019-10-19T16:29:52.3545044Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
2019-10-19T16:29:52.3545393Z ------------------------------------------
2019-10-19T16:29:52.3545427Z 
2019-10-19T16:29:52.3545657Z ------------------------------------------
2019-10-19T16:29:52.3545703Z stderr:
2019-10-19T16:29:52.3545703Z stderr:
2019-10-19T16:29:52.3545912Z ------------------------------------------
2019-10-19T16:29:52.3546216Z thread 'rustc' panicked at 'index out of bounds: the len is 6328 but the index is 7734', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3546343Z 
2019-10-19T16:29:52.3546387Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3546436Z 
2019-10-19T16:29:52.3546488Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3546488Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3546519Z 
2019-10-19T16:29:52.3546851Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3547451Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3547480Z 
2019-10-19T16:29:52.3547480Z 
2019-10-19T16:29:52.3547831Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3547872Z 
2019-10-19T16:29:52.3548135Z thread 'rustc' panicked at 'index out of bounds: the len is 6328 but the index is 8588', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3548336Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3548362Z 
2019-10-19T16:29:52.3548400Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3548496Z 
2019-10-19T16:29:52.3548496Z 
2019-10-19T16:29:52.3548802Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3549065Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3549093Z 
2019-10-19T16:29:52.3549093Z 
2019-10-19T16:29:52.3549424Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3549504Z 
2019-10-19T16:29:52.3549688Z ------------------------------------------
2019-10-19T16:29:52.3549716Z 
2019-10-19T16:29:52.3549738Z 
2019-10-19T16:29:52.3549738Z 
2019-10-19T16:29:52.3549960Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2019-10-19T16:29:52.3549999Z 
2019-10-19T16:29:52.3550211Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3550254Z status: exit code: 101
2019-10-19T16:29:52.3551116Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2019-10-19T16:29:52.3551423Z ------------------------------------------
2019-10-19T16:29:52.3551453Z 
2019-10-19T16:29:52.3551845Z ------------------------------------------
2019-10-19T16:29:52.3552068Z stderr:
2019-10-19T16:29:52.3552068Z stderr:
2019-10-19T16:29:52.3552315Z ------------------------------------------
2019-10-19T16:29:52.3552619Z thread 'rustc' panicked at 'index out of bounds: the len is 507 but the index is 507', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3552742Z 
2019-10-19T16:29:52.3552785Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3552834Z 
2019-10-19T16:29:52.3552880Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3552880Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3552911Z 
2019-10-19T16:29:52.3553240Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3553527Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3553561Z 
2019-10-19T16:29:52.3553561Z 
2019-10-19T16:29:52.3553967Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3554043Z 
2019-10-19T16:29:52.3554274Z ------------------------------------------
2019-10-19T16:29:52.3554308Z 
2019-10-19T16:29:52.3554334Z 
2019-10-19T16:29:52.3554334Z 
2019-10-19T16:29:52.3554568Z ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
2019-10-19T16:29:52.3554601Z 
2019-10-19T16:29:52.3554862Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3554912Z status: exit code: 101
2019-10-19T16:29:52.3555956Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
2019-10-19T16:29:52.3556467Z ------------------------------------------
2019-10-19T16:29:52.3556502Z 
2019-10-19T16:29:52.3556717Z ------------------------------------------
2019-10-19T16:29:52.3556762Z stderr:
2019-10-19T16:29:52.3556762Z stderr:
2019-10-19T16:29:52.3556991Z ------------------------------------------
2019-10-19T16:29:52.3557295Z thread 'rustc' panicked at 'index out of bounds: the len is 896 but the index is 896', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3557429Z 
2019-10-19T16:29:52.3557474Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3557504Z 
2019-10-19T16:29:52.3557762Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3557762Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3557792Z 
2019-10-19T16:29:52.3558398Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3558666Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3558696Z 
2019-10-19T16:29:52.3558696Z 
2019-10-19T16:29:52.3559036Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3559129Z 
2019-10-19T16:29:52.3559323Z ------------------------------------------
2019-10-19T16:29:52.3559351Z 
2019-10-19T16:29:52.3559391Z 
2019-10-19T16:29:52.3559391Z 
2019-10-19T16:29:52.3559613Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-10-19T16:29:52.3559644Z 
2019-10-19T16:29:52.3559858Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-10-19T16:29:52.3559921Z status: exit code: 101
2019-10-19T16:29:52.3560811Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-10-19T16:29:52.3561130Z ------------------------------------------
2019-10-19T16:29:52.3561179Z 
2019-10-19T16:29:52.3561371Z ------------------------------------------
2019-10-19T16:29:52.3561412Z stderr:
2019-10-19T16:29:52.3561412Z stderr:
2019-10-19T16:29:52.3561601Z ------------------------------------------
2019-10-19T16:29:52.3562246Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 7923', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3562356Z 
2019-10-19T16:29:52.3562422Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3562451Z 
2019-10-19T16:29:52.3562605Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3562605Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3562635Z 
2019-10-19T16:29:52.3563149Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3563543Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3563596Z 
2019-10-19T16:29:52.3563596Z 
2019-10-19T16:29:52.3563979Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3564029Z 
2019-10-19T16:29:52.3564352Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 8136', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3564437Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3564467Z 
2019-10-19T16:29:52.3564530Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3564571Z 
2019-10-19T16:29:52.3564571Z 
2019-10-19T16:29:52.3564878Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3565183Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3565216Z 
2019-10-19T16:29:52.3565216Z 
2019-10-19T16:29:52.3565594Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3576062Z 
2019-10-19T16:29:52.3576574Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 8141', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3576686Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3576716Z 
2019-10-19T16:29:52.3576757Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3576784Z 
2019-10-19T16:29:52.3576784Z 
2019-10-19T16:29:52.3577132Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3577411Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3577448Z 
2019-10-19T16:29:52.3577448Z 
2019-10-19T16:29:52.3577808Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3577850Z 
2019-10-19T16:29:52.3578112Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9459', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3578205Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3578234Z 
2019-10-19T16:29:52.3578274Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3578318Z 
2019-10-19T16:29:52.3578318Z 
2019-10-19T16:29:52.3578586Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3578857Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3578887Z 
2019-10-19T16:29:52.3578887Z 
2019-10-19T16:29:52.3579227Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3579268Z 
2019-10-19T16:29:52.3579551Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9463', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3579626Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3579654Z 
2019-10-19T16:29:52.3579712Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3579739Z 
2019-10-19T16:29:52.3579739Z 
2019-10-19T16:29:52.3580004Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3580469Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3580500Z 
2019-10-19T16:29:52.3580500Z 
2019-10-19T16:29:52.3580915Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3580983Z 
2019-10-19T16:29:52.3581989Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9468', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3582103Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3582134Z 
2019-10-19T16:29:52.3582180Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3582210Z 
2019-10-19T16:29:52.3582210Z 
2019-10-19T16:29:52.3582582Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3582866Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3582912Z 
2019-10-19T16:29:52.3582912Z 
2019-10-19T16:29:52.3583324Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3583370Z 
2019-10-19T16:29:52.3583671Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9472', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3583773Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3583803Z 
2019-10-19T16:29:52.3583849Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3583898Z 
2019-10-19T16:29:52.3583898Z 
2019-10-19T16:29:52.3584201Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3584495Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3584528Z 
2019-10-19T16:29:52.3584528Z 
2019-10-19T16:29:52.3584911Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3584974Z 
2019-10-19T16:29:52.3585420Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9476', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3585656Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3585682Z 
2019-10-19T16:29:52.3585738Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3585763Z 
2019-10-19T16:29:52.3585763Z 
2019-10-19T16:29:52.3586018Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3586269Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3586297Z 
2019-10-19T16:29:52.3586297Z 
2019-10-19T16:29:52.3586633Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3586680Z 
2019-10-19T16:29:52.3586944Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9480', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3587033Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3587060Z 
2019-10-19T16:29:52.3587097Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3587123Z 
2019-10-19T16:29:52.3587123Z 
2019-10-19T16:29:52.3587569Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3588033Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3588084Z 
2019-10-19T16:29:52.3588084Z 
2019-10-19T16:29:52.3588464Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3588630Z 
2019-10-19T16:29:52.3589037Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9484', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3589151Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3589182Z 
2019-10-19T16:29:52.3589227Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3589275Z 
2019-10-19T16:29:52.3589275Z 
2019-10-19T16:29:52.3589611Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3589911Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3589944Z 
2019-10-19T16:29:52.3589944Z 
2019-10-19T16:29:52.3590328Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3590401Z 
2019-10-19T16:29:52.3590706Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9489', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3590799Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3590848Z 
2019-10-19T16:29:52.3590894Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3590924Z 
2019-10-19T16:29:52.3590924Z 
2019-10-19T16:29:52.3591245Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3591520Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3591554Z 
2019-10-19T16:29:52.3591554Z 
2019-10-19T16:29:52.3592212Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3592265Z 
2019-10-19T16:29:52.3592586Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9490', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3592697Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3592727Z 
2019-10-19T16:29:52.3592772Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3592802Z 
2019-10-19T16:29:52.3592802Z 
2019-10-19T16:29:52.3593134Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3593409Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3593461Z 
2019-10-19T16:29:52.3593461Z 
2019-10-19T16:29:52.3593843Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3593888Z 
2019-10-19T16:29:52.3594207Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9495', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3594301Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3594331Z 
2019-10-19T16:29:52.3594401Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3594432Z 
2019-10-19T16:29:52.3594432Z 
2019-10-19T16:29:52.3594736Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3595031Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3595064Z 
2019-10-19T16:29:52.3595064Z 
2019-10-19T16:29:52.3595442Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3595506Z 
2019-10-19T16:29:52.3595807Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9496', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3596006Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3596057Z 
2019-10-19T16:29:52.3596162Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3596198Z 
2019-10-19T16:29:52.3596198Z 
2019-10-19T16:29:52.3596555Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3596835Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3596869Z 
2019-10-19T16:29:52.3596869Z 
2019-10-19T16:29:52.3597267Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3597313Z 
2019-10-19T16:29:52.3597613Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9502', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3597714Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3597753Z 
2019-10-19T16:29:52.3597984Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3598016Z 
2019-10-19T16:29:52.3598016Z 
2019-10-19T16:29:52.3598536Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3598792Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3598842Z 
2019-10-19T16:29:52.3598842Z 
2019-10-19T16:29:52.3599349Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3599391Z 
2019-10-19T16:29:52.3599677Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9507', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3599752Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3599788Z 
2019-10-19T16:29:52.3599845Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3599872Z 
2019-10-19T16:29:52.3599872Z 
2019-10-19T16:29:52.3600464Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-19T16:29:52.3600717Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-19T16:29:52.3600746Z 
2019-10-19T16:29:52.3600746Z 
2019-10-19T16:29:52.3601065Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-19T16:29:52.3601122Z 
2019-10-19T16:29:52.3601377Z thread 'rustc' panicked at 'index out of bounds: the len is 7332 but the index is 9508', /checkout/src/libcore/slice/mod.rs:2716:10
2019-10-19T16:29:52.3601448Z error: internal compiler error: unexpected panic
2019-10-19T16:29:52.3601491Z 
2019-10-19T16:29:52.3601529Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-19T16:29:52.3601563Z 
---
2019-10-19T16:29:52.4002743Z test result: FAILED. 66 passed; 38 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-19T16:29:52.4002782Z 
2019-10-19T16:29:52.4002955Z 
2019-10-19T16:29:52.4003040Z 
2019-10-19T16:29:52.4004746Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-19T16:29:52.4005101Z 
2019-10-19T16:29:52.4005132Z 
2019-10-19T16:29:52.4005180Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-19T16:29:52.4005232Z Build completed unsuccessfully in 1:11:42
2019-10-19T16:29:52.4005232Z Build completed unsuccessfully in 1:11:42
2019-10-19T16:29:52.4006157Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-19T16:29:52.4006498Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-19T16:29:52.4006623Z == clock drift check ==
2019-10-19T16:29:52.4006661Z   local time: Sat Oct 19 16:29:52 UTC 2019
2019-10-19T16:29:52.6474877Z   network time: Sat, 19 Oct 2019 16:29:52 GMT
2019-10-19T16:29:52.6478477Z == end clock drift check ==
2019-10-19T16:29:58.9666701Z 
2019-10-19T16:29:58.9752314Z ##[error]Bash exited with code '1'.
2019-10-19T16:29:58.9820903Z ##[section]Starting: Checkout
2019-10-19T16:29:58.9823808Z ==============================================================================
2019-10-19T16:29:58.9823864Z Task         : Get sources
2019-10-19T16:29:58.9823913Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
