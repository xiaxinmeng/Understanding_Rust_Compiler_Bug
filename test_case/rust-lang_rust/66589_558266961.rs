plain
2019-11-25T16:53:14.0574816Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T16:53:14.0587540Z ##[command]git config gc.auto 0
2019-11-25T16:53:14.0590136Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T16:53:14.0593229Z ##[command]git config --get-all http.proxy
2019-11-25T16:53:14.0595167Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66589/merge:refs/remotes/pull/66589/merge
---
2019-11-25T17:41:34.7068795Z .................................................................................................... 1600/9291
2019-11-25T17:41:38.5051444Z .................................................................................................... 1700/9291
2019-11-25T17:41:49.3305497Z ...............................i.................................................................... 1800/9291
2019-11-25T17:41:54.7856854Z .................................................................................................... 1900/9291
2019-11-25T17:42:05.9197470Z ................iiiii............................................................................... 2000/9291
2019-11-25T17:42:13.8933158Z .................................................................................................... 2200/9291
2019-11-25T17:42:15.8289160Z .................................................................................................... 2300/9291
2019-11-25T17:42:19.8952143Z .................................................................................................... 2400/9291
2019-11-25T17:42:36.6801131Z .................................................................................................... 2500/9291
---
2019-11-25T17:44:46.6788991Z ................i...............i................................................................... 4800/9291
2019-11-25T17:44:55.3258049Z .................................................................................................... 4900/9291
2019-11-25T17:45:00.0404732Z .................................................................................................... 5000/9291
2019-11-25T17:45:07.9312629Z .................................................................................................... 5100/9291
2019-11-25T17:45:13.1232799Z .....................ii.ii...........i.............................................................. 5200/9291
2019-11-25T17:45:20.6957254Z .................................................................................................... 5400/9291
2019-11-25T17:45:29.6996626Z .................................................................................................... 5500/9291
2019-11-25T17:45:35.7392049Z ...i................................................................................................ 5600/9291
2019-11-25T17:45:40.6159834Z .................................................................................................... 5700/9291
2019-11-25T17:45:40.6159834Z .................................................................................................... 5700/9291
2019-11-25T17:45:49.0663019Z .........................................................................................ii...i..ii. 5800/9291
2019-11-25T17:46:08.1796062Z .................................................................................................... 6000/9291
2019-11-25T17:46:14.0707164Z .................................................................................................... 6100/9291
2019-11-25T17:46:17.2241993Z .................................................................................................... 6200/9291
2019-11-25T17:46:17.2241993Z .................................................................................................... 6200/9291
2019-11-25T17:46:28.2561406Z ............i..ii................................................................................... 6300/9291
2019-11-25T17:46:43.6013209Z ................................................................................i................... 6500/9291
2019-11-25T17:46:45.3457344Z .................................................................................................... 6600/9291
2019-11-25T17:46:47.1393874Z .......................................................................i............................ 6700/9291
2019-11-25T17:46:49.5759988Z .................................................................................................... 6800/9291
---
2019-11-25T17:50:36.0992345Z ---- [ui] ui/terminal-width/non-whitespace-trimming-unicode.rs stdout ----
2019-11-25T17:50:36.0992685Z diff of stderr:
2019-11-25T17:50:36.0992818Z 
2019-11-25T17:50:36.0992893Z 3    |
2019-11-25T17:50:36.0993720Z 4 LL | ...♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄...
2019-11-25T17:50:36.0994175Z 5    |                                            --   ^^ expected `()`, found integer
2019-11-25T17:50:36.0994457Z +    |                                            |
2019-11-25T17:50:36.0994500Z 7    |                                            expected due to this
2019-11-25T17:50:36.0994554Z 8 
2019-11-25T17:50:36.0994590Z 9 error: aborting due to previous error
2019-11-25T17:50:36.0994590Z 9 error: aborting due to previous error
2019-11-25T17:50:36.0994616Z 
2019-11-25T17:50:36.0994637Z 
2019-11-25T17:50:36.0994691Z The actual stderr differed from the expected stderr.
2019-11-25T17:50:36.0994994Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-whitespace-trimming-unicode/non-whitespace-trimming-unicode.stderr
2019-11-25T17:50:36.0995269Z To update references, rerun the tests and pass the `--bless` flag
2019-11-25T17:50:36.0995543Z To only update this specific test, also pass `--test-args terminal-width/non-whitespace-trimming-unicode.rs`
2019-11-25T17:50:36.0995626Z error: 1 errors occurred comparing output.
2019-11-25T17:50:36.0995686Z status: exit code: 1
2019-11-25T17:50:36.0995686Z status: exit code: 1
2019-11-25T17:50:36.0996998Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/terminal-width/non-whitespace-trimming-unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-whitespace-trimming-unicode" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-whitespace-trimming-unicode/auxiliary" "-A" "unused"
2019-11-25T17:50:36.0997297Z ------------------------------------------
2019-11-25T17:50:36.0997326Z 
2019-11-25T17:50:36.0997530Z ------------------------------------------
2019-11-25T17:50:36.0997577Z stderr:
2019-11-25T17:50:36.0997577Z stderr:
2019-11-25T17:50:36.0997761Z ------------------------------------------
2019-11-25T17:50:36.0997819Z error[E0308]: mismatched types
2019-11-25T17:50:36.0998278Z   --> /checkout/src/test/ui/terminal-width/non-whitespace-trimming-unicode.rs:4:415
2019-11-25T17:50:36.0998324Z    |
2019-11-25T17:50:36.0998729Z LL | ...♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄...
2019-11-25T17:50:36.0998974Z    |                                            --   ^^ expected `()`, found integer
2019-11-25T17:50:36.0999193Z    |                                            expected due to this
2019-11-25T17:50:36.0999222Z 
2019-11-25T17:50:36.0999428Z error: aborting due to previous error
2019-11-25T17:50:36.0999453Z 
---
2019-11-25T17:50:36.1032293Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-25T17:50:36.1032621Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-25T17:50:36.1054464Z 
2019-11-25T17:50:36.1054524Z 
2019-11-25T17:50:36.1058733Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-25T17:50:36.1059720Z 
2019-11-25T17:50:36.1074281Z 
2019-11-25T17:50:36.1074367Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-25T17:50:36.1074870Z Build completed unsuccessfully in 0:52:10
2019-11-25T17:50:36.1074870Z Build completed unsuccessfully in 0:52:10
2019-11-25T17:50:36.1129079Z == clock drift check ==
2019-11-25T17:50:36.1139175Z   local time: Mon Nov 25 17:50:36 UTC 2019
2019-11-25T17:50:36.1953969Z   network time: Mon, 25 Nov 2019 17:50:36 GMT
2019-11-25T17:50:36.1957210Z == end clock drift check ==
2019-11-25T17:50:37.0326501Z 
2019-11-25T17:50:37.0406812Z ##[error]Bash exited with code '1'.
2019-11-25T17:50:37.0433508Z ##[section]Starting: Checkout
2019-11-25T17:50:37.0435097Z ==============================================================================
2019-11-25T17:50:37.0435141Z Task         : Get sources
2019-11-25T17:50:37.0435191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
