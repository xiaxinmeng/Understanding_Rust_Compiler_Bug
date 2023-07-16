plain
2020-03-18T15:57:01.0280532Z ========================== Starting Command Output ===========================
2020-03-18T15:57:01.0283232Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/00aa45d3-c38c-4e69-99de-4512b710f80a.sh
2020-03-18T15:57:01.0283509Z 
2020-03-18T15:57:01.0287536Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T15:57:01.0306351Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70110/merge to s
2020-03-18T15:57:01.0309417Z Task         : Get sources
2020-03-18T15:57:01.0309700Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T15:57:01.0309996Z Version      : 1.0.0
2020-03-18T15:57:01.0310184Z Author       : Microsoft
---
2020-03-18T15:57:02.0387909Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T15:57:02.0392821Z ##[command]git config gc.auto 0
2020-03-18T15:57:02.0396224Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T15:57:02.0399646Z ##[command]git config --get-all http.proxy
2020-03-18T15:57:02.0405353Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70110/merge:refs/remotes/pull/70110/merge
---
2020-03-18T16:50:12.8998179Z .................................................................................................... 1700/9798
2020-03-18T16:50:17.1788267Z .................................................................................................... 1800/9798
2020-03-18T16:50:28.2719924Z ..........................................................................i......................... 1900/9798
2020-03-18T16:50:34.5034649Z .................................................................................................... 2000/9798
2020-03-18T16:50:42.1374256Z ................................................................iiiii............................... 2100/9798
2020-03-18T16:50:59.1781891Z .................................................................................................... 2300/9798
2020-03-18T16:51:01.3885921Z .................................................................................................... 2400/9798
2020-03-18T16:51:04.2222580Z .................................................................................................... 2500/9798
2020-03-18T16:51:23.3823893Z .................................................................................................... 2600/9798
---
2020-03-18T16:53:56.5227932Z ....................................i...............i............................................... 5000/9798
2020-03-18T16:54:05.2335298Z .................................................................................................... 5100/9798
2020-03-18T16:54:11.3254854Z ...............................................................................i.................... 5200/9798
2020-03-18T16:54:16.7299361Z .................................................................................................... 5300/9798
2020-03-18T16:54:26.3807511Z ............................................................ii.ii........i...i...................... 5400/9798
2020-03-18T16:54:30.5327928Z ...................................................................................................i 5500/9798
2020-03-18T16:54:43.5220870Z .................................................................................................... 5700/9798
2020-03-18T16:54:49.5635092Z ......................................................i............................................. 5800/9798
2020-03-18T16:54:55.8766529Z .....................................................F.............................................. 5900/9798
2020-03-18T16:55:04.7918033Z .................................................................................................... 6000/9798
2020-03-18T16:55:04.7918033Z .................................................................................................... 6000/9798
2020-03-18T16:55:11.3512144Z ................................................ii...i..ii...........i.............................. 6100/9798
2020-03-18T16:55:30.8653362Z .................................................................................................... 6300/9798
2020-03-18T16:55:37.6317789Z .................................................................................................... 6400/9798
2020-03-18T16:55:37.6317789Z .................................................................................................... 6400/9798
2020-03-18T16:55:43.2892200Z ..............................................................................i..ii................. 6500/9798
2020-03-18T16:56:04.1428726Z .................................................................................................... 6700/9798
2020-03-18T16:56:12.6521810Z ............................................................................i....................... 6800/9798
2020-03-18T16:56:14.7130147Z .................................................................................................... 6900/9798
2020-03-18T16:56:16.8303146Z .................................................................................................... 7000/9798
---
2020-03-18T16:57:54.6325878Z .................................................................................................... 7800/9798
2020-03-18T16:57:59.8508253Z .................................................................................................... 7900/9798
2020-03-18T16:58:05.6790084Z ...............................................................i.................................... 8000/9798
2020-03-18T16:58:15.2458426Z .................................................................................................... 8100/9798
2020-03-18T16:58:20.5130782Z ............iiiiiiiiii.i............................................................................ 8200/9798
2020-03-18T16:58:33.7548685Z .................................................................................................... 8400/9798
2020-03-18T16:58:41.5034941Z .................................................................................................... 8500/9798
2020-03-18T16:58:54.3482405Z .................................................................................................... 8600/9798
2020-03-18T16:59:00.6627083Z .................................................................................................... 8700/9798
---
2020-03-18T17:00:49.7870432Z + 
2020-03-18T17:00:49.7870705Z + note: trace_macro
2020-03-18T17:00:49.7871240Z +   --> $DIR/trace_faulty_macros.rs:38:13
2020-03-18T17:00:49.7871596Z +    |
2020-03-18T17:00:49.7871894Z + LL |     let a = pat_macro!();
2020-03-18T17:00:49.7872550Z +    |
2020-03-18T17:00:49.7872550Z +    |
2020-03-18T17:00:49.7872868Z +    = note: expanding `pat_macro! {  }`
2020-03-18T17:00:49.7873550Z +    = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
2020-03-18T17:00:49.7874099Z +    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
2020-03-18T17:00:49.7874591Z +    = note: to `A { a: a, b: 0, c: _, .. }`
2020-03-18T17:00:49.7875252Z + error: aborting due to 4 previous errors
2020-03-18T17:00:49.7875561Z 64 
2020-03-18T17:00:49.7875816Z 65 
2020-03-18T17:00:49.7876033Z 
2020-03-18T17:00:49.7876033Z 
2020-03-18T17:00:49.7876244Z 
2020-03-18T17:00:49.7876565Z The actual stderr differed from the expected stderr.
2020-03-18T17:00:49.7877765Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/trace_faulty_macros.stderr
2020-03-18T17:00:49.7878564Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T17:00:49.7879528Z To only update this specific test, also pass `--test-args macros/trace_faulty_macros.rs`
2020-03-18T17:00:49.7880216Z error: 1 errors occurred comparing output.
2020-03-18T17:00:49.7880757Z status: exit code: 1
2020-03-18T17:00:49.7880757Z status: exit code: 1
2020-03-18T17:00:49.7882916Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/trace_faulty_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "trace-macros" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/trace_faulty_macros/auxiliary"
2020-03-18T17:00:49.7884792Z ------------------------------------------
2020-03-18T17:00:49.7885105Z 
2020-03-18T17:00:49.7885630Z ------------------------------------------
2020-03-18T17:00:49.7886120Z stderr:
2020-03-18T17:00:49.7886120Z stderr:
2020-03-18T17:00:49.7886649Z ------------------------------------------
2020-03-18T17:00:49.7887037Z error: no rules expected the token `bcd`
2020-03-18T17:00:49.7887662Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:7:26
2020-03-18T17:00:49.7888330Z    |
2020-03-18T17:00:49.7888726Z LL | macro_rules! my_faulty_macro {
2020-03-18T17:00:49.7889343Z    | ---------------------------- when calling this macro
2020-03-18T17:00:49.7889739Z LL |     () => {
2020-03-18T17:00:49.7890161Z LL |         my_faulty_macro!(bcd); //~ ERROR no rules
2020-03-18T17:00:49.7891227Z ...
2020-03-18T17:00:49.7891227Z ...
2020-03-18T17:00:49.7891417Z LL |     my_faulty_macro!();
2020-03-18T17:00:49.7892142Z    |
2020-03-18T17:00:49.7892819Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-03-18T17:00:49.7893241Z 
2020-03-18T17:00:49.7893421Z note: trace_macro
2020-03-18T17:00:49.7893421Z note: trace_macro
2020-03-18T17:00:49.7894000Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:33:5
2020-03-18T17:00:49.7894374Z    |
2020-03-18T17:00:49.7894579Z LL |     my_faulty_macro!();
2020-03-18T17:00:49.7895485Z    |
2020-03-18T17:00:49.7895485Z    |
2020-03-18T17:00:49.7895716Z    = note: expanding `my_faulty_macro! {  }`
2020-03-18T17:00:49.7896008Z    = note: to `my_faulty_macro ! (bcd) ;`
2020-03-18T17:00:49.7896398Z    = note: expanding `my_faulty_macro! { bcd }`
2020-03-18T17:00:49.7896857Z error: recursion limit reached while expanding `my_recursive_macro!`
2020-03-18T17:00:49.7898073Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:22:9
2020-03-18T17:00:49.7898442Z    |
2020-03-18T17:00:49.7898442Z    |
2020-03-18T17:00:49.7898765Z LL |         my_recursive_macro!(); //~ ERROR recursion limit
2020-03-18T17:00:49.7899434Z ...
2020-03-18T17:00:49.7899434Z ...
2020-03-18T17:00:49.7899981Z LL |     my_recursive_macro!();
2020-03-18T17:00:49.7902184Z    |
2020-03-18T17:00:49.7902184Z    |
2020-03-18T17:00:49.7904121Z    = help: consider adding a `#![recursion_limit="8"]` attribute to your crate (`trace_faulty_macros`)
2020-03-18T17:00:49.7907301Z 
2020-03-18T17:00:49.7907564Z note: trace_macro
2020-03-18T17:00:49.7908052Z   --> /checkout/src/test/ui/macros/trace_faulty_macros.rs:34:5
2020-03-18T17:00:49.7908436Z    |
2020-03-18T17:00:49.7908436Z    |
2020-03-18T17:00:49.7908640Z LL |     my_recursive_macro!();
2020-03-18T17:00:49.7909153Z    |
2020-03-18T17:00:49.7909153Z    |
2020-03-18T17:00:49.7909394Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T17:00:49.7909774Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T17:00:49.7910094Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T17:00:49.7910473Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T17:00:49.7910874Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T17:00:49.7911256Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T17:00:49.7911823Z    = note: expanding `my_recursive_macro! {  }`
2020-03-18T17:00:49.7912201Z    = note: to `my_recursive_macro ! () ;`
2020-03-18T17:00:49.7912383Z 
2020-03-18T17:00:49.7912635Z error: expected expression, found `A { a: a, b: 0, c: _, .. }`
2020-03-18T17:00:49.7913615Z    |
2020-03-18T17:00:49.7913615Z    |
2020-03-18T17:00:49.7913825Z LL |         $a //~ ERROR expected expression
2020-03-18T17:00:49.7914365Z ...
2020-03-18T17:00:49.7914547Z LL |     let a = pat_macro!();
2020-03-18T17:00:49.7915213Z    |             ------------ in this macro invocation
2020-03-18T17:00:49.7915524Z    |
---
2020-03-18T17:00:49.7921002Z    |
2020-03-18T17:00:49.7921187Z LL |     let a = pat_macro!();
2020-03-18T17:00:49.7922915Z    |             ^^^^^^^^^^^^
2020-03-18T17:00:49.7923199Z    |
2020-03-18T17:00:49.7923406Z    = note: expanding `pat_macro! {  }`
2020-03-18T17:00:49.7924026Z    = note: to `pat_macro ! (A { a : a, b : 0, c : _, .. }) ;`
2020-03-18T17:00:49.7924523Z    = note: expanding `pat_macro! { A { a : a, b : 0, c : _, .. } }`
2020-03-18T17:00:49.7924997Z    = note: to `A { a: a, b: 0, c: _, .. }`
2020-03-18T17:00:49.7925590Z error: aborting due to 4 previous errors
2020-03-18T17:00:49.7925773Z 
2020-03-18T17:00:49.7925994Z 
2020-03-18T17:00:49.7926513Z ------------------------------------------
---
2020-03-18T17:00:49.7929381Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T17:00:49.7929903Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T17:00:49.7930156Z 
2020-03-18T17:00:49.7930256Z 
2020-03-18T17:00:49.7934073Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T17:00:49.7937089Z 
2020-03-18T17:00:49.7937211Z 
2020-03-18T17:00:49.7937758Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T17:00:49.7938263Z Build completed unsuccessfully in 0:59:50
2020-03-18T17:00:49.7938263Z Build completed unsuccessfully in 0:59:50
2020-03-18T17:00:49.7938614Z == clock drift check ==
2020-03-18T17:00:49.7938905Z   local time: Wed Mar 18 17:00:49 UTC 2020
2020-03-18T17:00:49.7939315Z   network time: Wed, 18 Mar 2020 17:00:49 GMT
2020-03-18T17:00:49.7939678Z == end clock drift check ==
2020-03-18T17:00:50.1066048Z 
2020-03-18T17:00:50.1169679Z ##[error]Bash exited with code '1'.
2020-03-18T17:00:50.1183418Z ##[section]Finishing: Run build
2020-03-18T17:00:50.1240085Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70110/merge to s
2020-03-18T17:00:50.1245341Z Task         : Get sources
2020-03-18T17:00:50.1245678Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T17:00:50.1245988Z Version      : 1.0.0
2020-03-18T17:00:50.1246230Z Author       : Microsoft
2020-03-18T17:00:50.1246230Z Author       : Microsoft
2020-03-18T17:00:50.1246573Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T17:00:50.1246970Z ==============================================================================
2020-03-18T17:00:50.4545174Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T17:00:50.4596022Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70110/merge to s
2020-03-18T17:00:50.4688412Z Cleaning up task key
2020-03-18T17:00:50.4689751Z Start cleaning up orphan processes.
2020-03-18T17:00:50.4989428Z Terminate orphan process: pid (3691) (python)
2020-03-18T17:00:50.5031813Z ##[section]Finishing: Finalize Job
