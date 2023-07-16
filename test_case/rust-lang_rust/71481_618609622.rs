plain
2020-04-23T18:22:55.7563248Z ========================== Starting Command Output ===========================
2020-04-23T18:22:55.7566249Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/03dbf5c0-c161-4c64-b8d1-5f8bc328fa5f.sh
2020-04-23T18:22:55.7566645Z 
2020-04-23T18:22:55.7655236Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-23T18:22:55.7672891Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71481/merge to s
2020-04-23T18:22:55.7675776Z Task         : Get sources
2020-04-23T18:22:55.7676054Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T18:22:55.7676306Z Version      : 1.0.0
2020-04-23T18:22:55.7676482Z Author       : Microsoft
---
2020-04-23T18:22:56.7572733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-23T18:22:56.7579186Z ##[command]git config gc.auto 0
2020-04-23T18:22:56.7583440Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-23T18:22:56.7587328Z ##[command]git config --get-all http.proxy
2020-04-23T18:22:56.7594270Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71481/merge:refs/remotes/pull/71481/merge
---
2020-04-23T18:25:20.3386406Z  ---> 318032b5f0e2
2020-04-23T18:25:20.3387166Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-23T18:25:20.3391481Z  ---> Using cache
2020-04-23T18:25:20.3391859Z  ---> d44a858fd1ce
2020-04-23T18:25:20.3392814Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-23T18:25:20.3405320Z  ---> 58b910f50f5a
2020-04-23T18:25:20.3405555Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-23T18:25:20.3405903Z  ---> Using cache
2020-04-23T18:25:20.3406214Z  ---> ee7702aadba1
---
2020-04-23T18:25:20.3749594Z Looks like docker image is the same as before, not uploading
2020-04-23T18:25:27.1203484Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T18:25:27.1486011Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-23T18:25:27.1513459Z == clock drift check ==
2020-04-23T18:25:27.1525409Z   local time: Thu Apr 23 18:25:27 UTC 2020
2020-04-23T18:25:27.7030279Z   network time: Thu, 23 Apr 2020 18:25:27 GMT
2020-04-23T18:25:27.7030813Z Starting sccache server...
2020-04-23T18:25:27.7855073Z configure: processing command line
2020-04-23T18:25:27.7855354Z configure: 
2020-04-23T18:25:27.7856297Z configure: rust.dist-src        := False
---
2020-04-23T18:30:20.2460971Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T18:30:21.8805966Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T18:30:23.1721401Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T18:30:24.2751381Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T18:30:32.6912779Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T18:30:35.1664627Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T18:30:39.4524733Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T18:30:43.4983587Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T18:30:52.2845481Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T18:51:22.4296295Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-23T18:51:24.0156741Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-23T18:51:25.6735909Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-23T18:51:25.7967635Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-23T18:51:35.9566208Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-23T18:51:37.9104418Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-23T18:51:42.4708196Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-23T18:51:46.6820834Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-23T18:51:56.8123476Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-23T19:12:37.6227578Z .................................................................................................... 1700/9917
2020-04-23T19:12:41.5556377Z .................................................................................................... 1800/9917
2020-04-23T19:12:49.1308603Z .................................................................................................... 1900/9917
2020-04-23T19:12:56.6831347Z .....i.............................................................................................. 2000/9917
2020-04-23T19:13:02.5254491Z ...............................................................................................iiiii 2100/9917
2020-04-23T19:13:20.7729523Z .................................................................................................... 2300/9917
2020-04-23T19:13:22.6742604Z .................................................................................................... 2400/9917
2020-04-23T19:13:24.7343927Z .................................................................................................... 2500/9917
2020-04-23T19:13:29.8399293Z .................................................................................................... 2600/9917
---
2020-04-23T19:16:04.4620769Z .................................................................................................... 5100/9917
2020-04-23T19:16:10.0253006Z .................................................................................................... 5200/9917
2020-04-23T19:16:13.8059488Z ..................i................................................................................. 5300/9917
2020-04-23T19:16:22.1498593Z ........i........................................................................................... 5400/9917
2020-04-23T19:16:27.1213928Z ........ii.ii........i...i.......................................................................... 5500/9917
2020-04-23T19:16:33.9966047Z .......................................................i............................................ 5700/9917
2020-04-23T19:16:41.8763527Z ........................................................................................ii.......... 5800/9917
2020-04-23T19:16:47.9596527Z ...........................i........................................................................ 5900/9917
2020-04-23T19:16:52.7282426Z .................................................................................................... 6000/9917
2020-04-23T19:16:52.7282426Z .................................................................................................... 6000/9917
2020-04-23T19:17:02.0443823Z .................................................................................................... 6100/9917
2020-04-23T19:17:11.0754040Z .....................ii...i..ii...........i......................................................... 6200/9917
2020-04-23T19:17:23.8574869Z .................................................................................................... 6400/9917
2020-04-23T19:17:26.5104169Z .................................................................................................... 6500/9917
2020-04-23T19:17:26.5104169Z .................................................................................................... 6500/9917
2020-04-23T19:17:33.8228627Z ...................................................i..ii............................................ 6600/9917
2020-04-23T19:17:53.2795046Z .................................................................................................... 6800/9917
2020-04-23T19:17:55.2250352Z ....................................................i............................................... 6900/9917
2020-04-23T19:17:56.8290773Z .................................................................................................... 7000/9917
2020-04-23T19:17:58.5172214Z ............................................................................................i....... 7100/9917
---
2020-04-23T19:19:22.6668509Z .................................................................................................... 7900/9917
2020-04-23T19:19:27.9012282Z .................................................................................................... 8000/9917
2020-04-23T19:19:33.3773135Z ...........................................................i........................................ 8100/9917
2020-04-23T19:19:42.2281709Z .................................................................................................... 8200/9917
2020-04-23T19:19:47.2690962Z .........iiiiiiiiiii.i.............................................................................. 8300/9917
2020-04-23T19:19:59.5613910Z .................................................................................................... 8500/9917
2020-04-23T19:20:06.1796993Z .....................................................F.......F...................................... 8600/9917
2020-04-23T19:20:19.1844025Z .................................................................................................... 8700/9917
2020-04-23T19:20:25.5024098Z .................................................................................................... 8800/9917
---
2020-04-23T19:22:08.6293667Z ---- [ui] ui/stability-attribute/stability-attribute-issue-43027.rs stdout ----
2020-04-23T19:22:08.6293890Z 
2020-04-23T19:22:08.6294089Z error: ui test compiled successfully!
2020-04-23T19:22:08.6294303Z status: exit code: 0
2020-04-23T19:22:08.6296309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-issue-43027.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-issue-43027" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-issue-43027/auxiliary"
2020-04-23T19:22:08.6298122Z ------------------------------------------
2020-04-23T19:22:08.6298283Z 
2020-04-23T19:22:08.6298626Z ------------------------------------------
2020-04-23T19:22:08.6298816Z stderr:
---
2020-04-23T19:22:08.6299939Z 
2020-04-23T19:22:08.6300353Z ---- [ui] ui/stability-attribute/stability-attribute-sanity.rs stdout ----
2020-04-23T19:22:08.6301857Z diff of stderr:
2020-04-23T19:22:08.6302212Z 
2020-04-23T19:22:08.6302671Z 94 LL | #[rustc_const_unstable(feature = "d", issue = "none")]
2020-04-23T19:22:08.6303303Z 96 
2020-04-23T19:22:08.6303972Z - error: Invalid stability or deprecation version found
2020-04-23T19:22:08.6304257Z + error: invalid stability or deprecation version found
2020-04-23T19:22:08.6304709Z 98   --> $DIR/stability-attribute-sanity.rs:65:1
2020-04-23T19:22:08.6304709Z 98   --> $DIR/stability-attribute-sanity.rs:65:1
2020-04-23T19:22:08.6304907Z 99    |
2020-04-23T19:22:08.6305072Z 100 LL | pub const fn multiple4() { }
2020-04-23T19:22:08.6305227Z 
2020-04-23T19:22:08.6305315Z 
2020-04-23T19:22:08.6305500Z The actual stderr differed from the expected stderr.
2020-04-23T19:22:08.6306143Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/stability-attribute-sanity.stderr
2020-04-23T19:22:08.6306740Z To update references, rerun the tests and pass the `--bless` flag
2020-04-23T19:22:08.6307294Z To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-sanity.rs`
2020-04-23T19:22:08.6307714Z error: 1 errors occurred comparing output.
2020-04-23T19:22:08.6307917Z status: exit code: 1
2020-04-23T19:22:08.6307917Z status: exit code: 1
2020-04-23T19:22:08.6309696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/auxiliary"
2020-04-23T19:22:08.6311380Z ------------------------------------------
2020-04-23T19:22:08.6311533Z 
2020-04-23T19:22:08.6311860Z ------------------------------------------
2020-04-23T19:22:08.6312055Z stderr:
2020-04-23T19:22:08.6312055Z stderr:
2020-04-23T19:22:08.6312381Z ------------------------------------------
2020-04-23T19:22:08.6312758Z error[E0541]: unknown meta item 'reason'
2020-04-23T19:22:08.6313503Z    |
2020-04-23T19:22:08.6313503Z    |
2020-04-23T19:22:08.6313958Z LL |     #[stable(feature = "a", since = "b", reason)] //~ ERROR unknown meta item 'reason' [E0541]
2020-04-23T19:22:08.6314332Z    |                                          ^^^^^^ expected one of `since`, `note`
2020-04-23T19:22:08.6314697Z error[E0539]: incorrect meta item
2020-04-23T19:22:08.6315178Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:11:29
2020-04-23T19:22:08.6315434Z    |
2020-04-23T19:22:08.6315434Z    |
2020-04-23T19:22:08.6315655Z LL |     #[stable(feature = "a", since)] //~ ERROR incorrect meta item [E0539]
2020-04-23T19:22:08.6316073Z 
2020-04-23T19:22:08.6316227Z error[E0539]: incorrect meta item
2020-04-23T19:22:08.6316703Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:14:14
2020-04-23T19:22:08.6316958Z    |
2020-04-23T19:22:08.6316958Z    |
2020-04-23T19:22:08.6317176Z LL |     #[stable(feature, since = "a")] //~ ERROR incorrect meta item [E0539]
2020-04-23T19:22:08.6317655Z 
2020-04-23T19:22:08.6317805Z error[E0539]: incorrect meta item
2020-04-23T19:22:08.6318221Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:17:29
2020-04-23T19:22:08.6318434Z    |
2020-04-23T19:22:08.6318434Z    |
2020-04-23T19:22:08.6318642Z LL |     #[stable(feature = "a", since(b))] //~ ERROR incorrect meta item [E0539]
2020-04-23T19:22:08.6319002Z 
2020-04-23T19:22:08.6319156Z error[E0539]: incorrect meta item
2020-04-23T19:22:08.6319573Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:20:14
2020-04-23T19:22:08.6319780Z    |
2020-04-23T19:22:08.6319780Z    |
2020-04-23T19:22:08.6319972Z LL |     #[stable(feature(b), since = "a")] //~ ERROR incorrect meta item [E0539]
2020-04-23T19:22:08.6320316Z 
2020-04-23T19:22:08.6320576Z error[E0546]: missing 'feature'
2020-04-23T19:22:08.6321047Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:25:5
2020-04-23T19:22:08.6321254Z    |
2020-04-23T19:22:08.6321254Z    |
2020-04-23T19:22:08.6321607Z LL |     #[unstable(issue = "none")] //~ ERROR missing 'feature' [E0546]
2020-04-23T19:22:08.6321952Z 
2020-04-23T19:22:08.6322204Z error[E0547]: missing 'issue'
2020-04-23T19:22:08.6322625Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:28:5
2020-04-23T19:22:08.6322829Z    |
2020-04-23T19:22:08.6322829Z    |
2020-04-23T19:22:08.6323330Z LL |     #[unstable(feature = "b")] //~ ERROR missing 'issue' [E0547]
2020-04-23T19:22:08.6323731Z 
2020-04-23T19:22:08.6324028Z error[E0546]: missing 'feature'
2020-04-23T19:22:08.6324508Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:31:5
2020-04-23T19:22:08.6324744Z    |
2020-04-23T19:22:08.6324744Z    |
2020-04-23T19:22:08.6325119Z LL |     #[stable(since = "a")] //~ ERROR missing 'feature' [E0546]
2020-04-23T19:22:08.6325506Z 
2020-04-23T19:22:08.6325797Z error[E0542]: missing 'since'
2020-04-23T19:22:08.6326261Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:36:5
2020-04-23T19:22:08.6326570Z    |
2020-04-23T19:22:08.6326570Z    |
2020-04-23T19:22:08.6326959Z LL |     #[stable(feature = "a")] //~ ERROR missing 'since' [E0542]
2020-04-23T19:22:08.6327333Z 
2020-04-23T19:22:08.6327639Z error[E0542]: missing 'since'
2020-04-23T19:22:08.6328153Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:40:5
2020-04-23T19:22:08.6328393Z    |
2020-04-23T19:22:08.6328393Z    |
2020-04-23T19:22:08.6328817Z LL |     #[rustc_deprecated(reason = "a")] //~ ERROR missing 'since' [E0542]
2020-04-23T19:22:08.6329232Z 
2020-04-23T19:22:08.6329545Z error[E0543]: missing 'reason'
2020-04-23T19:22:08.6330008Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:44:5
2020-04-23T19:22:08.6330245Z    |
2020-04-23T19:22:08.6330245Z    |
2020-04-23T19:22:08.6330653Z LL |     #[rustc_deprecated(since = "a")] //~ ERROR missing 'reason' [E0543]
2020-04-23T19:22:08.6331070Z 
2020-04-23T19:22:08.6331232Z error[E0544]: multiple stability levels
2020-04-23T19:22:08.6331726Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:49:1
2020-04-23T19:22:08.6331964Z    |
2020-04-23T19:22:08.6331964Z    |
2020-04-23T19:22:08.6332198Z LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]
2020-04-23T19:22:08.6332639Z 
2020-04-23T19:22:08.6332802Z error[E0544]: multiple stability levels
2020-04-23T19:22:08.6333293Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:53:1
2020-04-23T19:22:08.6333531Z    |
2020-04-23T19:22:08.6333531Z    |
2020-04-23T19:22:08.6333765Z LL | #[unstable(feature = "b", issue = "none")] //~ ERROR multiple stability levels [E0544]
2020-04-23T19:22:08.6334224Z 
2020-04-23T19:22:08.6334386Z error[E0544]: multiple stability levels
2020-04-23T19:22:08.6334863Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:57:1
2020-04-23T19:22:08.6335116Z    |
2020-04-23T19:22:08.6335116Z    |
2020-04-23T19:22:08.6335345Z LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]
2020-04-23T19:22:08.6335783Z 
2020-04-23T19:22:08.6335964Z error[E0540]: multiple rustc_deprecated attributes
2020-04-23T19:22:08.6336457Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2020-04-23T19:22:08.6336710Z    |
2020-04-23T19:22:08.6336710Z    |
2020-04-23T19:22:08.6374430Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2020-04-23T19:22:08.6374980Z 
2020-04-23T19:22:08.6375208Z error[E0544]: multiple stability levels
2020-04-23T19:22:08.6376093Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:64:1
2020-04-23T19:22:08.6376352Z    |
2020-04-23T19:22:08.6376352Z    |
2020-04-23T19:22:08.6376662Z LL | #[rustc_const_unstable(feature = "d", issue = "none")] //~ ERROR multiple stability levels
2020-04-23T19:22:08.6377703Z 
2020-04-23T19:22:08.6377899Z error: invalid stability or deprecation version found
2020-04-23T19:22:08.6378434Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:65:1
2020-04-23T19:22:08.6378672Z    |
2020-04-23T19:22:08.6378672Z    |
2020-04-23T19:22:08.6378900Z LL | pub const fn multiple4() { } //~ ERROR multiple rustc_deprecated attributes [E0540]
2020-04-23T19:22:08.6379319Z 
2020-04-23T19:22:08.6379558Z error[E0549]: rustc_deprecated attribute must be paired with either stable or unstable attribute
2020-04-23T19:22:08.6380135Z   --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:69:1
2020-04-23T19:22:08.6380371Z    |
---
2020-04-23T19:22:08.6385720Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-23T19:22:08.6386071Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-23T19:22:08.6386267Z 
2020-04-23T19:22:08.6386369Z 
2020-04-23T19:22:08.6389491Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-23T19:22:08.6391676Z 
2020-04-23T19:22:08.6391764Z 
2020-04-23T19:22:08.6447542Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-23T19:22:08.6452982Z Build completed unsuccessfully in 0:55:10
2020-04-23T19:22:08.6452982Z Build completed unsuccessfully in 0:55:10
2020-04-23T19:22:08.6453243Z == clock drift check ==
2020-04-23T19:22:08.6468015Z   local time: Thu Apr 23 19:22:08 UTC 2020
2020-04-23T19:22:08.7488460Z   network time: Thu, 23 Apr 2020 19:22:08 GMT
2020-04-23T19:22:09.6478901Z 
2020-04-23T19:22:09.6478901Z 
2020-04-23T19:22:09.6511592Z ##[error]Bash exited with code '1'.
2020-04-23T19:22:09.6536508Z ##[section]Finishing: Run build
2020-04-23T19:22:09.6596033Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71481/merge to s
2020-04-23T19:22:09.6602071Z Task         : Get sources
2020-04-23T19:22:09.6602357Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-23T19:22:09.6602634Z Version      : 1.0.0
2020-04-23T19:22:09.6602821Z Author       : Microsoft
2020-04-23T19:22:09.6602821Z Author       : Microsoft
2020-04-23T19:22:09.6603116Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-23T19:22:09.6603461Z ==============================================================================
2020-04-23T19:22:09.9710360Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-23T19:22:09.9751975Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71481/merge to s
2020-04-23T19:22:09.9832515Z Cleaning up task key
2020-04-23T19:22:09.9833763Z Start cleaning up orphan processes.
2020-04-23T19:22:09.9999234Z Terminate orphan process: pid (3933) (python)
2020-04-23T19:22:10.0166243Z ##[section]Finishing: Finalize Job
