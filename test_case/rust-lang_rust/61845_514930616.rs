plain
2019-07-25T06:13:33.7899502Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T06:13:33.8096082Z ##[command]git config gc.auto 0
2019-07-25T06:13:33.8158736Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T06:13:33.8215066Z ##[command]git config --get-all http.proxy
2019-07-25T06:13:33.8356581Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61845/merge:refs/remotes/pull/61845/merge
---
2019-07-25T06:14:07.9861684Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T06:14:07.9861714Z 
2019-07-25T06:14:07.9862081Z   git checkout -b <new-branch-name>
2019-07-25T06:14:07.9862127Z 
2019-07-25T06:14:07.9862172Z HEAD is now at fec211d64 Merge 9e4488d524eb0caff22b578ff8928e89136d3f7e into 185b9acb66438894596f3c40d2ae4c6f7deeb8ab
2019-07-25T06:14:08.0003858Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T06:14:08.0007109Z ==============================================================================
2019-07-25T06:14:08.0007161Z Task         : Bash
2019-07-25T06:14:08.0007202Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T07:14:05.3742669Z .................................................................................................... 200/5856
2019-07-25T07:14:09.5324265Z .................................................................................................... 300/5856
2019-07-25T07:14:13.1935834Z .................................................................................................... 400/5856
2019-07-25T07:14:16.8884371Z .................................................................................................... 500/5856
2019-07-25T07:14:20.6887015Z ........................................................................i........................... 600/5856
2019-07-25T07:14:29.5323264Z .................................................................................................... 800/5856
2019-07-25T07:14:35.0581985Z .................................................................................................... 900/5856
2019-07-25T07:14:40.0116315Z ...................................................................................................i 1000/5856
2019-07-25T07:14:40.0116315Z ...................................................................................................i 1000/5856
2019-07-25T07:14:45.5662024Z ...........i........................................................................................ 1100/5856
2019-07-25T07:14:49.5681563Z .............................iiiii.................................................................. 1200/5856
2019-07-25T07:14:55.4942642Z .................................................................................................... 1400/5856
2019-07-25T07:14:58.1916973Z .................................................................................................... 1500/5856
2019-07-25T07:15:01.9460061Z .................................................................................................... 1600/5856
2019-07-25T07:15:04.6279437Z .................................................................................................... 1700/5856
2019-07-25T07:15:04.6279437Z .................................................................................................... 1700/5856
2019-07-25T07:15:08.0703268Z .....................................................................i.............................. 1800/5856
2019-07-25T07:15:16.5206098Z .................................................................................................... 2000/5856
2019-07-25T07:15:20.7511659Z .................................................................................................... 2100/5856
2019-07-25T07:15:24.4607624Z .................................................................................................... 2200/5856
2019-07-25T07:15:24.4607624Z .................................................................................................... 2200/5856
2019-07-25T07:15:28.3120926Z .....................................................i.............................................. 2300/5856
2019-07-25T07:15:38.0340973Z .................................................................................................... 2500/5856
2019-07-25T07:15:42.1128567Z .................................................................................................... 2600/5856
2019-07-25T07:15:47.1569461Z .................................................................................................... 2700/5856
2019-07-25T07:15:51.0323972Z .................................................................................................... 2800/5856
2019-07-25T07:15:51.0323972Z .................................................................................................... 2800/5856
2019-07-25T07:15:56.2181948Z .................................................................................................... 2900/5856
2019-07-25T07:16:00.7176942Z .................................................................................................... 3000/5856
2019-07-25T07:16:05.3025657Z .................................................................................................... 3100/5856
2019-07-25T07:16:10.6106449Z .................................................................................................... 3200/5856
2019-07-25T07:16:14.1599919Z .................................................................................................... 3300/5856
2019-07-25T07:16:17.8882014Z .................................................................................................... 3400/5856
2019-07-25T07:16:23.0297651Z .................................................................................................... 3500/5856
2019-07-25T07:16:26.7387553Z .....................i.............................................................................. 3600/5856
2019-07-25T07:16:30.8203721Z ..............................................................................................ii...i 3700/5856
2019-07-25T07:16:34.6230073Z ..ii................................................................................................ 3800/5856
2019-07-25T07:16:43.4033628Z .................................................................................................... 4000/5856
2019-07-25T07:16:43.4033628Z .................................................................................................... 4000/5856
2019-07-25T07:16:47.2585204Z .............ii..................................................................................... 4100/5856
2019-07-25T07:16:49.2426370Z ..................................i................................................................. 4200/5856
2019-07-25T07:16:51.4290706Z .................................................................................................... 4300/5856
2019-07-25T07:16:53.7800492Z .i.................................................................................................. 4400/5856
2019-07-25T07:17:15.7361163Z .................................................................................................... 4600/5856
2019-07-25T07:17:19.3398227Z .................................................................................................... 4700/5856
2019-07-25T07:17:22.9021149Z .................................................................................................... 4800/5856
2019-07-25T07:17:27.5626900Z .................................................................................................... 4900/5856
---
2019-07-25T07:18:12.1933372Z Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-25T07:18:12.4177158Z 
2019-07-25T07:18:12.4177552Z running 2905 tests
2019-07-25T07:18:27.2587311Z .................................................................................................... 100/2905
2019-07-25T07:18:42.5223513Z ............................................................................i....................... 200/2905
2019-07-25T07:19:07.1049134Z .................................................................................................... 400/2905
2019-07-25T07:19:18.7797767Z .................................................................................................... 500/2905
2019-07-25T07:19:33.1513754Z .................................................................................................... 600/2905
2019-07-25T07:19:54.4936488Z .................................................................................................... 700/2905
2019-07-25T07:19:54.4936488Z .................................................................................................... 700/2905
2019-07-25T07:20:08.5933614Z .................................................................................................... 800/2905
2019-07-25T07:20:19.9417785Z .................................................................................................... 900/2905
2019-07-25T07:20:37.7093343Z .................................................................................................... 1000/2905
2019-07-25T07:20:51.9428428Z .................................................................................................... 1100/2905
2019-07-25T07:21:03.0052384Z .................................................................................................... 1200/2905
2019-07-25T07:21:15.5174593Z .................................................................................................... 1300/2905
2019-07-25T07:21:31.6908226Z .......ii........................................................................................... 1400/2905
2019-07-25T07:21:44.0943829Z .................................................................................................... 1500/2905
2019-07-25T07:21:56.5181800Z ...........................................................i.......i................................ 1600/2905
2019-07-25T07:22:29.0877929Z .................................................................................................... 1800/2905
2019-07-25T07:22:29.0877929Z .................................................................................................... 1800/2905
2019-07-25T07:22:44.1716971Z ...........................................................................................i........ 1900/2905
2019-07-25T07:23:13.0527758Z ...............................................................i.................................... 2000/2905
2019-07-25T07:23:56.1412970Z .................................................................................................... 2200/2905
2019-07-25T07:24:10.7498665Z ..................................................................................................ii 2300/2905
2019-07-25T07:24:30.1591299Z .................................................................................................... 2400/2905
2019-07-25T07:24:44.0109914Z .................................................................................................... 2500/2905
---
2019-07-25T07:26:59.0362717Z  finished in 30.721
2019-07-25T07:26:59.0553775Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-25T07:26:59.2147967Z 
2019-07-25T07:26:59.2148243Z running 146 tests
2019-07-25T07:27:02.5083593Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-25T07:27:04.3698058Z iii..............i.........iii.i......ii......
2019-07-25T07:27:04.3698632Z 
2019-07-25T07:27:04.3702090Z  finished in 5.314
2019-07-25T07:27:04.3905070Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-25T07:27:04.5511440Z 
2019-07-25T07:27:04.5511440Z 
2019-07-25T07:27:04.5511701Z running 39 tests
2019-07-25T07:27:06.6103560Z i.........i......................i.....
2019-07-25T07:27:06.6104397Z 
2019-07-25T07:27:06.6104465Z  finished in 2.219
2019-07-25T07:27:06.6294342Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-25T07:27:06.7813218Z 
2019-07-25T07:27:06.7813218Z 
2019-07-25T07:27:06.7813590Z running 9 tests
2019-07-25T07:27:06.7814819Z iiiiiiiii
2019-07-25T07:27:06.7815900Z 
2019-07-25T07:27:06.7821419Z  finished in 0.153
2019-07-25T07:27:06.8008858Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-25T07:27:06.9592329Z 
2019-07-25T07:27:06.9592329Z 
2019-07-25T07:27:06.9593049Z running 104 tests
2019-07-25T07:27:23.5261672Z .....F.........F....F.FFFFFFFFFFFFFFFFFFFFFFFFF...F.......F.F..........F.F...F....................F. 100/104
2019-07-25T07:27:24.1220417Z F.FF
2019-07-25T07:27:24.1220561Z 
2019-07-25T07:27:24.1220847Z ---- [incremental] incremental/change_name_of_static_in_fn.rs stdout ----
2019-07-25T07:27:24.1220903Z 
2019-07-25T07:27:24.1220903Z 
2019-07-25T07:27:24.1220951Z error in revision `rpass3`: compilation failed!
2019-07-25T07:27:24.1220998Z status: exit code: 101
2019-07-25T07:27:24.1222747Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_name_of_static_in_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/change_name_of_static_in_fn.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_name_of_static_in_fn/auxiliary"
2019-07-25T07:27:24.1223148Z ------------------------------------------
2019-07-25T07:27:24.1223181Z 
2019-07-25T07:27:24.1223370Z ------------------------------------------
2019-07-25T07:27:24.1223438Z stderr:
2019-07-25T07:27:24.1223438Z stderr:
2019-07-25T07:27:24.1223626Z ------------------------------------------
2019-07-25T07:27:24.1223897Z thread 'rustc' panicked at 'index out of bounds: the len is 5579 but the index is 5579', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1223969Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T07:27:24.1224250Z thread 'rustc' panicked at 'index out of bounds: the len is 5579 but the index is 5580', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1224341Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1224367Z 
2019-07-25T07:27:24.1224367Z 
2019-07-25T07:27:24.1224407Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1224434Z 
2019-07-25T07:27:24.1224807Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1224841Z 
2019-07-25T07:27:24.1225057Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1225226Z 
2019-07-25T07:27:24.1225552Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1225617Z 
2019-07-25T07:27:24.1225818Z ------------------------------------------
2019-07-25T07:27:24.1225845Z 
2019-07-25T07:27:24.1225868Z 
2019-07-25T07:27:24.1225868Z 
2019-07-25T07:27:24.1226078Z ---- [incremental] incremental/crate_hash_reorder.rs stdout ----
2019-07-25T07:27:24.1226122Z 
2019-07-25T07:27:24.1226163Z error in revision `rpass3`: compilation failed!
2019-07-25T07:27:24.1226203Z status: exit code: 101
2019-07-25T07:27:24.1227024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
2019-07-25T07:27:24.1227338Z ------------------------------------------
2019-07-25T07:27:24.1227367Z 
2019-07-25T07:27:24.1228235Z ------------------------------------------
2019-07-25T07:27:24.1228307Z stderr:
2019-07-25T07:27:24.1228307Z stderr:
2019-07-25T07:27:24.1228522Z ------------------------------------------
2019-07-25T07:27:24.1228831Z thread 'rustc' panicked at 'index out of bounds: the len is 7032 but the index is 7032', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1228958Z 
2019-07-25T07:27:24.1229002Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1229032Z 
2019-07-25T07:27:24.1229032Z 
2019-07-25T07:27:24.1229089Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1229119Z 
2019-07-25T07:27:24.1229443Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1229487Z 
2019-07-25T07:27:24.1229730Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1229865Z 
2019-07-25T07:27:24.1230257Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1230342Z 
2019-07-25T07:27:24.1230558Z ------------------------------------------
2019-07-25T07:27:24.1230589Z 
2019-07-25T07:27:24.1230621Z 
2019-07-25T07:27:24.1230621Z 
2019-07-25T07:27:24.1230860Z ---- [incremental] incremental/hashes/call_expressions.rs stdout ----
2019-07-25T07:27:24.1230903Z 
2019-07-25T07:27:24.1231145Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1231202Z status: exit code: 101
2019-07-25T07:27:24.1232217Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
2019-07-25T07:27:24.1232604Z ------------------------------------------
2019-07-25T07:27:24.1232638Z 
2019-07-25T07:27:24.1232826Z ------------------------------------------
2019-07-25T07:27:24.1232865Z stderr:
2019-07-25T07:27:24.1232865Z stderr:
2019-07-25T07:27:24.1233057Z ------------------------------------------
2019-07-25T07:27:24.1233334Z thread 'rustc' panicked at 'index out of bounds: the len is 2852 but the index is 2904', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1233428Z 
2019-07-25T07:27:24.1233467Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1233493Z 
2019-07-25T07:27:24.1233493Z 
2019-07-25T07:27:24.1233532Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1233569Z 
2019-07-25T07:27:24.1233845Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1233885Z 
2019-07-25T07:27:24.1234108Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1234136Z 
2019-07-25T07:27:24.1234473Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1234553Z 
2019-07-25T07:27:24.1234746Z ------------------------------------------
2019-07-25T07:27:24.1234773Z 
2019-07-25T07:27:24.1234795Z 
2019-07-25T07:27:24.1234795Z 
2019-07-25T07:27:24.1235014Z ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
2019-07-25T07:27:24.1235044Z 
2019-07-25T07:27:24.1235255Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1235299Z status: exit code: 101
2019-07-25T07:27:24.1236406Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
2019-07-25T07:27:24.1236741Z ------------------------------------------
2019-07-25T07:27:24.1236769Z 
2019-07-25T07:27:24.1236960Z ------------------------------------------
2019-07-25T07:27:24.1236998Z stderr:
2019-07-25T07:27:24.1236998Z stderr:
2019-07-25T07:27:24.1237175Z ------------------------------------------
2019-07-25T07:27:24.1237617Z thread 'rustc' panicked at 'index out of bounds: the len is 2568 but the index is 2622', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1237916Z 
2019-07-25T07:27:24.1237961Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1238005Z 
2019-07-25T07:27:24.1238005Z 
2019-07-25T07:27:24.1238051Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1238091Z 
2019-07-25T07:27:24.1238450Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1238488Z 
2019-07-25T07:27:24.1238728Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1238760Z 
2019-07-25T07:27:24.1239160Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1239340Z 
2019-07-25T07:27:24.1239593Z ------------------------------------------
2019-07-25T07:27:24.1239624Z 
2019-07-25T07:27:24.1239650Z 
2019-07-25T07:27:24.1239650Z 
2019-07-25T07:27:24.1239880Z ---- [incremental] incremental/hashes/consts.rs stdout ----
2019-07-25T07:27:24.1239913Z 
2019-07-25T07:27:24.1240174Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1240223Z status: exit code: 101
2019-07-25T07:27:24.1241172Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/consts.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/auxiliary"
2019-07-25T07:27:24.1241647Z ------------------------------------------
2019-07-25T07:27:24.1241674Z 
2019-07-25T07:27:24.1241855Z ------------------------------------------
2019-07-25T07:27:24.1241900Z stderr:
2019-07-25T07:27:24.1241900Z stderr:
2019-07-25T07:27:24.1242094Z ------------------------------------------
2019-07-25T07:27:24.1242351Z thread 'rustc' panicked at 'index out of bounds: the len is 3484 but the index is 3484', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1242450Z 
2019-07-25T07:27:24.1242487Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1242512Z 
2019-07-25T07:27:24.1242512Z 
2019-07-25T07:27:24.1242570Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1242596Z 
2019-07-25T07:27:24.1242859Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1242889Z 
2019-07-25T07:27:24.1243103Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1243130Z 
2019-07-25T07:27:24.1243515Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1243601Z 
2019-07-25T07:27:24.1243805Z ------------------------------------------
2019-07-25T07:27:24.1243832Z 
2019-07-25T07:27:24.1243853Z 
2019-07-25T07:27:24.1243853Z 
2019-07-25T07:27:24.1244063Z ---- [incremental] incremental/hashes/enum_defs.rs stdout ----
2019-07-25T07:27:24.1244090Z 
2019-07-25T07:27:24.1244293Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1244358Z status: exit code: 101
2019-07-25T07:27:24.1245881Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
2019-07-25T07:27:24.1246210Z ------------------------------------------
2019-07-25T07:27:24.1246238Z 
2019-07-25T07:27:24.1246551Z ------------------------------------------
2019-07-25T07:27:24.1246590Z stderr:
2019-07-25T07:27:24.1246590Z stderr:
2019-07-25T07:27:24.1246769Z ------------------------------------------
2019-07-25T07:27:24.1247042Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6279', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1247096Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T07:27:24.1247365Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6284', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1248104Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6289', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1248418Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6508', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1248743Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6513', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1249066Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6518', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1249373Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6523', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1249710Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6528', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1250020Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6533', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1250329Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6538', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1250656Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6543', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1250974Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6544', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1251605Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6545', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1251871Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6554', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1252219Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6555', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1252532Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6556', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1252792Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6561', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1253051Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6566', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1253339Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6571', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1253599Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6572', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1253885Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6573', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1254150Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6579', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1254410Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6580', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1254688Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6581', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1255037Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6586', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1255339Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6587', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1255632Z thread 'rustc' panicked at 'index out of bounds: the len is 6277 but the index is 6588', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1255703Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1255744Z 
2019-07-25T07:27:24.1255744Z 
2019-07-25T07:27:24.1255784Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1255810Z 
2019-07-25T07:27:24.1256277Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1256308Z 
2019-07-25T07:27:24.1256511Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1256546Z 
2019-07-25T07:27:24.1257306Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1257374Z 
2019-07-25T07:27:24.1258210Z ------------------------------------------
2019-07-25T07:27:24.1258263Z 
2019-07-25T07:27:24.1258288Z 
2019-07-25T07:27:24.1258288Z 
2019-07-25T07:27:24.1258566Z ---- [incremental] incremental/hashes/enum_constructors.rs stdout ----
2019-07-25T07:27:24.1258600Z 
2019-07-25T07:27:24.1258865Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1258916Z status: exit code: 101
2019-07-25T07:27:24.1259994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
2019-07-25T07:27:24.1260405Z ------------------------------------------
2019-07-25T07:27:24.1260439Z 
2019-07-25T07:27:24.1260655Z ------------------------------------------
2019-07-25T07:27:24.1260701Z stderr:
2019-07-25T07:27:24.1260701Z stderr:
2019-07-25T07:27:24.1260930Z ------------------------------------------
2019-07-25T07:27:24.1261536Z thread 'rustc' panicked at 'index out of bounds: the len is 4800 but the index is 4800', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1261803Z 
2019-07-25T07:27:24.1261838Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1261862Z 
2019-07-25T07:27:24.1261862Z 
2019-07-25T07:27:24.1261910Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1261935Z 
2019-07-25T07:27:24.1262208Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1262237Z 
2019-07-25T07:27:24.1262448Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1262475Z 
2019-07-25T07:27:24.1262788Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1262864Z 
2019-07-25T07:27:24.1263041Z ------------------------------------------
2019-07-25T07:27:24.1263138Z 
2019-07-25T07:27:24.1263159Z 
2019-07-25T07:27:24.1263159Z 
2019-07-25T07:27:24.1263394Z ---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----
2019-07-25T07:27:24.1263422Z 
2019-07-25T07:27:24.1263619Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1263674Z status: exit code: 101
2019-07-25T07:27:24.1264665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
2019-07-25T07:27:24.1264998Z ------------------------------------------
2019-07-25T07:27:24.1265025Z 
2019-07-25T07:27:24.1268235Z ------------------------------------------
2019-07-25T07:27:24.1268296Z stderr:
2019-07-25T07:27:24.1268296Z stderr:
2019-07-25T07:27:24.1268531Z ------------------------------------------
2019-07-25T07:27:24.1268875Z thread 'rustc' panicked at 'index out of bounds: the len is 1646 but the index is 1646', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1268979Z 
2019-07-25T07:27:24.1269041Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1269072Z 
2019-07-25T07:27:24.1269072Z 
2019-07-25T07:27:24.1269117Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1269157Z 
2019-07-25T07:27:24.1269513Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1269552Z 
2019-07-25T07:27:24.1269796Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1269845Z 
2019-07-25T07:27:24.1270355Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1270443Z 
2019-07-25T07:27:24.1270711Z ------------------------------------------
2019-07-25T07:27:24.1270743Z 
2019-07-25T07:27:24.1270769Z 
2019-07-25T07:27:24.1270769Z 
2019-07-25T07:27:24.1271002Z ---- [incremental] incremental/hashes/extern_mods.rs stdout ----
2019-07-25T07:27:24.1271053Z 
2019-07-25T07:27:24.1271459Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1271542Z status: exit code: 101
2019-07-25T07:27:24.1272378Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
2019-07-25T07:27:24.1272675Z ------------------------------------------
2019-07-25T07:27:24.1272704Z 
2019-07-25T07:27:24.1272883Z ------------------------------------------
2019-07-25T07:27:24.1273028Z stderr:
2019-07-25T07:27:24.1273028Z stderr:
2019-07-25T07:27:24.1273232Z ------------------------------------------
2019-07-25T07:27:24.1273489Z thread 'rustc' panicked at 'index out of bounds: the len is 900 but the index is 900', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1273588Z 
2019-07-25T07:27:24.1273625Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1273658Z 
2019-07-25T07:27:24.1273658Z 
2019-07-25T07:27:24.1273713Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1273739Z 
2019-07-25T07:27:24.1274007Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1274053Z 
2019-07-25T07:27:24.1274255Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1274282Z 
2019-07-25T07:27:24.1274618Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1274689Z 
2019-07-25T07:27:24.1274869Z ------------------------------------------
2019-07-25T07:27:24.1274911Z 
2019-07-25T07:27:24.1274933Z 
2019-07-25T07:27:24.1274933Z 
2019-07-25T07:27:24.1275135Z ---- [incremental] incremental/hashes/function_interfaces.rs stdout ----
2019-07-25T07:27:24.1275163Z 
2019-07-25T07:27:24.1275389Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1275431Z status: exit code: 101
2019-07-25T07:27:24.1276665Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/function_interfaces.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/function_interfaces.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/function_interfaces/auxiliary"
2019-07-25T07:27:24.1277414Z ------------------------------------------
2019-07-25T07:27:24.1277612Z 
2019-07-25T07:27:24.1278065Z ------------------------------------------
2019-07-25T07:27:24.1278110Z stderr:
2019-07-25T07:27:24.1278110Z stderr:
2019-07-25T07:27:24.1278341Z ------------------------------------------
2019-07-25T07:27:24.1278649Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 8329', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1278711Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T07:27:24.1279991Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 8539', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1280528Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 8544', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1281379Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9851', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1281682Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9855', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1282065Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9860', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1282445Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9864', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1282808Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9868', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1283359Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9872', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1283656Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9876', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1284041Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9881', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1284424Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9882', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1284780Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9887', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1285256Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9888', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1285657Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9894', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1286011Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9899', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1286318Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9900', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1286688Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9901', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1287218Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9906', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1287986Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9907', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1288479Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9908', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1288929Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9912', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1289352Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9917', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1289904Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9922', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1290305Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9923', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1290752Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9924', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1291179Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9931', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1291745Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9932', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1292036Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9933', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1292435Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9937', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1292821Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9938', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1293181Z thread 'rustc' panicked at 'index out of bounds: the len is 7550 but the index is 9939', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1293278Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1293389Z 
2019-07-25T07:27:24.1293389Z 
2019-07-25T07:27:24.1293430Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1293466Z 
2019-07-25T07:27:24.1293917Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1293951Z 
2019-07-25T07:27:24.1294295Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1294326Z 
2019-07-25T07:27:24.1294766Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1294853Z 
2019-07-25T07:27:24.1295148Z ------------------------------------------
2019-07-25T07:27:24.1295183Z 
2019-07-25T07:27:24.1295205Z 
2019-07-25T07:27:24.1295205Z 
2019-07-25T07:27:24.1295436Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2019-07-25T07:27:24.1295465Z 
2019-07-25T07:27:24.1295784Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1295839Z status: exit code: 101
2019-07-25T07:27:24.1296868Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2019-07-25T07:27:24.1297961Z ------------------------------------------
2019-07-25T07:27:24.1298013Z 
2019-07-25T07:27:24.1298419Z ------------------------------------------
2019-07-25T07:27:24.1298469Z stderr:
2019-07-25T07:27:24.1298469Z stderr:
2019-07-25T07:27:24.1298685Z ------------------------------------------
2019-07-25T07:27:24.1299135Z thread 'rustc' panicked at 'index out of bounds: the len is 10129 but the index is 10136', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1299440Z 
2019-07-25T07:27:24.1299505Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1299554Z 
2019-07-25T07:27:24.1299554Z 
2019-07-25T07:27:24.1299600Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1299630Z 
2019-07-25T07:27:24.1300138Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1302218Z 
2019-07-25T07:27:24.1302478Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1302506Z 
2019-07-25T07:27:24.1302855Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1302919Z 
2019-07-25T07:27:24.1303107Z ------------------------------------------
2019-07-25T07:27:24.1303134Z 
2019-07-25T07:27:24.1303155Z 
2019-07-25T07:27:24.1303155Z 
2019-07-25T07:27:24.1303361Z ---- [incremental] incremental/hashes/if_expressions.rs stdout ----
2019-07-25T07:27:24.1303391Z 
2019-07-25T07:27:24.1303608Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1303651Z status: exit code: 101
2019-07-25T07:27:24.1304478Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
2019-07-25T07:27:24.1304895Z ------------------------------------------
2019-07-25T07:27:24.1304923Z 
2019-07-25T07:27:24.1305102Z ------------------------------------------
2019-07-25T07:27:24.1305150Z stderr:
2019-07-25T07:27:24.1305150Z stderr:
2019-07-25T07:27:24.1305329Z ------------------------------------------
2019-07-25T07:27:24.1305585Z thread 'rustc' panicked at 'index out of bounds: the len is 2500 but the index is 2873', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1305689Z 
2019-07-25T07:27:24.1305726Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1305752Z 
2019-07-25T07:27:24.1305752Z 
2019-07-25T07:27:24.1305798Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1305823Z 
2019-07-25T07:27:24.1306092Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1306123Z 
2019-07-25T07:27:24.1306343Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1306371Z 
2019-07-25T07:27:24.1306689Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1306764Z 
2019-07-25T07:27:24.1306945Z ------------------------------------------
2019-07-25T07:27:24.1306980Z 
2019-07-25T07:27:24.1307016Z 
2019-07-25T07:27:24.1307016Z 
2019-07-25T07:27:24.1307219Z ---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----
2019-07-25T07:27:24.1307247Z 
2019-07-25T07:27:24.1307595Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1307911Z status: exit code: 101
2019-07-25T07:27:24.1309040Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
2019-07-25T07:27:24.1309425Z ------------------------------------------
2019-07-25T07:27:24.1309472Z 
2019-07-25T07:27:24.1309689Z ------------------------------------------
2019-07-25T07:27:24.1309734Z stderr:
2019-07-25T07:27:24.1309734Z stderr:
2019-07-25T07:27:24.1309942Z ------------------------------------------
2019-07-25T07:27:24.1310276Z thread 'rustc' panicked at 'index out of bounds: the len is 2484 but the index is 2520', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1310374Z 
2019-07-25T07:27:24.1310429Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1310459Z 
2019-07-25T07:27:24.1310459Z 
2019-07-25T07:27:24.1310504Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1310612Z 
2019-07-25T07:27:24.1310961Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1310998Z 
2019-07-25T07:27:24.1311391Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1311427Z 
2019-07-25T07:27:24.1311756Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1311829Z 
2019-07-25T07:27:24.1312011Z ------------------------------------------
2019-07-25T07:27:24.1312037Z 
2019-07-25T07:27:24.1312059Z 
2019-07-25T07:27:24.1312059Z 
2019-07-25T07:27:24.1312262Z ---- [incremental] incremental/hashes/inline_asm.rs stdout ----
2019-07-25T07:27:24.1312291Z 
2019-07-25T07:27:24.1312495Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1312537Z status: exit code: 101
2019-07-25T07:27:24.1313343Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
2019-07-25T07:27:24.1313639Z ------------------------------------------
2019-07-25T07:27:24.1313666Z 
2019-07-25T07:27:24.1313844Z ------------------------------------------
2019-07-25T07:27:24.1313905Z stderr:
2019-07-25T07:27:24.1313905Z stderr:
2019-07-25T07:27:24.1314084Z ------------------------------------------
2019-07-25T07:27:24.1314341Z thread 'rustc' panicked at 'index out of bounds: the len is 1827 but the index is 1827', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1314435Z 
2019-07-25T07:27:24.1314473Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1314508Z 
2019-07-25T07:27:24.1314508Z 
2019-07-25T07:27:24.1314613Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1314645Z 
2019-07-25T07:27:24.1314945Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1314977Z 
2019-07-25T07:27:24.1315181Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-07-25T07:27:24.1315208Z 
2019-07-25T07:27:24.1315531Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-07-25T07:27:24.1315604Z 
2019-07-25T07:27:24.1315787Z ------------------------------------------
2019-07-25T07:27:24.1315825Z 
2019-07-25T07:27:24.1315846Z 
2019-07-25T07:27:24.1315846Z 
2019-07-25T07:27:24.1316045Z ---- [incremental] incremental/hashes/inherent_impls.rs stdout ----
2019-07-25T07:27:24.1316073Z 
2019-07-25T07:27:24.1316298Z error in revision `cfail3`: test compilation failed although it shouldn't!
2019-07-25T07:27:24.1316340Z status: exit code: 101
2019-07-25T07:27:24.1317317Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
2019-07-25T07:27:24.1318108Z ------------------------------------------
2019-07-25T07:27:24.1318144Z 
2019-07-25T07:27:24.1318362Z ------------------------------------------
2019-07-25T07:27:24.1318407Z stderr:
2019-07-25T07:27:24.1318407Z stderr:
2019-07-25T07:27:24.1318632Z ------------------------------------------
2019-07-25T07:27:24.1318940Z thread 'rustc' panicked at 'index out of bounds: the len is 6249 but the index is 7808', /checkout/src/libcore/slice/mod.rs:2690:10
2019-07-25T07:27:24.1319059Z 
2019-07-25T07:27:24.1319103Z error: internal compiler error: unexpected panic
2019-07-25T07:27:24.1319132Z 
2019-07-25T07:27:24.1319132Z 
2019-07-25T07:27:24.1319178Z note: the compiler unexpectedly panicked. this is a bug.
2019-07-25T07:27:24.1319218Z 
2019-07-25T07:27:24.1319534Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-07-25T07:27:24.1319570Z 
---
2019-07-25T07:27:24.1447366Z test result: FAILED. 66 passed; 38 failed; 0 ignored; 0 measured; 0 filtered out
2019-07-25T07:27:24.1448263Z 
2019-07-25T07:27:24.1448289Z 
2019-07-25T07:27:24.1448331Z 
2019-07-25T07:27:24.1449895Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-25T07:27:24.1450177Z 
2019-07-25T07:27:24.1450205Z 
2019-07-25T07:27:24.1450251Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-25T07:27:24.1450302Z Build completed unsuccessfully in 1:07:06
2019-07-25T07:27:24.1450302Z Build completed unsuccessfully in 1:07:06
2019-07-25T07:27:24.1450590Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-25T07:27:24.1450650Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T07:27:27.7298169Z ##[error]Bash exited with code '1'.
2019-07-25T07:27:27.7337113Z ##[section]Starting: Checkout
2019-07-25T07:27:27.7339369Z ==============================================================================
2019-07-25T07:27:27.7339446Z Task         : Get sources
2019-07-25T07:27:27.7339497Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
