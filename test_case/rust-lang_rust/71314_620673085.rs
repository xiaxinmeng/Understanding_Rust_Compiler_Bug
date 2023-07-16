plain
2020-04-28T14:11:47.0578425Z ========================== Starting Command Output ===========================
2020-04-28T14:11:47.0628819Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c7556745-30e5-47c8-a006-58b4c6923ac0.sh
2020-04-28T14:11:47.0629125Z 
2020-04-28T14:11:47.0634735Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T14:11:47.0655052Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71314/merge to s
2020-04-28T14:11:47.0658738Z Task         : Get sources
2020-04-28T14:11:47.0659038Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T14:11:47.0659333Z Version      : 1.0.0
2020-04-28T14:11:47.0659558Z Author       : Microsoft
---
2020-04-28T14:11:48.0439624Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T14:11:48.0445545Z ##[command]git config gc.auto 0
2020-04-28T14:11:48.0449611Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T14:11:48.0453339Z ##[command]git config --get-all http.proxy
2020-04-28T14:11:48.0460262Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71314/merge:refs/remotes/pull/71314/merge
---
2020-04-28T14:15:17.2159104Z  ---> cb2676f08729
2020-04-28T14:15:17.2159769Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T14:15:17.2160272Z  ---> Using cache
2020-04-28T14:15:17.2160551Z  ---> df25ce111862
2020-04-28T14:15:17.2161364Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T14:15:17.2162252Z  ---> 599b9ac96b27
2020-04-28T14:15:17.2162445Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T14:15:17.2162752Z  ---> Using cache
2020-04-28T14:15:17.2163035Z  ---> 091087e35a36
---
2020-04-28T14:15:17.2529471Z Looks like docker image is the same as before, not uploading
2020-04-28T14:15:23.9333331Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T14:15:23.9690400Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T14:15:23.9714627Z == clock drift check ==
2020-04-28T14:15:23.9721536Z   local time: Tue Apr 28 14:15:23 UTC 2020
2020-04-28T14:15:24.1366939Z   network time: Tue, 28 Apr 2020 14:15:24 GMT
2020-04-28T14:15:24.1394296Z Starting sccache server...
2020-04-28T14:15:24.2206676Z configure: processing command line
2020-04-28T14:15:24.2206959Z configure: 
2020-04-28T14:15:24.2207980Z configure: rust.dist-src        := False
---
2020-04-28T14:17:47.8347855Z    Compiling term v0.0.0 (/checkout/src/libterm)
2020-04-28T14:17:52.0086542Z    Compiling unicode-width v0.1.6
2020-04-28T14:17:52.1037524Z    Compiling getopts v0.2.21
2020-04-28T14:18:02.5591511Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T14:18:11.2148106Z {"reason":"build-finished","success":true}
2020-04-28T14:18:11.2294584Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T14:18:11.2407242Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T14:18:11.7943599Z    Compiling cfg-if v0.1.10
2020-04-28T14:18:11.7944079Z    Compiling libc v0.2.69
---
2020-04-28T14:20:52.8632211Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T14:20:54.4668458Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T14:20:56.0865778Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T14:20:58.0516369Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T14:21:06.4718884Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T14:21:09.5415074Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T14:21:14.0692145Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T14:21:18.4570471Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T14:21:27.7249262Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T14:36:56.1197723Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T14:37:27.9709093Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T14:40:18.3963649Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T14:40:18.9936358Z     Finished release [optimized] target(s) in 22m 07s
2020-04-28T14:40:18.9938576Z {"reason":"build-finished","success":true}
2020-04-28T14:40:19.0472592Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-28T14:40:19.0490583Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T14:40:19.3738833Z    Compiling cc v1.0.50
2020-04-28T14:40:19.3739494Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-28T14:41:10.7157605Z    Compiling unicode-width v0.1.6
2020-04-28T14:41:10.8178190Z    Compiling getopts v0.2.21
2020-04-28T14:41:22.1535515Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T14:41:31.8900688Z     Finished release [optimized] target(s) in 1m 12s
2020-04-28T14:41:31.8905138Z {"reason":"build-finished","success":true}
2020-04-28T14:41:31.9070758Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T14:41:32.5062683Z    Compiling cfg-if v0.1.10
2020-04-28T14:41:32.5065294Z    Compiling libc v0.2.69
2020-04-28T14:41:32.5532225Z    Compiling semver-parser v0.7.0
---
2020-04-28T14:44:24.8056450Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T14:44:26.5016819Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T14:44:28.5063064Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T14:44:29.9677562Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T14:44:39.3998445Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T14:44:43.3051581Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T14:44:48.3696808Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T14:44:53.2450099Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T14:45:02.7928299Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T15:01:17.6176128Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T15:01:50.9180942Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T15:05:07.8711043Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T15:05:08.4905813Z     Finished release [optimized] target(s) in 23m 36s
2020-04-28T15:05:08.4911517Z {"reason":"build-finished","success":true}
2020-04-28T15:05:08.5407462Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-28T15:05:08.5420256Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T15:05:08.5421218Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T15:05:08.5431171Z Building test helpers
---
2020-04-28T15:06:19.6391155Z    Compiling serde_json v1.0.40
2020-04-28T15:06:21.1739456Z    Compiling rustfix v0.5.0
2020-04-28T15:06:24.3654038Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-28T15:06:38.4753027Z     Finished release [optimized] target(s) in 1m 29s
2020-04-28T15:06:38.4754966Z {"reason":"build-finished","success":true}
2020-04-28T15:06:41.8920145Z 
2020-04-28T15:06:41.8920517Z running 9939 tests
2020-04-28T15:06:55.2286270Z .................................................................................................... 100/9939
2020-04-28T15:07:05.2050553Z .................................................................................................... 200/9939
---
2020-04-28T15:08:49.6501929Z .................................................................................................... 1800/9939
2020-04-28T15:08:56.6936628Z .................................................................................................... 1900/9939
2020-04-28T15:09:05.9455293Z ..............i..................................................................................... 2000/9939
2020-04-28T15:09:12.8079565Z .................................................................................................... 2100/9939
2020-04-28T15:09:25.7444596Z ....iiiii........................................................................................... 2200/9939
2020-04-28T15:09:35.2477402Z .................................................................................................... 2400/9939
2020-04-28T15:09:37.8239176Z .................................................................................................... 2500/9939
2020-04-28T15:09:43.7904419Z .................................................................................................... 2600/9939
2020-04-28T15:10:02.7143118Z ..............................................................................................F..... 2700/9939
---
2020-04-28T15:12:44.0721278Z ...i................................................................................................ 5100/9939
2020-04-28T15:12:52.1555047Z .................................................................................................... 5200/9939
2020-04-28T15:12:56.8898166Z .................................i.................................................................. 5300/9939
2020-04-28T15:13:05.7108554Z ........................i........................................................................... 5400/9939
2020-04-28T15:13:12.4969628Z .........................ii.ii........i...i......................................................... 5500/9939
2020-04-28T15:13:20.3475400Z ........................................................................i........................... 5700/9939
2020-04-28T15:13:29.1222572Z .................................................................................................... 5800/9939
2020-04-28T15:13:35.3142619Z .....ii.....................................i....................................................... 5900/9939
2020-04-28T15:13:41.6622387Z .................................................................................................... 6000/9939
2020-04-28T15:13:41.6622387Z .................................................................................................... 6000/9939
2020-04-28T15:13:51.6250424Z .................................................................................................... 6100/9939
2020-04-28T15:13:58.0637138Z ......................................ii...i..ii...........i........................................ 6200/9939
2020-04-28T15:14:17.7208542Z .................................................................................................... 6400/9939
2020-04-28T15:14:24.7117196Z .................................................................................................... 6500/9939
2020-04-28T15:14:24.7117196Z .................................................................................................... 6500/9939
2020-04-28T15:14:34.1592770Z ....................................................................i..ii........................... 6600/9939
2020-04-28T15:15:00.9242238Z .................................................................................................... 6800/9939
2020-04-28T15:15:09.2100712Z .....................................................................i.............................. 6900/9939
2020-04-28T15:15:11.2504138Z .................................................................................................... 7000/9939
2020-04-28T15:15:13.4246712Z .................................................................................................... 7100/9939
---
2020-04-28T15:16:52.1131872Z .................................................................................................... 7900/9939
2020-04-28T15:16:56.7436605Z .................................................................................................... 8000/9939
2020-04-28T15:17:03.1780472Z ...............................................................................i.................... 8100/9939
2020-04-28T15:17:12.7162250Z .................................................................................................... 8200/9939
2020-04-28T15:17:18.4036514Z ............................iiiiii.iiiii.i.......................................................... 8300/9939
2020-04-28T15:17:32.4592750Z .................................................................................................... 8500/9939
2020-04-28T15:17:37.5594417Z .................................................................................................... 8600/9939
2020-04-28T15:17:51.9291669Z .................................................................................................... 8700/9939
2020-04-28T15:17:59.6608555Z .................................................................................................... 8800/9939
---
2020-04-28T15:19:51.9746876Z diff of stderr:
2020-04-28T15:19:51.9747093Z 
2020-04-28T15:19:51.9747431Z 110    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9747746Z 111 
2020-04-28T15:19:51.9748194Z 112 error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9748993Z -   --> $DIR/feature-gate-cfg-version.rs:33:18
2020-04-28T15:19:51.9749643Z +   --> $DIR/feature-gate-cfg-version.rs:30:7
2020-04-28T15:19:51.9750016Z 114    |
2020-04-28T15:19:51.9750298Z + LL | #[cfg(version("1.65536.2"))]
2020-04-28T15:19:51.9751033Z +    |
2020-04-28T15:19:51.9751033Z +    |
2020-04-28T15:19:51.9751858Z +    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9752649Z +    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9753068Z + 
2020-04-28T15:19:51.9753381Z + error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9754482Z +   --> $DIR/feature-gate-cfg-version.rs:39:18
2020-04-28T15:19:51.9754930Z +    |
2020-04-28T15:19:51.9755215Z 115 LL |     assert!(cfg!(version("1.42")));
2020-04-28T15:19:51.9755781Z 117    |
2020-04-28T15:19:51.9756112Z 
2020-04-28T15:19:51.9756112Z 
2020-04-28T15:19:51.9756787Z 118    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9757289Z 119    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9758125Z - error: aborting due to 15 previous errors
2020-04-28T15:19:51.9758522Z + error: aborting due to 16 previous errors
2020-04-28T15:19:51.9759186Z 122 
2020-04-28T15:19:51.9759781Z 123 For more information about this error, try `rustc --explain E0658`.
2020-04-28T15:19:51.9759781Z 123 For more information about this error, try `rustc --explain E0658`.
2020-04-28T15:19:51.9760155Z 124 
2020-04-28T15:19:51.9760349Z 
2020-04-28T15:19:51.9760639Z 
2020-04-28T15:19:51.9760895Z The actual stderr differed from the expected stderr.
2020-04-28T15:19:51.9761660Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-version/feature-gate-cfg-version.stderr
2020-04-28T15:19:51.9762388Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T15:19:51.9763099Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg-version.rs`
2020-04-28T15:19:51.9763692Z error: 1 errors occurred comparing output.
2020-04-28T15:19:51.9763998Z status: exit code: 1
2020-04-28T15:19:51.9763998Z status: exit code: 1
2020-04-28T15:19:51.9765936Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-version" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-version/auxiliary"
2020-04-28T15:19:51.9767627Z ------------------------------------------
2020-04-28T15:19:51.9767926Z 
2020-04-28T15:19:51.9768380Z ------------------------------------------
2020-04-28T15:19:51.9768674Z stderr:
2020-04-28T15:19:51.9768674Z stderr:
2020-04-28T15:19:51.9769145Z ------------------------------------------
2020-04-28T15:19:51.9769521Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9770525Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:1:7
2020-04-28T15:19:51.9771059Z    |
2020-04-28T15:19:51.9771245Z LL | #[cfg(version("1.44"))]
2020-04-28T15:19:51.9771715Z    |
2020-04-28T15:19:51.9771715Z    |
2020-04-28T15:19:51.9772324Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9772782Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9773074Z 
2020-04-28T15:19:51.9773327Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9773994Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:4:11
2020-04-28T15:19:51.9774349Z    |
2020-04-28T15:19:51.9774557Z LL | #[cfg(not(version("1.44")))]
2020-04-28T15:19:51.9775108Z    |
2020-04-28T15:19:51.9775108Z    |
2020-04-28T15:19:51.9775664Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9775965Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9776298Z 
2020-04-28T15:19:51.9776522Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9777149Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:8:7
2020-04-28T15:19:51.9777509Z    |
2020-04-28T15:19:51.9777780Z LL | #[cfg(version("1.43", "1.44", "1.45"))] //~ ERROR: expected single version literal
2020-04-28T15:19:51.9778426Z    |
2020-04-28T15:19:51.9778426Z    |
2020-04-28T15:19:51.9778965Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9779413Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9779859Z error: expected single version literal
2020-04-28T15:19:51.9780463Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:8:7
2020-04-28T15:19:51.9780798Z    |
2020-04-28T15:19:51.9780798Z    |
2020-04-28T15:19:51.9781088Z LL | #[cfg(version("1.43", "1.44", "1.45"))] //~ ERROR: expected single version literal
2020-04-28T15:19:51.9781750Z 
2020-04-28T15:19:51.9781750Z 
2020-04-28T15:19:51.9781990Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9782678Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:11:7
2020-04-28T15:19:51.9783019Z    |
2020-04-28T15:19:51.9783306Z LL | #[cfg(version(false))] //~ ERROR: expected a version literal
2020-04-28T15:19:51.9783785Z    |
2020-04-28T15:19:51.9783785Z    |
2020-04-28T15:19:51.9784350Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9784775Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9785244Z error: expected a version literal
2020-04-28T15:19:51.9785826Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:11:15
2020-04-28T15:19:51.9786191Z    |
2020-04-28T15:19:51.9786191Z    |
2020-04-28T15:19:51.9786435Z LL | #[cfg(version(false))] //~ ERROR: expected a version literal
2020-04-28T15:19:51.9787098Z 
2020-04-28T15:19:51.9787098Z 
2020-04-28T15:19:51.9787317Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9787962Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:14:7
2020-04-28T15:19:51.9788331Z    |
2020-04-28T15:19:51.9788559Z LL | #[cfg(version("foo"))] //~ ERROR: invalid version literal
2020-04-28T15:19:51.9789036Z    |
2020-04-28T15:19:51.9789036Z    |
2020-04-28T15:19:51.9789572Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9790145Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9790612Z error: invalid version literal
2020-04-28T15:19:51.9791379Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:14:15
2020-04-28T15:19:51.9791746Z    |
2020-04-28T15:19:51.9791746Z    |
2020-04-28T15:19:51.9792031Z LL | #[cfg(version("foo"))] //~ ERROR: invalid version literal
2020-04-28T15:19:51.9792526Z 
2020-04-28T15:19:51.9792526Z 
2020-04-28T15:19:51.9792772Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9793455Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:17:7
2020-04-28T15:19:51.9794221Z    |
2020-04-28T15:19:51.9794432Z LL | #[cfg(version("999"))]
2020-04-28T15:19:51.9794885Z    |
2020-04-28T15:19:51.9794885Z    |
2020-04-28T15:19:51.9795601Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9796062Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9796374Z 
2020-04-28T15:19:51.9796604Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9797270Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:20:7
2020-04-28T15:19:51.9797647Z    |
2020-04-28T15:19:51.9798133Z LL | #[cfg(version("-1"))] //~ ERROR: invalid version literal
2020-04-28T15:19:51.9798689Z    |
2020-04-28T15:19:51.9798689Z    |
2020-04-28T15:19:51.9799251Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9799724Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9800192Z error: invalid version literal
2020-04-28T15:19:51.9800727Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:20:15
2020-04-28T15:19:51.9800959Z    |
2020-04-28T15:19:51.9800959Z    |
2020-04-28T15:19:51.9801530Z LL | #[cfg(version("-1"))] //~ ERROR: invalid version literal
2020-04-28T15:19:51.9802059Z 
2020-04-28T15:19:51.9802059Z 
2020-04-28T15:19:51.9802263Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9802942Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:23:7
2020-04-28T15:19:51.9803295Z    |
2020-04-28T15:19:51.9803540Z LL | #[cfg(version("65536"))] //~ ERROR: invalid version literal
2020-04-28T15:19:51.9804221Z    |
2020-04-28T15:19:51.9804221Z    |
2020-04-28T15:19:51.9804837Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9805295Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9805776Z error: invalid version literal
2020-04-28T15:19:51.9806389Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:23:15
2020-04-28T15:19:51.9806749Z    |
2020-04-28T15:19:51.9806749Z    |
2020-04-28T15:19:51.9807018Z LL | #[cfg(version("65536"))] //~ ERROR: invalid version literal
2020-04-28T15:19:51.9807494Z 
2020-04-28T15:19:51.9807494Z 
2020-04-28T15:19:51.9807733Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9808390Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:26:7
2020-04-28T15:19:51.9808749Z    |
2020-04-28T15:19:51.9808953Z LL | #[cfg(version("0"))]
2020-04-28T15:19:51.9809364Z    |
2020-04-28T15:19:51.9809364Z    |
2020-04-28T15:19:51.9810063Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9810541Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9810975Z 
2020-04-28T15:19:51.9811212Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9812035Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:30:7
2020-04-28T15:19:51.9812419Z    |
2020-04-28T15:19:51.9812601Z LL | #[cfg(version("1.65536.2"))]
2020-04-28T15:19:51.9813085Z    |
2020-04-28T15:19:51.9813085Z    |
2020-04-28T15:19:51.9813649Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9814241Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9814662Z 
2020-04-28T15:19:51.9814908Z error[E0658]: `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9815647Z   --> /checkout/src/test/ui/feature-gates/feature-gate-cfg-version.rs:39:18
2020-04-28T15:19:51.9816024Z    |
2020-04-28T15:19:51.9816336Z LL |     assert!(cfg!(version("1.42"))); //~ ERROR `cfg(version)` is experimental and subject to change
2020-04-28T15:19:51.9816936Z    |
2020-04-28T15:19:51.9816936Z    |
2020-04-28T15:19:51.9817558Z    = note: see issue #64796 <***/issues/64796> for more information
2020-04-28T15:19:51.9818033Z    = help: add `#![feature(cfg_version)]` to the crate attributes to enable
2020-04-28T15:19:51.9818670Z error: aborting due to 16 previous errors
2020-04-28T15:19:51.9818917Z 
2020-04-28T15:19:51.9819397Z For more information about this error, try `rustc --explain E0658`.
2020-04-28T15:19:51.9819727Z 
---
2020-04-28T15:19:51.9823151Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-28T15:19:51.9823617Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-28T15:19:51.9823907Z 
2020-04-28T15:19:51.9824040Z 
2020-04-28T15:19:51.9827440Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-28T15:19:51.9830068Z 
2020-04-28T15:19:51.9830160Z 
2020-04-28T15:19:51.9830912Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T15:19:51.9831399Z Build completed unsuccessfully in 1:02:47
2020-04-28T15:19:51.9831399Z Build completed unsuccessfully in 1:02:47
2020-04-28T15:19:51.9831712Z == clock drift check ==
2020-04-28T15:19:51.9831967Z   local time: Tue Apr 28 15:19:51 UTC 2020
2020-04-28T15:19:51.9832319Z   network time: Tue, 28 Apr 2020 15:19:51 GMT
2020-04-28T15:19:51.9832798Z 
2020-04-28T15:19:51.9832798Z 
2020-04-28T15:19:51.9871803Z ##[error]Bash exited with code '1'.
2020-04-28T15:19:51.9894985Z ##[section]Finishing: Run build
2020-04-28T15:19:51.9945321Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71314/merge to s
2020-04-28T15:19:51.9950324Z Task         : Get sources
2020-04-28T15:19:51.9950603Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T15:19:51.9950988Z Version      : 1.0.0
2020-04-28T15:19:51.9951204Z Author       : Microsoft
2020-04-28T15:19:51.9951204Z Author       : Microsoft
2020-04-28T15:19:51.9951637Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T15:19:51.9952119Z ==============================================================================
2020-04-28T15:19:52.3617045Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T15:19:52.3666756Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71314/merge to s
2020-04-28T15:19:52.3757945Z Cleaning up task key
2020-04-28T15:19:52.3759043Z Start cleaning up orphan processes.
2020-04-28T15:19:52.3951299Z Terminate orphan process: pid (3968) (python)
2020-04-28T15:19:52.4175646Z ##[section]Finishing: Finalize Job
