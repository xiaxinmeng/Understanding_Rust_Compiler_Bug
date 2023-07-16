plain
2019-08-17T15:50:19.3669227Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-17T15:50:19.3860996Z ##[command]git config gc.auto 0
2019-08-17T15:50:19.3949356Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-17T15:50:19.4001162Z ##[command]git config --get-all http.proxy
2019-08-17T15:50:19.4159434Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63580/merge:refs/remotes/pull/63580/merge
---
2019-08-17T15:50:53.7256010Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-17T15:50:53.7256044Z 
2019-08-17T15:50:53.7256281Z   git checkout -b <new-branch-name>
2019-08-17T15:50:53.7256334Z 
2019-08-17T15:50:53.7256404Z HEAD is now at 1af8fe9af Merge 027857044dde3fdc34d3d92ddf319930b56f5180 into ac60ca0643feb3452688a9ca97c839c155742915
2019-08-17T15:50:53.7416456Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-17T15:50:53.7419835Z ==============================================================================
2019-08-17T15:50:53.7419952Z Task         : Bash
2019-08-17T15:50:53.7420006Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-17T16:53:46.2894135Z .................................................................................................... 1500/8924
2019-08-17T16:53:51.8850662Z .................................................................................................... 1600/8924
2019-08-17T16:54:04.8408939Z ................................i...............i................................................... 1700/8924
2019-08-17T16:54:12.3028641Z .................................................................................................... 1800/8924
2019-08-17T16:54:26.7920114Z .......................iiiii........................................................................ 1900/8924
2019-08-17T16:54:37.3322968Z .................................................................................................... 2100/8924
2019-08-17T16:54:39.9548309Z .................................................................................................... 2200/8924
2019-08-17T16:54:44.8694178Z .................................................................................................... 2300/8924
2019-08-17T16:54:51.7498796Z .................................................................................................... 2400/8924
---
2019-08-17T16:57:45.8711989Z .................................................................................................... 4600/8924
2019-08-17T16:57:52.9727911Z .....i...............i.............................................................................. 4700/8924
2019-08-17T16:58:04.2504157Z .................................................................................................... 4800/8924
2019-08-17T16:58:10.0477737Z .................................................................................................... 4900/8924
2019-08-17T16:58:22.0118834Z ......................................................................................ii.ii......... 5000/8924
2019-08-17T16:58:31.3520378Z .................................................................................................... 5200/8924
2019-08-17T16:58:41.0081345Z .................................................................................................... 5300/8924
2019-08-17T16:58:47.9921789Z ..........................................i......................................................... 5400/8924
2019-08-17T16:58:54.4433460Z .................................................................................................... 5500/8924
2019-08-17T16:58:54.4433460Z .................................................................................................... 5500/8924
2019-08-17T16:59:05.3220191Z .................................................................................................... 5600/8924
2019-08-17T16:59:17.1817047Z ...................................ii...i..ii...........i........................................... 5700/8924
2019-08-17T16:59:34.0168073Z .................................................................................................... 5900/8924
2019-08-17T16:59:39.0073049Z .................................................................................................... 6000/8924
2019-08-17T16:59:39.0073049Z .................................................................................................... 6000/8924
2019-08-17T16:59:53.5053549Z ....................................i..ii........................................................... 6100/8924
2019-08-17T17:00:13.1631064Z ..............................................................................i..................... 6300/8924
2019-08-17T17:00:15.4536680Z .................................................................................................... 6400/8924
2019-08-17T17:00:17.7293303Z ..................................................i................................................. 6500/8924
2019-08-17T17:00:20.9493114Z .................................................................................................... 6600/8924
---
2019-08-17T17:05:02.1173308Z  finished in 21.091
2019-08-17T17:05:02.1359549Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-17T17:05:02.3014988Z 
2019-08-17T17:05:02.3016994Z running 148 tests
2019-08-17T17:05:05.6131876Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/148
2019-08-17T17:05:07.6319351Z .iiii..............i.........iii.i......ii......
2019-08-17T17:05:07.6323093Z 
2019-08-17T17:05:07.6388982Z  finished in 5.496
2019-08-17T17:05:07.6511325Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-17T17:05:07.8094559Z 
---
2019-08-17T17:05:09.8638574Z  finished in 2.212
2019-08-17T17:05:09.8834916Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-17T17:05:10.0403957Z 
2019-08-17T17:05:10.0404318Z running 9 tests
2019-08-17T17:05:10.0405396Z iiiiiiiii
2019-08-17T17:05:10.0405880Z 
2019-08-17T17:05:10.0410983Z  finished in 0.157
2019-08-17T17:05:10.0632718Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-17T17:05:10.2263585Z 
2019-08-17T17:05:10.2263585Z 
2019-08-17T17:05:10.2263833Z running 104 tests
2019-08-17T17:05:27.6531622Z ............................F....................................................................... 100/104
2019-08-17T17:05:28.2983106Z ....
2019-08-17T17:05:28.2983410Z failures:
2019-08-17T17:05:28.2983877Z 
2019-08-17T17:05:28.2985154Z ---- [incremental] incremental/hashes/for_loops.rs stdout ----
2019-08-17T17:05:28.2985212Z 
2019-08-17T17:05:28.2985511Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-08-17T17:05:28.2985570Z status: exit code: 1
2019-08-17T17:05:28.2986735Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/for_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary"
2019-08-17T17:05:28.2987155Z ------------------------------------------
2019-08-17T17:05:28.2987194Z 
2019-08-17T17:05:28.2987460Z ------------------------------------------
2019-08-17T17:05:28.2987511Z stderr:
2019-08-17T17:05:28.2987511Z stderr:
2019-08-17T17:05:28.2987746Z ------------------------------------------
2019-08-17T17:05:28.2987886Z error: `optimized_mir(change_iterable)` should be dirty but is not
2019-08-17T17:05:28.2988615Z    |
2019-08-17T17:05:28.2988615Z    |
2019-08-17T17:05:28.2988721Z LL | / pub fn change_iterable() {
2019-08-17T17:05:28.2988771Z LL | |     let mut _x = 0;
2019-08-17T17:05:28.2988821Z LL | |     for _ in &[0, 1, 3] {
2019-08-17T17:05:28.2988888Z LL | |         _x = 1;
2019-08-17T17:05:28.2988997Z LL | |     }
2019-08-17T17:05:28.2989045Z LL | | }
2019-08-17T17:05:28.2989109Z    | |_^
2019-08-17T17:05:28.2989140Z 
---
2019-08-17T17:05:28.3003737Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-17T17:05:28.3004571Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-17T17:05:28.3004757Z 
2019-08-17T17:05:28.3005131Z 
2019-08-17T17:05:28.3007193Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-17T17:05:28.3007744Z 
2019-08-17T17:05:28.3007902Z 
2019-08-17T17:05:28.3015264Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-17T17:05:28.3015557Z Build completed unsuccessfully in 1:07:59
2019-08-17T17:05:28.3015557Z Build completed unsuccessfully in 1:07:59
2019-08-17T17:05:28.3076813Z == clock drift check ==
2019-08-17T17:05:28.3094512Z   local time: Sat Aug 17 17:05:28 UTC 2019
2019-08-17T17:05:28.3955895Z   network time: Sat, 17 Aug 2019 17:05:28 GMT
2019-08-17T17:05:28.3961248Z == end clock drift check ==
2019-08-17T17:05:32.4049459Z ##[error]Bash exited with code '1'.
2019-08-17T17:05:32.4094165Z ##[section]Starting: Checkout
2019-08-17T17:05:32.4096182Z ==============================================================================
2019-08-17T17:05:32.4096243Z Task         : Get sources
2019-08-17T17:05:32.4096312Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
