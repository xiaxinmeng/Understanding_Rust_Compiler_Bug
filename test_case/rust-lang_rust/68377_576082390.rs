plain
2020-01-20T01:30:18.4081956Z ========================== Starting Command Output ===========================
2020-01-20T01:30:18.4084967Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f874ebd3-b18e-45ec-a5ba-bdb4dc2c2de0.sh
2020-01-20T01:30:18.4085287Z 
2020-01-20T01:30:18.4089833Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-20T01:30:18.4098707Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68377/merge to s
2020-01-20T01:30:18.4100892Z Task         : Get sources
2020-01-20T01:30:18.4100933Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T01:30:18.4101036Z Version      : 1.0.0
2020-01-20T01:30:18.4101074Z Author       : Microsoft
---
2020-01-20T01:30:19.3852112Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-20T01:30:19.3869399Z ##[command]git config gc.auto 0
2020-01-20T01:30:19.3872078Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-20T01:30:19.3874445Z ##[command]git config --get-all http.proxy
2020-01-20T01:30:19.3881109Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68377/merge:refs/remotes/pull/68377/merge
---
2020-01-20T02:22:03.4081226Z .................................................................................................... 1700/9539
2020-01-20T02:22:09.4890155Z .................................................................................................... 1800/9539
2020-01-20T02:22:20.4073901Z ...................i................................................................................ 1900/9539
2020-01-20T02:22:27.2365795Z .................................................................................................... 2000/9539
2020-01-20T02:22:41.7145482Z .........iiiii...................................................................................... 2100/9539
2020-01-20T02:22:50.7486595Z .................................................................................................... 2300/9539
2020-01-20T02:22:53.0763984Z .................................................................................................... 2400/9539
2020-01-20T02:22:58.1287735Z .................................................................................................... 2500/9539
2020-01-20T02:23:17.4156275Z .................................................................................................... 2600/9539
---
2020-01-20T02:25:52.4323027Z .....................................................i...............i.............................. 4900/9539
2020-01-20T02:25:59.9138319Z .................................................................................................... 5000/9539
2020-01-20T02:26:07.2428562Z ................................................................................................i... 5100/9539
2020-01-20T02:26:12.2143358Z .................................................................................................... 5200/9539
2020-01-20T02:26:22.3778317Z ....................................................................ii.ii...........i............... 5300/9539
2020-01-20T02:26:31.0870022Z .....i.............................................................................................. 5500/9539
2020-01-20T02:26:40.4688825Z .................................................................................................... 5600/9539
2020-01-20T02:26:46.7479245Z ......................................................i............................................. 5700/9539
2020-01-20T02:26:53.3724920Z .................................................................................................... 5800/9539
2020-01-20T02:26:53.3724920Z .................................................................................................... 5800/9539
2020-01-20T02:27:02.8477315Z .................................................................................................... 5900/9539
2020-01-20T02:27:09.4453565Z ............................................ii...i..ii...........i.................................. 6000/9539
2020-01-20T02:27:30.5945404Z .................................................................................................... 6200/9539
2020-01-20T02:27:38.2851798Z .................................................................................................... 6300/9539
2020-01-20T02:27:38.2851798Z .................................................................................................... 6300/9539
2020-01-20T02:27:45.2496457Z ........................................................................i..ii....................... 6400/9539
2020-01-20T02:28:12.2315248Z .................................................................................................... 6600/9539
2020-01-20T02:28:15.0906210Z ................................................i................................................... 6700/9539
2020-01-20T02:28:17.2912301Z .................................................................................................... 6800/9539
2020-01-20T02:28:19.5147757Z ...............................................i.................................................... 6900/9539
---
2020-01-20T02:29:51.7756486Z .................................................................................................... 7500/9539
2020-01-20T02:29:56.3594197Z .................................................................................................... 7600/9539
2020-01-20T02:30:02.0396829Z .................................................................................................... 7700/9539
2020-01-20T02:30:08.7976094Z .................................................................................................... 7800/9539
2020-01-20T02:30:19.3326570Z ..................................................................................................ii 7900/9539
2020-01-20T02:30:25.5776913Z iiiii............................................................................................... 8000/9539
2020-01-20T02:30:40.3336148Z .................................................................................................... 8200/9539
2020-01-20T02:30:51.7110433Z .................................................................................................... 8300/9539
2020-01-20T02:31:04.1403521Z .................................................................................................... 8400/9539
2020-01-20T02:31:09.8213325Z .................................................................................................... 8500/9539
---
2020-01-20T02:33:02.2774230Z 
2020-01-20T02:33:02.2775078Z ---- [ui] ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait.rs stdout ----
2020-01-20T02:33:02.2775405Z diff of stderr:
2020-01-20T02:33:02.2775614Z 
2020-01-20T02:33:02.2775861Z 1 error[E0038]: the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2776868Z +   --> $DIR/object-unsafe-trait-in-return-position-dyn-trait.rs:21:13
2020-01-20T02:33:02.2777160Z 3    |
2020-01-20T02:33:02.2777576Z 4 LL |     fn foo() -> Self;
2020-01-20T02:33:02.2778097Z 5    |        --- associated function `foo` has no `self` parameter
2020-01-20T02:33:02.2778097Z 5    |        --- associated function `foo` has no `self` parameter
2020-01-20T02:33:02.2778341Z 
2020-01-20T02:33:02.2778539Z 6 ...
2020-01-20T02:33:02.2778956Z 7 LL | fn car() -> dyn NotObjectSafe {
2020-01-20T02:33:02.2779507Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2779802Z +    |             ^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2780077Z 9 
2020-01-20T02:33:02.2780451Z 10 error[E0038]: the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2781536Z +   --> $DIR/object-unsafe-trait-in-return-position-dyn-trait.rs:28:13
2020-01-20T02:33:02.2781798Z 12    |
2020-01-20T02:33:02.2782204Z 13 LL |     fn foo() -> Self;
2020-01-20T02:33:02.2782729Z 14    |        --- associated function `foo` has no `self` parameter
2020-01-20T02:33:02.2782729Z 14    |        --- associated function `foo` has no `self` parameter
2020-01-20T02:33:02.2783091Z 
2020-01-20T02:33:02.2783421Z 15 ...
2020-01-20T02:33:02.2783924Z 16 LL | fn cat() -> Box<dyn NotObjectSafe> {
2020-01-20T02:33:02.2784399Z -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2784626Z +    |             ^^^^^^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2785102Z 19 error: aborting due to 2 previous errors
2020-01-20T02:33:02.2785267Z 20 
2020-01-20T02:33:02.2785388Z 
2020-01-20T02:33:02.2785518Z 
2020-01-20T02:33:02.2785518Z 
2020-01-20T02:33:02.2785683Z The actual stderr differed from the expected stderr.
2020-01-20T02:33:02.2786341Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait/object-unsafe-trait-in-return-position-dyn-trait.stderr
2020-01-20T02:33:02.2786879Z To update references, rerun the tests and pass the `--bless` flag
2020-01-20T02:33:02.2787320Z To only update this specific test, also pass `--test-args impl-trait/object-unsafe-trait-in-return-position-dyn-trait.rs`
2020-01-20T02:33:02.2787631Z error: 1 errors occurred comparing output.
2020-01-20T02:33:02.2787761Z status: exit code: 1
2020-01-20T02:33:02.2787761Z status: exit code: 1
2020-01-20T02:33:02.2788698Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/object-unsafe-trait-in-return-position-dyn-trait/auxiliary" "-A" "unused"
2020-01-20T02:33:02.2789297Z ------------------------------------------
2020-01-20T02:33:02.2789454Z 
2020-01-20T02:33:02.2789782Z ------------------------------------------
2020-01-20T02:33:02.2789969Z stderr:
2020-01-20T02:33:02.2789969Z stderr:
2020-01-20T02:33:02.2790296Z ------------------------------------------
2020-01-20T02:33:02.2790482Z error[E0038]: the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2791076Z    |
2020-01-20T02:33:02.2791412Z LL |     fn foo() -> Self;
2020-01-20T02:33:02.2791792Z    |        --- associated function `foo` has no `self` parameter
2020-01-20T02:33:02.2791960Z ...
2020-01-20T02:33:02.2791960Z ...
2020-01-20T02:33:02.2792388Z LL | fn car() -> dyn NotObjectSafe { //~ ERROR the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2792577Z    |             ^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2792694Z 
2020-01-20T02:33:02.2792844Z error[E0038]: the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2793403Z    |
2020-01-20T02:33:02.2794244Z LL |     fn foo() -> Self;
2020-01-20T02:33:02.2794476Z    |        --- associated function `foo` has no `self` parameter
2020-01-20T02:33:02.2794544Z ...
2020-01-20T02:33:02.2794544Z ...
2020-01-20T02:33:02.2794926Z LL | fn cat() -> Box<dyn NotObjectSafe> { //~ ERROR the trait `NotObjectSafe` cannot be made into an
2020-01-20T02:33:02.2794995Z    |             ^^^^^^^^^^^^^^^^^^^^^^ the trait `NotObjectSafe` cannot be made into an object
2020-01-20T02:33:02.2795085Z error: aborting due to 2 previous errors
2020-01-20T02:33:02.2795112Z 
2020-01-20T02:33:02.2795372Z For more information about this error, try `rustc --explain E0038`.
2020-01-20T02:33:02.2795421Z 
---
2020-01-20T02:33:02.2809447Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-20T02:33:02.2809769Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-20T02:33:02.2828958Z 
2020-01-20T02:33:02.2829195Z 
2020-01-20T02:33:02.2830933Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-20T02:33:02.2831425Z 
2020-01-20T02:33:02.2831538Z 
2020-01-20T02:33:02.2837906Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-20T02:33:02.2838474Z Build completed unsuccessfully in 0:57:19
2020-01-20T02:33:02.2838474Z Build completed unsuccessfully in 0:57:19
2020-01-20T02:33:02.2894837Z == clock drift check ==
2020-01-20T02:33:02.2914527Z   local time: Mon Jan 20 02:33:02 UTC 2020
2020-01-20T02:33:02.5634244Z   network time: Mon, 20 Jan 2020 02:33:02 GMT
2020-01-20T02:33:02.5644947Z == end clock drift check ==
2020-01-20T02:33:03.3253526Z 
2020-01-20T02:33:03.3379438Z ##[error]Bash exited with code '1'.
2020-01-20T02:33:03.3395474Z ##[section]Finishing: Run build
2020-01-20T02:33:03.3434946Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68377/merge to s
2020-01-20T02:33:03.3437394Z Task         : Get sources
2020-01-20T02:33:03.3437448Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-20T02:33:03.3437526Z Version      : 1.0.0
2020-01-20T02:33:03.3437573Z Author       : Microsoft
2020-01-20T02:33:03.3437573Z Author       : Microsoft
2020-01-20T02:33:03.3437626Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-20T02:33:03.3437732Z ==============================================================================
2020-01-20T02:33:03.7150059Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-20T02:33:03.7211790Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68377/merge to s
2020-01-20T02:33:03.7314152Z Cleaning up task key
2020-01-20T02:33:03.7315146Z Start cleaning up orphan processes.
2020-01-20T02:33:03.7424303Z Terminate orphan process: pid (3928) (python)
2020-01-20T02:33:03.7627778Z ##[section]Finishing: Finalize Job
