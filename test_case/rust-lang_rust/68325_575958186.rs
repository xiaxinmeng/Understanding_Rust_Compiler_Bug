plain
2020-01-19T00:52:05.8433847Z ========================== Starting Command Output ===========================
2020-01-19T00:52:05.8452967Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a7ef51d6-fe95-4f2b-b7f7-4baea0ac2577.sh
2020-01-19T00:52:05.8726551Z 
2020-01-19T00:52:05.8765347Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-19T00:52:05.8773899Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68325/merge to s
2020-01-19T00:52:05.8775566Z Task         : Get sources
2020-01-19T00:52:05.8775599Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T00:52:05.8775646Z Version      : 1.0.0
2020-01-19T00:52:05.8775679Z Author       : Microsoft
---
2020-01-19T00:52:06.7700038Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-19T00:52:06.7784037Z ##[command]git config gc.auto 0
2020-01-19T00:52:06.7875126Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-19T00:52:06.7931077Z ##[command]git config --get-all http.proxy
2020-01-19T00:52:06.8081038Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68325/merge:refs/remotes/pull/68325/merge
---
2020-01-19T01:48:43.4958193Z .................................................................................................... 1700/9537
2020-01-19T01:48:50.1646560Z .................................................................................................... 1800/9537
2020-01-19T01:49:02.1220357Z ...................i................................................................................ 1900/9537
2020-01-19T01:49:09.5344075Z .................................................................................................... 2000/9537
2020-01-19T01:49:25.7165329Z .........iiiii...................................................................................... 2100/9537
2020-01-19T01:49:35.8729448Z .................................................................................................... 2300/9537
2020-01-19T01:49:38.4361359Z .................................................................................................... 2400/9537
2020-01-19T01:49:44.0806317Z .................................................................................................... 2500/9537
2020-01-19T01:50:05.7910642Z .................................................................................................... 2600/9537
---
2020-01-19T01:52:53.7098035Z ....................................................i...............i............................... 4900/9537
2020-01-19T01:53:02.1333198Z .................................................................................................... 5000/9537
2020-01-19T01:53:10.2591342Z ...............................................................................................i.... 5100/9537
2020-01-19T01:53:15.6429930Z .................................................................................................... 5200/9537
2020-01-19T01:53:26.7072794Z ...................................................................ii.ii...........i................ 5300/9537
2020-01-19T01:53:36.0278977Z ....i............................................................................................... 5500/9537
2020-01-19T01:53:46.3408057Z .................................................................................................... 5600/9537
2020-01-19T01:53:53.2337327Z .....................................................i.............................................. 5700/9537
2020-01-19T01:54:00.4052293Z .................................................................................................... 5800/9537
2020-01-19T01:54:00.4052293Z .................................................................................................... 5800/9537
2020-01-19T01:54:10.9868550Z .................................................................................................... 5900/9537
2020-01-19T01:54:18.1102845Z ...........................................ii...i..ii...........i................................... 6000/9537
2020-01-19T01:54:41.0256643Z .................................................................................................... 6200/9537
2020-01-19T01:54:49.6810445Z .................................................................................................... 6300/9537
2020-01-19T01:54:49.6810445Z .................................................................................................... 6300/9537
2020-01-19T01:55:00.2147332Z .......................................................................i..ii........................ 6400/9537
2020-01-19T01:55:31.7039620Z .................................................................................................... 6600/9537
2020-01-19T01:55:34.8074635Z ...............................................i.................................................... 6700/9537
2020-01-19T01:55:37.0867590Z .................................................................................................... 6800/9537
2020-01-19T01:55:39.6487250Z ..............................................i..................................................... 6900/9537
---
2020-01-19T01:57:22.9810753Z .................................................................................................... 7500/9537
2020-01-19T01:57:27.3522640Z .................................................................................................... 7600/9537
2020-01-19T01:57:33.4806057Z .................................................................................................... 7700/9537
2020-01-19T01:57:40.4919905Z .................................................................................................... 7800/9537
2020-01-19T01:57:51.9072650Z .................................................................................................iii 7900/9537
2020-01-19T01:57:58.5942797Z iiii................................................................................................ 8000/9537
2020-01-19T01:58:14.5754422Z .................................................................................................... 8200/9537
2020-01-19T01:58:26.9128355Z .................................................................................................... 8300/9537
2020-01-19T01:58:39.9244209Z .................................................................................................... 8400/9537
2020-01-19T01:58:46.0457389Z .................................................................................................... 8500/9537
---
2020-01-19T02:01:15.9155287Z  finished in 7.910
2020-01-19T02:01:15.9351275Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-19T02:01:16.1556781Z 
2020-01-19T02:01:16.1556938Z running 166 tests
2020-01-19T02:01:19.3654405Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-19T02:01:21.7205227Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-19T02:01:21.7205916Z 
2020-01-19T02:01:21.7214040Z  finished in 5.786
2020-01-19T02:01:21.7403889Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-19T02:01:21.9103526Z 
---
2020-01-19T02:01:23.9656611Z  finished in 2.225
2020-01-19T02:01:23.9858691Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-19T02:01:24.1575602Z 
2020-01-19T02:01:24.1576467Z running 9 tests
2020-01-19T02:01:24.1577407Z iiiiiiiii
2020-01-19T02:01:24.1580238Z 
2020-01-19T02:01:24.1580526Z  finished in 0.172
2020-01-19T02:01:24.1777612Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-19T02:01:24.3778413Z 
---
2020-01-19T02:01:45.7591571Z  finished in 21.581
2020-01-19T02:01:45.7851623Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-19T02:01:45.9922307Z 
2020-01-19T02:01:45.9922869Z running 116 tests
2020-01-19T02:02:12.0240079Z .iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii 100/116
2020-01-19T02:02:15.6065024Z .....iiii.....ii
2020-01-19T02:02:15.6066301Z 
2020-01-19T02:02:15.6069461Z  finished in 29.822
2020-01-19T02:02:15.6075743Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-19T02:02:15.6077013Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-19T02:09:54.0143198Z ------------------------------------------
2020-01-19T02:09:54.0143458Z error[E0658]: use of unstable library feature 'assoc_int_consts': recently added
2020-01-19T02:09:54.0143702Z   --> /checkout/src/test/rustdoc/show-const-contents.rs:52:26
2020-01-19T02:09:54.0143770Z    |
2020-01-19T02:09:54.0143816Z 52 | pub const EPSILON: f32 = f32::EPSILON;
2020-01-19T02:09:54.0143935Z    |
2020-01-19T02:09:54.0143935Z    |
2020-01-19T02:09:54.0144327Z    = note: for more information, see ***/issues/1
2020-01-19T02:09:54.0144446Z 
2020-01-19T02:09:54.0144490Z error: aborting due to previous error
2020-01-19T02:09:54.0144520Z 
2020-01-19T02:09:54.0144780Z For more information about this error, try `rustc --explain E0658`.
---
2020-01-19T02:09:54.0150634Z . 331 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
2020-01-19T02:09:54.0150682Z 
2020-01-19T02:09:54.0150716Z 
2020-01-19T02:09:54.0150740Z 
2020-01-19T02:09:54.0153124Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-19T02:09:54.0153402Z 
2020-01-19T02:09:54.0153431Z 
2020-01-19T02:09:54.0167503Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-19T02:09:54.0167918Z Build completed unsuccessfully in 1:12:14
2020-01-19T02:09:54.0167918Z Build completed unsuccessfully in 1:12:14
2020-01-19T02:09:54.0225806Z == clock drift check ==
2020-01-19T02:09:54.0245734Z   local time: Sun Jan 19 02:09:54 UTC 2020
2020-01-19T02:09:54.1245714Z   network time: Sun, 19 Jan 2020 02:09:54 GMT
2020-01-19T02:09:54.1249537Z == end clock drift check ==
2020-01-19T02:09:55.2623876Z 
2020-01-19T02:09:55.2726511Z ##[error]Bash exited with code '1'.
2020-01-19T02:09:55.2740777Z ##[section]Finishing: Run build
2020-01-19T02:09:55.2762522Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68325/merge to s
2020-01-19T02:09:55.2765471Z Task         : Get sources
2020-01-19T02:09:55.2765516Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T02:09:55.2765579Z Version      : 1.0.0
2020-01-19T02:09:55.2765620Z Author       : Microsoft
2020-01-19T02:09:55.2765620Z Author       : Microsoft
2020-01-19T02:09:55.2765663Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-19T02:09:55.2765749Z ==============================================================================
2020-01-19T02:09:55.7364176Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-19T02:09:55.7410815Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68325/merge to s
2020-01-19T02:09:55.7546035Z Cleaning up task key
2020-01-19T02:09:55.7546980Z Start cleaning up orphan processes.
2020-01-19T02:09:55.7673813Z Terminate orphan process: pid (3595) (python)
2020-01-19T02:09:55.7969006Z ##[section]Finishing: Finalize Job
