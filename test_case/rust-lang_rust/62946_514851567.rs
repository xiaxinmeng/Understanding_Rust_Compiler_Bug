plain
2019-07-24T23:44:25.1771677Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T23:44:25.1980976Z ##[command]git config gc.auto 0
2019-07-24T23:44:25.2037213Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T23:44:25.2095114Z ##[command]git config --get-all http.proxy
2019-07-24T23:44:25.2220461Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62946/merge:refs/remotes/pull/62946/merge
---
2019-07-24T23:44:54.5187642Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T23:44:54.5187674Z 
2019-07-24T23:44:54.5187885Z   git checkout -b <new-branch-name>
2019-07-24T23:44:54.5187915Z 
2019-07-24T23:44:54.5187983Z HEAD is now at bcd4b5c5d Merge 2bebfd3416257e1785b48619d2d37bfad800afbd into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-24T23:44:54.5330478Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T23:44:54.5332792Z ==============================================================================
2019-07-24T23:44:54.5332838Z Task         : Bash
2019-07-24T23:44:54.5332876Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T00:45:11.1049550Z .................................................................................................... 200/5851
2019-07-25T00:45:15.3781574Z .................................................................................................... 300/5851
2019-07-25T00:45:19.1000030Z .................................................................................................... 400/5851
2019-07-25T00:45:22.9127519Z .................................................................................................... 500/5851
2019-07-25T00:45:26.7667801Z ........................................................................i........................... 600/5851
2019-07-25T00:45:35.8713431Z .................................................................................................... 800/5851
2019-07-25T00:45:41.4206803Z .............................................F...................................................... 900/5851
2019-07-25T00:45:46.4358688Z ...................................................................................................i 1000/5851
2019-07-25T00:45:46.4358688Z ...................................................................................................i 1000/5851
2019-07-25T00:45:52.1004993Z ...........i........................................................................................ 1100/5851
2019-07-25T00:45:56.0861704Z .............................iiiii.................................................................. 1200/5851
2019-07-25T00:46:02.2277790Z .................................................................................................... 1400/5851
2019-07-25T00:46:05.0237569Z .................................................................................................... 1500/5851
2019-07-25T00:46:08.8793907Z .................................................................................................... 1600/5851
2019-07-25T00:46:11.5937269Z .................................................................................................... 1700/5851
2019-07-25T00:46:11.5937269Z .................................................................................................... 1700/5851
2019-07-25T00:46:15.0798723Z .....................................................................i.............................. 1800/5851
2019-07-25T00:46:23.8923239Z .................................................................................................... 2000/5851
2019-07-25T00:46:28.2935356Z .................................................................................................... 2100/5851
2019-07-25T00:46:32.0967076Z .................................................................................................... 2200/5851
2019-07-25T00:46:32.0967076Z .................................................................................................... 2200/5851
2019-07-25T00:46:35.9828389Z .....................................................i.............................................. 2300/5851
2019-07-25T00:46:45.7477251Z .................................................................................................... 2500/5851
2019-07-25T00:46:49.9470792Z .................................................................................................... 2600/5851
2019-07-25T00:46:55.1543124Z .................................................................................................... 2700/5851
2019-07-25T00:46:59.0705602Z .................................................................................................... 2800/5851
2019-07-25T00:46:59.0705602Z .................................................................................................... 2800/5851
2019-07-25T00:47:03.6501594Z .................................................................................................... 2900/5851
2019-07-25T00:47:09.0495492Z .................................................................................................... 3000/5851
2019-07-25T00:47:13.6433922Z ................................................................................F................... 3100/5851
2019-07-25T00:47:19.0689891Z .................................................................................................... 3200/5851
2019-07-25T00:47:22.6713317Z .................................................................................................... 3300/5851
2019-07-25T00:47:26.4600064Z .................................................................................................... 3400/5851
2019-07-25T00:47:31.5813676Z .................................................................................................... 3500/5851
2019-07-25T00:47:35.3973622Z ....................i............................................................................... 3600/5851
2019-07-25T00:47:39.5669535Z ..............................................................................................ii...i 3700/5851
2019-07-25T00:47:43.3820193Z ..ii................................................................................................ 3800/5851
2019-07-25T00:47:52.2996625Z .................................................................................................... 4000/5851
2019-07-25T00:47:52.2996625Z .................................................................................................... 4000/5851
2019-07-25T00:47:56.1113630Z ........ii.......................................................................................... 4100/5851
2019-07-25T00:47:58.1272953Z .............................i...................................................................... 4200/5851
2019-07-25T00:48:00.1999066Z ................................................................................................i... 4300/5851
2019-07-25T00:48:06.8968514Z .................................................................................................... 4500/5851
2019-07-25T00:48:24.8224123Z .................................................................................................... 4600/5851
2019-07-25T00:48:28.4030773Z .................................................................................................... 4700/5851
2019-07-25T00:48:32.0799387Z .................................................................................................... 4800/5851
---
2019-07-25T00:49:05.6673552Z .................................................................................................... 5400/5851
2019-07-25T00:49:09.6076067Z .................................................................................................... 5500/5851
2019-07-25T00:49:13.7556390Z .................................................................................................... 5600/5851
2019-07-25T00:49:16.8507273Z .................................................................................................... 5700/5851
2019-07-25T00:49:19.8655843Z ...........................................................................................i........ 5800/5851
2019-07-25T00:49:21.6817680Z failures:
2019-07-25T00:49:21.6867882Z 
2019-07-25T00:49:21.6869418Z ---- [ui] ui/consts/const-eval/match-test-ptr-null.rs stdout ----
2019-07-25T00:49:21.6869509Z diff of stderr:
2019-07-25T00:49:21.6869509Z diff of stderr:
2019-07-25T00:49:21.6869544Z 
2019-07-25T00:49:21.6869589Z 1 error[E0658]: casting pointers to integers in constants is unstable
2019-07-25T00:49:21.6869894Z -   --> $DIR/match-test-ptr-null.rs:7:15
2019-07-25T00:49:21.6870297Z +   --> $DIR/match-test-ptr-null.rs:6:15
2019-07-25T00:49:21.6870338Z 3    |
2019-07-25T00:49:21.6870379Z 4 LL |         match &1 as *const i32 as usize {
2019-07-25T00:49:21.6870483Z 
2019-07-25T00:49:21.6870483Z 
2019-07-25T00:49:21.6870535Z 8    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-25T00:49:21.6870644Z 10 error[E0019]: constant contains unimplemented expression type
2019-07-25T00:49:21.6870894Z -   --> $DIR/match-test-ptr-null.rs:7:15
2019-07-25T00:49:21.6871098Z +   --> $DIR/match-test-ptr-null.rs:6:15
2019-07-25T00:49:21.6871138Z 12    |
2019-07-25T00:49:21.6871138Z 12    |
2019-07-25T00:49:21.6871179Z 13 LL |         match &1 as *const i32 as usize {
2019-07-25T00:49:21.6871268Z 
2019-07-25T00:49:21.6871303Z 15 
2019-07-25T00:49:21.6871345Z 16 error[E0019]: constant contains unimplemented expression type
2019-07-25T00:49:21.6871555Z -   --> $DIR/match-test-ptr-null.rs:11:13
---
2019-07-25T00:49:21.6872176Z 22 error[E0080]: evaluation of constant value failed
2019-07-25T00:49:21.6873050Z -   --> $DIR/match-test-ptr-null.rs:7:15
2019-07-25T00:49:21.6873302Z +   --> $DIR/match-test-ptr-null.rs:6:15
2019-07-25T00:49:21.6873349Z 24    |
2019-07-25T00:49:21.6873414Z 25 LL |         match &1 as *const i32 as usize {
2019-07-25T00:49:21.6873716Z 26    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-07-25T00:49:21.6873806Z 
2019-07-25T00:49:21.6873853Z The actual stderr differed from the expected stderr.
2019-07-25T00:49:21.6874170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
2019-07-25T00:49:21.6874170Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
2019-07-25T00:49:21.6874434Z To update references, rerun the tests and pass the `--bless` flag
2019-07-25T00:49:21.6874769Z To only update this specific test, also pass `--test-args consts/const-eval/match-test-ptr-null.rs`
2019-07-25T00:49:21.6874859Z error: 1 errors occurred comparing output.
2019-07-25T00:49:21.6874924Z status: exit code: 1
2019-07-25T00:49:21.6874924Z status: exit code: 1
2019-07-25T00:49:21.6875710Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/auxiliary" "-A" "unused"
2019-07-25T00:49:21.6876070Z ------------------------------------------
2019-07-25T00:49:21.6876106Z 
2019-07-25T00:49:21.6876620Z ------------------------------------------
2019-07-25T00:49:21.6876673Z stderr:
2019-07-25T00:49:21.6876673Z stderr:
2019-07-25T00:49:21.6877055Z ------------------------------------------
2019-07-25T00:49:21.6877118Z error[E0658]: casting pointers to integers in constants is unstable
2019-07-25T00:49:21.6877340Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-07-25T00:49:21.6877385Z    |
2019-07-25T00:49:21.6877558Z LL |         match &1 as *const i32 as usize {
2019-07-25T00:49:21.6877639Z    |
2019-07-25T00:49:21.6877639Z    |
2019-07-25T00:49:21.6877994Z    = note: for more information, see ***/issues/51910
2019-07-25T00:49:21.6878048Z    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-25T00:49:21.6878138Z error[E0019]: constant contains unimplemented expression type
2019-07-25T00:49:21.6878382Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-07-25T00:49:21.6878425Z    |
2019-07-25T00:49:21.6878425Z    |
2019-07-25T00:49:21.6878486Z LL |         match &1 as *const i32 as usize {
2019-07-25T00:49:21.6878556Z 
2019-07-25T00:49:21.6878597Z error[E0019]: constant contains unimplemented expression type
2019-07-25T00:49:21.6878835Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:10:13
2019-07-25T00:49:21.6878878Z    |
2019-07-25T00:49:21.6878878Z    |
2019-07-25T00:49:21.6878921Z LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
2019-07-25T00:49:21.6879006Z 
2019-07-25T00:49:21.6879046Z error[E0080]: evaluation of constant value failed
2019-07-25T00:49:21.6879354Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-07-25T00:49:21.6879398Z    |
2019-07-25T00:49:21.6879398Z    |
2019-07-25T00:49:21.6879438Z LL |         match &1 as *const i32 as usize {
2019-07-25T00:49:21.6879824Z    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-07-25T00:49:21.6879902Z error: aborting due to 4 previous errors
2019-07-25T00:49:21.6879927Z 
2019-07-25T00:49:21.6879984Z Some errors have detailed explanations: E0019, E0080, E0658.
2019-07-25T00:49:21.6880198Z For more information about an error, try `rustc --explain E0019`.
2019-07-25T00:49:21.6880198Z For more information about an error, try `rustc --explain E0019`.
2019-07-25T00:49:21.6880228Z 
2019-07-25T00:49:21.6880427Z ------------------------------------------
2019-07-25T00:49:21.6880455Z 
2019-07-25T00:49:21.6880479Z 
2019-07-25T00:49:21.6880688Z ---- [ui] ui/issues/issue-52023-array-size-pointer-cast.rs stdout ----
2019-07-25T00:49:21.6880747Z diff of stderr:
2019-07-25T00:49:21.6880772Z 
2019-07-25T00:49:21.6881196Z 7    = note: for more information, see ***/issues/51910
2019-07-25T00:49:21.6881268Z 8    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-25T00:49:21.6881540Z - error[E0080]: it is undefined behavior to use this value
2019-07-25T00:49:21.6881540Z - error[E0080]: it is undefined behavior to use this value
2019-07-25T00:49:21.6881594Z + error[E0080]: evaluation of constant value failed
2019-07-25T00:49:21.6884486Z 12    |
2019-07-25T00:49:21.6884486Z 12    |
2019-07-25T00:49:21.6884534Z 13 LL |     let _ = [0; (&0 as *const i32) as usize];
2019-07-25T00:49:21.6884592Z 
2019-07-25T00:49:21.6884994Z -    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-07-25T00:49:21.6885188Z -    |
2019-07-25T00:49:21.6885597Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-25T00:49:21.6885914Z +    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-07-25T00:49:21.6886055Z 18 error: aborting due to 2 previous errors
2019-07-25T00:49:21.6886381Z 19 
2019-07-25T00:49:21.6886415Z 
2019-07-25T00:49:21.6886438Z 
2019-07-25T00:49:21.6886438Z 
2019-07-25T00:49:21.6886496Z The actual stderr differed from the expected stderr.
2019-07-25T00:49:21.6886820Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast/issue-52023-array-size-pointer-cast.stderr
2019-07-25T00:49:21.6887038Z To update references, rerun the tests and pass the `--bless` flag
2019-07-25T00:49:21.6887302Z To only update this specific test, also pass `--test-args issues/issue-52023-array-size-pointer-cast.rs`
2019-07-25T00:49:21.6887375Z error: 1 errors occurred comparing output.
2019-07-25T00:49:21.6887431Z status: exit code: 1
2019-07-25T00:49:21.6887431Z status: exit code: 1
2019-07-25T00:49:21.6888118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast/auxiliary" "-A" "unused"
2019-07-25T00:49:21.6888414Z ------------------------------------------
2019-07-25T00:49:21.6888444Z 
2019-07-25T00:49:21.6888645Z ------------------------------------------
2019-07-25T00:49:21.6888684Z stderr:
2019-07-25T00:49:21.6888684Z stderr:
2019-07-25T00:49:21.6888865Z ------------------------------------------
2019-07-25T00:49:21.6888928Z error[E0658]: casting pointers to integers in constants is unstable
2019-07-25T00:49:21.6889154Z   --> /checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs:2:17
2019-07-25T00:49:21.6889275Z    |
2019-07-25T00:49:21.6889347Z LL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants
2019-07-25T00:49:21.6889435Z    |
2019-07-25T00:49:21.6889435Z    |
2019-07-25T00:49:21.6889917Z    = note: for more information, see ***/issues/51910
2019-07-25T00:49:21.6889974Z    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-25T00:49:21.6890064Z error[E0080]: evaluation of constant value failed
2019-07-25T00:49:21.6890306Z   --> /checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs:2:17
2019-07-25T00:49:21.6890350Z    |
2019-07-25T00:49:21.6890350Z    |
2019-07-25T00:49:21.6890636Z LL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants
2019-07-25T00:49:21.6895346Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
2019-07-25T00:49:21.6895488Z error: aborting due to 2 previous errors
2019-07-25T00:49:21.6895537Z 
2019-07-25T00:49:21.6895591Z Some errors have detailed explanations: E0080, E0658.
2019-07-25T00:49:21.6895983Z For more information about an error, try `rustc --explain E0080`.
---
2019-07-25T00:49:21.6898360Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-25T00:49:21.6898442Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T00:49:21.6913939Z 
2019-07-25T00:49:21.6914217Z 
2019-07-25T00:49:21.6916709Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-25T00:49:21.6917018Z 
2019-07-25T00:49:21.6917049Z 
2019-07-25T00:49:21.6925593Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-25T00:49:21.6925742Z Build completed unsuccessfully in 0:58:06
2019-07-25T00:49:21.6925742Z Build completed unsuccessfully in 0:58:06
2019-07-25T00:49:22.6595468Z ##[error]Bash exited with code '1'.
2019-07-25T00:49:22.6634481Z ##[section]Starting: Checkout
2019-07-25T00:49:22.6636445Z ==============================================================================
2019-07-25T00:49:22.6636998Z Task         : Get sources
2019-07-25T00:49:22.6637038Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
