plain
2020-04-19T23:21:00.4272931Z ========================== Starting Command Output ===========================
2020-04-19T23:21:00.4275283Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3fa526fc-feba-4a0f-885d-deff15f5e488.sh
2020-04-19T23:21:00.4275518Z 
2020-04-19T23:21:00.4279413Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T23:21:00.4296515Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71336/merge to s
2020-04-19T23:21:00.4299684Z Task         : Get sources
2020-04-19T23:21:00.4299959Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T23:21:00.4300246Z Version      : 1.0.0
2020-04-19T23:21:00.4300429Z Author       : Microsoft
---
2020-04-19T23:21:01.6949218Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T23:21:01.6955254Z ##[command]git config gc.auto 0
2020-04-19T23:21:01.6958818Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T23:21:01.6962434Z ##[command]git config --get-all http.proxy
2020-04-19T23:21:01.6969571Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71336/merge:refs/remotes/pull/71336/merge
---
2020-04-19T23:23:19.6574991Z  ---> 318032b5f0e2
2020-04-19T23:23:19.6576073Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T23:23:19.6578536Z  ---> Using cache
2020-04-19T23:23:19.6579126Z  ---> d44a858fd1ce
2020-04-19T23:23:19.6583024Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T23:23:19.6585582Z  ---> 58b910f50f5a
2020-04-19T23:23:19.6585796Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T23:23:19.6586165Z  ---> Using cache
2020-04-19T23:23:19.6586477Z  ---> ee7702aadba1
---
2020-04-19T23:23:19.7687233Z Looks like docker image is the same as before, not uploading
2020-04-19T23:23:27.6913253Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T23:23:27.7170007Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T23:23:27.7194816Z == clock drift check ==
2020-04-19T23:23:27.7210378Z   local time: Sun Apr 19 23:23:27 UTC 2020
2020-04-19T23:23:28.0624182Z   network time: Sun, 19 Apr 2020 23:23:28 GMT
2020-04-19T23:23:28.0645226Z Starting sccache server...
2020-04-19T23:23:28.1467273Z configure: processing command line
2020-04-19T23:23:28.1469778Z configure: 
2020-04-19T23:23:28.1471781Z configure: rust.dist-src        := False
---
2020-04-19T23:28:40.3836230Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T23:28:41.7804640Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T23:28:43.2835478Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T23:28:44.1541062Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T23:28:52.9332594Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T23:28:55.0681936Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T23:28:59.2623452Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T23:29:03.1255174Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T23:29:12.5970039Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T23:50:22.8139955Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T23:50:24.3465304Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T23:50:26.0709582Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T23:50:26.1078369Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T23:50:36.2087806Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T23:50:38.2046191Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T23:50:42.6616225Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T23:50:46.8404723Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T23:50:57.0821888Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T00:12:25.4220870Z .................................................................................................... 1600/9909
2020-04-20T00:12:31.4365120Z .................................................................................................... 1700/9909
2020-04-20T00:12:35.2687850Z .................................................................................................... 1800/9909
2020-04-20T00:12:43.5824478Z ......F............................................................................................. 1900/9909
2020-04-20T00:12:50.8822119Z i................................................................................................... 2000/9909
2020-04-20T00:12:56.8940856Z ..........................................................................................iiiii..... 2100/9909
2020-04-20T00:13:16.0340290Z .................................................................................................... 2300/9909
2020-04-20T00:13:18.1932445Z .................................................................................................... 2400/9909
2020-04-20T00:13:20.3450057Z .................................................................................................... 2500/9909
2020-04-20T00:13:26.0249036Z .................................................................................................... 2600/9909
---
2020-04-20T00:16:06.8683503Z ..................................................................i...............i................. 5000/9909
2020-04-20T00:16:13.9562305Z .................................................................................................... 5100/9909
2020-04-20T00:16:20.5465756Z .................................................................................................... 5200/9909
2020-04-20T00:16:25.3914308Z ............i....................................................................................... 5300/9909
2020-04-20T00:16:34.2063422Z ..i................................................................................................. 5400/9909
2020-04-20T00:16:39.0006137Z ..ii.ii........i...i................................................................................ 5500/9909
2020-04-20T00:16:46.2478529Z .................................................i.................................................. 5700/9909
2020-04-20T00:16:54.6492989Z .................................................................................ii................. 5800/9909
2020-04-20T00:17:01.2947893Z ....................i............................................................................... 5900/9909
2020-04-20T00:17:06.2980979Z .................................................................................................... 6000/9909
2020-04-20T00:17:06.2980979Z .................................................................................................... 6000/9909
2020-04-20T00:17:16.2466607Z .................................................................................................... 6100/9909
2020-04-20T00:17:25.7335580Z ..............ii...i..ii...........i................................................................ 6200/9909
2020-04-20T00:17:39.3560743Z .................................................................................................... 6400/9909
2020-04-20T00:17:42.6006386Z .................................................................................................... 6500/9909
2020-04-20T00:17:42.6006386Z .................................................................................................... 6500/9909
2020-04-20T00:17:52.5126623Z ............................................i..ii................................................... 6600/9909
2020-04-20T00:18:12.9629477Z .................................................................................................... 6800/9909
2020-04-20T00:18:15.0425263Z .............................................i...................................................... 6900/9909
2020-04-20T00:18:17.0361006Z .................................................................................................... 7000/9909
2020-04-20T00:18:18.9774345Z .....................................................................................i.............. 7100/9909
---
2020-04-20T00:19:49.5065191Z .................................................................................................... 7900/9909
2020-04-20T00:19:55.4355536Z .................................................................................................... 8000/9909
2020-04-20T00:20:00.6803561Z ...................................................i................................................ 8100/9909
2020-04-20T00:20:10.0837906Z .................................................................................................... 8200/9909
2020-04-20T00:20:14.9388497Z iiiiii.iiiii.i...................................................................................... 8300/9909
2020-04-20T00:20:27.3452950Z .................................................................................................... 8500/9909
2020-04-20T00:20:34.5355623Z .................................................................................................... 8600/9909
2020-04-20T00:20:47.1511460Z .................................................................................................... 8700/9909
2020-04-20T00:20:53.3706663Z .................................................................................................... 8800/9909
---
2020-04-20T00:22:32.1301100Z 
2020-04-20T00:22:32.1301293Z + warning: skipping const checks
2020-04-20T00:22:32.1301695Z +   --> $DIR/inline_asm.rs:9:1
2020-04-20T00:22:32.1301888Z +    |
2020-04-20T00:22:32.1302064Z + LL | / static TEST_BAD: () = {
2020-04-20T00:22:32.1302349Z + LL | |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
2020-04-20T00:22:32.1302605Z + LL | |
2020-04-20T00:22:32.1302738Z + LL | |
2020-04-20T00:22:32.1302868Z + LL | |
2020-04-20T00:22:32.1303023Z + LL | | };
2020-04-20T00:22:32.1303166Z +    | |__^
2020-04-20T00:22:32.1303496Z 1 error[E0080]: could not evaluate static initializer
2020-04-20T00:22:32.1303933Z 2   --> $DIR/inline_asm.rs:10:14
2020-04-20T00:22:32.1304110Z 3    |
2020-04-20T00:22:32.1304211Z 
---
2020-04-20T00:22:32.1307393Z 12 
2020-04-20T00:22:32.1307489Z 
2020-04-20T00:22:32.1307578Z 
2020-04-20T00:22:32.1307768Z The actual stderr differed from the expected stderr.
2020-04-20T00:22:32.1308492Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm/inline_asm.stderr
2020-04-20T00:22:32.1309026Z To update references, rerun the tests and pass the `--bless` flag
2020-04-20T00:22:32.1309543Z To only update this specific test, also pass `--test-args consts/miri_unleashed/inline_asm.rs`
2020-04-20T00:22:32.1309942Z error: 1 errors occurred comparing output.
2020-04-20T00:22:32.1310144Z status: exit code: 1
2020-04-20T00:22:32.1310144Z status: exit code: 1
2020-04-20T00:22:32.1311932Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/inline_asm/auxiliary"
2020-04-20T00:22:32.1313336Z ------------------------------------------
2020-04-20T00:22:32.1313483Z 
2020-04-20T00:22:32.1313798Z ------------------------------------------
2020-04-20T00:22:32.1314163Z stderr:
2020-04-20T00:22:32.1314163Z stderr:
2020-04-20T00:22:32.1314506Z ------------------------------------------
2020-04-20T00:22:32.1314731Z warning: skipping const checks
2020-04-20T00:22:32.1315204Z   --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:9:1
2020-04-20T00:22:32.1315440Z    |
2020-04-20T00:22:32.1315612Z LL | / static TEST_BAD: () = {
2020-04-20T00:22:32.1315908Z LL | |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
2020-04-20T00:22:32.1316224Z LL | |     //~^ ERROR could not evaluate static initializer
2020-04-20T00:22:32.1318246Z LL | |     //~| NOTE in this expansion of llvm_asm!
2020-04-20T00:22:32.1319548Z LL | |     //~| NOTE inline assembly is not supported
2020-04-20T00:22:32.1319876Z    | |__^
2020-04-20T00:22:32.1319977Z 
2020-04-20T00:22:32.1320183Z error[E0080]: could not evaluate static initializer
2020-04-20T00:22:32.1320826Z   --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
2020-04-20T00:22:32.1320826Z   --> /checkout/src/test/ui/consts/miri_unleashed/inline_asm.rs:10:14
2020-04-20T00:22:32.1321057Z    |
2020-04-20T00:22:32.1321302Z LL |     unsafe { llvm_asm!("xor %eax, %eax" ::: "eax"); }
2020-04-20T00:22:32.1321656Z    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ inline assembly is not supported
2020-04-20T00:22:32.1322465Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-20T00:22:32.1322722Z 
2020-04-20T00:22:32.1322922Z error: aborting due to previous error; 1 warning emitted
2020-04-20T00:22:32.1323118Z 
---
2020-04-20T00:22:32.1326128Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-20T00:22:32.1326666Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-20T00:22:32.1333627Z 
2020-04-20T00:22:32.1333810Z 
2020-04-20T00:22:32.1338461Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-20T00:22:32.1340837Z 
2020-04-20T00:22:32.1340931Z 
2020-04-20T00:22:32.1345849Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-20T00:22:32.1346204Z Build completed unsuccessfully in 0:57:20
2020-04-20T00:22:32.1346204Z Build completed unsuccessfully in 0:57:20
2020-04-20T00:22:32.1409780Z == clock drift check ==
2020-04-20T00:22:32.1430204Z   local time: Mon Apr 20 00:22:32 UTC 2020
2020-04-20T00:22:32.4529837Z   network time: Mon, 20 Apr 2020 00:22:32 GMT
2020-04-20T00:22:32.9268884Z 
2020-04-20T00:22:32.9268884Z 
2020-04-20T00:22:32.9335467Z ##[error]Bash exited with code '1'.
2020-04-20T00:22:32.9361633Z ##[section]Finishing: Run build
2020-04-20T00:22:32.9435480Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71336/merge to s
2020-04-20T00:22:32.9440525Z Task         : Get sources
2020-04-20T00:22:32.9440820Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T00:22:32.9441107Z Version      : 1.0.0
2020-04-20T00:22:32.9441303Z Author       : Microsoft
2020-04-20T00:22:32.9441303Z Author       : Microsoft
2020-04-20T00:22:32.9441607Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T00:22:32.9441968Z ==============================================================================
2020-04-20T00:22:33.2687610Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T00:22:33.2729645Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71336/merge to s
2020-04-20T00:22:33.2817232Z Cleaning up task key
2020-04-20T00:22:33.2818441Z Start cleaning up orphan processes.
2020-04-20T00:22:33.3006348Z Terminate orphan process: pid (3799) (python)
2020-04-20T00:22:33.3161704Z ##[section]Finishing: Finalize Job
