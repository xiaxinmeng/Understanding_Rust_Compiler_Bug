plain
2020-02-21T18:23:24.8941378Z ========================== Starting Command Output ===========================
2020-02-21T18:23:24.8944055Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2eae9e15-dac6-4431-9079-7cb50741ffcc.sh
2020-02-21T18:23:24.8944316Z 
2020-02-21T18:23:24.8948362Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T18:23:24.8969552Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T18:23:24.8973027Z Task         : Get sources
2020-02-21T18:23:24.8973390Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T18:23:24.8973679Z Version      : 1.0.0
2020-02-21T18:23:24.8973876Z Author       : Microsoft
---
2020-02-21T18:23:25.9241978Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T18:23:25.9247554Z ##[command]git config gc.auto 0
2020-02-21T18:23:25.9251050Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T18:23:25.9254324Z ##[command]git config --get-all http.proxy
2020-02-21T18:23:25.9260583Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-21T19:27:29.3455917Z .................................................................................................... 1700/9697
2020-02-21T19:27:34.0893233Z .................................................................................................... 1800/9697
2020-02-21T19:27:45.7871689Z ..........................................i......................................................... 1900/9697
2020-02-21T19:27:53.8850086Z .................................................................................................... 2000/9697
2020-02-21T19:28:07.9005756Z ................................iiiii............................................................... 2100/9697
2020-02-21T19:28:17.9182491Z .................................................................................................... 2300/9697
2020-02-21T19:28:20.4921872Z .................................................................................................... 2400/9697
2020-02-21T19:28:25.1659368Z .................................................................................................... 2500/9697
2020-02-21T19:28:46.4967853Z .................................................................................................... 2600/9697
---
2020-02-21T19:31:40.3352908Z .......i............................................................................................ 5000/9697
2020-02-21T19:31:49.7887947Z .................................................................................................... 5100/9697
2020-02-21T19:31:54.5761412Z ..................................i................................................................. 5200/9697
2020-02-21T19:32:04.8323718Z .................................................................................................... 5300/9697
2020-02-21T19:32:10.8182771Z ..........ii.ii........i...i........................................................................ 5400/9697
2020-02-21T19:32:19.7024447Z .................................................................................................... 5600/9697
2020-02-21T19:32:30.8937716Z ...................................................................F................................ 5700/9697
2020-02-21T19:32:39.0049214Z .i.................................................................................................. 5800/9697
2020-02-21T19:32:44.8829850Z .................................................................................................... 5900/9697
2020-02-21T19:32:44.8829850Z .................................................................................................... 5900/9697
2020-02-21T19:32:55.4063676Z ............................................................................................ii...i.. 6000/9697
2020-02-21T19:33:08.0406592Z ii...........i...................................................................................... 6100/9697
2020-02-21T19:33:26.1569394Z .................................................................................................... 6300/9697
2020-02-21T19:33:32.9309390Z .................................................................................................... 6400/9697
2020-02-21T19:33:32.9309390Z .................................................................................................... 6400/9697
2020-02-21T19:33:47.6384561Z .......................i..ii........................................................................ 6500/9697
2020-02-21T19:34:08.6894159Z .................................................................................................... 6700/9697
2020-02-21T19:34:11.0542735Z ...............i.................................................................................... 6800/9697
2020-02-21T19:34:13.3711894Z .................................................................................................... 6900/9697
2020-02-21T19:34:15.9393643Z .....................................i.............................................................. 7000/9697
---
2020-02-21T19:35:55.8561849Z .................................................................................................... 7600/9697
2020-02-21T19:36:01.6404308Z .................................................................................................... 7700/9697
2020-02-21T19:36:06.9186899Z .................................................................................................... 7800/9697
2020-02-21T19:36:13.6080619Z .................................................................................i.................. 7900/9697
2020-02-21T19:36:22.7964398Z ..........................................................F.F....................................... 8000/9697
2020-02-21T19:36:30.3231437Z .................................iiiiiii.i.......................................................... 8100/9697
2020-02-21T19:36:45.5791215Z .................................................................................................... 8300/9697
2020-02-21T19:36:54.2092505Z .................................................................................................... 8400/9697
2020-02-21T19:37:08.2760137Z .................................................................................................... 8500/9697
2020-02-21T19:37:15.4235920Z .................................................................................................... 8600/9697
---
2020-02-21T19:39:17.0191641Z 
2020-02-21T19:39:17.0207818Z ---- [ui] ui/macros/issue-68060.rs stdout ----
2020-02-21T19:39:17.0208166Z diff of stderr:
2020-02-21T19:39:17.0211105Z 
2020-02-21T19:39:17.0212685Z - error: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T19:39:17.0213531Z + error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T19:39:17.0214503Z 3    |
2020-02-21T19:39:17.0214503Z 3    |
2020-02-21T19:39:17.0214909Z 4 LL |             #[target_feature(enable = "")]
2020-02-21T19:39:17.0215537Z -    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only be applied to `unsafe` functions
2020-02-21T19:39:17.0215892Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-21T19:39:17.0216085Z 6 ...
2020-02-21T19:39:17.0216244Z 7 LL |             |_| (),
2020-02-21T19:39:17.0216244Z 7 LL |             |_| (),
2020-02-21T19:39:17.0216831Z 8    |             ------ not an `unsafe` function
2020-02-21T19:39:17.0217005Z 
2020-02-21T19:39:17.0217120Z +    |
2020-02-21T19:39:17.0217744Z +    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T19:39:17.0218113Z +    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-21T19:39:17.0218889Z 10 error: the feature named `` is not valid for this target
2020-02-21T19:39:17.0219531Z 11   --> $DIR/issue-68060.rs:8:30
2020-02-21T19:39:17.0219700Z 
2020-02-21T19:39:17.0219837Z 21 
---
2020-02-21T19:39:17.0222695Z 
2020-02-21T19:39:17.0222791Z 
2020-02-21T19:39:17.0222996Z The actual stderr differed from the expected stderr.
2020-02-21T19:39:17.0224332Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/issue-68060.stderr
2020-02-21T19:39:17.0225289Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T19:39:17.0226066Z To only update this specific test, also pass `--test-args macros/issue-68060.rs`
2020-02-21T19:39:17.0226831Z error: 1 errors occurred comparing output.
2020-02-21T19:39:17.0227812Z status: exit code: 1
2020-02-21T19:39:17.0227812Z status: exit code: 1
2020-02-21T19:39:17.0230014Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/auxiliary"
2020-02-21T19:39:17.0232084Z ------------------------------------------
2020-02-21T19:39:17.0232658Z 
2020-02-21T19:39:17.0233004Z ------------------------------------------
2020-02-21T19:39:17.0233210Z stderr:
2020-02-21T19:39:17.0233210Z stderr:
2020-02-21T19:39:17.0233587Z ------------------------------------------
2020-02-21T19:39:17.0233919Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T19:39:17.0235413Z    |
2020-02-21T19:39:17.0235413Z    |
2020-02-21T19:39:17.0235802Z LL |             #[target_feature(enable = "")]
2020-02-21T19:39:17.0236307Z ...
2020-02-21T19:39:17.0236653Z LL |             |_| (),
2020-02-21T19:39:17.0237249Z    |             ------ not an `unsafe` function
2020-02-21T19:39:17.0237861Z    |
2020-02-21T19:39:17.0237861Z    |
2020-02-21T19:39:17.0238591Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T19:39:17.0238977Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-21T19:39:17.0239778Z error: the feature named `` is not valid for this target
2020-02-21T19:39:17.0240753Z   --> /checkout/src/test/ui/macros/issue-68060.rs:8:30
2020-02-21T19:39:17.0241199Z    |
2020-02-21T19:39:17.0241199Z    |
2020-02-21T19:39:17.0241412Z LL |             #[target_feature(enable = "")]
2020-02-21T19:39:17.0241749Z    |                              ^^^^^^^^^^^ `` is not valid for this target
2020-02-21T19:39:17.0242565Z error[E0737]: `#[track_caller]` requires Rust ABI
2020-02-21T19:39:17.0243265Z   --> /checkout/src/test/ui/macros/issue-68060.rs:11:13
2020-02-21T19:39:17.0243493Z    |
2020-02-21T19:39:17.0243689Z LL |             #[track_caller]
---
2020-02-21T19:39:17.0245973Z 
2020-02-21T19:39:17.0246501Z ------------------------------------------
2020-02-21T19:39:17.0246679Z 
2020-02-21T19:39:17.0246779Z 
2020-02-21T19:39:17.0247794Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs stdout ----
2020-02-21T19:39:17.0248217Z 
2020-02-21T19:39:17.0248338Z 3    |
2020-02-21T19:39:17.0248338Z 3    |
2020-02-21T19:39:17.0248553Z 4 LL | #[target_feature(enable = "sse2")]
2020-02-21T19:39:17.0249167Z - LL | fn foo() {} 
2020-02-21T19:39:17.0249366Z + LL | fn foo() {}
2020-02-21T19:39:17.0249758Z 7    | ----------- not an `unsafe` function
2020-02-21T19:39:17.0249959Z 8    |
2020-02-21T19:39:17.0249959Z 8    |
2020-02-21T19:39:17.0250521Z 9    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T19:39:17.0250839Z 
2020-02-21T19:39:17.0251043Z The actual stderr differed from the expected stderr.
2020-02-21T19:39:17.0251043Z The actual stderr differed from the expected stderr.
2020-02-21T19:39:17.0251867Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11/feature-gate-target_feature_11.stderr
2020-02-21T19:39:17.0252576Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T19:39:17.0253264Z To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs`
2020-02-21T19:39:17.0253763Z error: 1 errors occurred comparing output.
2020-02-21T19:39:17.0253995Z status: exit code: 1
2020-02-21T19:39:17.0253995Z status: exit code: 1
2020-02-21T19:39:17.0256194Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11/auxiliary"
2020-02-21T19:39:17.0257936Z ------------------------------------------
2020-02-21T19:39:17.0258108Z 
2020-02-21T19:39:17.0258471Z ------------------------------------------
2020-02-21T19:39:17.0258671Z stderr:
2020-02-21T19:39:17.0258671Z stderr:
2020-02-21T19:39:17.0259025Z ------------------------------------------
2020-02-21T19:39:17.0259374Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T19:39:17.0260036Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs:1:1
2020-02-21T19:39:17.0260343Z    |
2020-02-21T19:39:17.0260642Z LL | #[target_feature(enable = "sse2")] //~ ERROR can only be applied to `unsafe` functions
2020-02-21T19:39:17.0261293Z LL | fn foo() {}
2020-02-21T19:39:17.0261736Z    | ----------- not an `unsafe` function
2020-02-21T19:39:17.0261931Z    |
2020-02-21T19:39:17.0261931Z    |
2020-02-21T19:39:17.0262469Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T19:39:17.0262874Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-21T19:39:17.0263293Z error: aborting due to previous error
2020-02-21T19:39:17.0263456Z 
2020-02-21T19:39:17.0263895Z For more information about this error, try `rustc --explain E0658`.
2020-02-21T19:39:17.0264197Z 
2020-02-21T19:39:17.0264197Z 
2020-02-21T19:39:17.0264580Z ------------------------------------------
2020-02-21T19:39:17.0264749Z 
2020-02-21T19:39:17.0264863Z 
2020-02-21T19:39:17.0265288Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs stdout ----
2020-02-21T19:39:17.0265674Z 
2020-02-21T19:39:17.0265991Z + error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T19:39:17.0266509Z +   --> $DIR/safe-calls.rs:69:18
2020-02-21T19:39:17.0266702Z +    |
2020-02-21T19:39:17.0266702Z +    |
2020-02-21T19:39:17.0266906Z + LL | const name: () = sse2();
2020-02-21T19:39:17.0269920Z + 
2020-02-21T19:39:17.0270244Z + error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T19:39:17.0270889Z +   --> $DIR/safe-calls.rs:70:33
2020-02-21T19:39:17.0271082Z +    |
2020-02-21T19:39:17.0271082Z +    |
2020-02-21T19:39:17.0271322Z + LL | const other_name: () = unsafe { sse2() };
2020-02-21T19:39:17.0271984Z + 
2020-02-21T19:39:17.0271984Z + 
2020-02-21T19:39:17.0272296Z 1 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0272985Z 3    |
2020-02-21T19:39:17.0273089Z 
2020-02-21T19:39:17.0273225Z 78    |
2020-02-21T19:39:17.0273491Z 79    = note: can only be called if the required target features are available
---
2020-02-21T19:39:17.0276505Z 84 
2020-02-21T19:39:17.0276611Z 
2020-02-21T19:39:17.0276710Z 
2020-02-21T19:39:17.0276920Z The actual stderr differed from the expected stderr.
2020-02-21T19:39:17.0277807Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/safe-calls.stderr
2020-02-21T19:39:17.0278450Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T19:39:17.0279218Z To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/safe-calls.rs`
2020-02-21T19:39:17.0279690Z error: 1 errors occurred comparing output.
2020-02-21T19:39:17.0279915Z status: exit code: 1
2020-02-21T19:39:17.0279915Z status: exit code: 1
2020-02-21T19:39:17.0282211Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/auxiliary"
2020-02-21T19:39:17.0284182Z ------------------------------------------
2020-02-21T19:39:17.0284368Z 
2020-02-21T19:39:17.0284764Z ------------------------------------------
2020-02-21T19:39:17.0285797Z stderr:
2020-02-21T19:39:17.0285797Z stderr:
2020-02-21T19:39:17.0286453Z ------------------------------------------
2020-02-21T19:39:17.0286829Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T19:39:17.0287890Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:69:18
2020-02-21T19:39:17.0288162Z    |
2020-02-21T19:39:17.0288451Z LL | const name: () = sse2(); //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0289078Z 
2020-02-21T19:39:17.0289368Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T19:39:17.0289368Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T19:39:17.0290071Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:70:33
2020-02-21T19:39:17.0290339Z    |
2020-02-21T19:39:17.0290559Z LL | const other_name: () = unsafe { sse2() };
2020-02-21T19:39:17.0291027Z 
2020-02-21T19:39:17.0291027Z 
2020-02-21T19:39:17.0291324Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0291964Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:28:5
2020-02-21T19:39:17.0292248Z    |
2020-02-21T19:39:17.0292529Z LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0292902Z    |     ^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0347859Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0348351Z 
2020-02-21T19:39:17.0348351Z 
2020-02-21T19:39:17.0348867Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0349773Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:29:5
2020-02-21T19:39:17.0350223Z    |
2020-02-21T19:39:17.0350697Z LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0351076Z    |     ^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0351571Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0351801Z 
2020-02-21T19:39:17.0351801Z 
2020-02-21T19:39:17.0352100Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0352762Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:31:5
2020-02-21T19:39:17.0353047Z    |
2020-02-21T19:39:17.0353331Z LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0353715Z    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0354222Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0354451Z 
2020-02-21T19:39:17.0354451Z 
2020-02-21T19:39:17.0354802Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0355445Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:37:5
2020-02-21T19:39:17.0355709Z    |
2020-02-21T19:39:17.0356004Z LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0356381Z    |     ^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0356882Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0357111Z 
2020-02-21T19:39:17.0357111Z 
2020-02-21T19:39:17.0357408Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0358067Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:39:5
2020-02-21T19:39:17.0358333Z    |
2020-02-21T19:39:17.0358768Z LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0359185Z    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0359672Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0359902Z 
2020-02-21T19:39:17.0359902Z 
2020-02-21T19:39:17.0360217Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0360913Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:44:5
2020-02-21T19:39:17.0361281Z    |
2020-02-21T19:39:17.0361579Z LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0361947Z    |     ^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0362440Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0362675Z 
2020-02-21T19:39:17.0362675Z 
2020-02-21T19:39:17.0362972Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0363672Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:45:5
2020-02-21T19:39:17.0363939Z    |
2020-02-21T19:39:17.0364221Z LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0364612Z    |     ^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0365091Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0365325Z 
2020-02-21T19:39:17.0365325Z 
2020-02-21T19:39:17.0365640Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0366280Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:47:5
2020-02-21T19:39:17.0366543Z    |
2020-02-21T19:39:17.0366843Z LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0367225Z    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0367728Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0367956Z 
2020-02-21T19:39:17.0367956Z 
2020-02-21T19:39:17.0368249Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0368907Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:53:5
2020-02-21T19:39:17.0369172Z    |
2020-02-21T19:39:17.0369457Z LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0369840Z    |     ^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0370310Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0370539Z 
2020-02-21T19:39:17.0370539Z 
2020-02-21T19:39:17.0370853Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T19:39:17.0371495Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:69:18
2020-02-21T19:39:17.0371761Z    |
2020-02-21T19:39:17.0372290Z LL | const name: () = sse2(); //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T19:39:17.0372691Z    |                  ^^^^^^ call to function with `#[target_feature]`
2020-02-21T19:39:17.0373198Z    = note: can only be called if the required target features are available
2020-02-21T19:39:17.0373432Z 
2020-02-21T19:39:17.0373618Z error: aborting due to 12 previous errors
2020-02-21T19:39:17.0373787Z 
---
2020-02-21T19:39:17.0375332Z 
2020-02-21T19:39:17.0375529Z 
2020-02-21T19:39:17.0375972Z ---- [ui] ui/target-feature/invalid-attribute.rs stdout ----
2020-02-21T19:39:17.0376190Z 
2020-02-21T19:39:17.0377166Z error: /checkout/src/test/ui/target-feature/invalid-attribute.rs:27: unexpected note: '27:1: 27:35: see issue #69098 <***/issues/69098> for more information'
2020-02-21T19:39:17.0378166Z error: /checkout/src/test/ui/target-feature/invalid-attribute.rs:27: expected note not found: can only be applied to `unsafe` functions
2020-02-21T19:39:17.0378504Z 
2020-02-21T19:39:17.0378721Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-21T19:39:17.0379090Z status: exit code: 1
2020-02-21T19:39:17.0379090Z status: exit code: 1
2020-02-21T19:39:17.0381016Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/invalid-attribute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute/auxiliary"
2020-02-21T19:39:17.0382496Z     Error {
2020-02-21T19:39:17.0382688Z         line_num: 27,
2020-02-21T19:39:17.0382881Z         kind: Some(
2020-02-21T19:39:17.0383060Z             Note,
2020-02-21T19:39:17.0383060Z             Note,
2020-02-21T19:39:17.0383218Z         ),
2020-02-21T19:39:17.0383834Z         msg: "27:1: 27:35: see issue #69098 <***/issues/69098> for more information",
2020-02-21T19:39:17.0384251Z ]
2020-02-21T19:39:17.0384348Z 
2020-02-21T19:39:17.0384534Z not found errors (from test file): [
2020-02-21T19:39:17.0384740Z     Error {
---
2020-02-21T19:39:17.0394525Z 
2020-02-21T19:39:17.0394619Z 
2020-02-21T19:39:17.0394744Z failures:
2020-02-21T19:39:17.0395212Z     [ui] ui/macros/issue-68060.rs
2020-02-21T19:39:17.0395725Z     [ui] ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs
2020-02-21T19:39:17.0396227Z     [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs
2020-02-21T19:39:17.0396926Z 
2020-02-21T19:39:17.0399817Z test result: FAILED. 9639 passed; 4 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-21T19:39:17.0400161Z 
2020-02-21T19:39:17.0400842Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-21T19:39:17.0400842Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-21T19:39:17.0401153Z 
2020-02-21T19:39:17.0401266Z 
2020-02-21T19:39:17.0405626Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-21T19:39:17.0408680Z 
2020-02-21T19:39:17.0408777Z 
2020-02-21T19:39:17.0409021Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-21T19:39:17.0409332Z Build completed unsuccessfully in 1:08:47
2020-02-21T19:39:17.0409332Z Build completed unsuccessfully in 1:08:47
2020-02-21T19:39:17.0409617Z == clock drift check ==
2020-02-21T19:39:17.0420338Z   local time: Fri Feb 21 19:39:17 UTC 2020
2020-02-21T19:39:17.3399330Z   network time: Fri, 21 Feb 2020 19:39:17 GMT
2020-02-21T19:39:17.3403000Z == end clock drift check ==
2020-02-21T19:39:18.0346518Z 
2020-02-21T19:39:18.0416740Z ##[error]Bash exited with code '1'.
2020-02-21T19:39:18.0438584Z ##[section]Finishing: Run build
2020-02-21T19:39:18.0491448Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T19:39:18.0497208Z Task         : Get sources
2020-02-21T19:39:18.0497569Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T19:39:18.0497873Z Version      : 1.0.0
2020-02-21T19:39:18.0498086Z Author       : Microsoft
2020-02-21T19:39:18.0498086Z Author       : Microsoft
2020-02-21T19:39:18.0498440Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T19:39:18.0498829Z ==============================================================================
2020-02-21T19:39:18.4025907Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T19:39:18.4072275Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T19:39:18.4163919Z Cleaning up task key
2020-02-21T19:39:18.4165081Z Start cleaning up orphan processes.
2020-02-21T19:39:18.4429340Z Terminate orphan process: pid (4202) (python)
2020-02-21T19:39:18.4712837Z ##[section]Finishing: Finalize Job
