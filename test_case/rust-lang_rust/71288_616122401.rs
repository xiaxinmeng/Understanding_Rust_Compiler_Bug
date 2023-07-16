plain
2020-04-19T11:33:05.0231226Z ========================== Starting Command Output ===========================
2020-04-19T11:33:05.0233385Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c7b1ab29-8d2e-490b-add6-af916723a244.sh
2020-04-19T11:33:05.0233601Z 
2020-04-19T11:33:05.0238861Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T11:33:05.0262413Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-19T11:33:05.0266459Z Task         : Get sources
2020-04-19T11:33:05.0266837Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T11:33:05.0267228Z Version      : 1.0.0
2020-04-19T11:33:05.0267479Z Author       : Microsoft
---
2020-04-19T11:33:06.2922614Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T11:33:06.2929563Z ##[command]git config gc.auto 0
2020-04-19T11:33:06.2932530Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T11:33:06.2935091Z ##[command]git config --get-all http.proxy
2020-04-19T11:33:06.2941267Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71288/merge:refs/remotes/pull/71288/merge
---
2020-04-19T11:35:01.6518189Z  ---> 318032b5f0e2
2020-04-19T11:35:01.6519161Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T11:35:01.6519764Z  ---> Using cache
2020-04-19T11:35:01.6520083Z  ---> d44a858fd1ce
2020-04-19T11:35:01.6521147Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T11:35:01.6522155Z  ---> 58b910f50f5a
2020-04-19T11:35:01.6522488Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T11:35:01.6522842Z  ---> Using cache
2020-04-19T11:35:01.6523157Z  ---> ee7702aadba1
---
2020-04-19T11:35:01.6861714Z Looks like docker image is the same as before, not uploading
2020-04-19T11:35:09.5937395Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T11:35:09.6278147Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T11:35:09.6310474Z == clock drift check ==
2020-04-19T11:35:09.6341790Z   local time: Sun Apr 19 11:35:09 UTC 2020
2020-04-19T11:35:09.9006046Z   network time: Sun, 19 Apr 2020 11:35:09 GMT
2020-04-19T11:35:09.9029610Z Starting sccache server...
2020-04-19T11:35:09.9781913Z configure: processing command line
2020-04-19T11:35:09.9782141Z configure: 
2020-04-19T11:35:09.9786193Z configure: rust.dist-src        := False
---
2020-04-19T11:39:45.5589754Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T11:39:46.8313420Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T11:39:48.2149065Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T11:39:49.0663639Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T11:39:56.9382093Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T11:39:59.0512462Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T11:40:02.8490865Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T11:40:06.4900798Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T11:40:14.8953016Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T11:59:44.8041298Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T11:59:46.2181268Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T11:59:47.8582158Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T11:59:48.0163756Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T11:59:57.2966718Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T11:59:59.2595732Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T12:00:03.3704588Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T12:00:07.3401343Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T12:00:16.4791691Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T12:20:35.6906386Z .................................................................................................... 1700/9907
2020-04-19T12:20:39.2699864Z .................................................................................................... 1800/9907
2020-04-19T12:20:46.8138642Z ..................................................................................................i. 1900/9907
2020-04-19T12:20:53.6770989Z .................................................................................................... 2000/9907
2020-04-19T12:20:59.2302365Z ........................................................................................iiiii....... 2100/9907
2020-04-19T12:21:16.4836825Z .................................................................................................... 2300/9907
2020-04-19T12:21:18.4434129Z .................................................................................................... 2400/9907
2020-04-19T12:21:20.4422175Z .................................................................................................... 2500/9907
2020-04-19T12:21:25.8678554Z .................................................................................................... 2600/9907
---
2020-04-19T12:23:55.9592616Z ................................................................i...............i................... 5000/9907
2020-04-19T12:24:02.6617010Z .................................................................................................... 5100/9907
2020-04-19T12:24:08.6902517Z .................................................................................................... 5200/9907
2020-04-19T12:24:13.2057244Z ..........i......................................................................................... 5300/9907
2020-04-19T12:24:21.7552630Z i................................................................................................... 5400/9907
2020-04-19T12:24:26.0542145Z ii.ii........i...i.................................................................................. 5500/9907
2020-04-19T12:24:32.8396617Z ...............................................i.................................................... 5700/9907
2020-04-19T12:24:40.7094754Z ...............................................................................ii................... 5800/9907
2020-04-19T12:24:46.9306548Z ..................i................................................................................. 5900/9907
2020-04-19T12:24:51.6648421Z .................................................................................................... 6000/9907
2020-04-19T12:24:51.6648421Z .................................................................................................... 6000/9907
2020-04-19T12:25:01.2161028Z .................................................................................................... 6100/9907
2020-04-19T12:25:10.8548824Z ............ii...i..i.i..........i.................................................................. 6200/9907
2020-04-19T12:25:24.3851897Z .................................................................................................... 6400/9907
2020-04-19T12:25:27.6115830Z .................................................................................................... 6500/9907
2020-04-19T12:25:27.6115830Z .................................................................................................... 6500/9907
2020-04-19T12:25:37.6577470Z ..........................................i..ii..................................................... 6600/9907
2020-04-19T12:25:58.4106976Z .................................................................................................... 6800/9907
2020-04-19T12:26:00.5442164Z ...........................................i........................................................ 6900/9907
2020-04-19T12:26:02.6039859Z .................................................................................................... 7000/9907
2020-04-19T12:26:04.7597383Z ...................................................................................i.F.............. 7100/9907
---
2020-04-19T12:27:31.3799814Z .................................................................................................... 7800/9907
2020-04-19T12:27:35.6371108Z .................................................................................................... 7900/9907
2020-04-19T12:27:41.6349471Z .................................................................................................... 8000/9907
2020-04-19T12:27:46.8404615Z .................................................i.................................................. 8100/9907
2020-04-19T12:27:56.0934420Z ..................................................................................................ii 8200/9907
2020-04-19T12:28:01.0271813Z iiii.iiiii.i........................................................................................ 8300/9907
2020-04-19T12:28:13.1802133Z .................................................................................................... 8500/9907
2020-04-19T12:28:20.4415962Z .................................................................................................... 8600/9907
2020-04-19T12:28:32.5932160Z .................................................................................................... 8700/9907
2020-04-19T12:28:38.5431893Z .................................................................................................... 8800/9907
---
2020-04-19T12:30:13.4302387Z failures:
2020-04-19T12:30:13.4328523Z 
2020-04-19T12:30:13.4329057Z ---- [ui] ui/parser/mod_file_not_exist.rs stdout ----
2020-04-19T12:30:13.4329315Z 
2020-04-19T12:30:13.4329921Z error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:3: unexpected help message: '3:1: 3:21: if there is `mod not_a_real_file` elsewhere in the crate already, import it with `use crate::` instead'
2020-04-19T12:30:13.4330266Z 
2020-04-19T12:30:13.4330531Z error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:4: expected help message not found: if there is `mod not_a_real_file` elsewhere in the crate already
2020-04-19T12:30:13.4330933Z error: 1 unexpected errors found, 1 expected errors not found
2020-04-19T12:30:13.4331103Z status: exit code: 1
2020-04-19T12:30:13.4331103Z status: exit code: 1
2020-04-19T12:30:13.4332367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mod_file_not_exist.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/auxiliary"
2020-04-19T12:30:13.4333605Z     Error {
2020-04-19T12:30:13.4333743Z         line_num: 3,
2020-04-19T12:30:13.4333891Z         kind: Some(
2020-04-19T12:30:13.4334029Z             Help,
2020-04-19T12:30:13.4334029Z             Help,
2020-04-19T12:30:13.4334165Z         ),
2020-04-19T12:30:13.4334468Z         msg: "3:1: 3:21: if there is `mod not_a_real_file` elsewhere in the crate already, import it with `use crate::` instead",
2020-04-19T12:30:13.4334878Z ]
2020-04-19T12:30:13.4334954Z 
2020-04-19T12:30:13.4335103Z not found errors (from test file): [
2020-04-19T12:30:13.4335273Z     Error {
2020-04-19T12:30:13.4335273Z     Error {
2020-04-19T12:30:13.4335406Z         line_num: 4,
2020-04-19T12:30:13.4335555Z         kind: Some(
2020-04-19T12:30:13.4335704Z             Help,
2020-04-19T12:30:13.4335826Z         ),
2020-04-19T12:30:13.4336029Z         msg: "if there is `mod not_a_real_file` elsewhere in the crate already",
2020-04-19T12:30:13.4336339Z ]
2020-04-19T12:30:13.4336413Z 
2020-04-19T12:30:13.4336875Z thread '[ui] ui/parser/mod_file_not_exist.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
2020-04-19T12:30:13.4337225Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
2020-04-19T12:30:13.4338655Z 
2020-04-19T12:30:13.4350704Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T12:30:13.4362880Z 
2020-04-19T12:30:13.4363849Z 
2020-04-19T12:30:13.4367700Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T12:30:13.4370024Z 
2020-04-19T12:30:13.4370132Z 
2020-04-19T12:30:13.4378409Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T12:30:13.4378792Z Build completed unsuccessfully in 0:53:40
2020-04-19T12:30:13.4378792Z Build completed unsuccessfully in 0:53:40
2020-04-19T12:30:13.4439945Z == clock drift check ==
2020-04-19T12:30:13.4466263Z   local time: Sun Apr 19 12:30:13 UTC 2020
2020-04-19T12:30:13.5656868Z   network time: Sun, 19 Apr 2020 12:30:13 GMT
2020-04-19T12:30:13.9960299Z 
2020-04-19T12:30:13.9960299Z 
2020-04-19T12:30:14.0019325Z ##[error]Bash exited with code '1'.
2020-04-19T12:30:14.0035713Z ##[section]Finishing: Run build
2020-04-19T12:30:14.0092267Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-19T12:30:14.0096948Z Task         : Get sources
2020-04-19T12:30:14.0097235Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T12:30:14.0097498Z Version      : 1.0.0
2020-04-19T12:30:14.0097709Z Author       : Microsoft
2020-04-19T12:30:14.0097709Z Author       : Microsoft
2020-04-19T12:30:14.0098020Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T12:30:14.0098356Z ==============================================================================
2020-04-19T12:30:14.3184112Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T12:30:14.3227689Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-19T12:30:14.3319517Z Cleaning up task key
2020-04-19T12:30:14.3320691Z Start cleaning up orphan processes.
2020-04-19T12:30:14.3478857Z Terminate orphan process: pid (5855) (python)
2020-04-19T12:30:14.3682182Z ##[section]Finishing: Finalize Job
