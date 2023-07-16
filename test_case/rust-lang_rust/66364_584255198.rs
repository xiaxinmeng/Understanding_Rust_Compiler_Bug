plain
2020-02-10T16:56:35.3138942Z ========================== Starting Command Output ===========================
2020-02-10T16:56:35.3140563Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/87e79905-4341-4899-bb25-a12c8a856463.sh
2020-02-10T16:56:35.3140597Z 
2020-02-10T16:56:35.3143692Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T16:56:35.3149978Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66364/merge to s
2020-02-10T16:56:35.3151856Z Task         : Get sources
2020-02-10T16:56:35.3151933Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T16:56:35.3151965Z Version      : 1.0.0
2020-02-10T16:56:35.3151995Z Author       : Microsoft
---
2020-02-10T16:56:36.5341197Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T16:56:36.5438529Z ##[command]git config gc.auto 0
2020-02-10T16:56:36.5495515Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T16:56:36.5559910Z ##[command]git config --get-all http.proxy
2020-02-10T16:56:36.5689580Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66364/merge:refs/remotes/pull/66364/merge
---
2020-02-10T17:56:07.9683774Z .................................................................................................... 1700/9625
2020-02-10T17:56:12.8395188Z .................................................................................................... 1800/9625
2020-02-10T17:56:25.2485883Z .............................i...................................................................... 1900/9625
2020-02-10T17:56:32.6005244Z .................................................................................................... 2000/9625
2020-02-10T17:56:47.1765982Z ...................iiiii............................................................................ 2100/9625
2020-02-10T17:56:56.9794105Z .................................................................................................... 2300/9625
2020-02-10T17:56:59.4489062Z .................................................................................................... 2400/9625
2020-02-10T17:57:04.3199933Z .................................................................................................... 2500/9625
2020-02-10T17:57:25.2370892Z .................................................................................................... 2600/9625
---
2020-02-10T18:00:05.4835932Z ......................................................................i...............i............. 4900/9625
2020-02-10T18:00:13.8999999Z .................................................................................................... 5000/9625
2020-02-10T18:00:22.2547607Z .................................................................................................... 5100/9625
2020-02-10T18:00:27.4284678Z ............i....................................................................................... 5200/9625
2020-02-10T18:00:39.4324318Z ......................................................................................ii.ii........i 5300/9625
2020-02-10T18:00:48.0411565Z ........................i........................................................................... 5500/9625
2020-02-10T18:00:56.8176672Z .................................................................................................... 5600/9625
2020-02-10T18:01:05.9018414Z ..........................................................................i......................... 5700/9625
2020-02-10T18:01:13.7625433Z .................................................................................................... 5800/9625
2020-02-10T18:01:13.7625433Z .................................................................................................... 5800/9625
2020-02-10T18:01:20.5001946Z .................................................................................................... 5900/9625
2020-02-10T18:01:31.1583647Z ..................................................................ii...i..ii...........i............ 6000/9625
2020-02-10T18:01:53.2343947Z .................................................................................................... 6200/9625
2020-02-10T18:02:01.2660586Z .................................................................................................... 6300/9625
2020-02-10T18:02:01.2660586Z .................................................................................................... 6300/9625
2020-02-10T18:02:09.2340683Z ..............................................................................................i..ii. 6400/9625
2020-02-10T18:02:32.4051474Z .................................................................................................... 6600/9625
2020-02-10T18:02:42.6657178Z .................................................................................i.................. 6700/9625
2020-02-10T18:02:44.8536398Z .................................................................................................... 6800/9625
2020-02-10T18:02:47.1676850Z ........................................................................................i........... 6900/9625
---
2020-02-10T18:04:30.2639187Z .................................................................................................... 7600/9625
2020-02-10T18:04:34.5247851Z .................................................................................................... 7700/9625
2020-02-10T18:04:40.1470056Z .................................................................................................... 7800/9625
2020-02-10T18:04:48.6187227Z .................................................................................................... 7900/9625
2020-02-10T18:04:57.7200568Z ...................................................................iiiiiii.i........................ 8000/9625
2020-02-10T18:05:13.8179489Z .......i......i..................................................................................... 8200/9625
2020-02-10T18:05:19.4838562Z .................................................................................................... 8300/9625
2020-02-10T18:05:33.6606030Z .................................................................................................... 8400/9625
2020-02-10T18:05:43.5151684Z .................................................................................................... 8500/9625
---
2020-02-10T18:07:47.2350100Z normalized stderr:
2020-02-10T18:07:47.2350345Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2350792Z   --> $DIR/assoc_ty_bindings.rs:15:14
2020-02-10T18:07:47.2351048Z    |
2020-02-10T18:07:47.2351283Z LL |     type A = Base<AssocTy = u8>;
2020-02-10T18:07:47.2351488Z    |              ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Base<AssocTy = u8>`
2020-02-10T18:07:47.2351896Z LL | mac!();
2020-02-10T18:07:47.2352305Z    | ------- in this macro invocation
2020-02-10T18:07:47.2352570Z    |
2020-02-10T18:07:47.2352798Z    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2352798Z    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2353291Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2353914Z 
2020-02-10T18:07:47.2354205Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2354660Z   --> $DIR/assoc_ty_bindings.rs:16:14
2020-02-10T18:07:47.2354935Z    |
2020-02-10T18:07:47.2355139Z LL |     type B = Derived<AssocTy = u8>;
2020-02-10T18:07:47.2355507Z    |              ^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Derived<AssocTy = u8>`
2020-02-10T18:07:47.2355904Z LL | mac!();
2020-02-10T18:07:47.2356321Z    | ------- in this macro invocation
2020-02-10T18:07:47.2356581Z    |
2020-02-10T18:07:47.2357045Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2357045Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2357265Z 
2020-02-10T18:07:47.2357435Z 
2020-02-10T18:07:47.2357628Z 
2020-02-10T18:07:47.2357793Z 
2020-02-10T18:07:47.2357984Z The actual stderr differed from the expected stderr.
2020-02-10T18:07:47.2358497Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/assoc_ty_bindings/assoc_ty_bindings.stderr
2020-02-10T18:07:47.2358992Z To update references, rerun the tests and pass the `--bless` flag
2020-02-10T18:07:47.2359482Z To only update this specific test, also pass `--test-args hygiene/assoc_ty_bindings.rs`
2020-02-10T18:07:47.2359924Z error: 1 errors occurred comparing output.
2020-02-10T18:07:47.2360115Z status: exit code: 0
2020-02-10T18:07:47.2360115Z status: exit code: 0
2020-02-10T18:07:47.2361092Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/assoc_ty_bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/assoc_ty_bindings" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/assoc_ty_bindings/auxiliary" "-A" "unused"
2020-02-10T18:07:47.2361830Z ------------------------------------------
2020-02-10T18:07:47.2362071Z 
2020-02-10T18:07:47.2362518Z ------------------------------------------
2020-02-10T18:07:47.2362761Z stderr:
2020-02-10T18:07:47.2362761Z stderr:
2020-02-10T18:07:47.2363161Z ------------------------------------------
2020-02-10T18:07:47.2363431Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2363864Z   --> /checkout/src/test/ui/hygiene/assoc_ty_bindings.rs:15:14
2020-02-10T18:07:47.2364121Z    |
2020-02-10T18:07:47.2364336Z LL |     type A = Base<AssocTy = u8>;
2020-02-10T18:07:47.2364564Z    |              ^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Base<AssocTy = u8>`
2020-02-10T18:07:47.2364959Z LL | mac!();
2020-02-10T18:07:47.2365351Z    | ------- in this macro invocation
2020-02-10T18:07:47.2365616Z    |
2020-02-10T18:07:47.2365809Z    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2365809Z    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2366294Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2367131Z 
2020-02-10T18:07:47.2367342Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2367806Z   --> /checkout/src/test/ui/hygiene/assoc_ty_bindings.rs:16:14
2020-02-10T18:07:47.2368067Z    |
2020-02-10T18:07:47.2368198Z LL |     type B = Derived<AssocTy = u8>;
2020-02-10T18:07:47.2368328Z    |              ^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Derived<AssocTy = u8>`
2020-02-10T18:07:47.2368601Z LL | mac!();
2020-02-10T18:07:47.2368947Z    | ------- in this macro invocation
2020-02-10T18:07:47.2369127Z    |
2020-02-10T18:07:47.2369504Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
2020-02-10T18:07:47.2371408Z 
2020-02-10T18:07:47.2371722Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2372229Z +   --> $DIR/associated-item-privacy-type-binding.rs:11:20
2020-02-10T18:07:47.2372819Z +    |
2020-02-10T18:07:47.2372968Z + LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2373099Z +    |                    ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2373397Z + ...
2020-02-10T18:07:47.2373538Z + LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2375521Z +    |
2020-02-10T18:07:47.2375648Z +    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2376041Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2376234Z + 
2020-02-10T18:07:47.2376234Z + 
2020-02-10T18:07:47.2376361Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2376692Z +   --> $DIR/associated-item-privacy-type-binding.rs:14:35
2020-02-10T18:07:47.2376867Z +    |
2020-02-10T18:07:47.2376991Z + LL |         type InSignatureTy2 = Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2377120Z +    |                                   ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2377264Z + ...
2020-02-10T18:07:47.2377386Z + LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2377894Z +    |
2020-02-10T18:07:47.2378730Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2378923Z + 
2020-02-10T18:07:47.2379049Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2379049Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2379403Z +   --> $DIR/associated-item-privacy-type-binding.rs:20:20
2020-02-10T18:07:47.2379578Z +    |
2020-02-10T18:07:47.2379701Z + LL |         let _: Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2380014Z +    |                    ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PrivTr<AssocTy = u8>`
2020-02-10T18:07:47.2380159Z + ...
2020-02-10T18:07:47.2380283Z + LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2381237Z +    |
2020-02-10T18:07:47.2381661Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2381842Z + 
2020-02-10T18:07:47.2382135Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2382135Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2382464Z +   --> $DIR/associated-item-privacy-type-binding.rs:23:35
2020-02-10T18:07:47.2382634Z +    |
2020-02-10T18:07:47.2382762Z + LL |         type InSignatureTy1 = Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2382919Z +    |                                   ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PrivTr<AssocTy = u8>`
2020-02-10T18:07:47.2383064Z + ...
2020-02-10T18:07:47.2383185Z + LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2383684Z +    |
2020-02-10T18:07:47.2384587Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2386279Z + 
2020-02-10T18:07:47.2386485Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2386485Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2387225Z +   --> $DIR/associated-item-privacy-type-binding.rs:44:20
2020-02-10T18:07:47.2387534Z +    |
2020-02-10T18:07:47.2387805Z + LL |         let _: Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2387958Z +    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTrWithParam<AssocTy = u8>`
2020-02-10T18:07:47.2388295Z + ...
2020-02-10T18:07:47.2388711Z + LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2389585Z +    |
2020-02-10T18:07:47.2390042Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2390415Z + 
2020-02-10T18:07:47.2390571Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2390571Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2391025Z +   --> $DIR/associated-item-privacy-type-binding.rs:47:20
2020-02-10T18:07:47.2391326Z +    |
2020-02-10T18:07:47.2391477Z + LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2391613Z +    |                    ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2391756Z + ...
2020-02-10T18:07:47.2391956Z + LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2402629Z +    |
2020-02-10T18:07:47.2403226Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2403420Z + 
2020-02-10T18:07:47.2403553Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2403553Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2403944Z +   --> $DIR/associated-item-privacy-type-binding.rs:50:39
2020-02-10T18:07:47.2404111Z +    |
2020-02-10T18:07:47.2404267Z + LL |         pub type InSignatureTy1 = Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2404407Z +    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTrWithParam<AssocTy = u8>`
2020-02-10T18:07:47.2404533Z + ...
2020-02-10T18:07:47.2404695Z + LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2405222Z +    |
2020-02-10T18:07:47.2405607Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2405810Z + 
2020-02-10T18:07:47.2405945Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2405945Z + warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2406307Z +   --> $DIR/associated-item-privacy-type-binding.rs:52:39
2020-02-10T18:07:47.2406710Z +    |
2020-02-10T18:07:47.2406868Z + LL |         pub type InSignatureTy2 = Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2407012Z +    |                                       ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2407141Z + ...
2020-02-10T18:07:47.2407464Z + LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2408080Z +    |
2020-02-10T18:07:47.2408463Z +    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2408630Z + 
2020-02-10T18:07:47.2408630Z + 
2020-02-10T18:07:47.2408753Z 1 error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2409298Z 3    |
2020-02-10T18:07:47.2409432Z 
2020-02-10T18:07:47.2409534Z 
2020-02-10T18:07:47.2409656Z The actual stderr differed from the expected stderr.
2020-02-10T18:07:47.2409656Z The actual stderr differed from the expected stderr.
2020-02-10T18:07:47.2410095Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-type-binding/associated-item-privacy-type-binding.stderr
2020-02-10T18:07:47.2410530Z To update references, rerun the tests and pass the `--bless` flag
2020-02-10T18:07:47.2410933Z To only update this specific test, also pass `--test-args privacy/associated-item-privacy-type-binding.rs`
2020-02-10T18:07:47.2411211Z error: 1 errors occurred comparing output.
2020-02-10T18:07:47.2411331Z status: exit code: 1
2020-02-10T18:07:47.2411331Z status: exit code: 1
2020-02-10T18:07:47.2413363Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-type-binding" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-type-binding/auxiliary" "-A" "unused"
2020-02-10T18:07:47.2414117Z ------------------------------------------
2020-02-10T18:07:47.2414266Z 
2020-02-10T18:07:47.2414573Z ------------------------------------------
2020-02-10T18:07:47.2414745Z stderr:
2020-02-10T18:07:47.2414745Z stderr:
2020-02-10T18:07:47.2415051Z ------------------------------------------
2020-02-10T18:07:47.2415208Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2415573Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:11:20
2020-02-10T18:07:47.2415741Z    |
2020-02-10T18:07:47.2415987Z LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2416132Z    |                    ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2416377Z LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2416701Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2416873Z    |
2020-02-10T18:07:47.2416995Z    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2416995Z    = note: `#[warn(bare_trait_objects)]` on by default
2020-02-10T18:07:47.2417360Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2417503Z 
2020-02-10T18:07:47.2417653Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2418005Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:14:35
2020-02-10T18:07:47.2418189Z    |
2020-02-10T18:07:47.2418411Z LL |         type InSignatureTy2 = Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2418622Z    |                                   ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2418986Z LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2419310Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2419499Z    |
2020-02-10T18:07:47.2419946Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2419946Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2420456Z 
2020-02-10T18:07:47.2420692Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2421118Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:20:20
2020-02-10T18:07:47.2421376Z    |
2020-02-10T18:07:47.2421579Z LL |         let _: Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2421963Z    |                    ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PrivTr<AssocTy = u8>`
2020-02-10T18:07:47.2422257Z LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2422867Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2423033Z    |
2020-02-10T18:07:47.2423443Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2423443Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2425328Z 
2020-02-10T18:07:47.2425396Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2425933Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:23:35
2020-02-10T18:07:47.2425981Z    |
2020-02-10T18:07:47.2426026Z LL |         type InSignatureTy1 = Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2426100Z    |                                   ^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PrivTr<AssocTy = u8>`
2020-02-10T18:07:47.2426188Z LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2426436Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2426496Z    |
2020-02-10T18:07:47.2426963Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2426963Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2427170Z 
2020-02-10T18:07:47.2427236Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2427511Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:44:20
2020-02-10T18:07:47.2427557Z    |
2020-02-10T18:07:47.2427616Z LL |         let _: Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2427668Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTrWithParam<AssocTy = u8>`
2020-02-10T18:07:47.2427713Z ...
2020-02-10T18:07:47.2427770Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2428040Z    |
2020-02-10T18:07:47.2428323Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2428358Z 
2020-02-10T18:07:47.2428401Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2428401Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2428669Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:47:20
2020-02-10T18:07:47.2428732Z    |
2020-02-10T18:07:47.2428773Z LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2428822Z    |                    ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2428884Z ...
2020-02-10T18:07:47.2428925Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2429210Z    |
2020-02-10T18:07:47.2429639Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2429838Z 
2020-02-10T18:07:47.2429878Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2429878Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2430128Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:50:39
2020-02-10T18:07:47.2430171Z    |
2020-02-10T18:07:47.2430213Z LL |         pub type InSignatureTy1 = Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2430297Z    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTrWithParam<AssocTy = u8>`
2020-02-10T18:07:47.2430340Z ...
2020-02-10T18:07:47.2430378Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2430883Z    |
2020-02-10T18:07:47.2431316Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2431575Z 
2020-02-10T18:07:47.2431619Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2431619Z warning: trait objects without an explicit `dyn` are deprecated
2020-02-10T18:07:47.2431874Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:52:39
2020-02-10T18:07:47.2431919Z    |
2020-02-10T18:07:47.2431980Z LL |         pub type InSignatureTy2 = Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2432035Z    |                                       ^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn PubTr<AssocTy = u8>`
2020-02-10T18:07:47.2432097Z ...
2020-02-10T18:07:47.2432155Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2432435Z    |
2020-02-10T18:07:47.2432914Z    = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2432952Z 
2020-02-10T18:07:47.2432952Z 
2020-02-10T18:07:47.2432996Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2433483Z    |
2020-02-10T18:07:47.2433483Z    |
2020-02-10T18:07:47.2433526Z LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2433626Z ...
2020-02-10T18:07:47.2433667Z LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2433894Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2433957Z    |
2020-02-10T18:07:47.2433957Z    |
2020-02-10T18:07:47.2434673Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2434927Z 
2020-02-10T18:07:47.2434969Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2435316Z    |
2020-02-10T18:07:47.2435316Z    |
2020-02-10T18:07:47.2435356Z LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2435455Z ...
2020-02-10T18:07:47.2435494Z LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2435732Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2435775Z    |
2020-02-10T18:07:47.2435775Z    |
2020-02-10T18:07:47.2436037Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2436072Z 
2020-02-10T18:07:47.2436129Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2436427Z    |
2020-02-10T18:07:47.2436427Z    |
2020-02-10T18:07:47.2436493Z LL |         type InSignatureTy2 = Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2436583Z ...
2020-02-10T18:07:47.2436639Z LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2436860Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2436903Z    |
2020-02-10T18:07:47.2436903Z    |
2020-02-10T18:07:47.2437164Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2437215Z 
2020-02-10T18:07:47.2437255Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2437560Z    |
2020-02-10T18:07:47.2437560Z    |
2020-02-10T18:07:47.2437602Z LL |         trait InSignatureTr2: PubTr<AssocTy = u8> {}
2020-02-10T18:07:47.2437713Z ...
2020-02-10T18:07:47.2437759Z LL |     priv_trait::mac1!();
2020-02-10T18:07:47.2438326Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2438365Z    |
2020-02-10T18:07:47.2438365Z    |
2020-02-10T18:07:47.2438628Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2438660Z 
2020-02-10T18:07:47.2438698Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2438982Z    |
2020-02-10T18:07:47.2438982Z    |
2020-02-10T18:07:47.2439020Z LL |         let _: Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2439113Z ...
2020-02-10T18:07:47.2439148Z LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2439352Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2439410Z    |
2020-02-10T18:07:47.2439410Z    |
2020-02-10T18:07:47.2439655Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2439696Z 
2020-02-10T18:07:47.2439756Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2440031Z    |
2020-02-10T18:07:47.2440031Z    |
2020-02-10T18:07:47.2440069Z LL |         let _: Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2440163Z ...
2020-02-10T18:07:47.2440199Z LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2440422Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2440461Z    |
2020-02-10T18:07:47.2440461Z    |
2020-02-10T18:07:47.2440707Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2440756Z 
2020-02-10T18:07:47.2440794Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2441066Z    |
2020-02-10T18:07:47.2441066Z    |
2020-02-10T18:07:47.2441252Z LL |         type InSignatureTy1 = Box<PrivTr<AssocTy = u8>>;
2020-02-10T18:07:47.2441336Z ...
2020-02-10T18:07:47.2441389Z LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2441613Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2441653Z    |
2020-02-10T18:07:47.2441653Z    |
2020-02-10T18:07:47.2441915Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2441947Z 
2020-02-10T18:07:47.2441985Z error: trait `priv_trait::PrivTr` is private
2020-02-10T18:07:47.2442271Z    |
2020-02-10T18:07:47.2442271Z    |
2020-02-10T18:07:47.2442311Z LL |         trait InSignatureTr1: PrivTr<AssocTy = u8> {}
2020-02-10T18:07:47.2442408Z ...
2020-02-10T18:07:47.2442446Z LL |     priv_trait::mac2!();
2020-02-10T18:07:47.2442669Z    |     -------------------- in this macro invocation
2020-02-10T18:07:47.2442725Z    |
2020-02-10T18:07:47.2442725Z    |
2020-02-10T18:07:47.2442974Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2443006Z 
2020-02-10T18:07:47.2443044Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2443290Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:44:13
2020-02-10T18:07:47.2443332Z    |
2020-02-10T18:07:47.2443371Z LL |         let _: Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2443462Z ...
2020-02-10T18:07:47.2443462Z ...
2020-02-10T18:07:47.2443499Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2443769Z    |
2020-02-10T18:07:47.2444034Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2444075Z 
2020-02-10T18:07:47.2444136Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2444136Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2444369Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:44:16
2020-02-10T18:07:47.2444411Z    |
2020-02-10T18:07:47.2444466Z LL |         let _: Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2444547Z ...
2020-02-10T18:07:47.2444547Z ...
2020-02-10T18:07:47.2444601Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2444854Z    |
2020-02-10T18:07:47.2445098Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2445146Z 
2020-02-10T18:07:47.2445185Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2445185Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2445595Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:47:13
2020-02-10T18:07:47.2445654Z    |
2020-02-10T18:07:47.2445707Z LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2445783Z ...
2020-02-10T18:07:47.2445783Z ...
2020-02-10T18:07:47.2445945Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2446394Z    |
2020-02-10T18:07:47.2446674Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2446708Z 
2020-02-10T18:07:47.2446748Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2446748Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2447008Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:47:16
2020-02-10T18:07:47.2447052Z    |
2020-02-10T18:07:47.2447093Z LL |         let _: Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2447195Z ...
2020-02-10T18:07:47.2447195Z ...
2020-02-10T18:07:47.2447235Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2447811Z    |
2020-02-10T18:07:47.2448092Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2448126Z 
2020-02-10T18:07:47.2448182Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2448182Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2448763Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:50:35
2020-02-10T18:07:47.2448806Z    |
2020-02-10T18:07:47.2448848Z LL |         pub type InSignatureTy1 = Box<PubTrWithParam<AssocTy = u8>>;
2020-02-10T18:07:47.2448953Z ...
2020-02-10T18:07:47.2448953Z ...
2020-02-10T18:07:47.2448992Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2449651Z    |
2020-02-10T18:07:47.2449912Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2449971Z 
2020-02-10T18:07:47.2450017Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2450017Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2450258Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:52:35
2020-02-10T18:07:47.2450302Z    |
2020-02-10T18:07:47.2450360Z LL |         pub type InSignatureTy2 = Box<PubTr<AssocTy = u8>>;
2020-02-10T18:07:47.2450445Z ...
2020-02-10T18:07:47.2450445Z ...
2020-02-10T18:07:47.2450501Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2450762Z    |
2020-02-10T18:07:47.2451031Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2451064Z 
2020-02-10T18:07:47.2451102Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2451102Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2451340Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:54:31
2020-02-10T18:07:47.2451407Z    |
2020-02-10T18:07:47.2451455Z LL |         trait InSignatureTr1: PubTrWithParam<AssocTy = u8> {}
2020-02-10T18:07:47.2451557Z ...
2020-02-10T18:07:47.2451557Z ...
2020-02-10T18:07:47.2451596Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2451874Z    |
2020-02-10T18:07:47.2452131Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2452164Z 
2020-02-10T18:07:47.2452203Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2452203Z error: type `priv_parent_substs::Priv` is private
2020-02-10T18:07:47.2452801Z   --> /checkout/src/test/ui/privacy/associated-item-privacy-type-binding.rs:56:31
2020-02-10T18:07:47.2452843Z    |
2020-02-10T18:07:47.2452883Z LL |         trait InSignatureTr2: PubTr<AssocTy = u8> {}
2020-02-10T18:07:47.2452988Z ...
2020-02-10T18:07:47.2452988Z ...
2020-02-10T18:07:47.2453031Z LL |     priv_parent_substs::mac!();
2020-02-10T18:07:47.2453304Z    |
2020-02-10T18:07:47.2453550Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-02-10T18:07:47.2453583Z 
2020-02-10T18:07:47.2453638Z error: aborting due to 16 previous errors
2020-02-10T18:07:47.2453638Z error: aborting due to 16 previous errors
2020-02-10T18:07:47.2453663Z 
2020-02-10T18:07:47.2453686Z 
2020-02-10T18:07:47.2453878Z ------------------------------------------
2020-02-10T18:07:47.2453924Z 
2020-02-10T18:07:47.2453946Z 
2020-02-10T18:07:47.2454152Z ---- [ui] ui/suggestions/vec-macro-in-pattern.rs stdout ----
2020-02-10T18:07:47.2454624Z thread '[ui] ui/suggestions/vec-macro-in-pattern.rs' panicked at 'index 23 out of range for slice of length 0', src/libcore/slice/mod.rs:2674:5
2020-02-10T18:07:47.2454877Z 
2020-02-10T18:07:47.2454900Z 
2020-02-10T18:07:47.2455009Z failures:
2020-02-10T18:07:47.2455273Z     [ui] ui/hygiene/assoc_ty_bindings.rs
---
2020-02-10T18:07:47.2456304Z 
2020-02-10T18:07:47.2456575Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-10T18:07:47.2456609Z 
2020-02-10T18:07:47.2456634Z 
2020-02-10T18:07:47.2458451Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-10T18:07:47.2459171Z 
2020-02-10T18:07:47.2459203Z 
2020-02-10T18:07:47.2459242Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-10T18:07:47.2459308Z Build completed unsuccessfully in 1:04:12
2020-02-10T18:07:47.2459308Z Build completed unsuccessfully in 1:04:12
2020-02-10T18:07:47.2467161Z == clock drift check ==
2020-02-10T18:07:47.2485748Z   local time: Mon Feb 10 18:07:47 UTC 2020
2020-02-10T18:07:47.5206943Z   network time: Mon, 10 Feb 2020 18:07:47 GMT
2020-02-10T18:07:47.5212554Z == end clock drift check ==
2020-02-10T18:07:48.1039755Z 
2020-02-10T18:07:48.1102157Z ##[error]Bash exited with code '1'.
2020-02-10T18:07:48.1121969Z ##[section]Finishing: Run build
2020-02-10T18:07:48.1143844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66364/merge to s
2020-02-10T18:07:48.1146236Z Task         : Get sources
2020-02-10T18:07:48.1146281Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T18:07:48.1146337Z Version      : 1.0.0
2020-02-10T18:07:48.1146392Z Author       : Microsoft
2020-02-10T18:07:48.1146392Z Author       : Microsoft
2020-02-10T18:07:48.1146433Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T18:07:48.1146478Z ==============================================================================
2020-02-10T18:07:48.5244873Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T18:07:48.5290875Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/66364/merge to s
2020-02-10T18:07:48.5438282Z Cleaning up task key
2020-02-10T18:07:48.5439302Z Start cleaning up orphan processes.
2020-02-10T18:07:48.5556403Z Terminate orphan process: pid (12196) (python)
2020-02-10T18:07:48.6739584Z ##[section]Finishing: Finalize Job
