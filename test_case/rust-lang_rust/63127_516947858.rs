plain
2019-07-31T16:25:28.1364145Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T16:25:28.1547254Z ##[command]git config gc.auto 0
2019-07-31T16:25:28.1632446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T16:25:28.1699731Z ##[command]git config --get-all http.proxy
2019-07-31T16:25:28.1839208Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-07-31T16:26:04.3349399Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T16:26:04.3349424Z 
2019-07-31T16:26:04.3349613Z   git checkout -b <new-branch-name>
2019-07-31T16:26:04.3349637Z 
2019-07-31T16:26:04.3349676Z HEAD is now at 100d05c20 Merge 00e00235b6f9f9674039d46bb23bc74373c951bb into 9152fe4ea053a29469691349f4b63aa94c9aac56
2019-07-31T16:26:04.3504762Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T16:26:04.3507159Z ==============================================================================
2019-07-31T16:26:04.3507207Z Task         : Bash
2019-07-31T16:26:04.3507245Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T17:25:35.0756736Z .................................................................................................... 1400/8819
2019-07-31T17:25:41.0408257Z .................................................................................................... 1500/8819
2019-07-31T17:25:53.7693789Z .................................................................i...............i.................. 1600/8819
2019-07-31T17:26:01.2258979Z .................................................................................................... 1700/8819
2019-07-31T17:26:16.2194189Z ...................................................iiiii............................................ 1800/8819
2019-07-31T17:26:27.5503027Z .................................................................................................... 2000/8819
2019-07-31T17:26:30.0588696Z .................................................................................................... 2100/8819
2019-07-31T17:26:33.6646685Z .................................................................................................... 2200/8819
2019-07-31T17:26:40.2687173Z .................................................................................................... 2300/8819
---
2019-07-31T17:30:38.3046787Z .................................................................................................... 5300/8819
2019-07-31T17:30:45.7949972Z ..............i..................................................................................... 5400/8819
2019-07-31T17:30:51.5500669Z .................................................................................................... 5500/8819
2019-07-31T17:31:03.9313359Z .................................................................................................... 5600/8819
2019-07-31T17:31:17.4521062Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-07-31T17:31:35.8222788Z .................................................................................................... 5900/8819
2019-07-31T17:31:40.6262623Z .................................................................................................... 6000/8819
2019-07-31T17:31:40.6262623Z .................................................................................................... 6000/8819
2019-07-31T17:31:54.5250033Z ........i..ii....................................................................................... 6100/8819
2019-07-31T17:32:14.0482920Z ...................................................i................................................ 6300/8819
2019-07-31T17:32:16.1689356Z .................................................................................................... 6400/8819
2019-07-31T17:32:18.6935124Z .....................i.............................................................................. 6500/8819
2019-07-31T17:32:23.3232074Z .................................................................................................... 6600/8819
---
2019-07-31T17:36:18.1944019Z 
2019-07-31T17:36:18.1944465Z ---- [ui] ui/async-await/no-args-non-move-async-closure.rs stdout ----
2019-07-31T17:36:18.1944513Z diff of stderr:
2019-07-31T17:36:18.1944537Z 
2019-07-31T17:36:18.1944769Z - error[E0708]: `async` non-`move` closures with arguments are not currently supported
2019-07-31T17:36:18.1945012Z + error[E0708]: `async` non-`move` closures with parameters are not currently supported
2019-07-31T17:36:18.1945262Z 3    |
2019-07-31T17:36:18.1945262Z 3    |
2019-07-31T17:36:18.1945317Z 4 LL |     let _ = async |x: u8| {};
2019-07-31T17:36:18.1945362Z 
2019-07-31T17:36:18.1945398Z The actual stderr differed from the expected stderr.
2019-07-31T17:36:18.1945693Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
2019-07-31T17:36:18.1945693Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
2019-07-31T17:36:18.1945913Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T17:36:18.1946320Z To only update this specific test, also pass `--test-args async-await/no-args-non-move-async-closure.rs`
2019-07-31T17:36:18.1946407Z error: 1 errors occurred comparing output.
2019-07-31T17:36:18.1946445Z status: exit code: 1
2019-07-31T17:36:18.1946445Z status: exit code: 1
2019-07-31T17:36:18.1947672Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-args-non-move-async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-args-non-move-async-closure/auxiliary" "-A" "unused"
2019-07-31T17:36:18.1948246Z ------------------------------------------
2019-07-31T17:36:18.1948283Z 
2019-07-31T17:36:18.1948509Z ------------------------------------------
2019-07-31T17:36:18.1948556Z stderr:
2019-07-31T17:36:18.1948556Z stderr:
2019-07-31T17:36:18.1948806Z ------------------------------------------
2019-07-31T17:36:18.1949075Z error[E0708]: `async` non-`move` closures with parameters are not currently supported
2019-07-31T17:36:18.1949498Z    |
2019-07-31T17:36:18.1949498Z    |
2019-07-31T17:36:18.1949647Z LL |     let _ = async |x: u8| {};
2019-07-31T17:36:18.1949767Z    |
2019-07-31T17:36:18.1949767Z    |
2019-07-31T17:36:18.1949821Z    = help: consider using `let` statements to manually capture variables by reference before entering an `async move` closure
2019-07-31T17:36:18.1949927Z error: aborting due to previous error
2019-07-31T17:36:18.1949966Z 
2019-07-31T17:36:18.1949993Z 
2019-07-31T17:36:18.1950545Z ------------------------------------------
2019-07-31T17:36:18.1950545Z ------------------------------------------
2019-07-31T17:36:18.1950573Z 
2019-07-31T17:36:18.1950614Z 
2019-07-31T17:36:18.1950814Z ---- [ui] ui/generator/no-arguments-on-generators.rs stdout ----
2019-07-31T17:36:18.1950855Z diff of stderr:
2019-07-31T17:36:18.1950879Z 
2019-07-31T17:36:18.1951108Z - error[E0628]: generators cannot have explicit arguments
2019-07-31T17:36:18.1951153Z + error[E0628]: generators cannot have explicit parameters
2019-07-31T17:36:18.1951410Z 3    |
2019-07-31T17:36:18.1951410Z 3    |
2019-07-31T17:36:18.1951446Z 4 LL |     let gen = |start| {
2019-07-31T17:36:18.1951492Z 
2019-07-31T17:36:18.1951549Z The actual stderr differed from the expected stderr.
2019-07-31T17:36:18.1951979Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators/no-arguments-on-generators.stderr
2019-07-31T17:36:18.1951979Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators/no-arguments-on-generators.stderr
2019-07-31T17:36:18.1952193Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T17:36:18.1952441Z To only update this specific test, also pass `--test-args generator/no-arguments-on-generators.rs`
2019-07-31T17:36:18.1952506Z error: 1 errors occurred comparing output.
2019-07-31T17:36:18.1952542Z status: exit code: 1
2019-07-31T17:36:18.1952542Z status: exit code: 1
2019-07-31T17:36:18.1953176Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/no-arguments-on-generators.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/no-arguments-on-generators/auxiliary" "-A" "unused"
2019-07-31T17:36:18.1953459Z ------------------------------------------
2019-07-31T17:36:18.1953487Z 
2019-07-31T17:36:18.1953666Z ------------------------------------------
2019-07-31T17:36:18.1953722Z stderr:
2019-07-31T17:36:18.1953722Z stderr:
2019-07-31T17:36:18.1953900Z ------------------------------------------
2019-07-31T17:36:18.1953941Z error[E0628]: generators cannot have explicit parameters
2019-07-31T17:36:18.1954208Z    |
2019-07-31T17:36:18.1954208Z    |
2019-07-31T17:36:18.1954254Z LL |     let gen = |start| { //~ ERROR generators cannot have explicit parameters
2019-07-31T17:36:18.1954338Z 
2019-07-31T17:36:18.1954373Z error: aborting due to previous error
2019-07-31T17:36:18.1954396Z 
2019-07-31T17:36:18.1954434Z 
---
2019-07-31T17:36:18.1981941Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-31T17:36:18.1982031Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-31T17:36:18.1997839Z 
2019-07-31T17:36:18.1997952Z 
2019-07-31T17:36:18.1999741Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-31T17:36:18.2000030Z 
2019-07-31T17:36:18.2000064Z 
2019-07-31T17:36:18.2009593Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-31T17:36:18.2010353Z Build completed unsuccessfully in 1:03:39
2019-07-31T17:36:18.2010353Z Build completed unsuccessfully in 1:03:39
2019-07-31T17:36:18.9978761Z ##[error]Bash exited with code '1'.
2019-07-31T17:36:19.0045963Z ##[section]Starting: Checkout
2019-07-31T17:36:19.0048490Z ==============================================================================
2019-07-31T17:36:19.0048547Z Task         : Get sources
2019-07-31T17:36:19.0048616Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
