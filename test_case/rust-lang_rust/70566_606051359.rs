plain
2020-03-30T13:44:14.1358276Z ========================== Starting Command Output ===========================
2020-03-30T13:44:14.1362703Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b1876859-32d6-4d39-a3a1-f838992a2334.sh
2020-03-30T13:44:14.1363213Z 
2020-03-30T13:44:14.1367097Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-30T13:44:14.1387638Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-03-30T13:44:14.1390904Z Task         : Get sources
2020-03-30T13:44:14.1391184Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T13:44:14.1391456Z Version      : 1.0.0
2020-03-30T13:44:14.1392057Z Author       : Microsoft
---
2020-03-30T13:44:15.3371757Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-30T13:44:15.3379317Z ##[command]git config gc.auto 0
2020-03-30T13:44:15.3386046Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-30T13:44:15.3391732Z ##[command]git config --get-all http.proxy
2020-03-30T13:44:15.3402944Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70566/merge:refs/remotes/pull/70566/merge
---
2020-03-30T13:53:23.0604297Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T13:53:24.6731045Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T13:53:26.4012785Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T13:53:26.8626229Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T13:53:37.0076185Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T13:53:39.0316196Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T13:53:43.8751374Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T13:53:48.3302351Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T13:53:59.2679844Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T14:18:08.2884721Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-30T14:18:10.2636502Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-30T14:18:12.5206758Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-30T14:18:14.5001626Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-30T14:18:25.8755434Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-30T14:18:29.7285844Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-30T14:18:35.5661924Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-30T14:18:41.7728583Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-30T14:18:52.7390027Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-30T14:46:04.6807849Z .................................................................................................... 1700/9858
2020-03-30T14:46:08.7894571Z .................................................................................................... 1800/9858
2020-03-30T14:46:17.9762918Z ............................................................................................i....... 1900/9858
2020-03-30T14:46:26.0342328Z .................................................................................................... 2000/9858
2020-03-30T14:46:32.6591596Z ..................................................................................iiiii............. 2100/9858
2020-03-30T14:46:54.7762808Z .................................................................................................... 2300/9858
2020-03-30T14:46:57.0261632Z .................................................................................................... 2400/9858
2020-03-30T14:46:59.4130291Z .................................................................................................... 2500/9858
2020-03-30T14:47:05.4882600Z .................................................................................................... 2600/9858
---
2020-03-30T14:49:59.6033418Z .................................................................................................... 4900/9858
2020-03-30T14:50:04.5890124Z ........................................................i...............i........................... 5000/9858
2020-03-30T14:50:12.6012479Z .................................................................................................... 5100/9858
2020-03-30T14:50:20.2840615Z .................................................................................................... 5200/9858
2020-03-30T14:50:25.5352749Z .i.................................................................................................. 5300/9858
2020-03-30T14:50:36.5258533Z .......................................................................................ii.ii........ 5400/9858
2020-03-30T14:50:40.3473919Z i...i............................................................................................... 5500/9858
2020-03-30T14:50:49.4976737Z ................................i................................................................... 5700/9858
2020-03-30T14:51:00.0028338Z ..................................................ii....................................i........... 5800/9858
2020-03-30T14:51:07.9951007Z .................................................................................................... 5900/9858
2020-03-30T14:51:13.2505902Z .................................................................................................... 6000/9858
2020-03-30T14:51:13.2505902Z .................................................................................................... 6000/9858
2020-03-30T14:51:22.8609173Z ..................................................................................ii...i..ii........ 6100/9858
2020-03-30T14:51:44.4564638Z .................................................................................................... 6300/9858
2020-03-30T14:51:51.6488368Z .................................................................................................... 6400/9858
2020-03-30T14:51:58.8759211Z .................................................................................................... 6500/9858
2020-03-30T14:51:58.8759211Z .................................................................................................... 6500/9858
2020-03-30T14:52:16.4229343Z ............i..ii................................................................................... 6600/9858
2020-03-30T14:52:38.0398998Z .................................................................................................... 6800/9858
2020-03-30T14:52:40.2965791Z ............i....................................................................................... 6900/9858
2020-03-30T14:52:42.5150112Z .................................................................................................... 7000/9858
2020-03-30T14:52:44.7623334Z .................................................i.................................................. 7100/9858
---
2020-03-30T14:54:31.7629326Z .................................................................................................... 7800/9858
2020-03-30T14:54:37.2251681Z .................................................................................................... 7900/9858
2020-03-30T14:54:43.2838399Z .................................................................................................... 8000/9858
2020-03-30T14:54:52.3998010Z .........i.......................................................................................... 8100/9858
2020-03-30T14:55:00.5650702Z ..........................................................iiiiiiiiii.i.............................. 8200/9858
2020-03-30T14:55:15.6542901Z ..i......i.......................................................................................... 8400/9858
2020-03-30T14:55:20.9056922Z .................................................................................................... 8500/9858
2020-03-30T14:55:33.6337130Z .................................................................................................... 8600/9858
2020-03-30T14:55:43.9263801Z .................................................................................................... 8700/9858
---
2020-03-30T14:57:30.8713732Z ..............................................................i..................................... 9800/9858
2020-03-30T14:57:43.7481516Z ..........................................................
2020-03-30T14:57:43.7484287Z failures:
2020-03-30T14:57:43.7526338Z 
2020-03-30T14:57:43.7527605Z ---- [ui] ui/associated-const/lints-used-unused.rs#unused stdout ----
2020-03-30T14:57:43.7528587Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7529210Z   --> $DIR/lints-used-unused.rs:10:20
2020-03-30T14:57:43.7529543Z    |
2020-03-30T14:57:43.7529831Z LL |     const N: i32 = 1 << 42;
---
2020-03-30T14:57:43.7536240Z 
2020-03-30T14:57:43.7536565Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7537180Z   --> $DIR/lints-used-unused.rs:21:29
2020-03-30T14:57:43.7537580Z    |
2020-03-30T14:57:43.7538129Z LL |     const N: i32 = --T::N + (-i32::MIN);
2020-03-30T14:57:43.7538950Z 
2020-03-30T14:57:43.7539265Z error: aborting due to 3 previous errors
2020-03-30T14:57:43.7539540Z 
2020-03-30T14:57:43.7539737Z 
2020-03-30T14:57:43.7539737Z 
2020-03-30T14:57:43.7539944Z 
2020-03-30T14:57:43.7540147Z 
2020-03-30T14:57:43.7540481Z The actual stderr differed from the expected stderr.
2020-03-30T14:57:43.7541381Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.unused/lints-used-unused.unused.stderr
2020-03-30T14:57:43.7542260Z To update references, rerun the tests and pass the `--bless` flag
2020-03-30T14:57:43.7543240Z To only update this specific test, also pass `--test-args associated-const/lints-used-unused.rs`
2020-03-30T14:57:43.7543972Z error in revision `unused`: 1 errors occurred comparing output.
2020-03-30T14:57:43.7544366Z status: exit code: 1
2020-03-30T14:57:43.7544366Z status: exit code: 1
2020-03-30T14:57:43.7546603Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/lints-used-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unused" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.unused/auxiliary"
2020-03-30T14:57:43.7550774Z ------------------------------------------
2020-03-30T14:57:43.7551094Z 
2020-03-30T14:57:43.7551572Z ------------------------------------------
2020-03-30T14:57:43.7551916Z stderr:
2020-03-30T14:57:43.7551916Z stderr:
2020-03-30T14:57:43.7552405Z ------------------------------------------
2020-03-30T14:57:43.7552799Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7553477Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:10:20
2020-03-30T14:57:43.7554038Z    |
2020-03-30T14:57:43.7554898Z LL |     const N: i32 = 1 << 42; //~ ERROR this arithmetic operation will overflow
2020-03-30T14:57:43.7556327Z    |
2020-03-30T14:57:43.7556871Z note: the lint level is defined here
2020-03-30T14:57:43.7558215Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:7:9
2020-03-30T14:57:43.7558605Z    |
---
2020-03-30T14:57:43.7561903Z 
2020-03-30T14:57:43.7562216Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7572916Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:21:29
2020-03-30T14:57:43.7573322Z    |
2020-03-30T14:57:43.7574054Z LL |     const N: i32 = --T::N + (-i32::MIN); //~ ERROR this arithmetic operation will overflow
2020-03-30T14:57:43.7574948Z 
2020-03-30T14:57:43.7575145Z error: aborting due to 3 previous errors
2020-03-30T14:57:43.7575498Z 
2020-03-30T14:57:43.7575594Z 
2020-03-30T14:57:43.7575594Z 
2020-03-30T14:57:43.7576009Z ------------------------------------------
2020-03-30T14:57:43.7576183Z 
2020-03-30T14:57:43.7576279Z 
2020-03-30T14:57:43.7576722Z ---- [ui] ui/associated-const/lints-used-unused.rs#used stdout ----
2020-03-30T14:57:43.7577223Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7577683Z   --> $DIR/lints-used-unused.rs:10:20
2020-03-30T14:57:43.7578195Z    |
2020-03-30T14:57:43.7578386Z LL |     const N: i32 = 1 << 42;
---
2020-03-30T14:57:43.7582760Z 
2020-03-30T14:57:43.7582967Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7583393Z   --> $DIR/lints-used-unused.rs:21:29
2020-03-30T14:57:43.7583587Z    |
2020-03-30T14:57:43.7583983Z LL |     const N: i32 = --T::N + (-i32::MIN);
2020-03-30T14:57:43.7584746Z 
2020-03-30T14:57:43.7584947Z error: aborting due to 3 previous errors
2020-03-30T14:57:43.7585280Z 
2020-03-30T14:57:43.7585372Z 
2020-03-30T14:57:43.7585372Z 
2020-03-30T14:57:43.7585468Z 
2020-03-30T14:57:43.7585559Z 
2020-03-30T14:57:43.7585951Z The actual stderr differed from the expected stderr.
2020-03-30T14:57:43.7586845Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.used/lints-used-unused.used.stderr
2020-03-30T14:57:43.7587881Z To update references, rerun the tests and pass the `--bless` flag
2020-03-30T14:57:43.7588689Z To only update this specific test, also pass `--test-args associated-const/lints-used-unused.rs`
2020-03-30T14:57:43.7589172Z error in revision `used`: 1 errors occurred comparing output.
2020-03-30T14:57:43.7589456Z status: exit code: 1
2020-03-30T14:57:43.7589456Z status: exit code: 1
2020-03-30T14:57:43.7591963Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-const/lints-used-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "used" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.used" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Copt-level=2" "--emit" "link" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-const/lints-used-unused.used/auxiliary"
2020-03-30T14:57:43.7594038Z ------------------------------------------
2020-03-30T14:57:43.7594214Z 
2020-03-30T14:57:43.7594867Z ------------------------------------------
2020-03-30T14:57:43.7595071Z stderr:
2020-03-30T14:57:43.7595071Z stderr:
2020-03-30T14:57:43.7595433Z ------------------------------------------
2020-03-30T14:57:43.7595725Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7596270Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:10:20
2020-03-30T14:57:43.7596534Z    |
2020-03-30T14:57:43.7596923Z LL |     const N: i32 = 1 << 42; //~ ERROR this arithmetic operation will overflow
2020-03-30T14:57:43.7597556Z    |
2020-03-30T14:57:43.7597761Z note: the lint level is defined here
2020-03-30T14:57:43.7598297Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:7:9
2020-03-30T14:57:43.7598553Z    |
---
2020-03-30T14:57:43.7600865Z 
2020-03-30T14:57:43.7601063Z error: this arithmetic operation will overflow
2020-03-30T14:57:43.7601789Z   --> /checkout/src/test/ui/associated-const/lints-used-unused.rs:21:29
2020-03-30T14:57:43.7602039Z    |
2020-03-30T14:57:43.7602553Z LL |     const N: i32 = --T::N + (-i32::MIN); //~ ERROR this arithmetic operation will overflow
2020-03-30T14:57:43.7603217Z 
2020-03-30T14:57:43.7603397Z error: aborting due to 3 previous errors
2020-03-30T14:57:43.7603561Z 
2020-03-30T14:57:43.7603653Z 
2020-03-30T14:57:43.7603653Z 
2020-03-30T14:57:43.7604013Z ------------------------------------------
2020-03-30T14:57:43.7604180Z 
2020-03-30T14:57:43.7604272Z 
2020-03-30T14:57:43.7604364Z 
2020-03-30T14:57:43.7604504Z failures:
2020-03-30T14:57:43.7604898Z     [ui] ui/associated-const/lints-used-unused.rs#unused
2020-03-30T14:57:43.7605369Z     [ui] ui/associated-const/lints-used-unused.rs#used
2020-03-30T14:57:43.7606100Z test result: FAILED. 9796 passed; 2 failed; 60 ignored; 0 measured; 0 filtered out
2020-03-30T14:57:43.7606366Z 
2020-03-30T14:57:43.7606833Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-30T14:57:43.7607250Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-30T14:57:43.7607250Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-30T14:57:43.7607476Z 
2020-03-30T14:57:43.7607569Z 
2020-03-30T14:57:43.7611183Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-30T14:57:43.7613876Z 
2020-03-30T14:57:43.7613971Z 
2020-03-30T14:57:43.7614528Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-30T14:57:43.7614898Z Build completed unsuccessfully in 1:08:09
2020-03-30T14:57:43.7614898Z Build completed unsuccessfully in 1:08:09
2020-03-30T14:57:43.7638632Z == clock drift check ==
2020-03-30T14:57:43.7664088Z   local time: Mon Mar 30 14:57:43 UTC 2020
2020-03-30T14:57:44.0357775Z   network time: Mon, 30 Mar 2020 14:57:44 GMT
2020-03-30T14:57:44.0360583Z == end clock drift check ==
2020-03-30T14:57:44.4751863Z 
2020-03-30T14:57:44.4831608Z ##[error]Bash exited with code '1'.
2020-03-30T14:57:44.4846888Z ##[section]Finishing: Run build
2020-03-30T14:57:44.4894984Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-03-30T14:57:44.4900472Z Task         : Get sources
2020-03-30T14:57:44.4900820Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-30T14:57:44.4901284Z Version      : 1.0.0
2020-03-30T14:57:44.4901488Z Author       : Microsoft
2020-03-30T14:57:44.4901488Z Author       : Microsoft
2020-03-30T14:57:44.4901853Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-30T14:57:44.4902232Z ==============================================================================
2020-03-30T14:57:44.8493413Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-30T14:57:44.8549042Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-03-30T14:57:44.8648215Z Cleaning up task key
2020-03-30T14:57:44.8649594Z Start cleaning up orphan processes.
2020-03-30T14:57:44.8843369Z Terminate orphan process: pid (6340) (python)
2020-03-30T14:57:44.9061483Z ##[section]Finishing: Finalize Job
