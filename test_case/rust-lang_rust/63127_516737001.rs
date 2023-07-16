plain
2019-07-31T06:29:18.0498919Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T06:29:18.0705745Z ##[command]git config gc.auto 0
2019-07-31T06:29:18.0789853Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T06:29:18.0849546Z ##[command]git config --get-all http.proxy
2019-07-31T06:29:18.7686253Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-07-31T06:29:54.6191935Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T06:29:54.6191973Z 
2019-07-31T06:29:54.6192212Z   git checkout -b <new-branch-name>
2019-07-31T06:29:54.6192245Z 
2019-07-31T06:29:54.6192299Z HEAD is now at 49faceba2 Merge 71a66ff6e0770e0d4b43f3a87d5720b10eaf1049 into 4a18848e05b0957474fdb5be162502742b5eb9fd
2019-07-31T06:29:54.6325725Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T06:29:54.6328496Z ==============================================================================
2019-07-31T06:29:54.6328549Z Task         : Bash
2019-07-31T06:29:54.6328625Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T07:33:33.7591514Z .................................................................................................... 1400/8819
2019-07-31T07:33:39.8753404Z .................................................................................................... 1500/8819
2019-07-31T07:33:53.0039754Z .................................................................i...............i.................. 1600/8819
2019-07-31T07:34:00.7890261Z .................................................................................................... 1700/8819
2019-07-31T07:34:16.3915648Z ...................................................iiiii............................................ 1800/8819
2019-07-31T07:34:28.0053481Z .................................................................................................... 2000/8819
2019-07-31T07:34:30.6565489Z .................................................................................................... 2100/8819
2019-07-31T07:34:34.4732077Z .................................................................................................... 2200/8819
2019-07-31T07:34:41.2355966Z .................................................................................................... 2300/8819
---
2019-07-31T07:38:46.0078881Z .................................................................................................... 5300/8819
2019-07-31T07:38:53.7013998Z ..............i..................................................................................... 5400/8819
2019-07-31T07:38:59.5068145Z .................................................................................................... 5500/8819
2019-07-31T07:39:12.3586238Z .................................................................................................... 5600/8819
2019-07-31T07:39:26.9841578Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-07-31T07:39:42.2075028Z .................................................................................................... 5900/8819
2019-07-31T07:39:47.2689238Z .................................................................................................... 6000/8819
2019-07-31T07:39:47.2689238Z .................................................................................................... 6000/8819
2019-07-31T07:40:01.8824792Z ........i..ii....................................................................................... 6100/8819
2019-07-31T07:40:21.6647458Z ....................................................i............................................... 6300/8819
2019-07-31T07:40:23.9023541Z .................................................................................................... 6400/8819
2019-07-31T07:40:26.4883887Z .....................i.............................................................................. 6500/8819
2019-07-31T07:40:31.1696809Z .................................................................................................... 6600/8819
---
2019-07-31T07:44:35.2162422Z 
2019-07-31T07:44:35.2163229Z ---- [ui] ui/async-await/no-args-non-move-async-closure.rs stdout ----
2019-07-31T07:44:35.2163290Z diff of stderr:
2019-07-31T07:44:35.2163478Z 
2019-07-31T07:44:35.2163882Z - error[E0708]: `async` non-`move` closures with arguments are not currently supported
2019-07-31T07:44:35.2164338Z + error[E0708]: `async` non-`move` closures with parameters are not currently supported
2019-07-31T07:44:35.2164990Z 3    |
2019-07-31T07:44:35.2164990Z 3    |
2019-07-31T07:44:35.2165104Z 4 LL |     let _ = async |x: u8| {};
2019-07-31T07:44:35.2165527Z 
2019-07-31T07:44:35.2165584Z The actual stderr differed from the expected stderr.
2019-07-31T07:44:35.2166023Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
2019-07-31T07:44:35.2166023Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
2019-07-31T07:44:35.2166289Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T07:44:35.2166979Z To only update this specific test, also pass `--test-args async-await/no-args-non-move-async-closure.rs`
2019-07-31T07:44:35.2167090Z error: 1 errors occurred comparing output.
2019-07-31T07:44:35.2167136Z status: exit code: 1
2019-07-31T07:44:35.2167136Z status: exit code: 1
2019-07-31T07:44:35.2167934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-args-non-move-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/auxiliary" "-A" "unused"
2019-07-31T07:44:35.2168300Z ------------------------------------------
2019-07-31T07:44:35.2168333Z 
2019-07-31T07:44:35.2168552Z ------------------------------------------
2019-07-31T07:44:35.2168597Z stderr:
2019-07-31T07:44:35.2168597Z stderr:
2019-07-31T07:44:35.2168823Z ------------------------------------------
2019-07-31T07:44:35.2169077Z error[E0708]: `async` non-`move` closures with parameters are not currently supported
2019-07-31T07:44:35.2169603Z    |
2019-07-31T07:44:35.2169603Z    |
2019-07-31T07:44:35.2169646Z LL |     let _ = async |x: u8| {};
2019-07-31T07:44:35.2169742Z    |
2019-07-31T07:44:35.2169742Z    |
2019-07-31T07:44:35.2169795Z    = help: consider using `let` statements to manually capture variables by reference before entering an `async move` closure
2019-07-31T07:44:35.2187744Z error: aborting due to previous error
2019-07-31T07:44:35.2187776Z 
2019-07-31T07:44:35.2187801Z 
2019-07-31T07:44:35.2188400Z ------------------------------------------
2019-07-31T07:44:35.2188400Z ------------------------------------------
2019-07-31T07:44:35.2188447Z 
2019-07-31T07:44:35.2188473Z 
2019-07-31T07:44:35.2188772Z ---- [ui] ui/generator/no-arguments-on-generators.rs stdout ----
2019-07-31T07:44:35.2188845Z diff of stderr:
2019-07-31T07:44:35.2188873Z 
2019-07-31T07:44:35.2189113Z - error[E0628]: generators cannot have explicit arguments
2019-07-31T07:44:35.2189165Z + error[E0628]: generators cannot have explicit parameters
2019-07-31T07:44:35.2189871Z 3    |
2019-07-31T07:44:35.2189871Z 3    |
2019-07-31T07:44:35.2189914Z 4 LL |     let gen = |start| {
2019-07-31T07:44:35.2189983Z 
2019-07-31T07:44:35.2190028Z The actual stderr differed from the expected stderr.
2019-07-31T07:44:35.2190658Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators/no-arguments-on-generators.stderr
2019-07-31T07:44:35.2190658Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators/no-arguments-on-generators.stderr
2019-07-31T07:44:35.2191178Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T07:44:35.2191730Z To only update this specific test, also pass `--test-args generator/no-arguments-on-generators.rs`
2019-07-31T07:44:35.2191833Z error: 1 errors occurred comparing output.
2019-07-31T07:44:35.2191880Z status: exit code: 1
2019-07-31T07:44:35.2191880Z status: exit code: 1
2019-07-31T07:44:35.2237708Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/no-arguments-on-generators.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators/auxiliary" "-A" "unused"
2019-07-31T07:44:35.2238399Z ------------------------------------------
2019-07-31T07:44:35.2238445Z 
2019-07-31T07:44:35.2238712Z ------------------------------------------
2019-07-31T07:44:35.2238764Z stderr:
2019-07-31T07:44:35.2238764Z stderr:
2019-07-31T07:44:35.2239018Z ------------------------------------------
2019-07-31T07:44:35.2239076Z error[E0628]: generators cannot have explicit parameters
2019-07-31T07:44:35.2239434Z    |
2019-07-31T07:44:35.2239434Z    |
2019-07-31T07:44:35.2239502Z LL |     let gen = |start| { //~ ERROR generators cannot have explicit arguments
2019-07-31T07:44:35.2239607Z 
2019-07-31T07:44:35.2239656Z error: aborting due to previous error
2019-07-31T07:44:35.2239689Z 
2019-07-31T07:44:35.2239717Z 
---
2019-07-31T07:44:35.2241399Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-31T07:44:35.2241465Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-31T07:44:35.2252301Z 
2019-07-31T07:44:35.2252486Z 
2019-07-31T07:44:35.2254652Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-31T07:44:35.2255025Z 
2019-07-31T07:44:35.2255069Z 
2019-07-31T07:44:35.2267845Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-31T07:44:35.2267927Z Build completed unsuccessfully in 1:07:43
2019-07-31T07:44:35.2267927Z Build completed unsuccessfully in 1:07:43
2019-07-31T07:44:36.0005430Z ##[error]Bash exited with code '1'.
2019-07-31T07:44:36.0053036Z ##[section]Starting: Checkout
2019-07-31T07:44:36.0054965Z ==============================================================================
2019-07-31T07:44:36.0055061Z Task         : Get sources
2019-07-31T07:44:36.0055113Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
