plain
2020-01-18T18:32:30.5294787Z ========================== Starting Command Output ===========================
2020-01-18T18:32:30.5298097Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/81e89bee-3806-4203-895b-3409a34b3328.sh
2020-01-18T18:32:30.5298258Z 
2020-01-18T18:32:30.5301482Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T18:32:30.5355179Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68350/merge to s
2020-01-18T18:32:30.5356877Z Task         : Get sources
2020-01-18T18:32:30.5356905Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T18:32:30.5356933Z Version      : 1.0.0
2020-01-18T18:32:30.5356967Z Author       : Microsoft
---
2020-01-18T18:32:32.3332223Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T18:32:32.3341798Z ##[command]git config gc.auto 0
2020-01-18T18:32:32.3343750Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T18:32:32.3345451Z ##[command]git config --get-all http.proxy
2020-01-18T18:32:32.3350647Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68350/merge:refs/remotes/pull/68350/merge
---
2020-01-18T19:22:06.7467864Z .................................................................................................... 1700/9544
2020-01-18T19:22:13.5945198Z .................................................................................................... 1800/9544
2020-01-18T19:22:22.9737899Z .................i.................................................................................. 1900/9544
2020-01-18T19:22:29.6337294Z .................................................................................................... 2000/9544
2020-01-18T19:22:43.5335911Z .......iiiii........................................................................................ 2100/9544
2020-01-18T19:22:52.0167321Z .................................................................................................... 2300/9544
2020-01-18T19:22:54.2997212Z .................................................................................................... 2400/9544
2020-01-18T19:22:59.4456337Z .................................................................................................... 2500/9544
2020-01-18T19:23:18.4339985Z .................................................................................................... 2600/9544
---
2020-01-18T19:25:43.1257754Z ....................................................i...............i............................... 4900/9544
2020-01-18T19:25:50.4463401Z .................................................................................................... 5000/9544
2020-01-18T19:25:57.2952001Z ...............................................................................................i.... 5100/9544
2020-01-18T19:26:01.8682542Z .................................................................................................... 5200/9544
2020-01-18T19:26:11.3185702Z ...................................................................ii.ii...........i................ 5300/9544
2020-01-18T19:26:19.3224432Z ....i............................................................................................... 5500/9544
2020-01-18T19:26:28.6451157Z .................................................................................................... 5600/9544
2020-01-18T19:26:34.4046947Z .....................................................i.............................................. 5700/9544
2020-01-18T19:26:40.3772791Z .................................................................................................... 5800/9544
2020-01-18T19:26:40.3772791Z .................................................................................................... 5800/9544
2020-01-18T19:26:49.2310803Z .................................................................................................... 5900/9544
2020-01-18T19:26:55.2855903Z ............................................ii...i..ii...........i.................................. 6000/9544
2020-01-18T19:27:14.4542073Z .................................................................................................... 6200/9544
2020-01-18T19:27:21.6351912Z .................................................................................................... 6300/9544
2020-01-18T19:27:21.6351912Z .................................................................................................... 6300/9544
2020-01-18T19:27:29.3054035Z .............................................................................i..ii.................. 6400/9544
2020-01-18T19:27:53.4140814Z .................................................................................................... 6600/9544
2020-01-18T19:27:57.8086232Z .....................................................i.............................................. 6700/9544
2020-01-18T19:27:59.8156316Z .................................................................................................... 6800/9544
2020-01-18T19:28:01.8393797Z .....................................................i.............................................. 6900/9544
---
2020-01-18T19:29:32.7146333Z .................................................................................................... 7600/9544
2020-01-18T19:29:37.8336286Z .................................................................................................... 7700/9544
2020-01-18T19:29:43.6097560Z .................................................................................................... 7800/9544
2020-01-18T19:29:53.2231403Z .................................................................................................... 7900/9544
2020-01-18T19:29:58.4970691Z ....iiiiiii......................................................................................... 8000/9544
2020-01-18T19:30:11.6946387Z .................................................................................................... 8200/9544
2020-01-18T19:30:21.8835975Z .................................................................................................... 8300/9544
2020-01-18T19:30:33.1422834Z .................................................................................................... 8400/9544
2020-01-18T19:30:38.4387546Z .................................................................................................... 8500/9544
---
2020-01-18T19:32:21.9046852Z ---- [ui] ui/never_type/diverging-fallback-control-flow.rs stdout ----
2020-01-18T19:32:21.9047153Z 
2020-01-18T19:32:21.9047624Z error: test compilation failed although it shouldn't!
2020-01-18T19:32:21.9047919Z status: exit code: 1
2020-01-18T19:32:21.9049135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-fallback-control-flow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-fallback-control-flow/auxiliary"
2020-01-18T19:32:21.9049960Z ------------------------------------------
2020-01-18T19:32:21.9050244Z 
2020-01-18T19:32:21.9050679Z ------------------------------------------
2020-01-18T19:32:21.9050936Z stderr:
---
2020-01-18T19:32:21.9098781Z 
2020-01-18T19:32:21.9098822Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T19:32:21.9099484Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:69:23
2020-01-18T19:32:21.9099548Z    |
2020-01-18T19:32:21.9099592Z LL |     let _x = match Ok(BadDefault::default()) {
2020-01-18T19:32:21.9099689Z    |
2020-01-18T19:32:21.9099749Z note: the type parameter Self here was inferred to `!`
2020-01-18T19:32:21.9099995Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:69:23
2020-01-18T19:32:21.9100040Z    |
2020-01-18T19:32:21.9100040Z    |
2020-01-18T19:32:21.9100111Z LL |     let _x = match Ok(BadDefault::default()) {
2020-01-18T19:32:21.9100200Z note: (type parameter defined here)
2020-01-18T19:32:21.9100459Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:16:1
2020-01-18T19:32:21.9100504Z    |
2020-01-18T19:32:21.9100542Z LL | / trait BadDefault {
2020-01-18T19:32:21.9100542Z LL | / trait BadDefault {
2020-01-18T19:32:21.9100750Z LL | |     fn default() -> Self;
2020-01-18T19:32:21.9100971Z LL | | }
2020-01-18T19:32:21.9101009Z    | |_^
2020-01-18T19:32:21.9101052Z note: ... due to this expression evaluating to `!`
2020-01-18T19:32:21.9101320Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:69:14
2020-01-18T19:32:21.9101377Z    |
2020-01-18T19:32:21.9101420Z LL |       let _x = match Ok(BadDefault::default()) {
2020-01-18T19:32:21.9101524Z LL | |         Ok(v) => v,
2020-01-18T19:32:21.9101733Z LL | |         Err(()) => return,
2020-01-18T19:32:21.9101812Z LL | |     };
2020-01-18T19:32:21.9101854Z    | |_____^
2020-01-18T19:32:21.9101854Z    | |_____^
2020-01-18T19:32:21.9101902Z    = note: If you want the `!` type to be used here, add explicit type annotations
2020-01-18T19:32:21.9101936Z 
2020-01-18T19:32:21.9101999Z error: Fallback to `!` may introduce undefined behavior
2020-01-18T19:32:21.9102299Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:76:23
2020-01-18T19:32:21.9102346Z    |
2020-01-18T19:32:21.9102408Z LL |     let _x = match Ok(BadDefault::default()) {
2020-01-18T19:32:21.9102497Z    |
2020-01-18T19:32:21.9102559Z note: the type parameter Self here was inferred to `!`
2020-01-18T19:32:21.9102944Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:76:23
2020-01-18T19:32:21.9102992Z    |
2020-01-18T19:32:21.9102992Z    |
2020-01-18T19:32:21.9103221Z LL |     let _x = match Ok(BadDefault::default()) {
2020-01-18T19:32:21.9103333Z note: (type parameter defined here)
2020-01-18T19:32:21.9103570Z   --> /checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs:16:1
2020-01-18T19:32:21.9103629Z    |
2020-01-18T19:32:21.9103667Z LL | / trait BadDefault {
---
2020-01-18T19:32:21.9109860Z test result: FAILED. 9493 passed; 1 failed; 50 ignored; 0 measured; 0 filtered out
2020-01-18T19:32:21.9109902Z 
2020-01-18T19:32:21.9109925Z 
2020-01-18T19:32:21.9109948Z 
2020-01-18T19:32:21.9111403Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-18T19:32:21.9111629Z 
2020-01-18T19:32:21.9111671Z 
2020-01-18T19:32:21.9111912Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-18T19:32:21.9111964Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-18T19:32:21.9111964Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-18T19:32:21.9112031Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-18T19:32:21.9112075Z Build completed unsuccessfully in 0:54:55
2020-01-18T19:32:21.9137734Z == clock drift check ==
2020-01-18T19:32:21.9153682Z   local time: Sat Jan 18 19:32:21 UTC 2020
2020-01-18T19:32:22.1909843Z   network time: Sat, 18 Jan 2020 19:32:22 GMT
2020-01-18T19:32:22.1921375Z == end clock drift check ==
2020-01-18T19:32:22.6815454Z 
2020-01-18T19:32:22.6913124Z ##[error]Bash exited with code '1'.
2020-01-18T19:32:22.6923880Z ##[section]Finishing: Run build
2020-01-18T19:32:22.6942059Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68350/merge to s
2020-01-18T19:32:22.6943678Z Task         : Get sources
2020-01-18T19:32:22.6943720Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T19:32:22.6943777Z Version      : 1.0.0
2020-01-18T19:32:22.6943813Z Author       : Microsoft
2020-01-18T19:32:22.6943813Z Author       : Microsoft
2020-01-18T19:32:22.6943854Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T19:32:22.6943897Z ==============================================================================
2020-01-18T19:32:23.0957737Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T19:32:23.0998208Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68350/merge to s
2020-01-18T19:32:23.1098465Z Cleaning up task key
2020-01-18T19:32:23.1099230Z Start cleaning up orphan processes.
2020-01-18T19:32:23.1215398Z Terminate orphan process: pid (3374) (python)
2020-01-18T19:32:23.1476452Z ##[section]Finishing: Finalize Job
