plain
2020-04-26T07:46:22.2413613Z ========================== Starting Command Output ===========================
2020-04-26T07:46:22.2415753Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fcaa8ae0-25fd-4f76-8c5e-2817bc74d49e.sh
2020-04-26T07:46:22.2415935Z 
2020-04-26T07:46:22.2419023Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-26T07:46:22.2432839Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71018/merge to s
2020-04-26T07:46:22.2435289Z Task         : Get sources
2020-04-26T07:46:22.2435661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-26T07:46:22.2435858Z Version      : 1.0.0
2020-04-26T07:46:22.2435991Z Author       : Microsoft
---
2020-04-26T07:46:23.5663847Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-26T07:46:23.5668954Z ##[command]git config gc.auto 0
2020-04-26T07:46:23.5673255Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-26T07:46:23.5679615Z ##[command]git config --get-all http.proxy
2020-04-26T07:46:23.5686944Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71018/merge:refs/remotes/pull/71018/merge
---
2020-04-26T07:48:13.6726481Z  ---> cb2676f08729
2020-04-26T07:48:13.6727071Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-26T07:48:13.6727500Z  ---> Using cache
2020-04-26T07:48:13.6727734Z  ---> df25ce111862
2020-04-26T07:48:13.6728424Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-26T07:48:13.6768916Z  ---> 599b9ac96b27
2020-04-26T07:48:13.6769099Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-26T07:48:13.6769394Z  ---> Using cache
2020-04-26T07:48:13.6769830Z  ---> 091087e35a36
---
2020-04-26T07:48:13.7028665Z Looks like docker image is the same as before, not uploading
2020-04-26T07:48:21.5317821Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-26T07:48:21.5620106Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-26T07:48:21.5641649Z == clock drift check ==
2020-04-26T07:48:21.5648923Z   local time: Sun Apr 26 07:48:21 UTC 2020
2020-04-26T07:48:21.7076867Z   network time: Sun, 26 Apr 2020 07:48:21 GMT
2020-04-26T07:48:21.7099874Z Starting sccache server...
2020-04-26T07:48:21.7747450Z configure: processing command line
2020-04-26T07:48:21.7747658Z configure: 
2020-04-26T07:48:21.7748425Z configure: rust.dist-src        := False
---
2020-04-26T07:50:26.5095753Z    Compiling unicode-width v0.1.6
2020-04-26T07:50:26.5899884Z    Compiling getopts v0.2.21
2020-04-26T07:50:35.3318624Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-26T07:50:42.4787143Z     Finished release [optimized] target(s) in 53.53s
2020-04-26T07:50:42.4788119Z {"reason":"build-finished","success":true}
2020-04-26T07:50:42.5037448Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-26T07:50:42.9668561Z    Compiling cfg-if v0.1.10
2020-04-26T07:50:42.9669036Z    Compiling libc v0.2.69
2020-04-26T07:50:43.0094887Z    Compiling semver-parser v0.7.0
---
2020-04-26T07:52:54.7425807Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-26T07:52:56.0707343Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-26T07:52:57.4776933Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-26T07:52:58.7374495Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-26T07:53:06.2115819Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-26T07:53:08.4074130Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-26T07:53:12.2266418Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-26T07:53:15.7236238Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-26T07:53:24.5216097Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-26T08:05:11.3262267Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-26T08:05:37.9074844Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-26T08:08:59.3325336Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-26T08:08:59.7891380Z     Finished release [optimized] target(s) in 18m 17s
2020-04-26T08:08:59.7892335Z {"reason":"build-finished","success":true}
2020-04-26T08:08:59.8299729Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-26T08:08:59.8317268Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-26T08:09:00.0720933Z    Compiling cc v1.0.50
2020-04-26T08:09:00.0721849Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-26T08:09:43.8214655Z    Compiling unicode-width v0.1.6
2020-04-26T08:09:43.8961120Z    Compiling getopts v0.2.21
2020-04-26T08:09:54.2216787Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-26T08:10:02.3266476Z     Finished release [optimized] target(s) in 1m 02s
2020-04-26T08:10:02.3271654Z {"reason":"build-finished","success":true}
2020-04-26T08:10:02.3371135Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-26T08:10:02.8036299Z    Compiling cfg-if v0.1.10
2020-04-26T08:10:02.8036786Z    Compiling libc v0.2.69
2020-04-26T08:10:02.8410333Z    Compiling semver-parser v0.7.0
---
2020-04-26T08:12:25.3468387Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-26T08:12:26.7231213Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-26T08:12:28.2412784Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-26T08:12:30.5477518Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-26T08:12:37.3173735Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-26T08:12:41.8022127Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-26T08:12:45.8960291Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-26T08:12:49.9164730Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-26T08:12:55.9005957Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-26T08:26:10.6227240Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-26T08:26:38.1234860Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-26T08:29:13.7608629Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-26T08:29:14.2133254Z     Finished release [optimized] target(s) in 19m 11s
2020-04-26T08:29:14.2136037Z {"reason":"build-finished","success":true}
2020-04-26T08:29:14.2545720Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-26T08:29:14.2556244Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-26T08:29:14.2557108Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-26T08:29:14.2564852Z Building test helpers
---
2020-04-26T08:30:12.1925903Z    Compiling serde_json v1.0.40
2020-04-26T08:30:13.3447608Z    Compiling rustfix v0.5.0
2020-04-26T08:30:15.9690142Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-26T08:30:27.6375882Z     Finished release [optimized] target(s) in 1m 13s
2020-04-26T08:30:27.6379490Z {"reason":"build-finished","success":true}
2020-04-26T08:30:30.6249444Z 
2020-04-26T08:30:30.6250117Z running 9927 tests
2020-04-26T08:30:41.6595137Z .................................................................................................... 100/9927
2020-04-26T08:30:49.7943220Z .................................................................................................... 200/9927
---
2020-04-26T08:32:09.1808116Z .................................................................................................... 1700/9927
2020-04-26T08:32:12.9066276Z .................................................................................................... 1800/9927
2020-04-26T08:32:19.4459142Z .................................................................................................... 1900/9927
2020-04-26T08:32:26.7782465Z .........i.......................................................................................... 2000/9927
2020-04-26T08:32:32.1883846Z ...................................................................................................i 2100/9927
2020-04-26T08:32:43.3319380Z iiii................................................................................................ 2200/9927
2020-04-26T08:32:50.3166722Z .................................................................................................... 2400/9927
2020-04-26T08:32:52.1221547Z .................................................................................................... 2500/9927
2020-04-26T08:32:56.6664883Z .................................................................................................... 2600/9927
2020-04-26T08:33:11.8475279Z .................................................................................................... 2700/9927
---
2020-04-26T08:35:20.9313121Z .................................................................................................... 5100/9927
2020-04-26T08:35:26.9130842Z .................................................................................................... 5200/9927
2020-04-26T08:35:30.9225291Z ......................i............................................................................. 5300/9927
2020-04-26T08:35:39.2971744Z .............i...................................................................................... 5400/9927
2020-04-26T08:35:44.1247969Z .............ii.ii........i...i..................................................................... 5500/9927
2020-04-26T08:35:50.8725689Z ............................................................i....................................... 5700/9927
2020-04-26T08:35:58.0947950Z .............................................................................................ii..... 5800/9927
2020-04-26T08:36:03.9371634Z ................................i................................................................... 5900/9927
2020-04-26T08:36:08.6637586Z .................................................................................................... 6000/9927
2020-04-26T08:36:08.6637586Z .................................................................................................... 6000/9927
2020-04-26T08:36:17.4091712Z .................................................................................................... 6100/9927
2020-04-26T08:36:25.7150934Z ..........................ii...i..ii...........i.................................................... 6200/9927
2020-04-26T08:36:38.7197213Z .................................................................................................... 6400/9927
2020-04-26T08:36:44.0652752Z .................................................................................................... 6500/9927
2020-04-26T08:36:44.0652752Z .................................................................................................... 6500/9927
2020-04-26T08:36:53.7222193Z ........................................................i..ii....................................... 6600/9927
2020-04-26T08:37:18.4537409Z .................................................................................................... 6800/9927
2020-04-26T08:37:21.9220319Z .........................................................i.......................................... 6900/9927
2020-04-26T08:37:23.6194563Z .................................................................................................... 7000/9927
2020-04-26T08:37:25.1996966Z ...................................................................................................i 7100/9927
---
2020-04-26T08:38:45.8058562Z .................................................................................................... 7900/9927
2020-04-26T08:38:50.0739210Z .................................................................................................... 8000/9927
2020-04-26T08:38:55.2206248Z ...................................................................i................................ 8100/9927
2020-04-26T08:39:03.7552891Z .................................................................................................... 8200/9927
2020-04-26T08:39:08.0239154Z ................iiiiii.iiiii.i...................................................................... 8300/9927
2020-04-26T08:39:19.1687669Z .................................................................................................... 8500/9927
2020-04-26T08:39:23.9169734Z .................................................................................................... 8600/9927
2020-04-26T08:39:36.1022453Z .................................................................................................... 8700/9927
2020-04-26T08:39:42.1303218Z .................................................................................................... 8800/9927
---
2020-04-26T08:41:16.5924522Z 
2020-04-26T08:41:16.5925463Z ---- [ui] ui/const-generics/different_byref.rs stdout ----
2020-04-26T08:41:16.5925674Z diff of stderr:
2020-04-26T08:41:16.5926980Z 
2020-04-26T08:41:16.5927181Z 12 LL |     x = Const::<{ [4] }> {};
2020-04-26T08:41:16.5927534Z 13    |         ^^^^^^^^^^^^^^^^^^^ expected `3usize`, found `4usize`
2020-04-26T08:41:16.5927863Z 14    |
2020-04-26T08:41:16.5929063Z -    = note: expected struct `Const<ByRef { alloc: Allocation { bytes: [3, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: [usize; 1]>`
2020-04-26T08:41:16.5930685Z -               found struct `Const<ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: [usize; 1]>`
2020-04-26T08:41:16.5931449Z +    = note: expected struct `Const<[3usize]>`
2020-04-26T08:41:16.5931770Z +               found struct `Const<[4usize]>`
2020-04-26T08:41:16.5932455Z - error: aborting due to previous error
2020-04-26T08:41:16.5932843Z + error: aborting due to previous error; 1 warning emitted
2020-04-26T08:41:16.5933107Z 19 
2020-04-26T08:41:16.5933639Z 20 For more information about this error, try `rustc --explain E0308`.
2020-04-26T08:41:16.5933639Z 20 For more information about this error, try `rustc --explain E0308`.
2020-04-26T08:41:16.5933989Z 21 
2020-04-26T08:41:16.5934171Z 
2020-04-26T08:41:16.5934342Z 
2020-04-26T08:41:16.5934614Z The actual stderr differed from the expected stderr.
2020-04-26T08:41:16.5935281Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/different_byref/different_byref.stderr
2020-04-26T08:41:16.5935931Z To update references, rerun the tests and pass the `--bless` flag
2020-04-26T08:41:16.5936580Z To only update this specific test, also pass `--test-args const-generics/different_byref.rs`
2020-04-26T08:41:16.5937137Z error: 1 errors occurred comparing output.
2020-04-26T08:41:16.5937430Z status: exit code: 1
2020-04-26T08:41:16.5937430Z status: exit code: 1
2020-04-26T08:41:16.5938973Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/different_byref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/different_byref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/different_byref/auxiliary"
2020-04-26T08:41:16.5940998Z ------------------------------------------
2020-04-26T08:41:16.5941322Z 
2020-04-26T08:41:16.5941813Z ------------------------------------------
2020-04-26T08:41:16.5941963Z stderr:
---
2020-04-26T08:41:16.5944093Z 
2020-04-26T08:41:16.5944210Z error[E0308]: mismatched types
2020-04-26T08:41:16.5944598Z   --> /checkout/src/test/ui/const-generics/different_byref.rs:8:9
2020-04-26T08:41:16.5944765Z    |
2020-04-26T08:41:16.5944893Z LL |     x = Const::<{ [4] }> {};
2020-04-26T08:41:16.5945107Z    |         ^^^^^^^^^^^^^^^^^^^ expected `3usize`, found `4usize`
2020-04-26T08:41:16.5945263Z    |
2020-04-26T08:41:16.5945400Z    = note: expected struct `Const<[3usize]>`
2020-04-26T08:41:16.5945598Z               found struct `Const<[4usize]>`
2020-04-26T08:41:16.5945855Z error: aborting due to previous error; 1 warning emitted
2020-04-26T08:41:16.5945993Z 
2020-04-26T08:41:16.5946386Z For more information about this error, try `rustc --explain E0308`.
2020-04-26T08:41:16.5946530Z 
---
2020-04-26T08:41:16.5948313Z 9 
2020-04-26T08:41:16.5948382Z 
2020-04-26T08:41:16.5948448Z 
2020-04-26T08:41:16.5948587Z The actual stderr differed from the expected stderr.
2020-04-26T08:41:16.5949094Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68615-adt/issue-68615-adt.stderr
2020-04-26T08:41:16.5949548Z To update references, rerun the tests and pass the `--bless` flag
2020-04-26T08:41:16.5949982Z To only update this specific test, also pass `--test-args const-generics/issues/issue-68615-adt.rs`
2020-04-26T08:41:16.5950306Z error: 1 errors occurred comparing output.
2020-04-26T08:41:16.5950464Z status: exit code: 0
2020-04-26T08:41:16.5950464Z status: exit code: 0
2020-04-26T08:41:16.5951888Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-68615-adt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68615-adt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68615-adt/auxiliary"
2020-04-26T08:41:16.5952996Z ------------------------------------------
2020-04-26T08:41:16.5953113Z 
2020-04-26T08:41:16.5953372Z ------------------------------------------
2020-04-26T08:41:16.5953509Z stderr:
---
2020-04-26T08:41:16.5961462Z 9 
2020-04-26T08:41:16.5961530Z 
2020-04-26T08:41:16.5961612Z 
2020-04-26T08:41:16.5961751Z The actual stderr differed from the expected stderr.
2020-04-26T08:41:16.5962308Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68615-array/issue-68615-array.stderr
2020-04-26T08:41:16.5962781Z To update references, rerun the tests and pass the `--bless` flag
2020-04-26T08:41:16.5963215Z To only update this specific test, also pass `--test-args const-generics/issues/issue-68615-array.rs`
2020-04-26T08:41:16.5964256Z error: 1 errors occurred comparing output.
2020-04-26T08:41:16.5964432Z status: exit code: 0
2020-04-26T08:41:16.5964432Z status: exit code: 0
2020-04-26T08:41:16.5965972Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-68615-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68615-array" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-68615-array/auxiliary"
2020-04-26T08:41:16.5967264Z ------------------------------------------
2020-04-26T08:41:16.5967404Z 
2020-04-26T08:41:16.5967660Z ------------------------------------------
2020-04-26T08:41:16.5967797Z stderr:
---
2020-04-26T08:41:16.5995245Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-26T08:41:16.5995557Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-26T08:41:16.5995742Z 
2020-04-26T08:41:16.5995834Z 
2020-04-26T08:41:16.5998718Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-26T08:41:16.6004815Z 
2020-04-26T08:41:16.6004884Z 
2020-04-26T08:41:16.6005420Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-26T08:41:16.6005676Z Build completed unsuccessfully in 0:51:29
2020-04-26T08:41:16.6005676Z Build completed unsuccessfully in 0:51:29
2020-04-26T08:41:16.6052655Z == clock drift check ==
2020-04-26T08:41:16.6074030Z   local time: Sun Apr 26 08:41:16 UTC 2020
2020-04-26T08:41:16.6425784Z   network time: Sun, 26 Apr 2020 08:41:16 GMT
2020-04-26T08:41:17.2715830Z 
2020-04-26T08:41:17.2715830Z 
2020-04-26T08:41:17.2798443Z ##[error]Bash exited with code '1'.
2020-04-26T08:41:17.2815204Z ##[section]Finishing: Run build
2020-04-26T08:41:17.2857540Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71018/merge to s
2020-04-26T08:41:17.2861747Z Task         : Get sources
2020-04-26T08:41:17.2862011Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-26T08:41:17.2862239Z Version      : 1.0.0
2020-04-26T08:41:17.2862422Z Author       : Microsoft
2020-04-26T08:41:17.2862422Z Author       : Microsoft
2020-04-26T08:41:17.2862685Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-26T08:41:17.2862976Z ==============================================================================
2020-04-26T08:41:17.5939122Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-26T08:41:17.5983966Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71018/merge to s
2020-04-26T08:41:17.6091293Z Cleaning up task key
2020-04-26T08:41:17.6092313Z Start cleaning up orphan processes.
2020-04-26T08:41:17.6251352Z Terminate orphan process: pid (3571) (python)
2020-04-26T08:41:17.6451155Z ##[section]Finishing: Finalize Job
