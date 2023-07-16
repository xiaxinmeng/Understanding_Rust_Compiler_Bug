plain
2019-08-02T00:27:37.3525795Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T00:27:37.3731679Z ##[command]git config gc.auto 0
2019-08-02T00:27:37.3771270Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T00:27:37.3840387Z ##[command]git config --get-all http.proxy
2019-08-02T00:27:37.3981024Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63180/merge:refs/remotes/pull/63180/merge
---
2019-08-02T00:28:13.0413487Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T00:28:13.0413515Z 
2019-08-02T00:28:13.0413740Z   git checkout -b <new-branch-name>
2019-08-02T00:28:13.0413768Z 
2019-08-02T00:28:13.0413824Z HEAD is now at 08ca2df43 Merge b09e8bd88aff29953aa341b1d431a530126591b5 into 435236b8877cdb98c82eaebfb7887782277265c5
2019-08-02T00:28:13.0566576Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T00:28:13.0569962Z ==============================================================================
2019-08-02T00:28:13.0570176Z Task         : Bash
2019-08-02T00:28:13.0570224Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T01:29:26.5869633Z .................................................................................................... 1400/8821
2019-08-02T01:29:32.6926154Z .................................................................................................... 1500/8821
2019-08-02T01:29:45.7275347Z .................................................................i...............i.................. 1600/8821
2019-08-02T01:29:53.4239824Z .................................................................................................... 1700/8821
2019-08-02T01:30:08.8033535Z ...................................................iiiii............................................ 1800/8821
2019-08-02T01:30:20.4723425Z .................................................................................................... 2000/8821
2019-08-02T01:30:23.0240312Z .................................................................................................... 2100/8821
2019-08-02T01:30:26.5452452Z ........................................................................................F........... 2200/8821
2019-08-02T01:30:36.4707437Z .................................................................................................... 2300/8821
---
2019-08-02T01:34:29.7956469Z .................................................................................................... 5200/8821
2019-08-02T01:34:38.2574064Z ..................................................................i................................. 5300/8821
2019-08-02T01:34:45.7647550Z .................................................................................................... 5400/8821
2019-08-02T01:34:52.9793832Z .................................................................................................... 5500/8821
2019-08-02T01:35:04.7595500Z ............................................................ii...i..ii...........i.................. 5600/8821
2019-08-02T01:35:27.1302958Z .................................................................................................... 5800/8821
2019-08-02T01:35:32.4882979Z .................................................................................................... 5900/8821
2019-08-02T01:35:32.4882979Z .................................................................................................... 5900/8821
2019-08-02T01:35:39.5618089Z ............................................................i..ii................................... 6000/8821
2019-08-02T01:36:08.3035331Z .................................................................................................... 6200/8821
2019-08-02T01:36:10.4821808Z ...i................................................................................................ 6300/8821
2019-08-02T01:36:12.7596625Z ..........................................................................i......................... 6400/8821
2019-08-02T01:36:15.6605583Z .................................................................................................... 6500/8821
---
2019-08-02T01:40:23.7210564Z failures:
2019-08-02T01:40:23.7238833Z 
2019-08-02T01:40:23.7239915Z ---- [ui] ui/existential_types/issue-58951.rs stdout ----
2019-08-02T01:40:23.7240136Z 
2019-08-02T01:40:23.7240550Z error: test compilation failed although it shouldn't!
2019-08-02T01:40:23.7240779Z status: exit code: 1
2019-08-02T01:40:23.7241858Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/issue-58951.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/issue-58951" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/issue-58951/auxiliary" "-A" "unused"
2019-08-02T01:40:23.7242555Z ------------------------------------------
2019-08-02T01:40:23.7242763Z 
2019-08-02T01:40:23.7243514Z ------------------------------------------
2019-08-02T01:40:23.7243714Z stderr:
2019-08-02T01:40:23.7243714Z stderr:
2019-08-02T01:40:23.7244099Z ------------------------------------------
2019-08-02T01:40:23.7244297Z error: expected one of `!` or `::`, found `type`
2019-08-02T01:40:23.7244886Z   --> /checkout/src/test/ui/existential_types/issue-58951.rs:4:13
2019-08-02T01:40:23.7245357Z    |
2019-08-02T01:40:23.7245533Z LL | existential type A: Iterator;
2019-08-02T01:40:23.7245714Z    |             ^^^^ expected one of `!` or `::` here
2019-08-02T01:40:23.7246036Z error: aborting due to previous error
2019-08-02T01:40:23.7246173Z 
2019-08-02T01:40:23.7246342Z 
2019-08-02T01:40:23.7246758Z ------------------------------------------
---
2019-08-02T01:40:23.7278510Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-08-02T01:40:23.7278849Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-02T01:40:23.7291254Z 
2019-08-02T01:40:23.7291561Z 
2019-08-02T01:40:23.7299229Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-02T01:40:23.7300688Z 
2019-08-02T01:40:23.7300866Z 
2019-08-02T01:40:23.7310410Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-02T01:40:23.7315898Z Build completed unsuccessfully in 1:06:14
2019-08-02T01:40:23.7315898Z Build completed unsuccessfully in 1:06:14
2019-08-02T01:40:24.5253533Z ##[error]Bash exited with code '1'.
2019-08-02T01:40:24.5328130Z ##[section]Starting: Checkout
2019-08-02T01:40:24.5330004Z ==============================================================================
2019-08-02T01:40:24.5330057Z Task         : Get sources
2019-08-02T01:40:24.5330104Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
