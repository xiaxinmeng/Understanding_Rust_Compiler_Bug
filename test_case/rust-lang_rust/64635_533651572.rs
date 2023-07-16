plain
2019-09-20T16:41:57.1984412Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T16:41:57.2166573Z ##[command]git config gc.auto 0
2019-09-20T16:41:57.2238783Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T16:41:57.2288399Z ##[command]git config --get-all http.proxy
2019-09-20T16:41:57.2440528Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64635/merge:refs/remotes/pull/64635/merge
---
2019-09-20T17:43:16.1417668Z .................................................................................................... 1500/9030
2019-09-20T17:43:21.9367603Z ...........................................................F........................................ 1600/9030
2019-09-20T17:43:34.0933494Z ......................................................................i...............i............. 1700/9030
2019-09-20T17:43:40.6289763Z .................................................................................................... 1800/9030
2019-09-20T17:43:54.2747004Z .............................................................iiiii.................................. 1900/9030
2019-09-20T17:44:07.5589711Z .................................................................................................... 2100/9030
2019-09-20T17:44:10.0775069Z .................................................................................................... 2200/9030
2019-09-20T17:44:13.3020469Z .................................................................................................... 2300/9030
2019-09-20T17:44:21.5459744Z .................................................................................................... 2400/9030
---
2019-09-20T17:47:14.7060311Z .................................................i...............i.................................. 4700/9030
2019-09-20T17:47:23.8104621Z .................................................................................................... 4800/9030
2019-09-20T17:47:31.7622232Z .................................................................................................... 4900/9030
2019-09-20T17:47:41.1849751Z .................................................................................................... 5000/9030
2019-09-20T17:47:48.9015932Z .................................ii.ii.............................................................. 5100/9030
2019-09-20T17:47:58.3894168Z .................................................................................................... 5300/9030
2019-09-20T17:48:08.7609603Z .................................................................................................i.. 5400/9030
2019-09-20T17:48:17.0754583Z .................................................................................................... 5500/9030
2019-09-20T17:48:21.8524367Z .................................................................................................... 5600/9030
2019-09-20T17:48:21.8524367Z .................................................................................................... 5600/9030
2019-09-20T17:48:32.3433463Z ............................................................................................ii...i.. 5700/9030
2019-09-20T17:48:46.3062320Z ii...........i...................................................................................... 5800/9030
2019-09-20T17:49:06.3717148Z .................................................................................................... 6000/9030
2019-09-20T17:49:06.3717148Z .................................................................................................... 6000/9030
2019-09-20T17:49:12.3751457Z ..............................................................................................i..ii. 6100/9030
2019-09-20T17:49:39.5963111Z .................................................................................................... 6300/9030
2019-09-20T17:49:44.2282900Z .....................................................i.............................................. 6400/9030
2019-09-20T17:49:46.3729854Z .................................................................................................... 6500/9030
2019-09-20T17:49:48.7862846Z .........................i.......................................................................... 6600/9030
---
2019-09-20T17:53:49.5804350Z failures:
2019-09-20T17:53:49.5845557Z 
2019-09-20T17:53:49.5846272Z ---- [ui] ui/consts/const-eval/const_fn_ptr_fail2.rs stdout ----
2019-09-20T17:53:49.5846520Z 
2019-09-20T17:53:49.5847009Z error: /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:8: unexpected error: '8:5: 8:9: any use of this value will cause an error [const_err]'
2019-09-20T17:53:49.5847193Z 
2019-09-20T17:53:49.5847672Z error: /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:8: unexpected error: '8:5: 8:9: any use of this value will cause an error [const_err]'
2019-09-20T17:53:49.5847848Z 
2019-09-20T17:53:49.5848279Z error: /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:11: expected error not found: any use of this value will cause an error
2019-09-20T17:53:49.5848496Z 
2019-09-20T17:53:49.5848936Z error: /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs:14: expected error not found: any use of this value will cause an error
2019-09-20T17:53:49.5849288Z error: 2 unexpected errors found, 2 expected errors not found
2019-09-20T17:53:49.5849435Z status: exit code: 1
2019-09-20T17:53:49.5849435Z status: exit code: 1
2019-09-20T17:53:49.5850296Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail2/auxiliary" "-A" "unused"
2019-09-20T17:53:49.5851472Z unexpected errors (from JSON output): [
2019-09-20T17:53:49.5851721Z     Error {
2019-09-20T17:53:49.5851894Z         line_num: 8,
2019-09-20T17:53:49.5852072Z         kind: Some(
2019-09-20T17:53:49.5852399Z         ),
2019-09-20T17:53:49.5852399Z         ),
2019-09-20T17:53:49.5852583Z         msg: "8:5: 8:9: any use of this value will cause an error [const_err]",
2019-09-20T17:53:49.5852915Z     Error {
2019-09-20T17:53:49.5853106Z         line_num: 8,
2019-09-20T17:53:49.5853268Z         kind: Some(
2019-09-20T17:53:49.5853427Z             Error,
2019-09-20T17:53:49.5853427Z             Error,
2019-09-20T17:53:49.5853608Z         ),
2019-09-20T17:53:49.5853777Z         msg: "8:5: 8:9: any use of this value will cause an error [const_err]",
2019-09-20T17:53:49.5854239Z ]
2019-09-20T17:53:49.5854480Z 
2019-09-20T17:53:49.5854624Z not found errors (from test file): [
2019-09-20T17:53:49.5854778Z     Error {
2019-09-20T17:53:49.5854778Z     Error {
2019-09-20T17:53:49.5854919Z         line_num: 11,
2019-09-20T17:53:49.5855072Z         kind: Some(
2019-09-20T17:53:49.5855236Z             Error,
2019-09-20T17:53:49.5855376Z         ),
2019-09-20T17:53:49.5855518Z         msg: "any use of this value will cause an error",
2019-09-20T17:53:49.5855816Z     Error {
2019-09-20T17:53:49.5855953Z         line_num: 14,
2019-09-20T17:53:49.5856108Z         kind: Some(
2019-09-20T17:53:49.5856247Z             Error,
2019-09-20T17:53:49.5856247Z             Error,
2019-09-20T17:53:49.5856385Z         ),
2019-09-20T17:53:49.5856545Z         msg: "any use of this value will cause an error",
2019-09-20T17:53:49.5856818Z ]
2019-09-20T17:53:49.5856959Z 
2019-09-20T17:53:49.5857458Z thread '[ui] ui/consts/const-eval/const_fn_ptr_fail2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1512:13
2019-09-20T17:53:49.5857674Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T17:53:49.5857674Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-20T17:53:49.5857834Z 
2019-09-20T17:53:49.5858193Z ---- [ui] ui/consts/issue-56164.rs stdout ----
2019-09-20T17:53:49.5858609Z diff of stderr:
2019-09-20T17:53:49.5858856Z 
2019-09-20T17:53:49.5859043Z 4 LL | const fn foo() { (||{})() }
2019-09-20T17:53:49.5859353Z 6 
2019-09-20T17:53:49.5859761Z - error: function pointers are not allowed in const fn
2019-09-20T17:53:49.5859960Z + error[E0658]: function pointers in const fn are unstable
2019-09-20T17:53:49.5860445Z 8   --> $DIR/issue-56164.rs:8:5
2019-09-20T17:53:49.5860445Z 8   --> $DIR/issue-56164.rs:8:5
2019-09-20T17:53:49.5861052Z 9    |
2019-09-20T17:53:49.5861256Z 10 LL |     input()
2019-09-20T17:53:49.5861430Z 
2019-09-20T17:53:49.5861592Z 11    |     ^^^^^^^
2019-09-20T17:53:49.5861747Z +    |
2019-09-20T17:53:49.5862395Z +    = note: for more information, see ***/issues/51909
2019-09-20T17:53:49.5862824Z 12 
2019-09-20T17:53:49.5862993Z 13 error: aborting due to 2 previous errors
2019-09-20T17:53:49.5863152Z 14 
2019-09-20T17:53:49.5863327Z 
---
2019-09-20T17:53:49.5864890Z 
2019-09-20T17:53:49.5865013Z 
2019-09-20T17:53:49.5865177Z The actual stderr differed from the expected stderr.
2019-09-20T17:53:49.5865596Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164/issue-56164.stderr
2019-09-20T17:53:49.5866010Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T17:53:49.5866466Z To only update this specific test, also pass `--test-args consts/issue-56164.rs`
2019-09-20T17:53:49.5866817Z error: 1 errors occurred comparing output.
2019-09-20T17:53:49.5866974Z status: exit code: 1
2019-09-20T17:53:49.5866974Z status: exit code: 1
2019-09-20T17:53:49.5867781Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-56164.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-56164/auxiliary" "-A" "unused"
2019-09-20T17:53:49.5868415Z ------------------------------------------
2019-09-20T17:53:49.5868623Z 
2019-09-20T17:53:49.5869000Z ------------------------------------------
2019-09-20T17:53:49.5869190Z stderr:
2019-09-20T17:53:49.5869190Z stderr:
2019-09-20T17:53:49.5869697Z ------------------------------------------
2019-09-20T17:53:49.5869908Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-20T17:53:49.5870432Z   --> /checkout/src/test/ui/consts/issue-56164.rs:3:18
2019-09-20T17:53:49.5871169Z    |
2019-09-20T17:53:49.5871364Z LL | const fn foo() { (||{})() }
2019-09-20T17:53:49.5871818Z 
2019-09-20T17:53:49.5872045Z error[E0658]: function pointers in const fn are unstable
2019-09-20T17:53:49.5872619Z   --> /checkout/src/test/ui/consts/issue-56164.rs:8:5
2019-09-20T17:53:49.5873794Z    |
2019-09-20T17:53:49.5873794Z    |
2019-09-20T17:53:49.5873860Z LL |     input()
2019-09-20T17:53:49.5873902Z    |     ^^^^^^^
2019-09-20T17:53:49.5874067Z    |
2019-09-20T17:53:49.5874566Z    = note: for more information, see ***/issues/51909
2019-09-20T17:53:49.5874648Z 
2019-09-20T17:53:49.5874686Z error: aborting due to 2 previous errors
2019-09-20T17:53:49.5874725Z 
2019-09-20T17:53:49.5874764Z Some errors have detailed explanations: E0015, E0658.
---
2019-09-20T17:53:49.5875795Z diff of stderr:
2019-09-20T17:53:49.5875819Z 
2019-09-20T17:53:49.5875866Z 8    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-20T17:53:49.5875927Z 9    |
2019-09-20T17:53:49.5876155Z 10    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-20T17:53:49.5876402Z -               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}`
2019-09-20T17:53:49.5876659Z +               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-20T17:53:49.5876703Z 12 
2019-09-20T17:53:49.5877318Z - error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::intrinsics::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T17:53:49.5877655Z + error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T17:53:49.5877852Z 14   --> $DIR/reify-intrinsic.rs:11:13
2019-09-20T17:53:49.5877891Z 15    |
2019-09-20T17:53:49.5878129Z 16 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-20T17:53:49.5878181Z 
2019-09-20T17:53:49.5878219Z The actual stderr differed from the expected stderr.
2019-09-20T17:53:49.5878484Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-20T17:53:49.5878484Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
2019-09-20T17:53:49.5878696Z To update references, rerun the tests and pass the `--bless` flag
2019-09-20T17:53:49.5878912Z To only update this specific test, also pass `--test-args reify-intrinsic.rs`
2019-09-20T17:53:49.5879005Z error: 1 errors occurred comparing output.
2019-09-20T17:53:49.5879044Z status: exit code: 1
2019-09-20T17:53:49.5879044Z status: exit code: 1
2019-09-20T17:53:49.5879663Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary" "-A" "unused"
2019-09-20T17:53:49.5879927Z ------------------------------------------
2019-09-20T17:53:49.5879969Z 
2019-09-20T17:53:49.5880159Z ------------------------------------------
2019-09-20T17:53:49.5880197Z stderr:
2019-09-20T17:53:49.5880197Z stderr:
2019-09-20T17:53:49.5880514Z ------------------------------------------
2019-09-20T17:53:49.5880576Z error[E0308]: cannot coerce intrinsics to function pointers
2019-09-20T17:53:49.5881255Z   --> /checkout/src/test/ui/reify-intrinsic.rs:6:64
2019-09-20T17:53:49.5881327Z    |
2019-09-20T17:53:49.5881588Z LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
2019-09-20T17:53:49.5881716Z    |                                                                |
2019-09-20T17:53:49.5881773Z    |                                                                cannot coerce intrinsics to function pointers
2019-09-20T17:53:49.5881839Z    |                                                                help: use parentheses to call this function: `std::mem::transmute(...)`
2019-09-20T17:53:49.5881907Z    |
2019-09-20T17:53:49.5881907Z    |
2019-09-20T17:53:49.5882155Z    = note: expected type `unsafe extern "rust-intrinsic" fn(isize) -> usize`
2019-09-20T17:53:49.5886092Z               found type `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}`
2019-09-20T17:53:49.5886170Z 
2019-09-20T17:53:49.5886502Z error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {std::mem::transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
2019-09-20T17:53:49.5886781Z    |
2019-09-20T17:53:49.5886781Z    |
2019-09-20T17:53:49.5887008Z LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
2019-09-20T17:53:49.5887099Z 
2019-09-20T17:53:49.5887135Z error: aborting due to 2 previous errors
2019-09-20T17:53:49.5887160Z 
2019-09-20T17:53:49.5887199Z Some errors have detailed explanations: E0308, E0606.
---
2019-09-20T17:53:49.5888652Z 
2019-09-20T17:53:49.5889576Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-20T17:53:49.5902706Z 
2019-09-20T17:53:49.5902873Z 
2019-09-20T17:53:49.5904646Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-20T17:53:49.5904898Z 
2019-09-20T17:53:49.5904926Z 
2019-09-20T17:53:49.5908823Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-20T17:53:49.5908908Z Build completed unsuccessfully in 1:04:41
2019-09-20T17:53:49.5908908Z Build completed unsuccessfully in 1:04:41
2019-09-20T17:53:49.5962565Z == clock drift check ==
2019-09-20T17:53:49.5977916Z   local time: Fri Sep 20 17:53:49 UTC 2019
2019-09-20T17:53:49.7565995Z   network time: Fri, 20 Sep 2019 17:53:49 GMT
2019-09-20T17:53:49.7571388Z == end clock drift check ==
2019-09-20T17:53:50.5269753Z ##[error]Bash exited with code '1'.
2019-09-20T17:53:50.5308346Z ##[section]Starting: Checkout
2019-09-20T17:53:50.5310107Z ==============================================================================
2019-09-20T17:53:50.5310157Z Task         : Get sources
2019-09-20T17:53:50.5310198Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
