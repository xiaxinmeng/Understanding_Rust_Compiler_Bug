plain
2020-04-01T19:45:24.4236856Z ========================== Starting Command Output ===========================
2020-04-01T19:45:24.4239245Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/121c1ea6-b093-439e-8a0b-0657e586e43f.sh
2020-04-01T19:45:24.4239526Z 
2020-04-01T19:45:24.4243202Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T19:45:24.4261781Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-01T19:45:24.4264759Z Task         : Get sources
2020-04-01T19:45:24.4265044Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T19:45:24.4265322Z Version      : 1.0.0
2020-04-01T19:45:24.4265508Z Author       : Microsoft
---
2020-04-01T19:45:25.4196405Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T19:45:25.4205307Z ##[command]git config gc.auto 0
2020-04-01T19:45:25.4211628Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T19:45:25.4217407Z ##[command]git config --get-all http.proxy
2020-04-01T19:45:25.4227373Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70566/merge:refs/remotes/pull/70566/merge
---
2020-04-01T19:47:35.2788434Z Looks like docker image is the same as before, not uploading
2020-04-01T19:47:42.8922731Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T19:47:42.9167308Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T19:47:42.9190472Z == clock drift check ==
2020-04-01T19:47:42.9198194Z   local time: Wed Apr  1 19:47:42 UTC 2020
2020-04-01T19:47:43.2153421Z   network time: Wed, 01 Apr 2020 19:47:43 GMT
2020-04-01T19:47:43.2181329Z Starting sccache server...
2020-04-01T19:47:43.2991543Z configure: processing command line
2020-04-01T19:47:43.2992359Z configure: 
2020-04-01T19:47:43.2993944Z configure: rust.dist-src        := False
---
2020-04-01T19:52:35.4291280Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T19:52:36.8263270Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T19:52:38.3299377Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T19:52:40.2089934Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T19:52:47.8196249Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T19:52:50.1624451Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T19:52:54.1439834Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T19:52:57.9646965Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T19:53:06.9035594Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T20:13:58.9126881Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T20:14:00.4890825Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T20:14:02.3200241Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T20:14:03.3314076Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T20:14:14.1016383Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T20:14:16.1925622Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T20:14:21.2628315Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T20:14:26.4144979Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T20:14:39.5539300Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T20:38:30.6713152Z .................................................................................................... 1700/9872
2020-04-01T20:38:34.7321252Z .................................................................................................... 1800/9872
2020-04-01T20:38:43.2609447Z .................................................................................................i.. 1900/9872
2020-04-01T20:38:50.6910305Z .................................................................................................... 2000/9872
2020-04-01T20:38:56.7936765Z .......................................................................................iiiii........ 2100/9872
2020-04-01T20:39:16.8480234Z .................................................................................................... 2300/9872
2020-04-01T20:39:18.9207364Z .................................................................................................... 2400/9872
2020-04-01T20:39:21.1438922Z .................................................................................................... 2500/9872
2020-04-01T20:39:26.7557853Z .................................................................................................... 2600/9872
---
2020-04-01T20:42:09.2295072Z .............................................................i...............i...................... 5000/9872
2020-04-01T20:42:16.4893983Z .................................................................................................... 5100/9872
2020-04-01T20:42:24.0758260Z .................................................................................................... 5200/9872
2020-04-01T20:42:29.3546860Z ......i............................................................................................. 5300/9872
2020-04-01T20:42:38.9807472Z ............................................................................................ii.ii... 5400/9872
2020-04-01T20:42:43.2482969Z .....i...i.......................................................................................... 5500/9872
2020-04-01T20:42:46.5327389Z ................................i.....................................FFF........................... 5600/9872
2020-04-01T20:42:51.3320005Z ........................................i........................................................... 5700/9872
2020-04-01T20:43:00.1257935Z ............................................................ii....................................i. 5800/9872
2020-04-01T20:43:11.4788049Z .................................................................................................... 6000/9872
2020-04-01T20:43:11.4788049Z .................................................................................................... 6000/9872
2020-04-01T20:43:20.6540832Z ............................................................................................ii...i.. 6100/9872
2020-04-01T20:43:31.9021718Z ii...........i...................................................................................... 6200/9872
2020-04-01T20:43:46.7344877Z .................................................................................................... 6400/9872
2020-04-01T20:43:49.4976874Z .................................................................................................... 6500/9872
2020-04-01T20:43:49.4976874Z .................................................................................................... 6500/9872
2020-04-01T20:44:01.2730017Z ......................i..ii......................................................................... 6600/9872
2020-04-01T20:44:20.8681600Z .................................................................................................... 6800/9872
2020-04-01T20:44:22.8629400Z ......................i............................................................................. 6900/9872
2020-04-01T20:44:24.8798976Z .................................................................................................... 7000/9872
2020-04-01T20:44:26.9935247Z .............................................................i...................................... 7100/9872
---
2020-04-01T20:46:01.3254036Z .................................................................................................... 7800/9872
2020-04-01T20:46:05.7915822Z .................................................................................................... 7900/9872
2020-04-01T20:46:11.4752729Z .................................................................................................... 8000/9872
2020-04-01T20:46:19.2322320Z .......................i............................................................................ 8100/9872
2020-04-01T20:46:27.0931455Z ........................................................................iiiiiiiiii.i................ 8200/9872
2020-04-01T20:46:42.5737481Z ................i......i............................................................................ 8400/9872
2020-04-01T20:46:47.0304998Z .................................................................................................... 8500/9872
2020-04-01T20:46:56.9645457Z .................................................................................................... 8600/9872
2020-04-01T20:47:07.6530673Z .................................................................................................... 8700/9872
---
2020-04-01T20:48:56.0822794Z normalized stderr:
2020-04-01T20:48:56.0823071Z error: this arithmetic operation will overflow
2020-04-01T20:48:56.0823589Z   --> $DIR/lint-exceeding-bitshifts-assoc-const.rs:19:20
2020-04-01T20:48:56.0823827Z    |
2020-04-01T20:48:56.0824037Z LL |     const N: i32 = T::N << 42;
2020-04-01T20:48:56.0824562Z    |
2020-04-01T20:48:56.0824758Z note: the lint level is defined here
2020-04-01T20:48:56.0825235Z   --> $DIR/lint-exceeding-bitshifts-assoc-const.rs:10:9
2020-04-01T20:48:56.0825463Z    |
---
2020-04-01T20:48:56.0826460Z 
2020-04-01T20:48:56.0826550Z 
2020-04-01T20:48:56.0826639Z 
2020-04-01T20:48:56.0826932Z The actual stderr differed from the expected stderr.
2020-04-01T20:48:56.0827639Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.noopt/lint-exceeding-bitshifts-assoc-const.noopt.stderr
2020-04-01T20:48:56.0828717Z To update references, rerun the tests and pass the `--bless` flag
2020-04-01T20:48:56.0829575Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts-assoc-const.rs`
2020-04-01T20:48:56.0829860Z 
2020-04-01T20:48:56.0830102Z error in revision `noopt`: 1 errors occurred comparing output.
2020-04-01T20:48:56.0830403Z status: exit code: 1
2020-04-01T20:48:56.0832634Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.noopt" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.noopt/auxiliary"
2020-04-01T20:48:56.0837072Z ------------------------------------------
2020-04-01T20:48:56.0837253Z 
2020-04-01T20:48:56.0837668Z ------------------------------------------
2020-04-01T20:48:56.0837877Z stderr:
2020-04-01T20:48:56.0837877Z stderr:
2020-04-01T20:48:56.0838270Z ------------------------------------------
2020-04-01T20:48:56.0838570Z error: this arithmetic operation will overflow
2020-04-01T20:48:56.0839164Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs:19:20
2020-04-01T20:48:56.0839453Z    |
2020-04-01T20:48:56.0839790Z LL |     const N: i32 = T::N << 42; //~ ERROR: arithmetic operation will overflow
2020-04-01T20:48:56.0840627Z    |
2020-04-01T20:48:56.0840901Z note: the lint level is defined here
2020-04-01T20:48:56.0841478Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs:10:9
2020-04-01T20:48:56.0841752Z    |
---
2020-04-01T20:48:56.0844230Z normalized stderr:
2020-04-01T20:48:56.0844448Z error: this arithmetic operation will overflow
2020-04-01T20:48:56.0844954Z   --> $DIR/lint-exceeding-bitshifts-assoc-const.rs:19:20
2020-04-01T20:48:56.0845189Z    |
2020-04-01T20:48:56.0845382Z LL |     const N: i32 = T::N << 42;
2020-04-01T20:48:56.0845921Z    |
2020-04-01T20:48:56.0846097Z note: the lint level is defined here
2020-04-01T20:48:56.0846584Z   --> $DIR/lint-exceeding-bitshifts-assoc-const.rs:10:9
2020-04-01T20:48:56.0846813Z    |
---
2020-04-01T20:48:56.0847781Z 
2020-04-01T20:48:56.0847874Z 
2020-04-01T20:48:56.0847954Z 
2020-04-01T20:48:56.0848122Z The actual stderr differed from the expected stderr.
2020-04-01T20:48:56.0848787Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.opt/lint-exceeding-bitshifts-assoc-const.opt.stderr
2020-04-01T20:48:56.0849413Z To update references, rerun the tests and pass the `--bless` flag
2020-04-01T20:48:56.0849948Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts-assoc-const.rs`
2020-04-01T20:48:56.0850387Z error in revision `opt`: 1 errors occurred comparing output.
2020-04-01T20:48:56.0850615Z status: exit code: 1
2020-04-01T20:48:56.0850615Z status: exit code: 1
2020-04-01T20:48:56.0852385Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.opt" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.opt/auxiliary"
2020-04-01T20:48:56.0853817Z ------------------------------------------
2020-04-01T20:48:56.0853962Z 
2020-04-01T20:48:56.0854273Z ------------------------------------------
2020-04-01T20:48:56.0854442Z stderr:
2020-04-01T20:48:56.0854442Z stderr:
2020-04-01T20:48:56.0854773Z ------------------------------------------
2020-04-01T20:48:56.0855000Z error: this arithmetic operation will overflow
2020-04-01T20:48:56.0855563Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs:19:20
2020-04-01T20:48:56.0855780Z    |
2020-04-01T20:48:56.0856001Z LL |     const N: i32 = T::N << 42; //~ ERROR: arithmetic operation will overflow
2020-04-01T20:48:56.0856469Z    |
2020-04-01T20:48:56.0856602Z note: the lint level is defined here
2020-04-01T20:48:56.0857000Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs:10:9
2020-04-01T20:48:56.0857255Z    |
---
2020-04-01T20:48:56.0859559Z normalized stderr:
2020-04-01T20:48:56.0859749Z error: this arithmetic operation will overflow
2020-04-01T20:48:56.0860174Z   --> $DIR/lint-exceeding-bitshifts-assoc-const.rs:19:20
2020-04-01T20:48:56.0860391Z    |
2020-04-01T20:48:56.0860555Z LL |     const N: i32 = T::N << 42;
2020-04-01T20:48:56.0861030Z    |
2020-04-01T20:48:56.0861190Z note: the lint level is defined here
2020-04-01T20:48:56.0861598Z   --> $DIR/lint-exceeding-bitshifts-assoc-const.rs:10:9
2020-04-01T20:48:56.0861795Z    |
---
2020-04-01T20:48:56.0862661Z 
2020-04-01T20:48:56.0862738Z 
2020-04-01T20:48:56.0862817Z 
2020-04-01T20:48:56.0862999Z The actual stderr differed from the expected stderr.
2020-04-01T20:48:56.0863746Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.opt_with_overflow_checks/lint-exceeding-bitshifts-assoc-const.opt_with_overflow_checks.stderr
2020-04-01T20:48:56.0864406Z To update references, rerun the tests and pass the `--bless` flag
2020-04-01T20:48:56.0864954Z To only update this specific test, also pass `--test-args lint/lint-exceeding-bitshifts-assoc-const.rs`
2020-04-01T20:48:56.0865188Z 
2020-04-01T20:48:56.0865818Z error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
2020-04-01T20:48:56.0866162Z status: exit code: 1
2020-04-01T20:48:56.0868569Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.opt_with_overflow_checks" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts-assoc-const.opt_with_overflow_checks/auxiliary"
2020-04-01T20:48:56.0870509Z ------------------------------------------
2020-04-01T20:48:56.0870704Z 
2020-04-01T20:48:56.0871091Z ------------------------------------------
2020-04-01T20:48:56.0871300Z stderr:
2020-04-01T20:48:56.0871300Z stderr:
2020-04-01T20:48:56.0871690Z ------------------------------------------
2020-04-01T20:48:56.0871988Z error: this arithmetic operation will overflow
2020-04-01T20:48:56.0872573Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs:19:20
2020-04-01T20:48:56.0872877Z    |
2020-04-01T20:48:56.0885891Z LL |     const N: i32 = T::N << 42; //~ ERROR: arithmetic operation will overflow
2020-04-01T20:48:56.0886543Z    |
2020-04-01T20:48:56.0886717Z note: the lint level is defined here
2020-04-01T20:48:56.0887564Z   --> /checkout/src/test/ui/lint/lint-exceeding-bitshifts-assoc-const.rs:10:9
2020-04-01T20:48:56.0887979Z    |
---
2020-04-01T20:48:56.0892652Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-01T20:48:56.0892971Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-01T20:48:56.0893138Z 
2020-04-01T20:48:56.0893207Z 
2020-04-01T20:48:56.0896400Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-01T20:48:56.0898610Z 
2020-04-01T20:48:56.0898694Z 
2020-04-01T20:48:56.0899135Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-01T20:48:56.0899445Z Build completed unsuccessfully in 0:59:35
2020-04-01T20:48:56.0899445Z Build completed unsuccessfully in 0:59:35
2020-04-01T20:48:56.0938874Z == clock drift check ==
2020-04-01T20:48:56.0950597Z   local time: Wed Apr  1 20:48:56 UTC 2020
2020-04-01T20:48:56.2644926Z   network time: Wed, 01 Apr 2020 20:48:56 GMT
2020-04-01T20:48:56.7024343Z 
2020-04-01T20:48:56.7024343Z 
2020-04-01T20:48:56.7110398Z ##[error]Bash exited with code '1'.
2020-04-01T20:48:56.7124981Z ##[section]Finishing: Run build
2020-04-01T20:48:56.7169449Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-01T20:48:56.7175606Z Task         : Get sources
2020-04-01T20:48:56.7175989Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T20:48:56.7176319Z Version      : 1.0.0
2020-04-01T20:48:56.7176553Z Author       : Microsoft
2020-04-01T20:48:56.7176553Z Author       : Microsoft
2020-04-01T20:48:56.7176933Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T20:48:56.7177457Z ==============================================================================
2020-04-01T20:48:57.0281863Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T20:48:57.0329988Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-01T20:48:57.0415599Z Cleaning up task key
2020-04-01T20:48:57.0416760Z Start cleaning up orphan processes.
2020-04-01T20:48:57.0596601Z Terminate orphan process: pid (3633) (python)
2020-04-01T20:48:57.0802757Z ##[section]Finishing: Finalize Job
