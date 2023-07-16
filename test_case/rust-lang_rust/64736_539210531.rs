plain
2019-10-07T20:12:08.6268394Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T20:12:08.6456458Z ##[command]git config gc.auto 0
2019-10-07T20:12:08.6538309Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T20:12:08.6575454Z ##[command]git config --get-all http.proxy
2019-10-07T20:12:08.6742162Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-07T21:11:52.4679135Z .................................................................................................... 1600/9123
2019-10-07T21:12:00.6202076Z .................................................................................................... 1700/9123
2019-10-07T21:12:11.6628402Z ..............i...............i..................................................................... 1800/9123
2019-10-07T21:12:17.9981781Z .................................................................................................... 1900/9123
2019-10-07T21:12:32.9625759Z .....iiiii.......................................................................................... 2000/9123
2019-10-07T21:12:42.4153070Z .................................................................................................... 2200/9123
2019-10-07T21:12:44.9833121Z .................................................................................................... 2300/9123
2019-10-07T21:12:50.7992875Z .................................................................................................... 2400/9123
2019-10-07T21:12:56.7023766Z .................................................................................................... 2500/9123
---
2019-10-07T21:15:50.7749460Z ..............................................................................................i..... 4700/9123
2019-10-07T21:15:58.2329555Z ..........i......................................................................................... 4800/9123
2019-10-07T21:16:09.1933631Z .................................................................................................... 4900/9123
2019-10-07T21:16:14.8137445Z .................................................................................................... 5000/9123
2019-10-07T21:16:26.8484239Z ........................................................................................ii.ii....... 5100/9123
2019-10-07T21:16:36.2960749Z .................................................................................................... 5300/9123
2019-10-07T21:16:45.7692002Z .................................................................................................... 5400/9123
2019-10-07T21:16:52.8072631Z ......................................................i............................................. 5500/9123
2019-10-07T21:16:59.7191915Z .................................................................................................... 5600/9123
2019-10-07T21:16:59.7191915Z .................................................................................................... 5600/9123
2019-10-07T21:17:08.2389315Z .................................................................................................... 5700/9123
2019-10-07T21:17:17.4514402Z ...................................................ii...i..ii...........i........................... 5800/9123
2019-10-07T21:17:42.9496460Z .................................................................................................... 6000/9123
2019-10-07T21:17:52.1639916Z .................................................................................................... 6100/9123
2019-10-07T21:17:52.1639916Z .................................................................................................... 6100/9123
2019-10-07T21:17:59.7211029Z .........................................................i..ii...................................... 6200/9123
2019-10-07T21:18:27.5214789Z .................................................................................................... 6400/9123
2019-10-07T21:18:29.6389704Z .................i.................................................................................. 6500/9123
2019-10-07T21:18:31.8515856Z ..........................................................................................i......... 6600/9123
2019-10-07T21:18:34.6785280Z .................................................................................................... 6700/9123
---
2019-10-07T21:23:07.7612840Z  finished in 5.410
2019-10-07T21:23:07.7795763Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-07T21:23:07.9425712Z 
2019-10-07T21:23:07.9426925Z running 150 tests
2019-10-07T21:23:11.1674791Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-10-07T21:23:13.1077856Z ..iiii..............i.........iii.i.......ii......
2019-10-07T21:23:13.1079521Z 
2019-10-07T21:23:13.1084507Z  finished in 5.329
2019-10-07T21:23:13.1371606Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-07T21:23:13.2993309Z 
---
2019-10-07T21:23:15.3903669Z  finished in 2.253
2019-10-07T21:23:15.4106562Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-07T21:23:15.5714311Z 
2019-10-07T21:23:15.5714700Z running 9 tests
2019-10-07T21:23:15.5715632Z iiiiiiiii
2019-10-07T21:23:15.5716046Z 
2019-10-07T21:23:15.5719790Z  finished in 0.161
2019-10-07T21:23:15.5914985Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-07T21:23:15.7588148Z 
2019-10-07T21:23:15.7588148Z 
2019-10-07T21:23:15.7588449Z running 104 tests
2019-10-07T21:23:31.2810103Z ..F.F..FFFFFF.F.....F.F.F...FFF.F..F..F...F.F.F..............F...F..F.F......FF.......FFF........... 100/104
2019-10-07T21:23:31.9099221Z failures:
2019-10-07T21:23:31.9137457Z 
2019-10-07T21:23:31.9138379Z ---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----
2019-10-07T21:23:31.9138588Z 
2019-10-07T21:23:31.9138588Z 
2019-10-07T21:23:31.9139005Z error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" failed to compile: 
2019-10-07T21:23:31.9139264Z status: exit code: 101
2019-10-07T21:23:31.9140599Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
2019-10-07T21:23:31.9148108Z ------------------------------------------
2019-10-07T21:23:31.9148154Z 
2019-10-07T21:23:31.9148434Z ------------------------------------------
2019-10-07T21:23:31.9148506Z stderr:
2019-10-07T21:23:31.9148506Z stderr:
2019-10-07T21:23:31.9148746Z ------------------------------------------
2019-10-07T21:23:31.9148797Z warning: unused variable: `x`
2019-10-07T21:23:31.9149085Z   --> /checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs:13:18
2019-10-07T21:23:31.9149161Z    |
2019-10-07T21:23:31.9149210Z LL | pub fn function1(x: u32) {
2019-10-07T21:23:31.9149261Z    |                  ^ help: consider prefixing with an underscore: `_x`
2019-10-07T21:23:31.9149373Z    = note: `#[warn(unused_variables)]` on by default
2019-10-07T21:23:31.9149424Z 
2019-10-07T21:23:31.9149424Z 
2019-10-07T21:23:31.9149864Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/callee_caller_cross_crate/auxiliary/a.rs:13:1: 14:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9149975Z 
2019-10-07T21:23:31.9150037Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9150068Z 
2019-10-07T21:23:31.9150115Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9150115Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9150145Z 
2019-10-07T21:23:31.9150614Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9150928Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9150961Z 
2019-10-07T21:23:31.9150961Z 
2019-10-07T21:23:31.9151359Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2019-10-07T21:23:31.9151439Z 
2019-10-07T21:23:31.9151851Z ------------------------------------------
2019-10-07T21:23:31.9151884Z 
2019-10-07T21:23:31.9151908Z 
2019-10-07T21:23:31.9151908Z 
2019-10-07T21:23:31.9152152Z ---- [incremental] incremental/change_crate_dep_kind.rs stdout ----
2019-10-07T21:23:31.9152186Z 
2019-10-07T21:23:31.9152443Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9152492Z status: exit code: 101
2019-10-07T21:23:31.9153602Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_crate_dep_kind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/change_crate_dep_kind.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Cpanic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_crate_dep_kind/auxiliary"
2019-10-07T21:23:31.9154072Z ------------------------------------------
2019-10-07T21:23:31.9154106Z 
2019-10-07T21:23:31.9154328Z ------------------------------------------
2019-10-07T21:23:31.9154389Z stderr:
2019-10-07T21:23:31.9154389Z stderr:
2019-10-07T21:23:31.9154607Z ------------------------------------------
2019-10-07T21:23:31.9155116Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_crate_dep_kind.rs:14:1: 14:13', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9155242Z 
2019-10-07T21:23:31.9155282Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9155467Z 
2019-10-07T21:23:31.9155531Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9155531Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9155560Z 
2019-10-07T21:23:31.9155868Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9156733Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9156771Z 
2019-10-07T21:23:31.9156771Z 
2019-10-07T21:23:31.9157160Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 -C panic=unwind
2019-10-07T21:23:31.9157254Z 
2019-10-07T21:23:31.9157477Z ------------------------------------------
2019-10-07T21:23:31.9157510Z 
2019-10-07T21:23:31.9157556Z 
2019-10-07T21:23:31.9157556Z 
2019-10-07T21:23:31.9157814Z ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
2019-10-07T21:23:31.9157862Z 
2019-10-07T21:23:31.9158123Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9158201Z status: exit code: 101
2019-10-07T21:23:31.9159176Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
2019-10-07T21:23:31.9159537Z ------------------------------------------
2019-10-07T21:23:31.9159588Z 
2019-10-07T21:23:31.9160065Z ------------------------------------------
2019-10-07T21:23:31.9160108Z stderr:
2019-10-07T21:23:31.9160108Z stderr:
2019-10-07T21:23:31.9160327Z ------------------------------------------
2019-10-07T21:23:31.9160977Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_private_fn/struct_point.rs:36:9: 38:10', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9161115Z 
2019-10-07T21:23:31.9161156Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9161182Z 
2019-10-07T21:23:31.9161223Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9161223Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9162519Z 
2019-10-07T21:23:31.9162924Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9163389Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9163421Z 
2019-10-07T21:23:31.9163421Z 
2019-10-07T21:23:31.9163832Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9163923Z 
2019-10-07T21:23:31.9164148Z ------------------------------------------
2019-10-07T21:23:31.9164177Z 
2019-10-07T21:23:31.9164200Z 
2019-10-07T21:23:31.9164200Z 
2019-10-07T21:23:31.9164443Z ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
2019-10-07T21:23:31.9164475Z 
2019-10-07T21:23:31.9164527Z error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" failed to compile: 
2019-10-07T21:23:31.9165360Z status: exit code: 101
2019-10-07T21:23:31.9167006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
2019-10-07T21:23:31.9167417Z ------------------------------------------
2019-10-07T21:23:31.9167453Z 
2019-10-07T21:23:31.9167708Z ------------------------------------------
2019-10-07T21:23:31.9167754Z stderr:
2019-10-07T21:23:31.9167754Z stderr:
2019-10-07T21:23:31.9167973Z ------------------------------------------
2019-10-07T21:23:31.9168379Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_private_fn_cc/auxiliary/point.rs:15:5: 17:6', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9168486Z 
2019-10-07T21:23:31.9168547Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9168577Z 
2019-10-07T21:23:31.9168623Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9168623Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9168654Z 
2019-10-07T21:23:31.9169017Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9169306Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9169340Z 
2019-10-07T21:23:31.9169340Z 
2019-10-07T21:23:31.9169733Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2019-10-07T21:23:31.9169811Z 
2019-10-07T21:23:31.9170199Z ------------------------------------------
2019-10-07T21:23:31.9170228Z 
2019-10-07T21:23:31.9170251Z 
2019-10-07T21:23:31.9170251Z 
2019-10-07T21:23:31.9170485Z ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
2019-10-07T21:23:31.9170517Z 
2019-10-07T21:23:31.9170758Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9170803Z status: exit code: 101
2019-10-07T21:23:31.9171861Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
2019-10-07T21:23:31.9172286Z ------------------------------------------
2019-10-07T21:23:31.9172317Z 
2019-10-07T21:23:31.9172518Z ------------------------------------------
2019-10-07T21:23:31.9172576Z stderr:
2019-10-07T21:23:31.9172576Z stderr:
2019-10-07T21:23:31.9172773Z ------------------------------------------
2019-10-07T21:23:31.9173116Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_private_impl_method/struct_point.rs:36:9: 38:10', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9173238Z 
2019-10-07T21:23:31.9173288Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9173314Z 
2019-10-07T21:23:31.9173371Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9173371Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9173399Z 
2019-10-07T21:23:31.9173693Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9173969Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9173999Z 
2019-10-07T21:23:31.9173999Z 
2019-10-07T21:23:31.9174509Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9174592Z 
2019-10-07T21:23:31.9174808Z ------------------------------------------
2019-10-07T21:23:31.9174838Z 
2019-10-07T21:23:31.9174878Z 
2019-10-07T21:23:31.9174878Z 
2019-10-07T21:23:31.9175126Z ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
2019-10-07T21:23:31.9175168Z 
2019-10-07T21:23:31.9175401Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9175464Z status: exit code: 101
2019-10-07T21:23:31.9177091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
2019-10-07T21:23:31.9177504Z ------------------------------------------
2019-10-07T21:23:31.9177557Z 
2019-10-07T21:23:31.9177785Z ------------------------------------------
2019-10-07T21:23:31.9177831Z stderr:
2019-10-07T21:23:31.9177831Z stderr:
2019-10-07T21:23:31.9178048Z ------------------------------------------
2019-10-07T21:23:31.9178456Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs:35:9: 37:10', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9178577Z 
2019-10-07T21:23:31.9178747Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9178776Z 
2019-10-07T21:23:31.9178821Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9178821Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9178868Z 
2019-10-07T21:23:31.9179316Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9179650Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9179684Z 
2019-10-07T21:23:31.9179684Z 
2019-10-07T21:23:31.9180203Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9180282Z 
2019-10-07T21:23:31.9180481Z ------------------------------------------
2019-10-07T21:23:31.9180510Z 
2019-10-07T21:23:31.9180536Z 
2019-10-07T21:23:31.9180536Z 
2019-10-07T21:23:31.9180772Z ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
2019-10-07T21:23:31.9180831Z 
2019-10-07T21:23:31.9180885Z error in revision `cfail2`: auxiliary build of "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" failed to compile: 
2019-10-07T21:23:31.9180933Z status: exit code: 101
2019-10-07T21:23:31.9181884Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
2019-10-07T21:23:31.9182222Z ------------------------------------------
2019-10-07T21:23:31.9182259Z 
2019-10-07T21:23:31.9182482Z ------------------------------------------
2019-10-07T21:23:31.9182523Z stderr:
2019-10-07T21:23:31.9182523Z stderr:
2019-10-07T21:23:31.9182721Z ------------------------------------------
2019-10-07T21:23:31.9183085Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_private_impl_method_cc/auxiliary/point.rs:15:5: 17:6', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9183182Z 
2019-10-07T21:23:31.9183238Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9183265Z 
2019-10-07T21:23:31.9183305Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9183305Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9183340Z 
2019-10-07T21:23:31.9183649Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9183913Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9183944Z 
2019-10-07T21:23:31.9183944Z 
2019-10-07T21:23:31.9184288Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
2019-10-07T21:23:31.9184351Z 
2019-10-07T21:23:31.9184564Z ------------------------------------------
2019-10-07T21:23:31.9184594Z 
2019-10-07T21:23:31.9184617Z 
2019-10-07T21:23:31.9184617Z 
2019-10-07T21:23:31.9184855Z ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
2019-10-07T21:23:31.9184887Z 
2019-10-07T21:23:31.9185130Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9185175Z status: exit code: 101
2019-10-07T21:23:31.9186901Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
2019-10-07T21:23:31.9187328Z ------------------------------------------
2019-10-07T21:23:31.9187372Z 
2019-10-07T21:23:31.9187602Z ------------------------------------------
2019-10-07T21:23:31.9187665Z stderr:
2019-10-07T21:23:31.9187665Z stderr:
2019-10-07T21:23:31.9188192Z ------------------------------------------
2019-10-07T21:23:31.9188609Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs:45:9: 47:10', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9188739Z 
2019-10-07T21:23:31.9188784Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9188813Z 
2019-10-07T21:23:31.9188876Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9188876Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9188906Z 
2019-10-07T21:23:31.9189244Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9190002Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9190058Z 
2019-10-07T21:23:31.9190058Z 
2019-10-07T21:23:31.9190749Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9190846Z 
2019-10-07T21:23:31.9191059Z ------------------------------------------
2019-10-07T21:23:31.9191089Z 
2019-10-07T21:23:31.9191129Z 
2019-10-07T21:23:31.9191129Z 
2019-10-07T21:23:31.9191347Z ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
2019-10-07T21:23:31.9191378Z 
2019-10-07T21:23:31.9191419Z error in revision `rpass2`: compilation failed!
2019-10-07T21:23:31.9191476Z status: exit code: 101
2019-10-07T21:23:31.9192332Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
2019-10-07T21:23:31.9192662Z ------------------------------------------
2019-10-07T21:23:31.9192693Z 
2019-10-07T21:23:31.9192918Z ------------------------------------------
2019-10-07T21:23:31.9192960Z stderr:
2019-10-07T21:23:31.9192960Z stderr:
2019-10-07T21:23:31.9193154Z ------------------------------------------
2019-10-07T21:23:31.9193504Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/crate_hash_reorder.rs:29:1: 29:18', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9193738Z 
2019-10-07T21:23:31.9193797Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9193824Z 
2019-10-07T21:23:31.9193931Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9193931Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9193964Z 
2019-10-07T21:23:31.9194321Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9194582Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9194629Z 
2019-10-07T21:23:31.9194629Z 
2019-10-07T21:23:31.9195007Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9195073Z 
2019-10-07T21:23:31.9195293Z ------------------------------------------
2019-10-07T21:23:31.9195323Z 
2019-10-07T21:23:31.9195357Z 
2019-10-07T21:23:31.9195357Z 
2019-10-07T21:23:31.9195583Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2019-10-07T21:23:31.9195637Z 
2019-10-07T21:23:31.9196085Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9196141Z status: exit code: 101
2019-10-07T21:23:31.9197599Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2019-10-07T21:23:31.9197977Z ------------------------------------------
2019-10-07T21:23:31.9198019Z 
2019-10-07T21:23:31.9198266Z ------------------------------------------
2019-10-07T21:23:31.9198312Z stderr:
2019-10-07T21:23:31.9198312Z stderr:
2019-10-07T21:23:31.9198532Z ------------------------------------------
2019-10-07T21:23:31.9198916Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/call_expressions.rs:17:1: 17:32', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9199020Z 
2019-10-07T21:23:31.9199082Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9199111Z 
2019-10-07T21:23:31.9199158Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9199158Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9199197Z 
2019-10-07T21:23:31.9199547Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9199835Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9199876Z 
2019-10-07T21:23:31.9199876Z 
2019-10-07T21:23:31.9200477Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9200548Z 
2019-10-07T21:23:31.9200768Z ------------------------------------------
2019-10-07T21:23:31.9200798Z 
2019-10-07T21:23:31.9200824Z 
2019-10-07T21:23:31.9200824Z 
2019-10-07T21:23:31.9201062Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2019-10-07T21:23:31.9201112Z 
2019-10-07T21:23:31.9201355Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9201402Z status: exit code: 101
2019-10-07T21:23:31.9202467Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2019-10-07T21:23:31.9202902Z ------------------------------------------
2019-10-07T21:23:31.9202933Z 
2019-10-07T21:23:31.9203143Z ------------------------------------------
2019-10-07T21:23:31.9203213Z stderr:
2019-10-07T21:23:31.9203213Z stderr:
2019-10-07T21:23:31.9203418Z ------------------------------------------
2019-10-07T21:23:31.9203768Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/closure_expressions.rs:19:1: 21:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9203883Z 
2019-10-07T21:23:31.9203923Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9203969Z 
2019-10-07T21:23:31.9204012Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9204012Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9204040Z 
2019-10-07T21:23:31.9204361Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9204628Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9204659Z 
2019-10-07T21:23:31.9204659Z 
2019-10-07T21:23:31.9205034Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9205117Z 
2019-10-07T21:23:31.9205329Z ------------------------------------------
2019-10-07T21:23:31.9205377Z 
2019-10-07T21:23:31.9205404Z 
2019-10-07T21:23:31.9205404Z 
2019-10-07T21:23:31.9205637Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2019-10-07T21:23:31.9205671Z 
2019-10-07T21:23:31.9206556Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9206618Z status: exit code: 101
2019-10-07T21:23:31.9207627Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2019-10-07T21:23:31.9208010Z ------------------------------------------
2019-10-07T21:23:31.9208043Z 
2019-10-07T21:23:31.9208269Z ------------------------------------------
2019-10-07T21:23:31.9208332Z stderr:
2019-10-07T21:23:31.9208332Z stderr:
2019-10-07T21:23:31.9208556Z ------------------------------------------
2019-10-07T21:23:31.9208922Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/enum_constructors.rs:51:1: 57:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9209174Z 
2019-10-07T21:23:31.9209284Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9209319Z 
2019-10-07T21:23:31.9209381Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9209381Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9209412Z 
2019-10-07T21:23:31.9209906Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9210187Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9210216Z 
2019-10-07T21:23:31.9210216Z 
2019-10-07T21:23:31.9210565Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9210652Z 
2019-10-07T21:23:31.9210853Z ------------------------------------------
2019-10-07T21:23:31.9210893Z 
2019-10-07T21:23:31.9210935Z 
2019-10-07T21:23:31.9210935Z 
2019-10-07T21:23:31.9211157Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2019-10-07T21:23:31.9211188Z 
2019-10-07T21:23:31.9211425Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9211490Z status: exit code: 101
2019-10-07T21:23:31.9212350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2019-10-07T21:23:31.9212684Z ------------------------------------------
2019-10-07T21:23:31.9212731Z 
2019-10-07T21:23:31.9212938Z ------------------------------------------
2019-10-07T21:23:31.9212979Z stderr:
2019-10-07T21:23:31.9212979Z stderr:
2019-10-07T21:23:31.9213362Z ------------------------------------------
2019-10-07T21:23:31.9213700Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/for_loops.rs:133:1: 139:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9213905Z 
2019-10-07T21:23:31.9213946Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9213973Z 
2019-10-07T21:23:31.9214015Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9214015Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9214068Z 
2019-10-07T21:23:31.9214364Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9214654Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9214685Z 
2019-10-07T21:23:31.9214685Z 
2019-10-07T21:23:31.9215050Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9215140Z 
2019-10-07T21:23:31.9215343Z ------------------------------------------
2019-10-07T21:23:31.9215375Z 
2019-10-07T21:23:31.9215402Z 
2019-10-07T21:23:31.9215402Z 
2019-10-07T21:23:31.9215827Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-10-07T21:23:31.9215863Z 
2019-10-07T21:23:31.9216319Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9216397Z status: exit code: 101
2019-10-07T21:23:31.9217622Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-10-07T21:23:31.9218005Z ------------------------------------------
2019-10-07T21:23:31.9218039Z 
2019-10-07T21:23:31.9218294Z ------------------------------------------
2019-10-07T21:23:31.9218342Z stderr:
2019-10-07T21:23:31.9218342Z stderr:
2019-10-07T21:23:31.9218557Z ------------------------------------------
2019-10-07T21:23:31.9218954Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/function_interfaces.rs:259:1: 261:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9219059Z 
2019-10-07T21:23:31.9219120Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9219150Z 
2019-10-07T21:23:31.9219196Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9219196Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9219225Z 
2019-10-07T21:23:31.9219751Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9220174Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9220229Z 
2019-10-07T21:23:31.9220229Z 
2019-10-07T21:23:31.9220589Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9220656Z 
2019-10-07T21:23:31.9220874Z ------------------------------------------
2019-10-07T21:23:31.9220903Z 
2019-10-07T21:23:31.9220929Z 
2019-10-07T21:23:31.9220929Z 
2019-10-07T21:23:31.9221151Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2019-10-07T21:23:31.9221204Z 
2019-10-07T21:23:31.9221468Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9221518Z status: exit code: 101
2019-10-07T21:23:31.9222434Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2019-10-07T21:23:31.9222781Z ------------------------------------------
2019-10-07T21:23:31.9222814Z 
2019-10-07T21:23:31.9223051Z ------------------------------------------
2019-10-07T21:23:31.9223096Z stderr:
2019-10-07T21:23:31.9223096Z stderr:
2019-10-07T21:23:31.9223314Z ------------------------------------------
2019-10-07T21:23:31.9223661Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/if_expressions.rs:86:1: 94:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9223875Z 
2019-10-07T21:23:31.9223973Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9224023Z 
2019-10-07T21:23:31.9224067Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9224067Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9224096Z 
2019-10-07T21:23:31.9224459Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9224740Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9224772Z 
2019-10-07T21:23:31.9224772Z 
2019-10-07T21:23:31.9225156Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9225225Z 
2019-10-07T21:23:31.9225468Z ------------------------------------------
2019-10-07T21:23:31.9225500Z 
2019-10-07T21:23:31.9225527Z 
2019-10-07T21:23:31.9225527Z 
2019-10-07T21:23:31.9225768Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2019-10-07T21:23:31.9225801Z 
2019-10-07T21:23:31.9226249Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9226801Z status: exit code: 101
2019-10-07T21:23:31.9229482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2019-10-07T21:23:31.9231760Z ------------------------------------------
2019-10-07T21:23:31.9231796Z 
2019-10-07T21:23:31.9238906Z ------------------------------------------
2019-10-07T21:23:31.9239228Z stderr:
2019-10-07T21:23:31.9239228Z stderr:
2019-10-07T21:23:31.9239597Z ------------------------------------------
2019-10-07T21:23:31.9240141Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/inherent_impls.rs:134:5: 134:42', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9240277Z 
2019-10-07T21:23:31.9240321Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9240350Z 
2019-10-07T21:23:31.9240431Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9240431Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9240460Z 
2019-10-07T21:23:31.9240968Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9241443Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9241476Z 
2019-10-07T21:23:31.9241476Z 
2019-10-07T21:23:31.9241859Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9241929Z 
2019-10-07T21:23:31.9242135Z ------------------------------------------
2019-10-07T21:23:31.9242183Z 
2019-10-07T21:23:31.9242211Z 
2019-10-07T21:23:31.9242211Z 
2019-10-07T21:23:31.9242447Z ---- [incremental] incremental/hashes/loop_expressions.rs stdout ----
2019-10-07T21:23:31.9242481Z 
2019-10-07T21:23:31.9242738Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-07T21:23:31.9242984Z status: exit code: 101
2019-10-07T21:23:31.9244032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
2019-10-07T21:23:31.9244780Z ------------------------------------------
2019-10-07T21:23:31.9244824Z 
2019-10-07T21:23:31.9245056Z ------------------------------------------
2019-10-07T21:23:31.9245101Z stderr:
2019-10-07T21:23:31.9245101Z stderr:
2019-10-07T21:23:31.9245338Z ------------------------------------------
2019-10-07T21:23:31.9246055Z thread 'rustc' panicked at 'Expected cache.predecessors to be `Some(...)` for block at: /checkout/src/test/incremental/hashes/loop_expressions.rs:64:1: 70:2', src/librustc/mir/mod.rs:230:9
2019-10-07T21:23:31.9246670Z 
2019-10-07T21:23:31.9246718Z error: internal compiler error: unexpected panic
2019-10-07T21:23:31.9246749Z 
2019-10-07T21:23:31.9246815Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9246815Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T21:23:31.9246846Z 
2019-10-07T21:23:31.9247230Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T21:23:31.9247541Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T21:23:31.9247586Z 
2019-10-07T21:23:31.9247586Z 
2019-10-07T21:23:31.9247984Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T21:23:31.9248077Z 
2019-10-07T21:23:31.9248300Z ------------------------------------------
2019-10-07T21:23:31.9248333Z 
2019-10-07T21:23:31.9248378Z 
2019-10-07T21:23:31.9248378Z 
2019-10-07T21:23:31.9248631Z ---- [incremental] incremental/hashes/struct_constructors.rs stdout ----
---
2019-10-07T21:23:31.9332887Z test result: FAILED. 74 passed; 30 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-07T21:23:31.9332922Z 
2019-10-07T21:23:31.9332948Z 
2019-10-07T21:23:31.9332992Z 
2019-10-07T21:23:31.9334497Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-07T21:23:31.9334814Z 
2019-10-07T21:23:31.9335021Z 
2019-10-07T21:23:31.9335073Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-07T21:23:31.9335156Z Build completed unsuccessfully in 1:04:20
2019-10-07T21:23:31.9335156Z Build completed unsuccessfully in 1:04:20
2019-10-07T21:23:31.9335203Z == clock drift check ==
2019-10-07T21:23:31.9335249Z   local time: Mon Oct  7 21:23:31 UTC 2019
2019-10-07T21:23:32.2071035Z   network time: Mon, 07 Oct 2019 21:23:32 GMT
2019-10-07T21:23:32.2071199Z == end clock drift check ==
2019-10-07T21:23:38.0592419Z ##[error]Bash exited with code '1'.
2019-10-07T21:23:38.0645109Z ##[section]Starting: Checkout
2019-10-07T21:23:38.0648558Z ==============================================================================
2019-10-07T21:23:38.0648640Z Task         : Get sources
2019-10-07T21:23:38.0648690Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
