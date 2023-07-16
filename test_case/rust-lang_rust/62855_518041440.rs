plain
2019-08-04T21:10:39.9069354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-04T21:10:39.9312043Z ##[command]git config gc.auto 0
2019-08-04T21:10:39.9374162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-04T21:10:39.9424258Z ##[command]git config --get-all http.proxy
2019-08-04T21:10:39.9571358Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62855/merge:refs/remotes/pull/62855/merge
---
2019-08-04T21:11:13.0529102Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-04T21:11:13.0529135Z 
2019-08-04T21:11:13.0529309Z   git checkout -b <new-branch-name>
2019-08-04T21:11:13.0529336Z 
2019-08-04T21:11:13.0529372Z HEAD is now at 045cbe7fb Merge e38ddc4773dbcb1f8624390ebb71213270f366da into f01b9f803b59f170f5dabaaa8aedc96abe45bfea
2019-08-04T21:11:13.0687310Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-04T21:11:13.0689486Z ==============================================================================
2019-08-04T21:11:13.0689529Z Task         : Bash
2019-08-04T21:11:13.0689562Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-04T22:10:44.0969139Z .................................................................................................... 1400/8827
2019-08-04T22:10:50.1387125Z .................................................................................................... 1500/8827
2019-08-04T22:11:02.9088236Z ....................................................................i...............i............... 1600/8827
2019-08-04T22:11:10.1395401Z .................................................................................................... 1700/8827
2019-08-04T22:11:25.5104843Z ......................................................iiiii......................................... 1800/8827
2019-08-04T22:11:37.0785447Z .................................................................................................... 2000/8827
2019-08-04T22:11:39.6342265Z .................................................................................................... 2100/8827
2019-08-04T22:11:42.9513693Z .................................................................................................... 2200/8827
2019-08-04T22:11:50.8300198Z .................................................................................................... 2300/8827
---
2019-08-04T22:15:38.4542732Z .................................................................................................... 5200/8827
2019-08-04T22:15:46.7682446Z .....................................................................i.............................. 5300/8827
2019-08-04T22:15:54.2151821Z .................................................................................................... 5400/8827
2019-08-04T22:16:01.1877253Z .................................................................................................... 5500/8827
2019-08-04T22:16:12.4092259Z ...............................................................ii...i..ii............i.............. 5600/8827
2019-08-04T22:16:33.7731948Z .................................................................................................... 5800/8827
2019-08-04T22:16:38.9085596Z .................................................................................................... 5900/8827
2019-08-04T22:16:38.9085596Z .................................................................................................... 5900/8827
2019-08-04T22:16:45.1421004Z ................................................................i..ii............................... 6000/8827
2019-08-04T22:17:14.8565186Z .................................................................................................... 6200/8827
2019-08-04T22:17:17.0076567Z .......i............................................................................................ 6300/8827
2019-08-04T22:17:19.0542313Z ...............................................................................i.................... 6400/8827
2019-08-04T22:17:21.6424845Z .................................................................................................... 6500/8827
---
2019-08-04T22:22:01.0209151Z  finished in 20.220
2019-08-04T22:22:01.0388435Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-04T22:22:01.2059902Z 
2019-08-04T22:22:01.2060142Z running 146 tests
2019-08-04T22:22:04.4389300Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-04T22:22:06.2708245Z iii..............i.........iii.i......ii......
2019-08-04T22:22:06.2709907Z 
2019-08-04T22:22:06.2713218Z  finished in 5.232
2019-08-04T22:22:06.2887258Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-04T22:22:06.4444740Z 
---
2019-08-04T22:22:08.5098635Z  finished in 2.221
2019-08-04T22:22:08.5280709Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-04T22:22:08.6897399Z 
2019-08-04T22:22:08.6898826Z running 9 tests
2019-08-04T22:22:08.6901121Z iiiiiiiii
2019-08-04T22:22:08.6901602Z 
2019-08-04T22:22:08.6901650Z  finished in 0.161
2019-08-04T22:22:08.7083942Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-04T22:22:08.8733528Z 
---
2019-08-04T22:22:26.9084498Z  finished in 18.200
2019-08-04T22:22:26.9270804Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-04T22:22:27.0904386Z 
2019-08-04T22:22:27.0904892Z running 122 tests
2019-08-04T22:22:49.9620711Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-04T22:22:54.4860635Z .i.i......iii.i.....ii
2019-08-04T22:22:54.4861405Z 
2019-08-04T22:22:54.4862160Z  finished in 27.559
2019-08-04T22:22:54.4870515Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-04T22:22:54.4870786Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-04T22:30:22.8028924Z failures:
2019-08-04T22:30:22.8028992Z 
2019-08-04T22:30:22.8029477Z ---- [rustdoc] rustdoc/inline_cross/proc_macro.rs stdout ----
2019-08-04T22:30:22.8029511Z 
2019-08-04T22:30:22.8029553Z error: htmldocck failed!
2019-08-04T22:30:22.8029616Z status: exit code: 1
2019-08-04T22:30:22.8030123Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/proc_macro" "/checkout/src/test/rustdoc/inline_cross/proc_macro.rs"
2019-08-04T22:30:22.8030401Z ------------------------------------------
2019-08-04T22:30:22.8030431Z 
2019-08-04T22:30:22.8030625Z ------------------------------------------
2019-08-04T22:30:22.8030666Z stderr:
2019-08-04T22:30:22.8030666Z stderr:
2019-08-04T22:30:22.8030878Z ------------------------------------------
2019-08-04T22:30:22.8030921Z 16: @has check failed
2019-08-04T22:30:22.8030961Z  `PATTERN` did not match
2019-08-04T22:30:22.8031181Z  // @has - 'Doc comment from the original crate'
2019-08-04T22:30:22.8031461Z Encountered 1 errors
2019-08-04T22:30:22.8031513Z 
2019-08-04T22:30:22.8031791Z ------------------------------------------
2019-08-04T22:30:22.8031820Z 
---
2019-08-04T22:30:22.8032528Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-04T22:30:22.8032584Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-04T22:30:22.8039953Z 
2019-08-04T22:30:22.8040046Z 
2019-08-04T22:30:22.8041684Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-04T22:30:22.8042162Z 
2019-08-04T22:30:22.8042192Z 
2019-08-04T22:30:22.8048115Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-04T22:30:22.8048375Z Build completed unsuccessfully in 1:13:15
2019-08-04T22:30:22.8048375Z Build completed unsuccessfully in 1:13:15
2019-08-04T22:30:24.7581533Z ##[error]Bash exited with code '1'.
2019-08-04T22:30:24.7663490Z ##[section]Starting: Checkout
2019-08-04T22:30:24.7665169Z ==============================================================================
2019-08-04T22:30:24.7665628Z Task         : Get sources
2019-08-04T22:30:24.7665698Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
