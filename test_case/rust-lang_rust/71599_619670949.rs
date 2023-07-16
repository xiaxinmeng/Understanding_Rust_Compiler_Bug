plain
2020-04-27T01:17:17.6245595Z ========================== Starting Command Output ===========================
2020-04-27T01:17:17.6249136Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9632c393-a9a2-4631-a715-4821a5106e62.sh
2020-04-27T01:17:17.6249556Z 
2020-04-27T01:17:17.6254127Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T01:17:17.6274088Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71599/merge to s
2020-04-27T01:17:17.6276757Z Task         : Get sources
2020-04-27T01:17:17.6276968Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T01:17:17.6277169Z Version      : 1.0.0
2020-04-27T01:17:17.6277349Z Author       : Microsoft
---
2020-04-27T01:17:18.6177138Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T01:17:18.6181705Z ##[command]git config gc.auto 0
2020-04-27T01:17:18.6184619Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T01:17:18.6187321Z ##[command]git config --get-all http.proxy
2020-04-27T01:17:18.6192661Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71599/merge:refs/remotes/pull/71599/merge
---
2020-04-27T01:19:36.8025965Z  ---> cb2676f08729
2020-04-27T01:19:36.8026781Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-27T01:19:36.8029540Z  ---> Using cache
2020-04-27T01:19:36.8030226Z  ---> df25ce111862
2020-04-27T01:19:36.8031770Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-27T01:19:36.8036469Z  ---> 599b9ac96b27
2020-04-27T01:19:36.8036637Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-27T01:19:36.8046273Z  ---> Using cache
2020-04-27T01:19:36.8046595Z  ---> 091087e35a36
---
2020-04-27T01:19:36.8382343Z Looks like docker image is the same as before, not uploading
2020-04-27T01:19:45.0968956Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T01:19:45.1260152Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-27T01:19:45.1296856Z == clock drift check ==
2020-04-27T01:19:45.1297373Z   local time: Mon Apr 27 01:19:45 UTC 2020
2020-04-27T01:19:45.2232756Z   network time: Mon, 27 Apr 2020 01:19:45 GMT
2020-04-27T01:19:45.2255895Z Starting sccache server...
2020-04-27T01:19:45.3039396Z configure: processing command line
2020-04-27T01:19:45.3039771Z configure: 
2020-04-27T01:19:45.3040738Z configure: rust.dist-src        := False
---
2020-04-27T01:21:49.4229584Z    Compiling unicode-width v0.1.6
2020-04-27T01:21:49.4963534Z    Compiling getopts v0.2.21
2020-04-27T01:21:58.5089344Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T01:22:06.0440787Z     Finished release [optimized] target(s) in 54.58s
2020-04-27T01:22:06.0443496Z {"reason":"build-finished","success":true}
2020-04-27T01:22:06.0678823Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:22:06.5430507Z    Compiling cfg-if v0.1.10
2020-04-27T01:22:06.5431208Z    Compiling libc v0.2.69
2020-04-27T01:22:06.5877019Z    Compiling semver-parser v0.7.0
---
2020-04-27T01:24:23.0231591Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T01:24:24.3493631Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T01:24:25.7513940Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T01:24:27.2341633Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T01:24:34.8398407Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T01:24:37.2545412Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T01:24:41.1977823Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T01:24:44.9022832Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T01:24:53.0092754Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T01:38:40.1377865Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T01:38:46.4068790Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T01:41:00.0497058Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T01:41:00.5249213Z     Finished release [optimized] target(s) in 18m 54s
2020-04-27T01:41:00.5250124Z {"reason":"build-finished","success":true}
2020-04-27T01:41:00.5712313Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-27T01:41:00.5725563Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:41:00.8132653Z    Compiling cc v1.0.50
2020-04-27T01:41:00.8133695Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-27T01:41:44.1720424Z    Compiling unicode-width v0.1.6
2020-04-27T01:41:44.2643119Z    Compiling getopts v0.2.21
2020-04-27T01:41:54.5246350Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-27T01:42:02.5992406Z     Finished release [optimized] target(s) in 1m 02s
2020-04-27T01:42:02.5993417Z {"reason":"build-finished","success":true}
2020-04-27T01:42:02.6121372Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T01:42:03.0769638Z    Compiling cfg-if v0.1.10
2020-04-27T01:42:03.0775342Z    Compiling libc v0.2.69
2020-04-27T01:42:03.1257156Z    Compiling semver-parser v0.7.0
---
2020-04-27T01:44:30.7048995Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-27T01:44:32.1081340Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-27T01:44:33.6914850Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-27T01:44:34.8649349Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-27T01:44:43.1659321Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-27T01:44:46.4025230Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-27T01:44:50.5825953Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-27T01:44:54.5350686Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-27T01:45:02.7096876Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-27T01:56:51.0261473Z    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2020-04-27T01:58:11.1931693Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-27T01:58:18.2490080Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-27T02:01:57.2409874Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-27T02:01:57.6989978Z     Finished{"reason":"build-finished","success":true}
2020-04-27T02:01:57.7367910Z Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-27T02:01:57.7406865Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-27T02:01:57.7420610Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T02:01:57.7421401Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-04-27T02:02:34.3471728Z    Compiling serde_derive v1.0.81
2020-04-27T02:02:58.1246829Z    Compiling serde_json v1.0.40
2020-04-27T02:02:59.3410331Z    Compiling rustfix v0.5.0
2020-04-27T02:03:02.0465508Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-27T02:03:14.0681202Z   {"reason":"build-finished","success":true}
2020-04-27T02:03:14.0929141Z Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-27T02:03:18.3918198Z 
2020-04-27T02:03:18.3918917Z running 9927 tests
2020-04-27T02:03:29.4893186Z .................................................................................................... 100/9927
---
2020-04-27T02:04:58.9090326Z .................................................................................................... 1700/9927
2020-04-27T02:05:02.5269689Z .................................................................................................... 1800/9927
2020-04-27T02:05:09.0388671Z .................................................................................................... 1900/9927
2020-04-27T02:05:16.1796706Z ........i........................................................................................... 2000/9927
2020-04-27T02:05:21.4417730Z ..................................................................................................ii 2100/9927
2020-04-27T02:05:32.9132315Z iii................................................................................................. 2200/9927
2020-04-27T02:05:39.9522702Z .................................................................................................... 2400/9927
2020-04-27T02:05:41.7917971Z .................................................................................................... 2500/9927
2020-04-27T02:05:46.3437821Z .................................................................................................... 2600/9927
2020-04-27T02:06:01.7362495Z .................................................................................................... 2700/9927
---
2020-04-27T02:08:16.3948987Z .................................................................................................... 5100/9927
2020-04-27T02:08:22.5973264Z .................................................................................................... 5200/9927
2020-04-27T02:08:26.4718894Z ......................i............................................................................. 5300/9927
2020-04-27T02:08:34.7325004Z .............i...................................................................................... 5400/9927
2020-04-27T02:08:39.5836104Z .............ii.ii........i...i..................................................................... 5500/9927
2020-04-27T02:08:46.0598043Z ............................................................i....................................... 5700/9927
2020-04-27T02:08:53.3692482Z .............................................................................................ii..... 5800/9927
2020-04-27T02:08:59.0129966Z ................................i................................................................... 5900/9927
2020-04-27T02:09:03.7930050Z .................................................................................................... 6000/9927
2020-04-27T02:09:03.7930050Z .................................................................................................... 6000/9927
2020-04-27T02:09:13.0682131Z .................................................................................................... 6100/9927
2020-04-27T02:09:21.6280527Z ..........................ii...i..ii...........i.................................................... 6200/9927
2020-04-27T02:09:35.1881416Z .................................................................................................... 6400/9927
2020-04-27T02:09:40.6362949Z .................................................................................................... 6500/9927
2020-04-27T02:09:40.6362949Z .................................................................................................... 6500/9927
2020-04-27T02:09:50.4638713Z ........................................................i..ii....................................... 6600/9927
2020-04-27T02:10:17.1622430Z .................................................................................................... 6800/9927
2020-04-27T02:10:20.7383949Z .........................................................i.......................................... 6900/9927
2020-04-27T02:10:22.4479104Z .................................................................................................... 7000/9927
2020-04-27T02:10:24.1022973Z ...................................................................................................i 7100/9927
---
2020-04-27T02:11:45.6874158Z .................................................................................................... 7900/9927
2020-04-27T02:11:50.1641180Z .................................................................................................... 8000/9927
2020-04-27T02:11:55.4897360Z ...................................................................i................................ 8100/9927
2020-04-27T02:12:03.6528674Z .................................................................................................... 8200/9927
2020-04-27T02:12:08.2193943Z ................iiiiii.iiiii.i...................................................................... 8300/9927
2020-04-27T02:12:19.5958616Z .................................................................................................... 8500/9927
2020-04-27T02:12:24.3937150Z .................................................................................................... 8600/9927
2020-04-27T02:12:36.5094469Z .................................................................................................... 8700/9927
2020-04-27T02:12:42.2492532Z .................................................................................................... 8800/9927
---
2020-04-27T02:14:13.9461444Z +   --> $DIR/issue-24036.rs:9:15
2020-04-27T02:14:13.9461918Z 16    |
2020-04-27T02:14:13.9462374Z - LL |       let x = match 1usize {
2020-04-27T02:14:13.9463006Z -    |  _____________-
2020-04-27T02:14:13.9463486Z - LL | |         1 => |c| c + 1,
2020-04-27T02:14:13.9464112Z -    | |              --------- this is found to be of type `[closure@$DIR/issue-24036.rs:9:14: 9:23]`
2020-04-27T02:14:13.9464889Z - LL | |         2 => |c| c - 1,
2020-04-27T02:14:13.9465643Z -    | |              ^^^^^^^^^ expected closure, found a different closure
2020-04-27T02:14:13.9466211Z - LL | |         _ => |c| c - 1
2020-04-27T02:14:13.9466841Z - LL | |     };
2020-04-27T02:14:13.9467437Z -    | |_____- `match` arms have incompatible types
2020-04-27T02:14:13.9469086Z -    = note: expected type `[closure@$DIR/issue-24036.rs:9:14: 9:23]`
2020-04-27T02:14:13.9469753Z -            found closure `[closure@$DIR/issue-24036.rs:10:14: 10:23]`
2020-04-27T02:14:13.9470543Z -    = note: no two closures, even if identical, have the same type
2020-04-27T02:14:13.9471345Z -    = help: consider boxing your closure and/or using it as a trait object
2020-04-27T02:14:13.9471345Z -    = help: consider boxing your closure and/or using it as a trait object
2020-04-27T02:14:13.9471746Z + LL |         1 => |c| c + 1,
2020-04-27T02:14:13.9472083Z +    |               ^ consider giving this closure parameter a type
2020-04-27T02:14:13.9472652Z 32 error: aborting due to 2 previous errors
2020-04-27T02:14:13.9472910Z 33 
2020-04-27T02:14:13.9473277Z 
2020-04-27T02:14:13.9473818Z - For more information about this error, try `rustc --explain E0308`.
2020-04-27T02:14:13.9473818Z - For more information about this error, try `rustc --explain E0308`.
2020-04-27T02:14:13.9474577Z + Some errors have detailed explanations: E0282, E0308.
2020-04-27T02:14:13.9475577Z + For more information about an error, try `rustc --explain E0282`.
2020-04-27T02:14:13.9475934Z 35 
2020-04-27T02:14:13.9476136Z 
2020-04-27T02:14:13.9476564Z 
2020-04-27T02:14:13.9476842Z The actual stderr differed from the expected stderr.
2020-04-27T02:14:13.9477525Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24036/issue-24036.stderr
2020-04-27T02:14:13.9478614Z To update references, rerun the tests and pass the `--bless` flag
2020-04-27T02:14:13.9479661Z To only update this specific test, also pass `--test-args issues/issue-24036.rs`
2020-04-27T02:14:13.9480561Z error: 1 errors occurred comparing output.
2020-04-27T02:14:13.9481220Z status: exit code: 1
2020-04-27T02:14:13.9481220Z status: exit code: 1
2020-04-27T02:14:13.9483388Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24036" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24036/auxiliary"
2020-04-27T02:14:13.9487088Z ------------------------------------------
2020-04-27T02:14:13.9487255Z 
2020-04-27T02:14:13.9487559Z ------------------------------------------
2020-04-27T02:14:13.9487854Z stderr:
2020-04-27T02:14:13.9487854Z stderr:
2020-04-27T02:14:13.9488404Z ------------------------------------------
2020-04-27T02:14:13.9488607Z error[E0308]: mismatched types
2020-04-27T02:14:13.9489158Z   --> /checkout/src/test/ui/issues/issue-24036.rs:3:9
2020-04-27T02:14:13.9489335Z    |
2020-04-27T02:14:13.9489668Z LL |     let mut x = |c| c + 1;
2020-04-27T02:14:13.9490047Z    |                 --------- the expected closure
2020-04-27T02:14:13.9490242Z LL |     x = |c| c + 1;
2020-04-27T02:14:13.9490662Z    |
2020-04-27T02:14:13.9490662Z    |
2020-04-27T02:14:13.9491120Z    = note: expected closure `[closure@/checkout/src/test/ui/issues/issue-24036.rs:2:17: 2:26]`
2020-04-27T02:14:13.9491875Z               found closure `[closure@/checkout/src/test/ui/issues/issue-24036.rs:3:9: 3:18]`
2020-04-27T02:14:13.9492189Z    = note: no two closures, even if identical, have the same type
2020-04-27T02:14:13.9492671Z    = help: consider boxing your closure and/or using it as a trait object
2020-04-27T02:14:13.9493008Z error[E0282]: type annotations needed
2020-04-27T02:14:13.9493743Z   --> /checkout/src/test/ui/issues/issue-24036.rs:9:15
2020-04-27T02:14:13.9493941Z    |
2020-04-27T02:14:13.9493941Z    |
2020-04-27T02:14:13.9494322Z LL |         1 => |c| c + 1,
2020-04-27T02:14:13.9494729Z    |               ^ consider giving this closure parameter a type
2020-04-27T02:14:13.9495175Z error: aborting due to 2 previous errors
2020-04-27T02:14:13.9495318Z 
2020-04-27T02:14:13.9495511Z Some errors have detailed explanations: E0282, E0308.
2020-04-27T02:14:13.9496172Z For more information about an error, try `rustc --explain E0282`.
---
2020-04-27T02:14:13.9499433Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-27T02:14:13.9499784Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-27T02:14:13.9507122Z 
2020-04-27T02:14:13.9507485Z 
2020-04-27T02:14:13.9511411Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-27T02:14:13.9514034Z 
2020-04-27T02:14:13.9514261Z 
2020-04-27T02:14:13.9519086Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-27T02:14:13.9519769Z Build completed unsuccessfully in 0:53:03
2020-04-27T02:14:13.9519769Z Build completed unsuccessfully in 0:53:03
2020-04-27T02:14:13.9568069Z == clock drift check ==
2020-04-27T02:14:13.9588802Z   local time: Mon Apr 27 02:14:13 UTC 2020
2020-04-27T02:14:14.2536698Z   network time: Mon, 27 Apr 2020 02:14:14 GMT
2020-04-27T02:14:14.6847796Z 
2020-04-27T02:14:14.6847796Z 
2020-04-27T02:14:14.6925831Z ##[error]Bash exited with code '1'.
2020-04-27T02:14:14.6938968Z ##[section]Finishing: Run build
2020-04-27T02:14:14.6979465Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71599/merge to s
2020-04-27T02:14:14.6983519Z Task         : Get sources
2020-04-27T02:14:14.6983873Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T02:14:14.6984101Z Version      : 1.0.0
2020-04-27T02:14:14.6984261Z Author       : Microsoft
2020-04-27T02:14:14.6984261Z Author       : Microsoft
2020-04-27T02:14:14.6984532Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T02:14:14.6984835Z ==============================================================================
2020-04-27T02:14:14.9907933Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T02:14:14.9945490Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71599/merge to s
2020-04-27T02:14:15.0016113Z Cleaning up task key
2020-04-27T02:14:15.0017619Z Start cleaning up orphan processes.
2020-04-27T02:14:15.0164292Z Terminate orphan process: pid (4078) (python)
2020-04-27T02:14:15.0305842Z ##[section]Finishing: Finalize Job
