plain
2020-02-21T16:50:11.4487867Z ========================== Starting Command Output ===========================
2020-02-21T16:50:11.4491310Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad583e29-c13b-45de-a223-fe7c6c120d04.sh
2020-02-21T16:50:11.4491765Z 
2020-02-21T16:50:11.4495707Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T16:50:11.4515496Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T16:50:11.4519316Z Task         : Get sources
2020-02-21T16:50:11.4519639Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T16:50:11.4519970Z Version      : 1.0.0
2020-02-21T16:50:11.4520179Z Author       : Microsoft
---
2020-02-21T16:50:12.4401096Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T16:50:12.4410191Z ##[command]git config gc.auto 0
2020-02-21T16:50:12.4416381Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T16:50:12.4422164Z ##[command]git config --get-all http.proxy
2020-02-21T16:50:12.4432223Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-21T17:51:49.6132290Z .................................................................................................... 1700/9697
2020-02-21T17:51:54.4221359Z .................................................................................................... 1800/9697
2020-02-21T17:52:06.1856131Z ..........................................i......................................................... 1900/9697
2020-02-21T17:52:14.3553994Z .................................................................................................... 2000/9697
2020-02-21T17:52:28.3449918Z ................................iiiii............................................................... 2100/9697
2020-02-21T17:52:38.5105468Z .................................................................................................... 2300/9697
2020-02-21T17:52:41.0508688Z .................................................................................................... 2400/9697
2020-02-21T17:52:45.5736250Z .................................................................................................... 2500/9697
2020-02-21T17:53:06.3499074Z .................................................................................................... 2600/9697
---
2020-02-21T17:55:49.2562841Z .......i............................................................................................ 5000/9697
2020-02-21T17:55:58.2789897Z .................................................................................................... 5100/9697
2020-02-21T17:56:03.0254106Z ..................................i................................................................. 5200/9697
2020-02-21T17:56:13.0430077Z .................................................................................................... 5300/9697
2020-02-21T17:56:18.8596599Z ..........ii.ii........i...i........................................................................ 5400/9697
2020-02-21T17:56:27.5683450Z .................................................................................................... 5600/9697
2020-02-21T17:56:38.4515794Z ...................................................................F................................ 5700/9697
2020-02-21T17:56:46.0473693Z .i.................................................................................................. 5800/9697
2020-02-21T17:56:51.4577391Z .................................................................................................... 5900/9697
2020-02-21T17:56:51.4577391Z .................................................................................................... 5900/9697
2020-02-21T17:57:01.4267295Z ............................................................................................ii...i.. 6000/9697
2020-02-21T17:57:13.6276219Z ii...........i...................................................................................... 6100/9697
2020-02-21T17:57:30.7580964Z .................................................................................................... 6300/9697
2020-02-21T17:57:35.7613310Z .................................................................................................... 6400/9697
2020-02-21T17:57:35.7613310Z .................................................................................................... 6400/9697
2020-02-21T17:57:48.7675024Z .......................i..ii........................................................................ 6500/9697
2020-02-21T17:58:09.1779175Z .................................................................................................... 6700/9697
2020-02-21T17:58:11.4982038Z ...............i.................................................................................... 6800/9697
2020-02-21T17:58:14.1160895Z .................................................................................................... 6900/9697
2020-02-21T17:58:16.2477576Z .....................................i.............................................................. 7000/9697
---
2020-02-21T17:59:51.1101390Z .................................................................................................... 7600/9697
2020-02-21T17:59:56.9083434Z .................................................................................................... 7700/9697
2020-02-21T18:00:02.2818491Z .................................................................................................... 7800/9697
2020-02-21T18:00:08.8123042Z .................................................................................i.................. 7900/9697
2020-02-21T18:00:18.0537404Z ...........................................................F.F...................................... 8000/9697
2020-02-21T18:00:25.4129105Z .................................iiiiiii.i.......................................................... 8100/9697
2020-02-21T18:00:40.1225928Z .................................................................................................... 8300/9697
2020-02-21T18:00:48.6262032Z .................................................................................................... 8400/9697
2020-02-21T18:01:02.3098260Z .................................................................................................... 8500/9697
2020-02-21T18:01:09.3861190Z .................................................................................................... 8600/9697
---
2020-02-21T18:03:04.2324738Z 
2020-02-21T18:03:04.2325525Z ---- [ui] ui/macros/issue-68060.rs stdout ----
2020-02-21T18:03:04.2325794Z diff of stderr:
2020-02-21T18:03:04.2325944Z 
2020-02-21T18:03:04.2326451Z - error: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T18:03:04.2327084Z + error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T18:03:04.2327854Z 3    |
2020-02-21T18:03:04.2327854Z 3    |
2020-02-21T18:03:04.2328102Z 4 LL |             #[target_feature(enable = "")]
2020-02-21T18:03:04.2328815Z -    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only be applied to `unsafe` functions
2020-02-21T18:03:04.2329196Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-21T18:03:04.2329433Z 6 ...
2020-02-21T18:03:04.2329617Z 7 LL |             |_| (),
2020-02-21T18:03:04.2329617Z 7 LL |             |_| (),
2020-02-21T18:03:04.2330076Z 8    |             ------ not an `unsafe` function
2020-02-21T18:03:04.2330290Z 
2020-02-21T18:03:04.2330425Z +    |
2020-02-21T18:03:04.2331111Z +    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T18:03:04.2331555Z +    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-21T18:03:04.2332214Z 10 error: the feature named `` is not valid for this target
2020-02-21T18:03:04.2332739Z 11   --> $DIR/issue-68060.rs:8:30
2020-02-21T18:03:04.2332919Z 
2020-02-21T18:03:04.2333046Z 21 
---
2020-02-21T18:03:04.2335276Z 
2020-02-21T18:03:04.2335379Z 
2020-02-21T18:03:04.2335612Z The actual stderr differed from the expected stderr.
2020-02-21T18:03:04.2336287Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/issue-68060.stderr
2020-02-21T18:03:04.2336938Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T18:03:04.2337572Z To only update this specific test, also pass `--test-args macros/issue-68060.rs`
2020-02-21T18:03:04.2338031Z error: 1 errors occurred comparing output.
2020-02-21T18:03:04.2338301Z status: exit code: 1
2020-02-21T18:03:04.2338301Z status: exit code: 1
2020-02-21T18:03:04.2340251Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-68060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-68060/auxiliary"
2020-02-21T18:03:04.2341870Z ------------------------------------------
2020-02-21T18:03:04.2342054Z 
2020-02-21T18:03:04.2342452Z ------------------------------------------
2020-02-21T18:03:04.2342669Z stderr:
2020-02-21T18:03:04.2342669Z stderr:
2020-02-21T18:03:04.2343053Z ------------------------------------------
2020-02-21T18:03:04.2343428Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T18:03:04.2344251Z    |
2020-02-21T18:03:04.2344251Z    |
2020-02-21T18:03:04.2344491Z LL |             #[target_feature(enable = "")]
2020-02-21T18:03:04.2344994Z ...
2020-02-21T18:03:04.2345184Z LL |             |_| (),
2020-02-21T18:03:04.2345628Z    |             ------ not an `unsafe` function
2020-02-21T18:03:04.2345849Z    |
2020-02-21T18:03:04.2345849Z    |
2020-02-21T18:03:04.2346448Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T18:03:04.2346863Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-21T18:03:04.2347441Z error: the feature named `` is not valid for this target
2020-02-21T18:03:04.2347995Z   --> /checkout/src/test/ui/macros/issue-68060.rs:8:30
2020-02-21T18:03:04.2348236Z    |
2020-02-21T18:03:04.2348236Z    |
2020-02-21T18:03:04.2348453Z LL |             #[target_feature(enable = "")]
2020-02-21T18:03:04.2348819Z    |                              ^^^^^^^^^^^ `` is not valid for this target
2020-02-21T18:03:04.2349281Z error[E0737]: `#[track_caller]` requires Rust ABI
2020-02-21T18:03:04.2349822Z   --> /checkout/src/test/ui/macros/issue-68060.rs:11:13
2020-02-21T18:03:04.2350067Z    |
2020-02-21T18:03:04.2350257Z LL |             #[track_caller]
---
2020-02-21T18:03:04.2352211Z 
2020-02-21T18:03:04.2352590Z ------------------------------------------
2020-02-21T18:03:04.2352791Z 
2020-02-21T18:03:04.2352893Z 
2020-02-21T18:03:04.2353405Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs stdout ----
2020-02-21T18:03:04.2353870Z 
2020-02-21T18:03:04.2354002Z 3    |
2020-02-21T18:03:04.2354002Z 3    |
2020-02-21T18:03:04.2354213Z 4 LL | #[target_feature(enable = "sse2")]
2020-02-21T18:03:04.2354885Z - LL | fn foo() {} 
2020-02-21T18:03:04.2355083Z + LL | fn foo() {}
2020-02-21T18:03:04.2355512Z 7    | ----------- not an `unsafe` function
2020-02-21T18:03:04.2355728Z 8    |
2020-02-21T18:03:04.2355728Z 8    |
2020-02-21T18:03:04.2360542Z 9    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T18:03:04.2364226Z 
2020-02-21T18:03:04.2364456Z The actual stderr differed from the expected stderr.
2020-02-21T18:03:04.2364456Z The actual stderr differed from the expected stderr.
2020-02-21T18:03:04.2365555Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11/feature-gate-target_feature_11.stderr
2020-02-21T18:03:04.2366354Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T18:03:04.2367092Z To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs`
2020-02-21T18:03:04.2367649Z error: 1 errors occurred comparing output.
2020-02-21T18:03:04.2367898Z status: exit code: 1
2020-02-21T18:03:04.2367898Z status: exit code: 1
2020-02-21T18:03:04.2370289Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11/auxiliary"
2020-02-21T18:03:04.2372177Z ------------------------------------------
2020-02-21T18:03:04.2372364Z 
2020-02-21T18:03:04.2372744Z ------------------------------------------
2020-02-21T18:03:04.2372980Z stderr:
2020-02-21T18:03:04.2372980Z stderr:
2020-02-21T18:03:04.2373369Z ------------------------------------------
2020-02-21T18:03:04.2373728Z error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
2020-02-21T18:03:04.2374460Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs:1:1
2020-02-21T18:03:04.2374972Z    |
2020-02-21T18:03:04.2375281Z LL | #[target_feature(enable = "sse2")] //~ ERROR can only be applied to `unsafe` functions
2020-02-21T18:03:04.2376988Z LL | fn foo() {}
2020-02-21T18:03:04.2377470Z    | ----------- not an `unsafe` function
2020-02-21T18:03:04.2377680Z    |
2020-02-21T18:03:04.2377680Z    |
2020-02-21T18:03:04.2378311Z    = note: see issue #69098 <***/issues/69098> for more information
2020-02-21T18:03:04.2378723Z    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable
2020-02-21T18:03:04.2379193Z error: aborting due to previous error
2020-02-21T18:03:04.2379369Z 
2020-02-21T18:03:04.2379829Z For more information about this error, try `rustc --explain E0658`.
2020-02-21T18:03:04.2380079Z 
2020-02-21T18:03:04.2380079Z 
2020-02-21T18:03:04.2380456Z ------------------------------------------
2020-02-21T18:03:04.2380639Z 
2020-02-21T18:03:04.2380743Z 
2020-02-21T18:03:04.2381216Z ---- [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs stdout ----
2020-02-21T18:03:04.2381732Z 
2020-02-21T18:03:04.2382053Z + error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T18:03:04.2382645Z +   --> $DIR/safe-calls.rs:69:18
2020-02-21T18:03:04.2382856Z +    |
2020-02-21T18:03:04.2382856Z +    |
2020-02-21T18:03:04.2383059Z + LL | const name: () = sse2();
2020-02-21T18:03:04.2383503Z + 
2020-02-21T18:03:04.2383825Z + error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T18:03:04.2384394Z +   --> $DIR/safe-calls.rs:70:33
2020-02-21T18:03:04.2384598Z +    |
2020-02-21T18:03:04.2384598Z +    |
2020-02-21T18:03:04.2384837Z + LL | const other_name: () = unsafe { sse2() };
2020-02-21T18:03:04.2385370Z + 
2020-02-21T18:03:04.2385370Z + 
2020-02-21T18:03:04.2385698Z 1 error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2386485Z 3    |
2020-02-21T18:03:04.2386601Z 
2020-02-21T18:03:04.2386735Z 78    |
2020-02-21T18:03:04.2387043Z 79    = note: can only be called if the required target features are available
---
2020-02-21T18:03:04.2391002Z 84 
2020-02-21T18:03:04.2391111Z 
2020-02-21T18:03:04.2391232Z 
2020-02-21T18:03:04.2391449Z The actual stderr differed from the expected stderr.
2020-02-21T18:03:04.2392190Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/safe-calls.stderr
2020-02-21T18:03:04.2392908Z To update references, rerun the tests and pass the `--bless` flag
2020-02-21T18:03:04.2393590Z To only update this specific test, also pass `--test-args rfcs/rfc-2396-target_feature-11/safe-calls.rs`
2020-02-21T18:03:04.2394105Z error: 1 errors occurred comparing output.
2020-02-21T18:03:04.2394356Z status: exit code: 1
2020-02-21T18:03:04.2394356Z status: exit code: 1
2020-02-21T18:03:04.2396851Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls/auxiliary"
2020-02-21T18:03:04.2398749Z ------------------------------------------
2020-02-21T18:03:04.2398960Z 
2020-02-21T18:03:04.2399352Z ------------------------------------------
2020-02-21T18:03:04.2399569Z stderr:
2020-02-21T18:03:04.2399569Z stderr:
2020-02-21T18:03:04.2399975Z ------------------------------------------
2020-02-21T18:03:04.2400369Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T18:03:04.2401068Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:69:18
2020-02-21T18:03:04.2401375Z    |
2020-02-21T18:03:04.2401688Z LL | const name: () = sse2(); //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2402209Z 
2020-02-21T18:03:04.2402520Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T18:03:04.2402520Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-21T18:03:04.2403293Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:70:33
2020-02-21T18:03:04.2403604Z    |
2020-02-21T18:03:04.2403833Z LL | const other_name: () = unsafe { sse2() };
2020-02-21T18:03:04.2404313Z 
2020-02-21T18:03:04.2404313Z 
2020-02-21T18:03:04.2404651Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2405340Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:28:5
2020-02-21T18:03:04.2405625Z    |
2020-02-21T18:03:04.2405942Z LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2406336Z    |     ^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2406861Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2407106Z 
2020-02-21T18:03:04.2407106Z 
2020-02-21T18:03:04.2407423Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2408138Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:29:5
2020-02-21T18:03:04.2408425Z    |
2020-02-21T18:03:04.2408726Z LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2409141Z    |     ^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2409655Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2409899Z 
2020-02-21T18:03:04.2409899Z 
2020-02-21T18:03:04.2410233Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2410920Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:31:5
2020-02-21T18:03:04.2411206Z    |
2020-02-21T18:03:04.2411523Z LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2411935Z    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2412473Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2412719Z 
2020-02-21T18:03:04.2412719Z 
2020-02-21T18:03:04.2413035Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2413784Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:37:5
2020-02-21T18:03:04.2414070Z    |
2020-02-21T18:03:04.2414367Z LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2414783Z    |     ^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2415293Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2415537Z 
2020-02-21T18:03:04.2415537Z 
2020-02-21T18:03:04.2416374Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2417619Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:39:5
2020-02-21T18:03:04.2418036Z    |
2020-02-21T18:03:04.2418364Z LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2418773Z    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2419311Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2419556Z 
2020-02-21T18:03:04.2419556Z 
2020-02-21T18:03:04.2419872Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2420591Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:44:5
2020-02-21T18:03:04.2420877Z    |
2020-02-21T18:03:04.2421176Z LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2421585Z    |     ^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2422171Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2422431Z 
2020-02-21T18:03:04.2422431Z 
2020-02-21T18:03:04.2422747Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2423445Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:45:5
2020-02-21T18:03:04.2423746Z    |
2020-02-21T18:03:04.2424044Z LL |     avx_bmi2();          //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2424445Z    |     ^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2424969Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2425214Z 
2020-02-21T18:03:04.2425214Z 
2020-02-21T18:03:04.2425529Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2426233Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:47:5
2020-02-21T18:03:04.2426523Z    |
2020-02-21T18:03:04.2426825Z LL |     Quux.avx_bmi2();     //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2427250Z    |     ^^^^^^^^^^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2427767Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2428028Z 
2020-02-21T18:03:04.2428028Z 
2020-02-21T18:03:04.2428344Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2429027Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:53:5
2020-02-21T18:03:04.2429332Z    |
2020-02-21T18:03:04.2429630Z LL |     sse2();              //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2430020Z    |     ^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2430542Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2430791Z 
2020-02-21T18:03:04.2430791Z 
2020-02-21T18:03:04.2431111Z error[E0133]: call to function with `#[target_feature]` is unsafe and requires unsafe function or block
2020-02-21T18:03:04.2431817Z   --> /checkout/src/test/ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs:69:18
2020-02-21T18:03:04.2436959Z    |
2020-02-21T18:03:04.2437545Z LL | const name: () = sse2(); //~ ERROR call to function with `#[target_feature]` is unsafe
2020-02-21T18:03:04.2437996Z    |                  ^^^^^^ call to function with `#[target_feature]`
2020-02-21T18:03:04.2438524Z    = note: can only be called if the required target features are available
2020-02-21T18:03:04.2438788Z 
2020-02-21T18:03:04.2438989Z error: aborting due to 12 previous errors
2020-02-21T18:03:04.2439173Z 
---
2020-02-21T18:03:04.2441162Z 
2020-02-21T18:03:04.2441292Z 
2020-02-21T18:03:04.2441741Z ---- [ui] ui/target-feature/invalid-attribute.rs stdout ----
2020-02-21T18:03:04.2441960Z 
2020-02-21T18:03:04.2442880Z error: /checkout/src/test/ui/target-feature/invalid-attribute.rs:27: unexpected note: '27:1: 27:35: see issue #69098 <***/issues/69098> for more information'
2020-02-21T18:03:04.2444321Z error: /checkout/src/test/ui/target-feature/invalid-attribute.rs:27: expected note not found: can only be applied to `unsafe` functions
2020-02-21T18:03:04.2444717Z 
2020-02-21T18:03:04.2444954Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-21T18:03:04.2445239Z status: exit code: 1
2020-02-21T18:03:04.2445239Z status: exit code: 1
2020-02-21T18:03:04.2447369Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/target-feature/invalid-attribute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/target-feature/invalid-attribute/auxiliary"
2020-02-21T18:03:04.2449265Z     Error {
2020-02-21T18:03:04.2449463Z         line_num: 27,
2020-02-21T18:03:04.2449679Z         kind: Some(
2020-02-21T18:03:04.2449897Z             Note,
2020-02-21T18:03:04.2449897Z             Note,
2020-02-21T18:03:04.2450069Z         ),
2020-02-21T18:03:04.2450792Z         msg: "27:1: 27:35: see issue #69098 <***/issues/69098> for more information",
2020-02-21T18:03:04.2451242Z ]
2020-02-21T18:03:04.2451349Z 
2020-02-21T18:03:04.2451579Z not found errors (from test file): [
2020-02-21T18:03:04.2451807Z     Error {
---
2020-02-21T18:03:04.2455356Z 
2020-02-21T18:03:04.2455458Z 
2020-02-21T18:03:04.2455614Z failures:
2020-02-21T18:03:04.2455993Z     [ui] ui/macros/issue-68060.rs
2020-02-21T18:03:04.2456731Z     [ui] ui/rfcs/rfc-2396-target_feature-11/feature-gate-target_feature_11.rs
2020-02-21T18:03:04.2457633Z     [ui] ui/rfcs/rfc-2396-target_feature-11/safe-calls.rs
2020-02-21T18:03:04.2458350Z 
2020-02-21T18:03:04.2458903Z test result: FAILED. 9639 passed; 4 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-21T18:03:04.2459187Z 
2020-02-21T18:03:04.2459767Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-21T18:03:04.2459767Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-21T18:03:04.2466201Z 
2020-02-21T18:03:04.2466359Z 
2020-02-21T18:03:04.2477773Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-21T18:03:04.2480812Z 
2020-02-21T18:03:04.2480923Z 
2020-02-21T18:03:04.2481173Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-21T18:03:04.2481533Z Build completed unsuccessfully in 1:06:38
2020-02-21T18:03:04.2481533Z Build completed unsuccessfully in 1:06:38
2020-02-21T18:03:04.2481910Z == clock drift check ==
2020-02-21T18:03:04.2482178Z   local time: Fri Feb 21 18:03:04 UTC 2020
2020-02-21T18:03:04.5428995Z   network time: Fri, 21 Feb 2020 18:03:04 GMT
2020-02-21T18:03:04.5433585Z == end clock drift check ==
2020-02-21T18:03:05.0245531Z 
2020-02-21T18:03:05.0312174Z ##[error]Bash exited with code '1'.
2020-02-21T18:03:05.0332021Z ##[section]Finishing: Run build
2020-02-21T18:03:05.0381145Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T18:03:05.0387291Z Task         : Get sources
2020-02-21T18:03:05.0387654Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T18:03:05.0388006Z Version      : 1.0.0
2020-02-21T18:03:05.0389033Z Author       : Microsoft
2020-02-21T18:03:05.0389033Z Author       : Microsoft
2020-02-21T18:03:05.0389615Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T18:03:05.0390083Z ==============================================================================
2020-02-21T18:03:05.3827477Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T18:03:05.3877504Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T18:03:05.3968437Z Cleaning up task key
2020-02-21T18:03:05.3969860Z Start cleaning up orphan processes.
2020-02-21T18:03:05.4153591Z Terminate orphan process: pid (4848) (python)
2020-02-21T18:03:05.4400376Z ##[section]Finishing: Finalize Job
