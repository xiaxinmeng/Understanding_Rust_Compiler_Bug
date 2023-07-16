plain
2020-02-08T22:34:30.5713044Z ========================== Starting Command Output ===========================
2020-02-08T22:34:30.5715762Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2a1de31a-ed1b-4181-82df-2adfc0ff4379.sh
2020-02-08T22:34:30.5716100Z 
2020-02-08T22:34:30.5720426Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T22:34:30.5728103Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-02-08T22:34:30.5730254Z Task         : Get sources
2020-02-08T22:34:30.5730291Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T22:34:30.5730327Z Version      : 1.0.0
2020-02-08T22:34:30.5730419Z Author       : Microsoft
---
2020-02-08T22:34:31.5725596Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T22:34:31.5738213Z ##[command]git config gc.auto 0
2020-02-08T22:34:31.5741570Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T22:34:31.5745889Z ##[command]git config --get-all http.proxy
2020-02-08T22:34:31.5753704Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67742/merge:refs/remotes/pull/67742/merge
---
2020-02-08T23:34:39.9680434Z .................................................................................................... 1700/9611
2020-02-08T23:34:45.2622626Z .................................................................................................... 1800/9611
2020-02-08T23:34:57.6278076Z ...............................i.................................................................... 1900/9611
2020-02-08T23:35:05.5236044Z .................................................................................................... 2000/9611
2020-02-08T23:35:20.0693194Z .....................iiiii.......................................................................... 2100/9611
2020-02-08T23:35:30.2954097Z .................................................................................................... 2300/9611
2020-02-08T23:35:32.9116959Z .................................................................................................... 2400/9611
2020-02-08T23:35:37.7645434Z .................................................................................................... 2500/9611
2020-02-08T23:35:59.2300437Z .................................................................................................... 2600/9611
---
2020-02-08T23:38:39.1731485Z .........................................................................i...............i.......... 4900/9611
2020-02-08T23:38:46.9248847Z .................................................................................................... 5000/9611
2020-02-08T23:38:55.4299396Z .................................................................................................... 5100/9611
2020-02-08T23:39:00.4400292Z ................i................................................................................... 5200/9611
2020-02-08T23:39:11.8198383Z ..........................................................................................ii.ii..... 5300/9611
2020-02-08T23:39:16.1997403Z ...i...i............................................................................................ 5400/9611
2020-02-08T23:39:26.3920687Z .................................................................................................... 5600/9611
2020-02-08T23:39:37.0954925Z ..............................................................................i..................... 5700/9611
2020-02-08T23:39:44.8385912Z .................................................................................................... 5800/9611
2020-02-08T23:39:51.2569621Z .................................................................................................... 5900/9611
2020-02-08T23:39:51.2569621Z .................................................................................................... 5900/9611
2020-02-08T23:40:01.5129209Z .....................................................................ii...i..ii...........i......... 6000/9611
2020-02-08T23:40:23.5128298Z .................................................................................................... 6200/9611
2020-02-08T23:40:31.4643313Z ...........................................................F........................................ 6300/9611
2020-02-08T23:40:39.8749897Z .................................................................................................i.. 6400/9611
2020-02-08T23:40:54.9236658Z ii.................................................................................................. 6500/9611
---
2020-02-08T23:43:02.7265442Z .................................................................................................... 7600/9611
2020-02-08T23:43:07.9084599Z .................................................................................................... 7700/9611
2020-02-08T23:43:13.3160953Z .................................................................................................... 7800/9611
2020-02-08T23:43:23.0067690Z .................................................................................................... 7900/9611
2020-02-08T23:43:31.9105337Z .......................................................iiiiiii.i.................................... 8000/9611
2020-02-08T23:43:47.0504966Z ..i................................................................................................. 8200/9611
2020-02-08T23:43:52.6600352Z .................................................................................................... 8300/9611
2020-02-08T23:44:08.4726262Z .................................................................................................... 8400/9611
2020-02-08T23:44:17.2421572Z .................................................................................................... 8500/9611
---
2020-02-08T23:46:20.4663835Z - error[E0521]: borrowed data escapes outside of function
2020-02-08T23:46:20.4663895Z + error[E0521]: borrowed data escapes outside of method
2020-02-08T23:46:20.4664181Z 97   --> $DIR/outlives-suggestion-simple.rs:73:9
2020-02-08T23:46:20.4664235Z 98    |
2020-02-08T23:46:20.4664482Z 99 LL |     fn get_bar(&self) -> Bar2 {
2020-02-08T23:46:20.4664771Z 100    |                -----
2020-02-08T23:46:20.4664825Z 101    |                |
2020-02-08T23:46:20.4665113Z -    |                `self` declared here, outside of the function body
2020-02-08T23:46:20.4665450Z -    |                `self` is a reference that is only valid in the function body
2020-02-08T23:46:20.4665450Z -    |                `self` is a reference that is only valid in the function body
2020-02-08T23:46:20.4665514Z +    |                `self` declared here, outside of the method body
2020-02-08T23:46:20.4665573Z +    |                `self` is a reference that is only valid in the method body
2020-02-08T23:46:20.4665654Z 104 LL |         Bar2::new(&self)
2020-02-08T23:46:20.4665936Z -    |         ^^^^^^^^^^^^^^^^ `self` escapes the function body here
2020-02-08T23:46:20.4665997Z +    |         ^^^^^^^^^^^^^^^^ `self` escapes the method body here
2020-02-08T23:46:20.4666118Z 107 error: aborting due to 9 previous errors
2020-02-08T23:46:20.4666165Z 108 
2020-02-08T23:46:20.4666195Z 
2020-02-08T23:46:20.4666224Z 
2020-02-08T23:46:20.4666224Z 
2020-02-08T23:46:20.4666295Z The actual stderr differed from the expected stderr.
2020-02-08T23:46:20.4666666Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/outlives-suggestion-simple.stderr
2020-02-08T23:46:20.4667269Z To update references, rerun the tests and pass the `--bless` flag
2020-02-08T23:46:20.4667609Z To only update this specific test, also pass `--test-args nll/outlives-suggestion-simple.rs`
2020-02-08T23:46:20.4667700Z error: 1 errors occurred comparing output.
2020-02-08T23:46:20.4667910Z status: exit code: 1
2020-02-08T23:46:20.4667910Z status: exit code: 1
2020-02-08T23:46:20.4668892Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/auxiliary" "-A" "unused"
2020-02-08T23:46:20.4669292Z ------------------------------------------
2020-02-08T23:46:20.4669331Z 
2020-02-08T23:46:20.4669604Z ------------------------------------------
2020-02-08T23:46:20.4669656Z stderr:
---
2020-02-08T23:46:20.4670579Z LL | fn foo1<'a, 'b>(x: &'a usize) -> &'b usize {
2020-02-08T23:46:20.4670858Z    |         --  -- lifetime `'b` defined here
2020-02-08T23:46:20.4670911Z    |         |
2020-02-08T23:46:20.4671151Z    |         lifetime `'a` defined here
2020-02-08T23:46:20.4671230Z LL |     x //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4671512Z    |     ^ returning this value requires that `'a` must outlive `'b`
2020-02-08T23:46:20.4671858Z    = help: consider adding the following bound: `'a: 'b`
2020-02-08T23:46:20.4671896Z 
2020-02-08T23:46:20.4671943Z error: lifetime may not live long enough
2020-02-08T23:46:20.4672219Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:10:5
2020-02-08T23:46:20.4672219Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:10:5
2020-02-08T23:46:20.4672293Z    |
2020-02-08T23:46:20.4672556Z LL | fn foo2<'a>(x: &'a usize) -> &'static usize {
2020-02-08T23:46:20.4672882Z LL |     x //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4672882Z LL |     x //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4673164Z    |     ^ returning this value requires that `'a` must outlive `'static`
2020-02-08T23:46:20.4673488Z    = help: consider replacing `'a` with `'static`
2020-02-08T23:46:20.4673526Z 
2020-02-08T23:46:20.4673572Z error: lifetime may not live long enough
2020-02-08T23:46:20.4673847Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-08T23:46:20.4673847Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-08T23:46:20.4673931Z    |
2020-02-08T23:46:20.4674213Z LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
2020-02-08T23:46:20.4674541Z    |         |
2020-02-08T23:46:20.4674783Z    |         lifetime `'a` defined here
2020-02-08T23:46:20.4674783Z    |         lifetime `'a` defined here
2020-02-08T23:46:20.4674841Z LL |     (x, y) //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4675201Z    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
2020-02-08T23:46:20.4675533Z    = help: consider adding the following bound: `'a: 'b`
2020-02-08T23:46:20.4675592Z 
2020-02-08T23:46:20.4675639Z error: lifetime may not live long enough
2020-02-08T23:46:20.4675914Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-08T23:46:20.4675914Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
2020-02-08T23:46:20.4676126Z    |
2020-02-08T23:46:20.4676428Z LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
2020-02-08T23:46:20.4676900Z    |         |
2020-02-08T23:46:20.4677172Z    |         lifetime `'a` defined here
2020-02-08T23:46:20.4677172Z    |         lifetime `'a` defined here
2020-02-08T23:46:20.4677230Z LL |     (x, y) //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4677562Z    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
2020-02-08T23:46:20.4678043Z    = help: consider adding the following bound: `'b: 'a`
2020-02-08T23:46:20.4678083Z 
2020-02-08T23:46:20.4678350Z help: `'a` and `'b` must be the same: replace one with the other
2020-02-08T23:46:20.4678421Z 
2020-02-08T23:46:20.4678421Z 
2020-02-08T23:46:20.4678468Z error: lifetime may not live long enough
2020-02-08T23:46:20.4678748Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:22:5
2020-02-08T23:46:20.4678823Z    |
2020-02-08T23:46:20.4679093Z LL | fn foo4<'a, 'b, 'c>(x: &'a usize) -> (&'b usize, &'c usize) {
2020-02-08T23:46:20.4679431Z    |         |
2020-02-08T23:46:20.4679683Z    |         lifetime `'a` defined here
2020-02-08T23:46:20.4679733Z ...
2020-02-08T23:46:20.4679733Z ...
2020-02-08T23:46:20.4679784Z LL |     (x, x) //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4680092Z    |     ^^^^^^ returning this value requires that `'a` must outlive `'b`
2020-02-08T23:46:20.4680414Z    = help: consider adding the following bound: `'a: 'b`
2020-02-08T23:46:20.4680478Z 
2020-02-08T23:46:20.4680525Z error: lifetime may not live long enough
2020-02-08T23:46:20.4680803Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:31:9
2020-02-08T23:46:20.4680803Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:31:9
2020-02-08T23:46:20.4680856Z    |
2020-02-08T23:46:20.4681126Z LL |     pub fn foo<'a>(x: &'a usize) -> Self {
2020-02-08T23:46:20.4681383Z    |                -- lifetime `'a` defined here
2020-02-08T23:46:20.4681443Z LL |         Foo { x } //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4681764Z    |         ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
2020-02-08T23:46:20.4682081Z    = help: consider replacing `'a` with `'static`
2020-02-08T23:46:20.4682138Z 
2020-02-08T23:46:20.4682186Z error: lifetime may not live long enough
2020-02-08T23:46:20.4682463Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:41:9
2020-02-08T23:46:20.4682463Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:41:9
2020-02-08T23:46:20.4682515Z    |
2020-02-08T23:46:20.4682770Z LL | impl<'a> Bar<'a> {
2020-02-08T23:46:20.4683016Z    |      -- lifetime `'a` defined here
2020-02-08T23:46:20.4683267Z LL |     pub fn get<'b>(&self) -> &'b usize {
2020-02-08T23:46:20.4683546Z    |                -- lifetime `'b` defined here
2020-02-08T23:46:20.4683606Z LL |         self.x //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4683891Z    |         ^^^^^^ returning this value requires that `'a` must outlive `'b`
2020-02-08T23:46:20.4684226Z    = help: consider adding the following bound: `'a: 'b`
2020-02-08T23:46:20.4684265Z 
2020-02-08T23:46:20.4684320Z error: lifetime may not live long enough
2020-02-08T23:46:20.4684620Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:52:9
2020-02-08T23:46:20.4684620Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:52:9
2020-02-08T23:46:20.4684674Z    |
2020-02-08T23:46:20.4684893Z LL | impl<'a> Baz<'a> {
2020-02-08T23:46:20.4685156Z    |      -- lifetime `'a` defined here
2020-02-08T23:46:20.4685408Z LL |     fn get<'b>(&'b self) -> &'a i32 {
2020-02-08T23:46:20.4685667Z    |            -- lifetime `'b` defined here
2020-02-08T23:46:20.4685747Z LL |         self.x //~ERROR lifetime may not live long enough
2020-02-08T23:46:20.4686104Z    |         ^^^^^^ returning this value requires that `'b` must outlive `'a`
2020-02-08T23:46:20.4686439Z    = help: consider adding the following bound: `'b: 'a`
2020-02-08T23:46:20.4686478Z 
2020-02-08T23:46:20.4686527Z error[E0521]: borrowed data escapes outside of method
2020-02-08T23:46:20.4686803Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:73:9
2020-02-08T23:46:20.4686803Z   --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:73:9
2020-02-08T23:46:20.4686878Z    |
2020-02-08T23:46:20.4687282Z LL |     fn get_bar(&self) -> Bar2 {
2020-02-08T23:46:20.4687586Z    |                |
2020-02-08T23:46:20.4687586Z    |                |
2020-02-08T23:46:20.4687640Z    |                `self` declared here, outside of the method body
2020-02-08T23:46:20.4687698Z    |                `self` is a reference that is only valid in the method body
2020-02-08T23:46:20.4687864Z LL |         Bar2::new(&self) //~ERROR borrowed data escapes outside of function
2020-02-08T23:46:20.4687931Z    |         ^^^^^^^^^^^^^^^^ `self` escapes the method body here
2020-02-08T23:46:20.4706165Z error: aborting due to 9 previous errors
2020-02-08T23:46:20.4706201Z 
2020-02-08T23:46:20.4706230Z 
2020-02-08T23:46:20.4706853Z ------------------------------------------
---
2020-02-08T23:46:20.4726636Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-08T23:46:20.4727695Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-08T23:46:20.4728922Z 
2020-02-08T23:46:20.4729090Z 
2020-02-08T23:46:20.4735069Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-08T23:46:20.4735419Z 
2020-02-08T23:46:20.4735458Z 
2020-02-08T23:46:20.4749621Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-08T23:46:20.4750445Z Build completed unsuccessfully in 1:05:25
2020-02-08T23:46:20.4750445Z Build completed unsuccessfully in 1:05:25
2020-02-08T23:46:20.4823821Z == clock drift check ==
2020-02-08T23:46:20.4849946Z   local time: Sat Feb  8 23:46:20 UTC 2020
2020-02-08T23:46:21.0386886Z   network time: Sat, 08 Feb 2020 23:46:21 GMT
2020-02-08T23:46:21.0387351Z == end clock drift check ==
2020-02-08T23:46:21.4520149Z 
2020-02-08T23:46:21.4620322Z ##[error]Bash exited with code '1'.
2020-02-08T23:46:21.4634279Z ##[section]Finishing: Run build
2020-02-08T23:46:21.4654060Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-02-08T23:46:21.4655853Z Task         : Get sources
2020-02-08T23:46:21.4655913Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T23:46:21.4655957Z Version      : 1.0.0
2020-02-08T23:46:21.4655995Z Author       : Microsoft
2020-02-08T23:46:21.4655995Z Author       : Microsoft
2020-02-08T23:46:21.4656056Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T23:46:21.4656239Z ==============================================================================
2020-02-08T23:46:21.9194021Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T23:46:21.9234720Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67742/merge to s
2020-02-08T23:46:21.9366635Z Cleaning up task key
2020-02-08T23:46:21.9367519Z Start cleaning up orphan processes.
2020-02-08T23:46:21.9488208Z Terminate orphan process: pid (3675) (python)
2020-02-08T23:46:21.9734699Z ##[section]Finishing: Finalize Job
