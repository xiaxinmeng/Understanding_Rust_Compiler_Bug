plain
2020-02-22T19:17:51.0044694Z ========================== Starting Command Output ===========================
2020-02-22T19:17:51.0046998Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/693c1c32-d6cc-470a-8795-1384710b0b8d.sh
2020-02-22T19:17:51.0047209Z 
2020-02-22T19:17:51.0051073Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T19:17:51.0070557Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-02-22T19:17:51.0074225Z Task         : Get sources
2020-02-22T19:17:51.0074459Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T19:17:51.0074685Z Version      : 1.0.0
2020-02-22T19:17:51.0074930Z Author       : Microsoft
---
2020-02-22T19:17:52.2757340Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T19:17:52.2767641Z ##[command]git config gc.auto 0
2020-02-22T19:17:52.2773784Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T19:17:52.2780092Z ##[command]git config --get-all http.proxy
2020-02-22T19:17:52.2789932Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67742/merge:refs/remotes/pull/67742/merge
---
2020-02-22T20:16:25.7199974Z .................................................................................................... 1700/9696
2020-02-22T20:16:29.6903078Z .................................................................................................... 1800/9696
2020-02-22T20:16:39.8955038Z ...........................................i........................................................ 1900/9696
2020-02-22T20:16:47.0703878Z .................................................................................................... 2000/9696
2020-02-22T20:16:59.5487375Z .................................iiiii.............................................................. 2100/9696
2020-02-22T20:17:08.2543754Z .................................................................................................... 2300/9696
2020-02-22T20:17:10.3784698Z .................................................................................................... 2400/9696
2020-02-22T20:17:14.2191783Z .................................................................................................... 2500/9696
2020-02-22T20:17:33.0212851Z .................................................................................................... 2600/9696
---
2020-02-22T20:20:00.6747201Z .........i.......................................................................................... 5000/9696
2020-02-22T20:20:08.6980824Z .................................................................................................... 5100/9696
2020-02-22T20:20:12.7877177Z ....................................i............................................................... 5200/9696
2020-02-22T20:20:21.7182615Z .................................................................................................... 5300/9696
2020-02-22T20:20:27.1130578Z ............ii.ii........i...i...................................................................... 5400/9696
2020-02-22T20:20:35.0450029Z .................................................................................................... 5600/9696
2020-02-22T20:20:44.9458417Z .................................................................................................... 5700/9696
2020-02-22T20:20:51.5997907Z ...i................................................................................................ 5800/9696
2020-02-22T20:20:57.0181518Z .................................................................................................... 5900/9696
2020-02-22T20:20:57.0181518Z .................................................................................................... 5900/9696
2020-02-22T20:21:06.4601514Z ..............................................................................................ii...i 6000/9696
2020-02-22T20:21:17.8769278Z ..ii...........i.................................................................................... 6100/9696
2020-02-22T20:21:32.7609483Z .......................................................................................F............ 6300/9696
2020-02-22T20:21:38.5472753Z .................................................................................................... 6400/9696
2020-02-22T20:21:38.5472753Z .................................................................................................... 6400/9696
2020-02-22T20:21:53.2649142Z .........................i..ii...................................................................... 6500/9696
2020-02-22T20:22:12.3161501Z .................................................................................................... 6700/9696
2020-02-22T20:22:14.3695363Z .................i.................................................................................. 6800/9696
2020-02-22T20:22:16.3308358Z .................................................................................................... 6900/9696
2020-02-22T20:22:18.5765669Z .......................................i............................................................ 7000/9696
---
2020-02-22T20:23:50.1031077Z .................................................................................................... 7700/9696
2020-02-22T20:23:54.5335114Z .................................................................................................... 7800/9696
2020-02-22T20:24:00.3231453Z ...................................................................................i................ 7900/9696
2020-02-22T20:24:08.3281913Z .................................................................................................... 8000/9696
2020-02-22T20:24:14.8084655Z ................................iiiiiii.i........................................................... 8100/9696
2020-02-22T20:24:27.4695452Z .................................................................................................... 8300/9696
2020-02-22T20:24:34.8429427Z .................................................................................................... 8400/9696
2020-02-22T20:24:47.3982028Z .................................................................................................... 8500/9696
2020-02-22T20:24:53.7381142Z .................................................................................................... 8600/9696
---
2020-02-22T20:26:33.2504451Z - error[E0521]: borrowed data escapes outside of function
2020-02-22T20:26:33.2504828Z + error[E0521]: borrowed data escapes outside of method
2020-02-22T20:26:33.2505335Z 97   --> $DIR/outlives-suggestion-simple.rs:73:9
2020-02-22T20:26:33.2505654Z 98    |
2020-02-22T20:26:33.2506059Z 99 LL |     fn get_bar(&self) -> Bar2 {
2020-02-22T20:26:33.2506700Z 100    |                -----
2020-02-22T20:26:33.2506978Z 101    |                |
2020-02-22T20:26:33.2507659Z -    |                `self` declared here, outside of the function body
2020-02-22T20:26:33.2508310Z -    |                `self` is a reference that is only valid in the function body
2020-02-22T20:26:33.2508310Z -    |                `self` is a reference that is only valid in the function body
2020-02-22T20:26:33.2508742Z +    |                `self` declared here, outside of the method body
2020-02-22T20:26:33.2509161Z +    |                `self` is a reference that is only valid in the method body
2020-02-22T20:26:33.2509519Z 104 LL |         Bar2::new(&self)
2020-02-22T20:26:33.2510051Z -    |         ^^^^^^^^^^^^^^^^ `self` escapes the function body here
2020-02-22T20:26:33.2510642Z +    |         ^^^^^^^^^^^^^^^^ `self` escapes the method body here
2020-02-22T20:26:33.2511349Z 107 error: aborting due to 9 previous errors
2020-02-22T20:26:33.2511623Z 108 
2020-02-22T20:26:33.2511799Z 
2020-02-22T20:26:33.2511970Z 
2020-02-22T20:26:33.2511970Z 
2020-02-22T20:26:33.2512245Z The actual stderr differed from the expected stderr.
2020-02-22T20:26:33.2512950Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/outlives-suggestion-simple.stderr
2020-02-22T20:26:33.2513631Z To update references, rerun the tests and pass the `--bless` flag
2020-02-22T20:26:33.2514423Z To only update this specific test, also pass `--test-args nll/outlives-suggestion-simple.rs`
2020-02-22T20:26:33.2514989Z error: 1 errors occurred comparing output.
2020-02-22T20:26:33.2515291Z status: exit code: 1
2020-02-22T20:26:33.2515291Z status: exit code: 1
2020-02-22T20:26:33.2518600Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/auxiliary"
2020-02-22T20:26:33.2520275Z ------------------------------------------
2020-02-22T20:26:33.2520531Z 
2020-02-22T20:26:33.2520940Z ------------------------------------------
2020-02-22T20:26:33.2521228Z stderr:
---
2020-02-22T20:26:33.2526756Z LL | fn foo1<'a, 'b>(x: &'a usize) -> &'b usize {
2020-02-22T20:26:33.2527331Z    |         --  -- lifetime `'b` defined here
2020-02-22T20:26:33.2527505Z    |         |
2020-02-22T20:26:33.2527813Z    |         lifetime `'a` defined here
2020-02-22T20:26:33.2528026Z LL |     x //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2528423Z    |     ^ returning this value requires that `'a` must outlive `'b`
2020-02-22T20:26:33.2528953Z    = help: consider adding the following bound: `'a: 'b`
2020-02-22T20:26:33.2529121Z 
2020-02-22T20:26:33.2529261Z error: lifetime may not live long enough
2020-02-22T20:26:33.2529670Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:10:5
2020-02-22T20:26:33.2529670Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:10:5
2020-02-22T20:26:33.2529861Z    |
2020-02-22T20:26:33.2530167Z LL | fn foo2<'a>(x: &'a usize) -> &'static usize {
2020-02-22T20:26:33.2530856Z LL |     x //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2530856Z LL |     x //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2531259Z    |     ^ returning this value requires that `'a` must outlive `'static`
2020-02-22T20:26:33.2531759Z    = help: consider replacing `'a` with `'static`
2020-02-22T20:26:33.2532074Z 
2020-02-22T20:26:33.2532222Z error: lifetime may not live long enough
2020-02-22T20:26:33.2532652Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-22T20:26:33.2532652Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-22T20:26:33.2532853Z    |
2020-02-22T20:26:33.2533236Z LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
2020-02-22T20:26:33.2534060Z    |         |
2020-02-22T20:26:33.2534506Z    |         lifetime `'a` defined here
2020-02-22T20:26:33.2534506Z    |         lifetime `'a` defined here
2020-02-22T20:26:33.2534767Z LL |     (x, y) //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2535481Z    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
2020-02-22T20:26:33.2536089Z    = help: consider adding the following bound: `'a: 'b`
2020-02-22T20:26:33.2536259Z 
2020-02-22T20:26:33.2536399Z error: lifetime may not live long enough
2020-02-22T20:26:33.2536808Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-22T20:26:33.2536808Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-22T20:26:33.2536998Z    |
2020-02-22T20:26:33.2537360Z LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
2020-02-22T20:26:33.2538864Z    |         |
2020-02-22T20:26:33.2539152Z    |         lifetime `'a` defined here
2020-02-22T20:26:33.2539152Z    |         lifetime `'a` defined here
2020-02-22T20:26:33.2539385Z LL |     (x, y) //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2539882Z    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
2020-02-22T20:26:33.2540467Z    = help: consider adding the following bound: `'b: 'a`
2020-02-22T20:26:33.2540641Z 
2020-02-22T20:26:33.2540978Z help: `'a` and `'b` must be the same: replace one with the other
2020-02-22T20:26:33.2541153Z 
2020-02-22T20:26:33.2541153Z 
2020-02-22T20:26:33.2541308Z error: lifetime may not live long enough
2020-02-22T20:26:33.2541698Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:22:5
2020-02-22T20:26:33.2541888Z    |
2020-02-22T20:26:33.2542239Z LL | fn foo4<'a, 'b, 'c>(x: &'a usize) -> (&'b usize, &'c usize) {
2020-02-22T20:26:33.2542977Z    |         |
2020-02-22T20:26:33.2543278Z    |         lifetime `'a` defined here
2020-02-22T20:26:33.2543447Z ...
2020-02-22T20:26:33.2543447Z ...
2020-02-22T20:26:33.2543624Z LL |     (x, x) //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2544058Z    |     ^^^^^^ returning this value requires that `'a` must outlive `'b`
2020-02-22T20:26:33.2544619Z    = help: consider adding the following bound: `'a: 'b`
2020-02-22T20:26:33.2544801Z 
2020-02-22T20:26:33.2544966Z error: lifetime may not live long enough
2020-02-22T20:26:33.2545489Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:31:9
2020-02-22T20:26:33.2545489Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:31:9
2020-02-22T20:26:33.2545859Z    |
2020-02-22T20:26:33.2546202Z LL |     pub fn foo<'a>(x: &'a usize) -> Self {
2020-02-22T20:26:33.2546565Z    |                -- lifetime `'a` defined here
2020-02-22T20:26:33.2546805Z LL |         Foo { x } //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2547270Z    |         ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
2020-02-22T20:26:33.2547772Z    = help: consider replacing `'a` with `'static`
2020-02-22T20:26:33.2547915Z 
2020-02-22T20:26:33.2548071Z error: lifetime may not live long enough
2020-02-22T20:26:33.2548460Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:41:9
2020-02-22T20:26:33.2548460Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:41:9
2020-02-22T20:26:33.2548651Z    |
2020-02-22T20:26:33.2548911Z LL | impl<'a> Bar<'a> {
2020-02-22T20:26:33.2549309Z    |      -- lifetime `'a` defined here
2020-02-22T20:26:33.2549645Z LL |     pub fn get<'b>(&self) -> &'b usize {
2020-02-22T20:26:33.2550010Z    |                -- lifetime `'b` defined here
2020-02-22T20:26:33.2550246Z LL |         self.x //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2550671Z    |         ^^^^^^ returning this value requires that `'a` must outlive `'b`
2020-02-22T20:26:33.2551207Z    = help: consider adding the following bound: `'a: 'b`
2020-02-22T20:26:33.2551376Z 
2020-02-22T20:26:33.2551515Z error: lifetime may not live long enough
2020-02-22T20:26:33.2551920Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:52:9
2020-02-22T20:26:33.2551920Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:52:9
2020-02-22T20:26:33.2552110Z    |
2020-02-22T20:26:33.2552350Z LL | impl<'a> Baz<'a> {
2020-02-22T20:26:33.2552666Z    |      -- lifetime `'a` defined here
2020-02-22T20:26:33.2552993Z LL |     fn get<'b>(&'b self) -> &'a i32 {
2020-02-22T20:26:33.2553513Z    |            -- lifetime `'b` defined here
2020-02-22T20:26:33.2553776Z LL |         self.x //~ERROR lifetime may not live long enough
2020-02-22T20:26:33.2554228Z    |         ^^^^^^ returning this value requires that `'b` must outlive `'a`
2020-02-22T20:26:33.2554793Z    = help: consider adding the following bound: `'b: 'a`
2020-02-22T20:26:33.2554970Z 
2020-02-22T20:26:33.2555139Z error[E0521]: borrowed data escapes outside of method
2020-02-22T20:26:33.2555570Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:73:9
2020-02-22T20:26:33.2555570Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:73:9
2020-02-22T20:26:33.2555787Z    |
2020-02-22T20:26:33.2556075Z LL |     fn get_bar(&self) -> Bar2 {
2020-02-22T20:26:33.2556697Z    |                |
2020-02-22T20:26:33.2556697Z    |                |
2020-02-22T20:26:33.2556900Z    |                `self` declared here, outside of the method body
2020-02-22T20:26:33.2557178Z    |                `self` is a reference that is only valid in the method body
2020-02-22T20:26:33.2557498Z LL |         Bar2::new(&self) //~ERROR borrowed data escapes outside of function
2020-02-22T20:26:33.2557787Z    |         ^^^^^^^^^^^^^^^^ `self` escapes the method body here
2020-02-22T20:26:33.2558101Z error: aborting due to 9 previous errors
2020-02-22T20:26:33.2558229Z 
2020-02-22T20:26:33.2558300Z 
2020-02-22T20:26:33.2558581Z ------------------------------------------
---
2020-02-22T20:26:33.2560365Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-22T20:26:33.2560697Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-22T20:26:33.2560872Z 
2020-02-22T20:26:33.2560943Z 
2020-02-22T20:26:33.2563978Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-22T20:26:33.2566140Z 
2020-02-22T20:26:33.2566219Z 
2020-02-22T20:26:33.2567094Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-22T20:26:33.2567439Z Build completed unsuccessfully in 1:01:47
2020-02-22T20:26:33.2567439Z Build completed unsuccessfully in 1:01:47
2020-02-22T20:26:33.2583273Z == clock drift check ==
2020-02-22T20:26:33.2605500Z   local time: Sat Feb 22 20:26:33 UTC 2020
2020-02-22T20:26:33.5283787Z   network time: Sat, 22 Feb 2020 20:26:33 GMT
2020-02-22T20:26:33.5287712Z == end clock drift check ==
2020-02-22T20:26:34.0076805Z 
2020-02-22T20:26:34.0158656Z ##[error]Bash exited with code '1'.
2020-02-22T20:26:34.0170719Z ##[section]Finishing: Run build
2020-02-22T20:26:34.0210088Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-02-22T20:26:34.0214149Z Task         : Get sources
2020-02-22T20:26:34.0214404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T20:26:34.0214639Z Version      : 1.0.0
2020-02-22T20:26:34.0214820Z Author       : Microsoft
2020-02-22T20:26:34.0214820Z Author       : Microsoft
2020-02-22T20:26:34.0215084Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T20:26:34.0215386Z ==============================================================================
2020-02-22T20:26:34.3061062Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T20:26:34.3100952Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-02-22T20:26:34.3174046Z Cleaning up task key
2020-02-22T20:26:34.3175005Z Start cleaning up orphan processes.
2020-02-22T20:26:34.3311749Z Terminate orphan process: pid (3732) (python)
2020-02-22T20:26:34.3516283Z ##[section]Finishing: Finalize Job
