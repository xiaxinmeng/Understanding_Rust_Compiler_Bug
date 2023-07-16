plain
2019-08-26T07:29:10.2929743Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T07:29:10.3125895Z ##[command]git config gc.auto 0
2019-08-26T07:29:10.3198704Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T07:29:10.3256449Z ##[command]git config --get-all http.proxy
2019-08-26T07:29:10.3398844Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63884/merge:refs/remotes/pull/63884/merge
---
2019-08-26T07:29:45.8589517Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T07:29:45.8589543Z 
2019-08-26T07:29:45.8589727Z   git checkout -b <new-branch-name>
2019-08-26T07:29:45.8589771Z 
2019-08-26T07:29:45.8589811Z HEAD is now at 6cd052735 Merge 2214932a30091c54654422edbd39489c102285f4 into 4c58535d09d1261d21569df0036b974811544256
2019-08-26T07:29:45.8760651Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T07:29:45.8763789Z ==============================================================================
2019-08-26T07:29:45.8763867Z Task         : Bash
2019-08-26T07:29:45.8763912Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T08:30:18.4440806Z .................................................................................................... 1500/8953
2019-08-26T08:30:23.9298771Z .................................................................................................... 1600/8953
2019-08-26T08:30:36.3860609Z .............................................i...............i...................................... 1700/8953
2019-08-26T08:30:44.4044764Z .................................................................................................... 1800/8953
2019-08-26T08:30:58.4270704Z .....................................iiiii.......................................................... 1900/8953
2019-08-26T08:31:08.4280835Z .................................................................................................... 2100/8953
2019-08-26T08:31:10.9825438Z .................................................................................................... 2200/8953
2019-08-26T08:31:15.1046373Z .................................................................................................... 2300/8953
2019-08-26T08:31:22.0470345Z .................................................................................................... 2400/8953
---
2019-08-26T08:34:10.5313142Z .........................i...............i.......................................................... 4700/8953
2019-08-26T08:34:21.6475030Z .................................................................................................... 4800/8953
2019-08-26T08:34:27.4781450Z .................................................................................................... 4900/8953
2019-08-26T08:34:37.6026927Z .................................................................................................... 5000/8953
2019-08-26T08:34:42.9131343Z ......ii.ii......................................................................................... 5100/8953
2019-08-26T08:34:56.8367734Z .................................................................................................... 5300/8953
2019-08-26T08:35:03.4965681Z ..............................................................i..................................... 5400/8953
2019-08-26T08:35:10.4428848Z .................................................................................................... 5500/8953
2019-08-26T08:35:17.8449953Z .................................................................................................... 5600/8953
2019-08-26T08:35:17.8449953Z .................................................................................................... 5600/8953
2019-08-26T08:35:28.3848747Z ........................................................ii...i..ii...........i...................... 5700/8953
2019-08-26T08:35:49.6383633Z .................................................................................................... 5900/8953
2019-08-26T08:35:54.4453310Z .................................................................................................... 6000/8953
2019-08-26T08:35:54.4453310Z .................................................................................................... 6000/8953
2019-08-26T08:36:01.5743607Z .........................................................i..ii...................................... 6100/8953
2019-08-26T08:36:29.3867501Z .................................................................................................... 6300/8953
2019-08-26T08:36:31.6105092Z ...i................................................................................................ 6400/8953
2019-08-26T08:36:33.9692993Z ...........................................................................i........................ 6500/8953
2019-08-26T08:36:36.8068867Z .................................................................................................... 6600/8953
---
2019-08-26T08:41:09.0833518Z  finished in 18.838
2019-08-26T08:41:09.1011315Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T08:41:09.2915534Z 
2019-08-26T08:41:09.2915810Z running 150 tests
2019-08-26T08:41:12.4416098Z i....iii.......iii.iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-08-26T08:41:14.4241468Z ..iiii..............i.........iii.i.......ii......
2019-08-26T08:41:14.4244562Z 
2019-08-26T08:41:14.4247465Z  finished in 5.323
2019-08-26T08:41:14.4423438Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T08:41:14.6065536Z 
---
2019-08-26T08:41:16.6257594Z  finished in 2.183
2019-08-26T08:41:16.6430365Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T08:41:16.8046219Z 
2019-08-26T08:41:16.8046423Z running 9 tests
2019-08-26T08:41:16.8048022Z iiiiiiiii
2019-08-26T08:41:16.8048401Z 
2019-08-26T08:41:16.8049165Z  finished in 0.161
2019-08-26T08:41:16.8227745Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-26T08:41:17.0032334Z 
2019-08-26T08:41:17.0032334Z 
2019-08-26T08:41:17.0032641Z running 104 tests
2019-08-26T08:41:34.0607232Z ..................................................................................................F. 100/104
2019-08-26T08:41:34.6628923Z F...
2019-08-26T08:41:35.1670249Z failures:
2019-08-26T08:41:35.1670314Z 
2019-08-26T08:41:35.1671088Z ---- [incremental] incremental/thinlto/cgu_invalidated_via_import.rs stdout ----
2019-08-26T08:41:35.1671132Z 
2019-08-26T08:41:35.1671439Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-08-26T08:41:35.1671511Z status: exit code: 1
2019-08-26T08:41:35.1672618Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/cgu_invalidated_via_import.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/auxiliary"
2019-08-26T08:41:35.1673415Z ------------------------------------------
2019-08-26T08:41:35.1673471Z 
2019-08-26T08:41:35.1673729Z ------------------------------------------
2019-08-26T08:41:35.1673779Z stderr:
2019-08-26T08:41:35.1673779Z stderr:
2019-08-26T08:41:35.1674037Z ------------------------------------------
2019-08-26T08:41:35.1674347Z error: CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`
2019-08-26T08:41:35.1674711Z    |
2019-08-26T08:41:35.1674711Z    |
2019-08-26T08:41:35.1675000Z LL | / #![rustc_expected_cgu_reuse(module="cgu_invalidated_via_import-bar",
2019-08-26T08:41:35.1675058Z LL | |                             cfg="cfail2",
2019-08-26T08:41:35.1675475Z LL | |                             kind="pre-lto")]
2019-08-26T08:41:35.1675571Z 
2019-08-26T08:41:35.1675609Z error: aborting due to previous error
2019-08-26T08:41:35.1675636Z 
2019-08-26T08:41:35.1675843Z 
2019-08-26T08:41:35.1675843Z 
2019-08-26T08:41:35.1676239Z ------------------------------------------
2019-08-26T08:41:35.1676264Z 
2019-08-26T08:41:35.1676285Z 
2019-08-26T08:41:35.1676520Z ---- [incremental] incremental/thinlto/independent_cgus_dont_affect_each_other.rs stdout ----
2019-08-26T08:41:35.1676557Z 
2019-08-26T08:41:35.1676763Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-08-26T08:41:35.1676804Z status: exit code: 1
2019-08-26T08:41:35.1678135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/independent_cgus_dont_affect_each_other.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/auxiliary"
2019-08-26T08:41:35.1678482Z ------------------------------------------
2019-08-26T08:41:35.1678511Z 
2019-08-26T08:41:35.1678709Z ------------------------------------------
2019-08-26T08:41:35.1678753Z stderr:
2019-08-26T08:41:35.1678753Z stderr:
2019-08-26T08:41:35.1678932Z ------------------------------------------
2019-08-26T08:41:35.1679353Z error: CGU-reuse for `independent_cgus_dont_affect_each_other-bar` is `No` but should be `PreLto`
2019-08-26T08:41:35.1679633Z    |
2019-08-26T08:41:35.1679633Z    |
2019-08-26T08:41:35.1679869Z LL | / #![rustc_expected_cgu_reuse(module="independent_cgus_dont_affect_each_other-bar",
2019-08-26T08:41:35.1679915Z LL | |                             cfg="cfail2",
2019-08-26T08:41:35.1680107Z LL | |                             kind="pre-lto")]
2019-08-26T08:41:35.1680273Z 
2019-08-26T08:41:35.1680473Z error: aborting due to previous error
2019-08-26T08:41:35.1680497Z 
2019-08-26T08:41:35.1680533Z 
---
2019-08-26T08:41:35.1681657Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-26T08:41:35.1681820Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-26T08:41:35.1681855Z 
2019-08-26T08:41:35.1682076Z 
2019-08-26T08:41:35.1683695Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-26T08:41:35.1683952Z 
2019-08-26T08:41:35.1683982Z 
2019-08-26T08:41:35.1684045Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T08:41:35.1684096Z Build completed unsuccessfully in 1:05:08
2019-08-26T08:41:35.1684096Z Build completed unsuccessfully in 1:05:08
2019-08-26T08:41:35.1684141Z == clock drift check ==
2019-08-26T08:41:35.1684205Z   local time: Mon Aug 26 08:41:34 UTC 2019
2019-08-26T08:41:35.1684251Z   network time: Mon, 26 Aug 2019 08:41:34 GMT
2019-08-26T08:41:35.1684297Z == end clock drift check ==
2019-08-26T08:41:38.0282780Z ##[error]Bash exited with code '1'.
2019-08-26T08:41:38.0325435Z ##[section]Starting: Checkout
2019-08-26T08:41:38.0327193Z ==============================================================================
2019-08-26T08:41:38.0327240Z Task         : Get sources
2019-08-26T08:41:38.0327281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
