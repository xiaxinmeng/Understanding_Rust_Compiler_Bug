plain
2020-04-28T07:35:17.7644275Z ========================== Starting Command Output ===========================
2020-04-28T07:35:17.7647420Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dad84218-4bd0-4560-81d6-8976f6c0aa1e.sh
2020-04-28T07:35:17.7647797Z 
2020-04-28T07:35:17.7652079Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T07:35:17.7670028Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71631/merge to s
2020-04-28T07:35:17.7673173Z Task         : Get sources
2020-04-28T07:35:17.7673456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T07:35:17.7673734Z Version      : 1.0.0
2020-04-28T07:35:17.7673924Z Author       : Microsoft
---
2020-04-28T07:35:19.0049083Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T07:35:19.0057202Z ##[command]git config gc.auto 0
2020-04-28T07:35:19.0062380Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T07:35:19.0067271Z ##[command]git config --get-all http.proxy
2020-04-28T07:35:19.0077218Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71631/merge:refs/remotes/pull/71631/merge
---
2020-04-28T07:37:47.0342167Z  ---> cb2676f08729
2020-04-28T07:37:47.0343032Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T07:37:47.0353823Z  ---> Using cache
2020-04-28T07:37:47.0354386Z  ---> df25ce111862
2020-04-28T07:37:47.0355463Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T07:37:47.0368887Z  ---> 599b9ac96b27
2020-04-28T07:37:47.0369252Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T07:37:47.0375410Z  ---> Using cache
2020-04-28T07:37:47.0375963Z  ---> 091087e35a36
---
2020-04-28T07:37:47.0889543Z Looks like docker image is the same as before, not uploading
2020-04-28T07:37:53.6427990Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T07:37:53.6855381Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T07:37:53.6885658Z == clock drift check ==
2020-04-28T07:37:53.6894930Z   local time: Tue Apr 28 07:37:53 UTC 2020
2020-04-28T07:37:53.8415298Z   network time: Tue, 28 Apr 2020 07:37:53 GMT
2020-04-28T07:37:53.8440738Z Starting sccache server...
2020-04-28T07:37:53.9280521Z configure: processing command line
2020-04-28T07:37:53.9294437Z configure: 
2020-04-28T07:37:53.9301505Z configure: rust.dist-src        := False
---
2020-04-28T07:40:19.3259154Z    Compiling unicode-width v0.1.6
2020-04-28T07:40:19.4176892Z    Compiling getopts v0.2.21
2020-04-28T07:40:29.9387989Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T07:40:38.2527110Z     Finished release [optimized] target(s) in 1m 00s
2020-04-28T07:40:38.2531982Z {"reason":"build-finished","success":true}
2020-04-28T07:40:38.2790830Z Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T07:40:38.8780279Z    Compiling cfg-if v0.1.10
2020-04-28T07:40:38.8782244Z    Compiling libc v0.2.69
2020-04-28T07:40:38.9209936Z    Compiling semver-parser v0.7.0
---
2020-04-28T07:43:08.3458106Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T07:43:09.7742610Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T07:43:11.3211648Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T07:43:12.2348242Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T07:43:20.7850011Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T07:43:22.9408462Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T07:43:27.2048313Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T07:43:31.2381456Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T07:43:40.5720482Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T07:57:32.6402890Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T07:58:02.0186855Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T08:01:25.5893542Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T08:01:26.3002952Z     Finished release [optimized] target(s) in 20m 48s
2020-04-28T08:01:26.3007928Z {"reason":"build-finished","success":true}
2020-04-28T08:01:26.3602670Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-28T08:01:26.3622003Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T08:01:26.6665605Z    Compiling cc v1.0.50
2020-04-28T08:01:26.6677628Z    Compiling core v0.0.0 (/checkout/src/libcore)
---
2020-04-28T08:02:10.9727986Z    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
2020-04-28T08:02:15.4060561Z    Compiling unicode-width v0.1.6
2020-04-28T08:02:15.5013282Z    Compiling getopts v0.2.21
2020-04-28T08:02:27.3854686Z    Compiling test v0.0.0 (/checkout/src/libtest)
2020-04-28T08:02:36.8374605Z {"reason":"build-finished","success":true}
2020-04-28T08:02:36.8497111Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T08:02:36.8509384Z Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T08:02:37.4707871Z    Compiling cfg-if v0.1.10
2020-04-28T08:02:37.4708380Z    Compiling libc v0.2.69
---
2020-04-28T08:05:25.1934544Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T08:05:26.8184820Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T08:05:28.6735153Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T08:05:28.7617313Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T08:05:39.4212847Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T08:05:41.5023767Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T08:05:46.2720257Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T08:05:50.8690264Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T08:06:02.1047937Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T08:22:33.0361418Z    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-28T08:22:38.8148732Z    Compiling rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-28T08:25:17.3816458Z    Compiling rustc-main v0.0.0 (/checkout/src/rustc)
2020-04-28T08:25:18.0584164Z     Finished release [optimized] target(s) in 22m 41s
2020-04-28T08:25:18.0585013Z {"reason":"build-finished","success":true}
2020-04-28T08:25:18.1123088Z Assembling stage2 compiler (x86_64-unknown-linux-gnu)
2020-04-28T08:25:18.1135666Z Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-28T08:25:18.1136613Z Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-28T08:25:18.1143578Z Building test helpers
---
2020-04-28T08:26:26.2520922Z    Compiling serde_json v1.0.40
2020-04-28T08:26:27.6828845Z    Compiling rustfix v0.5.0
2020-04-28T08:26:30.6926437Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2020-04-28T08:26:44.6669851Z     Finished release [optimized] target(s) in 1m 26s
2020-04-28T08:26:44.6670465Z {"reason":"build-finished","success":true}
2020-04-28T08:26:48.0301206Z 
2020-04-28T08:26:48.0301523Z running 9928 tests
2020-04-28T08:27:01.3910802Z .................................................................................................... 100/9928
2020-04-28T08:27:11.2177130Z .................................................................................................... 200/9928
---
2020-04-28T08:28:36.6231065Z .................................................................................................... 1500/9928
2020-04-28T08:28:41.6551390Z .................................................................................................... 1600/9928
2020-04-28T08:28:48.1554049Z .................................................................................................... 1700/9928
2020-04-28T08:28:52.7471830Z .................................................................................................... 1800/9928
2020-04-28T08:29:00.5719534Z ......F..FFF...FFF...F.............................................................................. 1900/9928
2020-04-28T08:29:15.8237988Z ...................................................................................................i 2100/9928
2020-04-28T08:29:15.8237988Z ...................................................................................................i 2100/9928
2020-04-28T08:29:28.9808061Z iiii................................................................................................ 2200/9928
2020-04-28T08:29:37.8338389Z .................................................................................................... 2400/9928
2020-04-28T08:29:40.2583276Z .................................................................................................... 2500/9928
2020-04-28T08:29:45.9162046Z .................................................................................................... 2600/9928
2020-04-28T08:30:04.0996990Z .................................................................................................... 2700/9928
---
2020-04-28T08:32:43.0092651Z .................................................................................................... 5100/9928
2020-04-28T08:32:50.0582397Z .................................................................................................... 5200/9928
2020-04-28T08:32:54.6517898Z ......................i............................................................................. 5300/9928
2020-04-28T08:33:04.2368409Z .............i...................................................................................... 5400/9928
2020-04-28T08:33:09.8830349Z ..............ii.ii........i...i.................................................................... 5500/9928
2020-04-28T08:33:17.8110213Z .............................................................i...................................... 5700/9928
2020-04-28T08:33:26.6574141Z ..............................................................................................ii.... 5800/9928
2020-04-28T08:33:33.7106033Z .................................i.................................................................. 5900/9928
2020-04-28T08:33:39.3664388Z .................................................................................................... 6000/9928
2020-04-28T08:33:39.3664388Z .................................................................................................... 6000/9928
2020-04-28T08:33:49.6952400Z .................................................................................................... 6100/9928
2020-04-28T08:34:00.4131105Z ...........................ii...i..ii...........i................................................... 6200/9928
2020-04-28T08:34:17.1778485Z .................................................................................................... 6400/9928
2020-04-28T08:34:21.0281844Z .................................................................................................... 6500/9928
2020-04-28T08:34:21.0281844Z .................................................................................................... 6500/9928
2020-04-28T08:34:27.5353195Z .........................................................i..ii...................................... 6600/9928
2020-04-28T08:34:51.3566996Z .................................................................................................... 6800/9928
2020-04-28T08:34:55.7354452Z ..........................................................i......................................... 6900/9928
2020-04-28T08:34:57.9089405Z .................................................................................................... 7000/9928
2020-04-28T08:35:00.1129857Z .................................................................................................... 7100/9928
---
2020-04-28T08:36:37.0580235Z .................................................................................................... 7900/9928
2020-04-28T08:36:42.4473375Z .................................................................................................... 8000/9928
2020-04-28T08:36:48.6613685Z ....................................................................i............................... 8100/9928
2020-04-28T08:36:58.4259885Z .................................................................................................... 8200/9928
2020-04-28T08:37:03.7028555Z .................iiiiii.iiiii.i..................................................................... 8300/9928
2020-04-28T08:37:17.4648259Z .................................................................................................... 8500/9928
2020-04-28T08:37:22.5921509Z .................................................................................................... 8600/9928
2020-04-28T08:37:36.9409240Z .................................................................................................... 8700/9928
2020-04-28T08:37:43.8269631Z .................................................................................................... 8800/9928
---
2020-04-28T08:39:32.5327476Z -   --> $DIR/abi-mismatch.rs:10:5
2020-04-28T08:39:32.5328033Z + error[E0658]: `const extern fn` definitions are unstable
2020-04-28T08:39:32.5328632Z +   --> $DIR/abi-mismatch.rs:6:1
2020-04-28T08:39:32.5328931Z 3    |
2020-04-28T08:39:32.5329345Z - LL |     my_fn();
2020-04-28T08:39:32.5330209Z - 
2020-04-28T08:39:32.5330666Z - warning: skipping const checks
2020-04-28T08:39:32.5331203Z -   --> $DIR/abi-mismatch.rs:17:40
2020-04-28T08:39:32.5331203Z -   --> $DIR/abi-mismatch.rs:17:40
2020-04-28T08:39:32.5331545Z + LL | const extern "C" fn c_fn() {}
2020-04-28T08:39:32.5332166Z 9    |
2020-04-28T08:39:32.5332166Z 9    |
2020-04-28T08:39:32.5332788Z - LL | static VAL: () = call_rust_fn(unsafe { std::mem::transmute(c_fn as extern "C" fn()) });
2020-04-28T08:39:32.5333554Z -    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-04-28T08:39:32.5334421Z +    = note: see issue #64926 <***/issues/64926> for more information
2020-04-28T08:39:32.5334936Z +    = help: add `#![feature(const_extern_fn)]` to the crate attributes to enable
2020-04-28T08:39:32.5335788Z - error[E0080]: could not evaluate static initializer
2020-04-28T08:39:32.5336357Z -   --> $DIR/abi-mismatch.rs:10:5
2020-04-28T08:39:32.5336801Z -    |
2020-04-28T08:39:32.5336801Z -    |
2020-04-28T08:39:32.5337471Z - LL |     my_fn();
2020-04-28T08:39:32.5338890Z -    |     |
2020-04-28T08:39:32.5338890Z -    |     |
2020-04-28T08:39:32.5339330Z -    |     calling a function with ABI C using caller ABI Rust
2020-04-28T08:39:32.5339882Z -    |     inside `call_rust_fn` at $DIR/abi-mismatch.rs:10:5
2020-04-28T08:39:32.5340268Z - ...
2020-04-28T08:39:32.5341169Z - LL | static VAL: () = call_rust_fn(unsafe { std::mem::transmute(c_fn as extern "C" fn()) });
2020-04-28T08:39:32.5342144Z -    |                  --------------------------------------------------------------------- inside `VAL` at $DIR/abi-mismatch.rs:17:18
2020-04-28T08:39:32.5343094Z 24 
2020-04-28T08:39:32.5343499Z - error: aborting due to previous error; 2 warnings emitted
2020-04-28T08:39:32.5343836Z - 
2020-04-28T08:39:32.5344261Z - For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5344261Z - For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5344770Z + For more information about this error, try `rustc --explain E0658`.
2020-04-28T08:39:32.5344985Z 28 
2020-04-28T08:39:32.5345094Z 
2020-04-28T08:39:32.5345184Z 
2020-04-28T08:39:32.5345371Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5346001Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/abi-mismatch/abi-mismatch.stderr
2020-04-28T08:39:32.5346605Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5347714Z To only update this specific test, also pass `--test-args consts/miri_unleashed/abi-mismatch.rs`
2020-04-28T08:39:32.5348327Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5348542Z status: exit code: 1
2020-04-28T08:39:32.5348542Z status: exit code: 1
2020-04-28T08:39:32.5351172Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/abi-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/abi-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/abi-mismatch/auxiliary"
2020-04-28T08:39:32.5352938Z ------------------------------------------
2020-04-28T08:39:32.5353096Z 
2020-04-28T08:39:32.5353443Z ------------------------------------------
2020-04-28T08:39:32.5353627Z stderr:
2020-04-28T08:39:32.5353627Z stderr:
2020-04-28T08:39:32.5354000Z ------------------------------------------
2020-04-28T08:39:32.5354636Z error[E0658]: `const extern fn` definitions are unstable
2020-04-28T08:39:32.5355353Z   --> /checkout/src/test/ui/consts/miri_unleashed/abi-mismatch.rs:6:1
2020-04-28T08:39:32.5355602Z    |
2020-04-28T08:39:32.5355769Z LL | const extern "C" fn c_fn() {}
2020-04-28T08:39:32.5356416Z    |
2020-04-28T08:39:32.5356416Z    |
2020-04-28T08:39:32.5356998Z    = note: see issue #64926 <***/issues/64926> for more information
2020-04-28T08:39:32.5357339Z    = help: add `#![feature(const_extern_fn)]` to the crate attributes to enable
2020-04-28T08:39:32.5357743Z error: aborting due to previous error
2020-04-28T08:39:32.5357889Z 
2020-04-28T08:39:32.5358315Z For more information about this error, try `rustc --explain E0658`.
2020-04-28T08:39:32.5358506Z 
2020-04-28T08:39:32.5358506Z 
2020-04-28T08:39:32.5359160Z ------------------------------------------
2020-04-28T08:39:32.5359317Z 
2020-04-28T08:39:32.5359409Z 
2020-04-28T08:39:32.5359820Z ---- [ui] ui/consts/miri_unleashed/box.rs stdout ----
2020-04-28T08:39:32.5360042Z diff of stderr:
2020-04-28T08:39:32.5360163Z 
2020-04-28T08:39:32.5360322Z 4 LL |     &mut *(box 0)
2020-04-28T08:39:32.5360648Z 6 
2020-04-28T08:39:32.5360815Z + warning: skipping const checks
2020-04-28T08:39:32.5361203Z +   --> $DIR/box.rs:10:16
2020-04-28T08:39:32.5361364Z +    |
2020-04-28T08:39:32.5361364Z +    |
2020-04-28T08:39:32.5361518Z + LL |     &mut *(box 0)
2020-04-28T08:39:32.5361852Z + 
2020-04-28T08:39:32.5362015Z + warning: skipping const checks
2020-04-28T08:39:32.5362404Z +   --> $DIR/box.rs:10:5
2020-04-28T08:39:32.5362562Z +    |
2020-04-28T08:39:32.5362562Z +    |
2020-04-28T08:39:32.5362713Z + LL |     &mut *(box 0)
2020-04-28T08:39:32.5363048Z + 
2020-04-28T08:39:32.5363212Z + warning: skipping const checks
2020-04-28T08:39:32.5363573Z +   --> $DIR/box.rs:10:5
2020-04-28T08:39:32.5363746Z +    |
2020-04-28T08:39:32.5363746Z +    |
2020-04-28T08:39:32.5363905Z + LL |     &mut *(box 0)
2020-04-28T08:39:32.5364241Z + 
2020-04-28T08:39:32.5364441Z 7 error[E0080]: could not evaluate static initializer
2020-04-28T08:39:32.5364836Z 8   --> $DIR/box.rs:10:11
2020-04-28T08:39:32.5365009Z 9    |
2020-04-28T08:39:32.5365009Z 9    |
2020-04-28T08:39:32.5365108Z 
2020-04-28T08:39:32.5365252Z 10 LL |     &mut *(box 0)
2020-04-28T08:39:32.5365565Z 11    |           ^^^^^^^ "heap allocations via `box` keyword" needs an rfc before being allowed inside constants
2020-04-28T08:39:32.5366259Z - error: aborting due to previous error; 1 warning emitted
2020-04-28T08:39:32.5366556Z + error: aborting due to previous error; 4 warnings emitted
2020-04-28T08:39:32.5366770Z 14 
2020-04-28T08:39:32.5367190Z 15 For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5367190Z 15 For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5367409Z 16 
2020-04-28T08:39:32.5367518Z 
2020-04-28T08:39:32.5367607Z 
2020-04-28T08:39:32.5367793Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5368387Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/box/box.stderr
2020-04-28T08:39:32.5369588Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5370157Z To only update this specific test, also pass `--test-args consts/miri_unleashed/box.rs`
2020-04-28T08:39:32.5370779Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5370992Z status: exit code: 1
2020-04-28T08:39:32.5370992Z status: exit code: 1
2020-04-28T08:39:32.5372992Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/box" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/box/auxiliary"
2020-04-28T08:39:32.5375014Z ------------------------------------------
2020-04-28T08:39:32.5375188Z 
2020-04-28T08:39:32.5375552Z ------------------------------------------
2020-04-28T08:39:32.5375738Z stderr:
2020-04-28T08:39:32.5375738Z stderr:
2020-04-28T08:39:32.5376118Z ------------------------------------------
2020-04-28T08:39:32.5376338Z warning: skipping const checks
2020-04-28T08:39:32.5376790Z   --> /checkout/src/test/ui/consts/miri_unleashed/box.rs:10:11
2020-04-28T08:39:32.5377029Z    |
2020-04-28T08:39:32.5377180Z LL |     &mut *(box 0)
2020-04-28T08:39:32.5377477Z 
2020-04-28T08:39:32.5377655Z warning: skipping const checks
2020-04-28T08:39:32.5378157Z   --> /checkout/src/test/ui/consts/miri_unleashed/box.rs:10:16
2020-04-28T08:39:32.5378374Z    |
2020-04-28T08:39:32.5378374Z    |
2020-04-28T08:39:32.5378541Z LL |     &mut *(box 0)
2020-04-28T08:39:32.5378839Z 
2020-04-28T08:39:32.5379003Z warning: skipping const checks
2020-04-28T08:39:32.5379489Z   --> /checkout/src/test/ui/consts/miri_unleashed/box.rs:10:5
2020-04-28T08:39:32.5379939Z    |
2020-04-28T08:39:32.5379939Z    |
2020-04-28T08:39:32.5380292Z LL |     &mut *(box 0)
2020-04-28T08:39:32.5380615Z 
2020-04-28T08:39:32.5380769Z warning: skipping const checks
2020-04-28T08:39:32.5381471Z   --> /checkout/src/test/ui/consts/miri_unleashed/box.rs:10:5
2020-04-28T08:39:32.5381694Z    |
2020-04-28T08:39:32.5381694Z    |
2020-04-28T08:39:32.5381840Z LL |     &mut *(box 0)
2020-04-28T08:39:32.5382144Z 
2020-04-28T08:39:32.5382336Z error[E0080]: could not evaluate static initializer
2020-04-28T08:39:32.5382821Z   --> /checkout/src/test/ui/consts/miri_unleashed/box.rs:10:11
2020-04-28T08:39:32.5383046Z    |
2020-04-28T08:39:32.5383046Z    |
2020-04-28T08:39:32.5383192Z LL |     &mut *(box 0)
2020-04-28T08:39:32.5383499Z    |           ^^^^^^^ "heap allocations via `box` keyword" needs an rfc before being allowed inside constants
2020-04-28T08:39:32.5383967Z error: aborting due to previous error; 4 warnings emitted
2020-04-28T08:39:32.5384147Z 
2020-04-28T08:39:32.5384556Z For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5384760Z 
---
2020-04-28T08:39:32.5386757Z 1 warning: skipping const checks
2020-04-28T08:39:32.5387215Z -   --> $DIR/const_refers_to_static2.rs:13:18
2020-04-28T08:39:32.5387660Z +   --> $DIR/const_refers_to_static2.rs:11:18
2020-04-28T08:39:32.5387859Z 3    |
2020-04-28T08:39:32.5388080Z 4 LL |     unsafe { &*(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5388451Z 
2020-04-28T08:39:32.5388561Z 6 
2020-04-28T08:39:32.5388739Z 7 warning: skipping const checks
2020-04-28T08:39:32.5389165Z -   --> $DIR/const_refers_to_static2.rs:20:6
2020-04-28T08:39:32.5389165Z -   --> $DIR/const_refers_to_static2.rs:20:6
2020-04-28T08:39:32.5389608Z +   --> $DIR/const_refers_to_static2.rs:11:14
2020-04-28T08:39:32.5389822Z 9    |
2020-04-28T08:39:32.5390029Z + LL |     unsafe { &*(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5390601Z + 
2020-04-28T08:39:32.5390763Z + warning: skipping const checks
2020-04-28T08:39:32.5391387Z +   --> $DIR/const_refers_to_static2.rs:18:6
2020-04-28T08:39:32.5391603Z +    |
---
2020-04-28T08:39:32.5392531Z 13 error[E0080]: it is undefined behavior to use this value
2020-04-28T08:39:32.5393083Z -   --> $DIR/const_refers_to_static2.rs:11:1
2020-04-28T08:39:32.5393560Z +   --> $DIR/const_refers_to_static2.rs:9:1
2020-04-28T08:39:32.5393759Z 15    |
2020-04-28T08:39:32.5393957Z 16 LL | / const REF_INTERIOR_MUT: &usize = {
2020-04-28T08:39:32.5394276Z 17 LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
2020-04-28T08:39:32.5394483Z 
2020-04-28T08:39:32.5395448Z 23    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-04-28T08:39:32.5396342Z 25 error[E0080]: it is undefined behavior to use this value
2020-04-28T08:39:32.5396826Z -   --> $DIR/const_refers_to_static2.rs:18:1
2020-04-28T08:39:32.5397284Z +   --> $DIR/const_refers_to_static2.rs:16:1
2020-04-28T08:39:32.5397481Z 27    |
2020-04-28T08:39:32.5397481Z 27    |
2020-04-28T08:39:32.5397665Z 28 LL | / const READ_IMMUT: &usize = {
2020-04-28T08:39:32.5397924Z 29 LL | |     static FOO: usize = 0;
2020-04-28T08:39:32.5398200Z 34    |
2020-04-28T08:39:32.5398200Z 34    |
2020-04-28T08:39:32.5399108Z 35    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-04-28T08:39:32.5399567Z 36 
2020-04-28T08:39:32.5399991Z - error: aborting due to 2 previous errors; 2 warnings emitted
2020-04-28T08:39:32.5400293Z + error: aborting due to 2 previous errors; 3 warnings emitted
2020-04-28T08:39:32.5401140Z 39 For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5401356Z 40 
2020-04-28T08:39:32.5401464Z 
2020-04-28T08:39:32.5401553Z 
2020-04-28T08:39:32.5401553Z 
2020-04-28T08:39:32.5401738Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5402425Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/const_refers_to_static2.stderr
2020-04-28T08:39:32.5403495Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5404286Z To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static2.rs`
2020-04-28T08:39:32.5404731Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5404944Z status: exit code: 1
2020-04-28T08:39:32.5404944Z status: exit code: 1
2020-04-28T08:39:32.5407160Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/auxiliary"
2020-04-28T08:39:32.5408763Z ------------------------------------------
2020-04-28T08:39:32.5408919Z 
2020-04-28T08:39:32.5409256Z ------------------------------------------
2020-04-28T08:39:32.5409683Z stderr:
2020-04-28T08:39:32.5409683Z stderr:
2020-04-28T08:39:32.5410066Z ------------------------------------------
2020-04-28T08:39:32.5410286Z warning: skipping const checks
2020-04-28T08:39:32.5411051Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:11:18
2020-04-28T08:39:32.5411875Z    |
2020-04-28T08:39:32.5412081Z LL |     unsafe { &*(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5412630Z 
2020-04-28T08:39:32.5412789Z warning: skipping const checks
2020-04-28T08:39:32.5413330Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:11:14
2020-04-28T08:39:32.5413593Z    |
2020-04-28T08:39:32.5413593Z    |
2020-04-28T08:39:32.5413867Z LL |     unsafe { &*(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5417759Z 
2020-04-28T08:39:32.5417947Z warning: skipping const checks
2020-04-28T08:39:32.5418840Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:18:6
2020-04-28T08:39:32.5419093Z    |
2020-04-28T08:39:32.5419093Z    |
2020-04-28T08:39:32.5419243Z LL |     &FOO
2020-04-28T08:39:32.5419391Z    |      ^^^
2020-04-28T08:39:32.5419499Z 
2020-04-28T08:39:32.5419699Z error[E0080]: it is undefined behavior to use this value
2020-04-28T08:39:32.5422738Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:9:1
2020-04-28T08:39:32.5423163Z    |
2020-04-28T08:39:32.5423425Z LL | / const REF_INTERIOR_MUT: &usize = { //~ ERROR undefined behavior to use this value
2020-04-28T08:39:32.5423982Z LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
2020-04-28T08:39:32.5424303Z LL | |     unsafe { &*(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5424559Z LL | |     //~^ WARN skipping const checks
2020-04-28T08:39:32.5424755Z LL | | };
2020-04-28T08:39:32.5425018Z    | |__^ type validation failed: encountered a reference pointing to a static variable
2020-04-28T08:39:32.5425489Z    |
2020-04-28T08:39:32.5426316Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-04-28T08:39:32.5427204Z error[E0080]: it is undefined behavior to use this value
2020-04-28T08:39:32.5427971Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:16:1
2020-04-28T08:39:32.5428222Z    |
2020-04-28T08:39:32.5428222Z    |
2020-04-28T08:39:32.5428476Z LL | / const READ_IMMUT: &usize = { //~ ERROR it is undefined behavior to use this value
2020-04-28T08:39:32.5428978Z LL | |     static FOO: usize = 0;
2020-04-28T08:39:32.5443984Z LL | |     &FOO
2020-04-28T08:39:32.5445649Z LL | |     //~^ WARN skipping const checks
2020-04-28T08:39:32.5445884Z LL | | };
2020-04-28T08:39:32.5446151Z    | |__^ type validation failed: encountered a reference pointing to a static variable
2020-04-28T08:39:32.5446408Z    |
2020-04-28T08:39:32.5447685Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-04-28T08:39:32.5448306Z 
2020-04-28T08:39:32.5448510Z error: aborting due to 2 previous errors; 3 warnings emitted
2020-04-28T08:39:32.5449156Z For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5449348Z 
2020-04-28T08:39:32.5449694Z ------------------------------------------
2020-04-28T08:39:32.5450065Z 
---
2020-04-28T08:39:32.5451408Z 1 warning: skipping const checks
2020-04-28T08:39:32.5451842Z -   --> $DIR/const_refers_to_static.rs:14:5
2020-04-28T08:39:32.5452496Z +   --> $DIR/const_refers_to_static.rs:12:5
2020-04-28T08:39:32.5452885Z 3    |
2020-04-28T08:39:32.5453098Z 4 LL |     FOO.fetch_add(1, Ordering::Relaxed)
2020-04-28T08:39:32.5453440Z 
2020-04-28T08:39:32.5453554Z 6 
2020-04-28T08:39:32.5453718Z 7 warning: skipping const checks
2020-04-28T08:39:32.5454567Z -   --> $DIR/const_refers_to_static.rs:14:5
2020-04-28T08:39:32.5454567Z -   --> $DIR/const_refers_to_static.rs:14:5
2020-04-28T08:39:32.5455034Z +   --> $DIR/const_refers_to_static.rs:12:5
2020-04-28T08:39:32.5455419Z 9    |
2020-04-28T08:39:32.5455633Z 10 LL |     FOO.fetch_add(1, Ordering::Relaxed)
2020-04-28T08:39:32.5456075Z 
2020-04-28T08:39:32.5456184Z 12 
2020-04-28T08:39:32.5456352Z 13 warning: skipping const checks
2020-04-28T08:39:32.5456884Z -   --> $DIR/const_refers_to_static.rs:21:17
2020-04-28T08:39:32.5456884Z -   --> $DIR/const_refers_to_static.rs:21:17
2020-04-28T08:39:32.5457596Z +   --> $DIR/const_refers_to_static.rs:19:17
2020-04-28T08:39:32.5457796Z 15    |
2020-04-28T08:39:32.5458020Z 16 LL |     unsafe { *(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5458385Z 
2020-04-28T08:39:32.5458509Z 18 
2020-04-28T08:39:32.5458675Z 19 warning: skipping const checks
2020-04-28T08:39:32.5459086Z -   --> $DIR/const_refers_to_static.rs:26:32
2020-04-28T08:39:32.5459086Z -   --> $DIR/const_refers_to_static.rs:26:32
2020-04-28T08:39:32.5459532Z +   --> $DIR/const_refers_to_static.rs:19:14
2020-04-28T08:39:32.5459734Z 21    |
2020-04-28T08:39:32.5459938Z + LL |     unsafe { *(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5460417Z + 
2020-04-28T08:39:32.5460582Z + warning: skipping const checks
2020-04-28T08:39:32.5460999Z +   --> $DIR/const_refers_to_static.rs:24:32
2020-04-28T08:39:32.5461210Z +    |
---
2020-04-28T08:39:32.5470082Z 46 LL |     READ_MUT;
2020-04-28T08:39:32.5470298Z 47    |     ^^^^^^^^ referenced constant has errors
2020-04-28T08:39:32.5470461Z 
2020-04-28T08:39:32.5470571Z 48 
2020-04-28T08:39:32.5470989Z - error: aborting due to 3 previous errors; 5 warnings emitted
2020-04-28T08:39:32.5472341Z + error: aborting due to 3 previous errors; 6 warnings emitted
2020-04-28T08:39:32.5473044Z 51 For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5473683Z 52 
2020-04-28T08:39:32.5473780Z 
2020-04-28T08:39:32.5473869Z 
2020-04-28T08:39:32.5473869Z 
2020-04-28T08:39:32.5474070Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5474783Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static/const_refers_to_static.stderr
2020-04-28T08:39:32.5475556Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5476157Z To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static.rs`
2020-04-28T08:39:32.5476587Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5476878Z status: exit code: 1
2020-04-28T08:39:32.5476878Z status: exit code: 1
2020-04-28T08:39:32.5479352Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static/auxiliary"
2020-04-28T08:39:32.5480896Z ------------------------------------------
2020-04-28T08:39:32.5481054Z 
2020-04-28T08:39:32.5481409Z ------------------------------------------
2020-04-28T08:39:32.5481594Z stderr:
2020-04-28T08:39:32.5481594Z stderr:
2020-04-28T08:39:32.5481948Z ------------------------------------------
2020-04-28T08:39:32.5482181Z warning: skipping const checks
2020-04-28T08:39:32.5482677Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:12:5
2020-04-28T08:39:32.5482925Z    |
2020-04-28T08:39:32.5483145Z LL |     FOO.fetch_add(1, Ordering::Relaxed)
2020-04-28T08:39:32.5483466Z 
2020-04-28T08:39:32.5483623Z warning: skipping const checks
2020-04-28T08:39:32.5484131Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:12:5
2020-04-28T08:39:32.5484381Z    |
2020-04-28T08:39:32.5484381Z    |
2020-04-28T08:39:32.5484581Z LL |     FOO.fetch_add(1, Ordering::Relaxed)
2020-04-28T08:39:32.5485017Z 
2020-04-28T08:39:32.5485172Z warning: skipping const checks
2020-04-28T08:39:32.5485672Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:19:17
2020-04-28T08:39:32.5485936Z    |
2020-04-28T08:39:32.5485936Z    |
2020-04-28T08:39:32.5486138Z LL |     unsafe { *(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5486507Z 
2020-04-28T08:39:32.5486664Z warning: skipping const checks
2020-04-28T08:39:32.5487158Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:19:14
2020-04-28T08:39:32.5487405Z    |
2020-04-28T08:39:32.5487405Z    |
2020-04-28T08:39:32.5487618Z LL |     unsafe { *(&FOO as *const _ as *const usize) }
2020-04-28T08:39:32.5488051Z 
2020-04-28T08:39:32.5488222Z warning: skipping const checks
2020-04-28T08:39:32.5488724Z   --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static.rs:24:32
2020-04-28T08:39:32.5488972Z    |
---
2020-04-28T08:39:32.5495214Z    |
2020-04-28T08:39:32.5495370Z LL |     READ_MUT;
2020-04-28T08:39:32.5495577Z    |     ^^^^^^^^ referenced constant has errors
2020-04-28T08:39:32.5495737Z 
2020-04-28T08:39:32.5495951Z error: aborting due to 3 previous errors; 6 warnings emitted
2020-04-28T08:39:32.5496531Z For more information about this error, try `rustc --explain E0080`.
2020-04-28T08:39:32.5496724Z 
2020-04-28T08:39:32.5497078Z ------------------------------------------
2020-04-28T08:39:32.5497231Z 
---
2020-04-28T08:39:32.5498236Z 1 warning: skipping const checks
2020-04-28T08:39:32.5498630Z -   --> $DIR/mutable_const.rs:15:38
2020-04-28T08:39:32.5499589Z +   --> $DIR/mutable_const.rs:13:38
2020-04-28T08:39:32.5499777Z 3    |
2020-04-28T08:39:32.5500051Z 4 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-04-28T08:39:32.5500608Z 
2020-04-28T08:39:32.5500717Z 6 
2020-04-28T08:39:32.5500881Z + warning: skipping const checks
2020-04-28T08:39:32.5501294Z +   --> $DIR/mutable_const.rs:19:9
---
2020-04-28T08:39:32.5503867Z 7 error: any use of this value will cause an error
2020-04-28T08:39:32.5504277Z -   --> $DIR/mutable_const.rs:21:9
2020-04-28T08:39:32.5504687Z +   --> $DIR/mutable_const.rs:19:9
2020-04-28T08:39:32.5504864Z 9    |
2020-04-28T08:39:32.5505057Z 10 LL | / const MUTATING_BEHIND_RAW: () = {
2020-04-28T08:39:32.5505403Z 11 LL | |     // Test that `MUTABLE_BEHIND_RAW` is actually immutable, by doing this at const time.
2020-04-28T08:39:32.5505903Z 18    | |__-
2020-04-28T08:39:32.5506065Z 19    |
2020-04-28T08:39:32.5506246Z 20 note: the lint level is defined here
2020-04-28T08:39:32.5506634Z -   --> $DIR/mutable_const.rs:6:9
---
2020-04-28T08:39:32.5509076Z 
2020-04-28T08:39:32.5509180Z 
2020-04-28T08:39:32.5509367Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5509996Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/mutable_const.stderr
2020-04-28T08:39:32.5510718Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5511287Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const.rs`
2020-04-28T08:39:32.5511708Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5511922Z status: exit code: 1
2020-04-28T08:39:32.5511922Z status: exit code: 1
2020-04-28T08:39:32.5513892Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/auxiliary"
2020-04-28T08:39:32.5515413Z ------------------------------------------
2020-04-28T08:39:32.5515584Z 
2020-04-28T08:39:32.5515921Z ------------------------------------------
2020-04-28T08:39:32.5516105Z stderr:
2020-04-28T08:39:32.5516105Z stderr:
2020-04-28T08:39:32.5516471Z ------------------------------------------
2020-04-28T08:39:32.5516695Z warning: skipping const checks
2020-04-28T08:39:32.5517166Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:13:38
2020-04-28T08:39:32.5517412Z    |
2020-04-28T08:39:32.5517681Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-04-28T08:39:32.5518214Z 
2020-04-28T08:39:32.5518384Z warning: skipping const checks
2020-04-28T08:39:32.5519071Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:19:9
2020-04-28T08:39:32.5519307Z    |
2020-04-28T08:39:32.5519307Z    |
2020-04-28T08:39:32.5519570Z LL |         *MUTABLE_BEHIND_RAW = 99 //~ ERROR any use of this value will cause an error
2020-04-28T08:39:32.5520007Z 
2020-04-28T08:39:32.5520176Z warning: skipping const checks
2020-04-28T08:39:32.5520657Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:19:9
2020-04-28T08:39:32.5520885Z    |
2020-04-28T08:39:32.5520885Z    |
2020-04-28T08:39:32.5521940Z LL |         *MUTABLE_BEHIND_RAW = 99 //~ ERROR any use of this value will cause an error
2020-04-28T08:39:32.5522389Z 
2020-04-28T08:39:32.5522572Z error: any use of this value will cause an error
2020-04-28T08:39:32.5523126Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:19:9
2020-04-28T08:39:32.5523354Z    |
2020-04-28T08:39:32.5523354Z    |
2020-04-28T08:39:32.5523553Z LL | / const MUTATING_BEHIND_RAW: () = { //~ NOTE
2020-04-28T08:39:32.5523903Z LL | |     // Test that `MUTABLE_BEHIND_RAW` is actually immutable, by doing this at const time.
2020-04-28T08:39:32.5524188Z LL | |     unsafe {
2020-04-28T08:39:32.5524461Z LL | |         *MUTABLE_BEHIND_RAW = 99 //~ ERROR any use of this value will cause an error
2020-04-28T08:39:32.5525034Z    | |         ^^^^^^^^^^^^^^^^^^^^^^^^ writing to alloc2 which is read-only
2020-04-28T08:39:32.5525403Z LL | |     }
2020-04-28T08:39:32.5525558Z LL | | };
2020-04-28T08:39:32.5525836Z    | |__-
2020-04-28T08:39:32.5525965Z    |
---
2020-04-28T08:39:32.5529588Z 1 warning: skipping const checks
2020-04-28T08:39:32.5529983Z -   --> $DIR/mutable_const2.rs:15:38
2020-04-28T08:39:32.5530447Z +   --> $DIR/mutable_const2.rs:13:38
2020-04-28T08:39:32.5530635Z 3    |
2020-04-28T08:39:32.5530926Z 4 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-04-28T08:39:32.5531464Z 
2020-04-28T08:39:32.5531632Z 7 warning: 1 warning emitted
2020-04-28T08:39:32.5531790Z 8 
2020-04-28T08:39:32.5532013Z 9 error: internal compiler error: mutable allocation in constant
2020-04-28T08:39:32.5532013Z 9 error: internal compiler error: mutable allocation in constant
2020-04-28T08:39:32.5532481Z -   --> $DIR/mutable_const2.rs:15:1
2020-04-28T08:39:32.5532884Z +   --> $DIR/mutable_const2.rs:13:1
2020-04-28T08:39:32.5533064Z 11    |
2020-04-28T08:39:32.5533355Z 12 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-04-28T08:39:32.5533961Z 
2020-04-28T08:39:32.5534049Z 
2020-04-28T08:39:32.5534252Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5534889Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-04-28T08:39:32.5534889Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-04-28T08:39:32.5535943Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5536548Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2020-04-28T08:39:32.5536959Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5537191Z status: exit code: 101
2020-04-28T08:39:32.5537191Z status: exit code: 101
2020-04-28T08:39:32.5539130Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary"
2020-04-28T08:39:32.5540632Z ------------------------------------------
2020-04-28T08:39:32.5540787Z 
2020-04-28T08:39:32.5541139Z ------------------------------------------
2020-04-28T08:39:32.5541323Z stderr:
2020-04-28T08:39:32.5541323Z stderr:
2020-04-28T08:39:32.5541668Z ------------------------------------------
2020-04-28T08:39:32.5541905Z warning: skipping const checks
2020-04-28T08:39:32.5542380Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:13:38
2020-04-28T08:39:32.5542615Z    |
2020-04-28T08:39:32.5542898Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-04-28T08:39:32.5543434Z 
2020-04-28T08:39:32.5543600Z warning: 1 warning emitted
2020-04-28T08:39:32.5543733Z 
2020-04-28T08:39:32.5543944Z error: internal compiler error: mutable allocation in constant
2020-04-28T08:39:32.5543944Z error: internal compiler error: mutable allocation in constant
2020-04-28T08:39:32.5544471Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:13:1
2020-04-28T08:39:32.5544716Z    |
2020-04-28T08:39:32.5545607Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-04-28T08:39:32.5546328Z 
2020-04-28T08:39:32.5546328Z 
2020-04-28T08:39:32.5547878Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17
2020-04-28T08:39:32.5548532Z 
2020-04-28T08:39:32.5549071Z error: internal compiler error: unexpected panic
2020-04-28T08:39:32.5549248Z 
2020-04-28T08:39:32.5549558Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-28T08:39:32.5549558Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-28T08:39:32.5549755Z 
2020-04-28T08:39:32.5550453Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-28T08:39:32.5551162Z note: rustc 1.45.0-nightly (4d60d217d 2020-04-28) running on x86_64-unknown-linux-gnu
2020-04-28T08:39:32.5551379Z 
2020-04-28T08:39:32.5551379Z 
2020-04-28T08:39:32.5552043Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-28T08:39:32.5552510Z 
2020-04-28T08:39:32.5552854Z ------------------------------------------
2020-04-28T08:39:32.5553011Z 
2020-04-28T08:39:32.5553101Z 
2020-04-28T08:39:32.5553101Z 
2020-04-28T08:39:32.5553515Z ---- [ui] ui/consts/miri_unleashed/mutable_references.rs stdout ----
2020-04-28T08:39:32.5553760Z diff of stderr:
2020-04-28T08:39:32.5553883Z 
2020-04-28T08:39:32.5554056Z 1 warning: skipping const checks
2020-04-28T08:39:32.5554464Z -   --> $DIR/mutable_references.rs:28:8
2020-04-28T08:39:32.5554879Z +   --> $DIR/mutable_references.rs:9:26
2020-04-28T08:39:32.5555083Z 3    |
2020-04-28T08:39:32.5555273Z + LL | static FOO: &&mut u32 = &&mut 42;
2020-04-28T08:39:32.5555676Z + 
2020-04-28T08:39:32.5555856Z + warning: skipping const checks
2020-04-28T08:39:32.5556256Z +   --> $DIR/mutable_references.rs:13:23
2020-04-28T08:39:32.5556444Z +    |
2020-04-28T08:39:32.5556444Z +    |
2020-04-28T08:39:32.5556638Z + LL | static BAR: &mut () = &mut ();
2020-04-28T08:39:32.5557029Z + 
2020-04-28T08:39:32.5557206Z + warning: skipping const checks
2020-04-28T08:39:32.5557605Z +   --> $DIR/mutable_references.rs:18:28
2020-04-28T08:39:32.5557794Z +    |
2020-04-28T08:39:32.5557794Z +    |
2020-04-28T08:39:32.5558006Z + LL | static BOO: &mut Foo<()> = &mut Foo(());
2020-04-28T08:39:32.5558437Z + 
2020-04-28T08:39:32.5558603Z + warning: skipping const checks
2020-04-28T08:39:32.5559484Z +   --> $DIR/mutable_references.rs:27:8
2020-04-28T08:39:32.5559673Z +    |
2020-04-28T08:39:32.5559673Z +    |
2020-04-28T08:39:32.5559858Z 4 LL |     x: &UnsafeCell::new(42),
2020-04-28T08:39:32.5560097Z 5    |        ^^^^^^^^^^^^^^^^^^^^
2020-04-28T08:39:32.5560254Z 6 
2020-04-28T08:39:32.5560347Z 
2020-04-28T08:39:32.5560520Z + warning: skipping const checks
2020-04-28T08:39:32.5560919Z +   --> $DIR/mutable_references.rs:32:27
2020-04-28T08:39:32.5561107Z +    |
2020-04-28T08:39:32.5561297Z + LL | static OH_YES: &mut i32 = &mut 42;
2020-04-28T08:39:32.5561723Z + 
2020-04-28T08:39:32.5561723Z + 
2020-04-28T08:39:32.5561966Z 7 error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2020-04-28T08:39:32.5562871Z +   --> $DIR/mutable_references.rs:38:5
2020-04-28T08:39:32.5563057Z 9    |
2020-04-28T08:39:32.5563057Z 9    |
2020-04-28T08:39:32.5563229Z 10 LL |     *OH_YES = 99;
2020-04-28T08:39:32.5563582Z 
2020-04-28T08:39:32.5563694Z 12 
2020-04-28T08:39:32.5564101Z - error: aborting due to previous error; 1 warning emitted
2020-04-28T08:39:32.5564391Z + error: aborting due to previous error; 5 warnings emitted
2020-04-28T08:39:32.5564391Z + error: aborting due to previous error; 5 warnings emitted
2020-04-28T08:39:32.5564593Z 14 
2020-04-28T08:39:32.5565022Z 15 For more information about this error, try `rustc --explain E0594`.
2020-04-28T08:39:32.5565239Z 16 
2020-04-28T08:39:32.5565333Z 
2020-04-28T08:39:32.5565421Z 
2020-04-28T08:39:32.5565623Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5566385Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/mutable_references.stderr
2020-04-28T08:39:32.5566990Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5567583Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references.rs`
2020-04-28T08:39:32.5568076Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5568305Z status: exit code: 1
2020-04-28T08:39:32.5568305Z status: exit code: 1
2020-04-28T08:39:32.5570291Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/auxiliary"
2020-04-28T08:39:32.5571826Z ------------------------------------------
2020-04-28T08:39:32.5571998Z 
2020-04-28T08:39:32.5572340Z ------------------------------------------
2020-04-28T08:39:32.5572526Z stderr:
2020-04-28T08:39:32.5572526Z stderr:
2020-04-28T08:39:32.5572875Z ------------------------------------------
2020-04-28T08:39:32.5573108Z warning: skipping const checks
2020-04-28T08:39:32.5573589Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:9:26
2020-04-28T08:39:32.5573829Z    |
2020-04-28T08:39:32.5574026Z LL | static FOO: &&mut u32 = &&mut 42;
2020-04-28T08:39:32.5574400Z 
2020-04-28T08:39:32.5574571Z warning: skipping const checks
2020-04-28T08:39:32.5575062Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:13:23
2020-04-28T08:39:32.5575305Z    |
2020-04-28T08:39:32.5575305Z    |
2020-04-28T08:39:32.5575492Z LL | static BAR: &mut () = &mut ();
2020-04-28T08:39:32.5575849Z 
2020-04-28T08:39:32.5576006Z warning: skipping const checks
2020-04-28T08:39:32.5576506Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:18:28
2020-04-28T08:39:32.5576747Z    |
2020-04-28T08:39:32.5576747Z    |
2020-04-28T08:39:32.5576938Z LL | static BOO: &mut Foo<()> = &mut Foo(());
2020-04-28T08:39:32.5577357Z 
2020-04-28T08:39:32.5577512Z warning: skipping const checks
2020-04-28T08:39:32.5577994Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:27:8
2020-04-28T08:39:32.5578248Z    |
2020-04-28T08:39:32.5578248Z    |
2020-04-28T08:39:32.5578424Z LL |     x: &UnsafeCell::new(42),
2020-04-28T08:39:32.5578640Z    |        ^^^^^^^^^^^^^^^^^^^^
2020-04-28T08:39:32.5579304Z 
2020-04-28T08:39:32.5579468Z warning: skipping const checks
2020-04-28T08:39:32.5579996Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:32:27
2020-04-28T08:39:32.5580252Z    |
2020-04-28T08:39:32.5580433Z LL | static OH_YES: &mut i32 = &mut 42;
2020-04-28T08:39:32.5580808Z 
2020-04-28T08:39:32.5580808Z 
2020-04-28T08:39:32.5581070Z error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2020-04-28T08:39:32.5581870Z    |
2020-04-28T08:39:32.5581870Z    |
2020-04-28T08:39:32.5582153Z LL |     *OH_YES = 99; //~ ERROR cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
2020-04-28T08:39:32.5582609Z 
2020-04-28T08:39:32.5582820Z error: aborting due to previous error; 5 warnings emitted
2020-04-28T08:39:32.5582998Z 
2020-04-28T08:39:32.5583401Z For more information about this error, try `rustc --explain E0594`.
---
2020-04-28T08:39:32.5584937Z normalized stderr:
2020-04-28T08:39:32.5585760Z warning: skipping const checks
2020-04-28T08:39:32.5586286Z   --> $DIR/read_from_static.rs:5:27
2020-04-28T08:39:32.5586485Z    |
2020-04-28T08:39:32.5586668Z LL | static OH_YES: &mut i32 = &mut 42;
2020-04-28T08:39:32.5587043Z 
2020-04-28T08:39:32.5587208Z warning: 1 warning emitted
2020-04-28T08:39:32.5587340Z 
2020-04-28T08:39:32.5587427Z 
2020-04-28T08:39:32.5587427Z 
2020-04-28T08:39:32.5587514Z 
2020-04-28T08:39:32.5587601Z 
2020-04-28T08:39:32.5587799Z The actual stderr differed from the expected stderr.
2020-04-28T08:39:32.5588452Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/read_from_static/read_from_static.stderr
2020-04-28T08:39:32.5589051Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T08:39:32.5589640Z To only update this specific test, also pass `--test-args consts/miri_unleashed/read_from_static.rs`
2020-04-28T08:39:32.5590048Z error: 1 errors occurred comparing output.
2020-04-28T08:39:32.5590281Z status: exit code: 0
2020-04-28T08:39:32.5590281Z status: exit code: 0
2020-04-28T08:39:32.5592114Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/read_from_static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/read_from_static/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/read_from_static/auxiliary"
2020-04-28T08:39:32.5593573Z ------------------------------------------
2020-04-28T08:39:32.5593727Z 
2020-04-28T08:39:32.5594079Z ------------------------------------------
2020-04-28T08:39:32.5594263Z stderr:
2020-04-28T08:39:32.5594263Z stderr:
2020-04-28T08:39:32.5594610Z ------------------------------------------
2020-04-28T08:39:32.5594848Z warning: skipping const checks
2020-04-28T08:39:32.5595324Z   --> /checkout/src/test/ui/consts/miri_unleashed/read_from_static.rs:5:27
2020-04-28T08:39:32.5595558Z    |
2020-04-28T08:39:32.5595758Z LL | static OH_YES: &mut i32 = &mut 42;
2020-04-28T08:39:32.5596142Z 
2020-04-28T08:39:32.5596306Z warning: 1 warning emitted
2020-04-28T08:39:32.5596439Z 
2020-04-28T08:39:32.5596527Z 
---
2020-04-28T08:39:32.5601187Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-28T08:39:32.5601639Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-28T08:39:32.5601845Z 
2020-04-28T08:39:32.5601949Z 
2020-04-28T08:39:32.5605374Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-28T08:39:32.5607662Z 
2020-04-28T08:39:32.5607754Z 
2020-04-28T08:39:32.5608228Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T08:39:32.5608573Z Build completed unsuccessfully in 0:59:56
2020-04-28T08:39:32.5608573Z Build completed unsuccessfully in 0:59:56
2020-04-28T08:39:32.5608951Z == clock drift check ==
2020-04-28T08:39:32.5609174Z   local time: Tue Apr 28 08:39:32 UTC 2020
2020-04-28T08:39:32.6527167Z   network time: Tue, 28 Apr 2020 08:39:32 GMT
2020-04-28T08:39:33.1201936Z 
2020-04-28T08:39:33.1201936Z 
2020-04-28T08:39:33.1275737Z ##[error]Bash exited with code '1'.
2020-04-28T08:39:33.1299443Z ##[section]Finishing: Run build
2020-04-28T08:39:33.1354889Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71631/merge to s
2020-04-28T08:39:33.1360177Z Task         : Get sources
2020-04-28T08:39:33.1360486Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T08:39:33.1360782Z Version      : 1.0.0
2020-04-28T08:39:33.1360985Z Author       : Microsoft
2020-04-28T08:39:33.1360985Z Author       : Microsoft
2020-04-28T08:39:33.1361305Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T08:39:33.1361682Z ==============================================================================
2020-04-28T08:39:33.5045851Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T08:39:33.5166289Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71631/merge to s
2020-04-28T08:39:33.5263327Z Cleaning up task key
2020-04-28T08:39:33.5264707Z Start cleaning up orphan processes.
2020-04-28T08:39:33.5468483Z Terminate orphan process: pid (4036) (python)
2020-04-28T08:39:33.5630104Z ##[section]Finishing: Finalize Job
