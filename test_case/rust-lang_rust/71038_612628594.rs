plain
2020-04-12T13:35:28.5733873Z ========================== Starting Command Output ===========================
2020-04-12T13:35:28.5738844Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8c2dd2c2-252a-49d3-9ea1-c9c86631d819.sh
2020-04-12T13:35:28.5739275Z 
2020-04-12T13:35:28.5743367Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T13:35:28.5764522Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71038/merge to s
2020-04-12T13:35:28.5768041Z Task         : Get sources
2020-04-12T13:35:28.5768322Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T13:35:28.5768589Z Version      : 1.0.0
2020-04-12T13:35:28.5768823Z Author       : Microsoft
---
2020-04-12T13:35:29.6777419Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T13:35:29.6788257Z ##[command]git config gc.auto 0
2020-04-12T13:35:29.6795700Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T13:35:29.6802209Z ##[command]git config --get-all http.proxy
2020-04-12T13:35:29.6812964Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71038/merge:refs/remotes/pull/71038/merge
---
2020-04-12T13:37:41.2734259Z Looks like docker image is the same as before, not uploading
2020-04-12T13:37:46.0382660Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T13:37:46.0703030Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-12T13:37:46.0744006Z == clock drift check ==
2020-04-12T13:37:46.0744308Z   local time: Sun Apr 12 13:37:46 UTC 2020
2020-04-12T13:37:46.2333764Z   network time: Sun, 12 Apr 2020 13:37:46 GMT
2020-04-12T13:37:46.2360207Z Starting sccache server...
2020-04-12T13:37:46.3322973Z configure: processing command line
2020-04-12T13:37:46.3323234Z configure: 
2020-04-12T13:37:46.3324266Z configure: rust.dist-src        := False
---
2020-04-12T13:44:15.7497186Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T13:44:17.5913874Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T13:44:19.5909305Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T13:44:22.6067421Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T13:44:32.3963790Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T13:44:37.3894938Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T13:44:43.3351499Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T13:44:48.8972614Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T13:44:59.3242292Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T14:11:34.4436067Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T14:11:36.5224840Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T14:11:38.8337256Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-12T14:11:41.8824635Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T14:11:53.2483963Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T14:11:58.4119056Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T14:12:04.7079445Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-12T14:12:10.9848824Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T14:12:22.9579677Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-12T14:41:17.3500827Z .................................................................................................... 1700/9891
2020-04-12T14:41:22.4367020Z .................................................................................................... 1800/9891
2020-04-12T14:41:32.3786267Z .................................................................................................... 1900/9891
2020-04-12T14:41:41.9208691Z .....i.............................................................................................. 2000/9891
2020-04-12T14:41:49.2648274Z ...............................................................................................iiiii 2100/9891
2020-04-12T14:42:13.5221765Z .................................................................................................... 2300/9891
2020-04-12T14:42:15.8274666Z .................................................................................................... 2400/9891
2020-04-12T14:42:18.2606802Z .................................................................................................... 2500/9891
2020-04-12T14:42:24.3045060Z .................................................................................................... 2600/9891
---
2020-04-12T14:45:42.4435042Z .................................................................................................... 5100/9891
2020-04-12T14:45:51.0192700Z .................................................................................................... 5200/9891
2020-04-12T14:45:56.6674539Z ...............i.................................................................................... 5300/9891
2020-04-12T14:46:07.5024347Z .................................................................................................... 5400/9891
2020-04-12T14:46:13.3080055Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-12T14:46:21.7102484Z .................................................i.................................................. 5700/9891
2020-04-12T14:46:33.1420923Z .....................................................................ii............................. 5800/9891
2020-04-12T14:46:40.2934567Z ........i........................................................................................... 5900/9891
2020-04-12T14:46:46.9914017Z .................................................................................................... 6000/9891
2020-04-12T14:46:46.9914017Z .................................................................................................... 6000/9891
2020-04-12T14:46:58.9050696Z .................................................................................................... 6100/9891
2020-04-12T14:47:11.2279536Z ...ii...i..ii...........i........................................................................... 6200/9891
2020-04-12T14:47:28.4343243Z .................................................................................................... 6400/9891
2020-04-12T14:47:35.5462753Z .................................................................................................... 6500/9891
2020-04-12T14:47:35.5462753Z .................................................................................................... 6500/9891
2020-04-12T14:47:52.4747725Z .................................i..ii.............................................................. 6600/9891
2020-04-12T14:48:16.0322763Z .................................................................................................... 6800/9891
2020-04-12T14:48:18.1160349Z .................................i.................................................................. 6900/9891
2020-04-12T14:48:20.4850023Z .................................................................................................... 7000/9891
2020-04-12T14:48:23.0777360Z ........................................................................i........................... 7100/9891
---
2020-04-12T14:50:20.5025724Z .................................................................................................... 7800/9891
2020-04-12T14:50:25.3344568Z .................................................................................................... 7900/9891
2020-04-12T14:50:33.3030798Z .................................................................................................... 8000/9891
2020-04-12T14:50:40.8334143Z .......................................i............................................................ 8100/9891
2020-04-12T14:50:51.8459712Z ......................................................................................iiiiii.iiiii.i 8200/9891
2020-04-12T14:51:10.6473745Z ................................i......i............................................................ 8400/9891
2020-04-12T14:51:14.7777144Z .................................................................................................... 8500/9891
2020-04-12T14:51:27.0907727Z .................................................................................................... 8600/9891
2020-04-12T14:51:42.2101340Z .................................................................................................... 8700/9891
---
2020-04-12T14:53:58.6757205Z 
2020-04-12T14:53:58.6758179Z ---- [ui] ui/const-generics/issues/issue-63322-forbid-dyn.rs stdout ----
2020-04-12T14:53:58.6758876Z diff of stderr:
2020-04-12T14:53:58.6759188Z 
2020-04-12T14:53:58.6759806Z 12 LL | fn test<const T: &'static dyn A>() {
2020-04-12T14:53:58.6761002Z 13    |                  ^^^^^^^^^^^^^^ `&'static (dyn A + 'static)` doesn't derive both `PartialEq` and `Eq`
2020-04-12T14:53:58.6762011Z - error: aborting due to previous error
2020-04-12T14:53:58.6762646Z + error: aborting due to previous error; 1 warning emitted
2020-04-12T14:53:58.6763166Z 16 
2020-04-12T14:53:58.6763805Z 17 For more information about this error, try `rustc --explain E0741`.
2020-04-12T14:53:58.6763805Z 17 For more information about this error, try `rustc --explain E0741`.
2020-04-12T14:53:58.6764413Z 18 
2020-04-12T14:53:58.6765336Z 
2020-04-12T14:53:58.6765452Z 
2020-04-12T14:53:58.6765682Z The actual stderr differed from the expected stderr.
2020-04-12T14:53:58.6766728Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn/issue-63322-forbid-dyn.stderr
2020-04-12T14:53:58.6767463Z To update references, rerun the tests and pass the `--bless` flag
2020-04-12T14:53:58.6768142Z To only update this specific test, also pass `--test-args const-generics/issues/issue-63322-forbid-dyn.rs`
2020-04-12T14:53:58.6768634Z error: 1 errors occurred comparing output.
2020-04-12T14:53:58.6768895Z status: exit code: 1
2020-04-12T14:53:58.6768895Z status: exit code: 1
2020-04-12T14:53:58.6771070Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn/auxiliary"
2020-04-12T14:53:58.6773032Z ------------------------------------------
2020-04-12T14:53:58.6773231Z 
2020-04-12T14:53:58.6773605Z ------------------------------------------
2020-04-12T14:53:58.6773813Z stderr:
---
2020-04-12T14:53:58.6776992Z 
2020-04-12T14:53:58.6777257Z error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
2020-04-12T14:53:58.6778096Z   --> /checkout/src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs:8:18
2020-04-12T14:53:58.6778380Z    |
2020-04-12T14:53:58.6778771Z LL | fn test<const T: &'static dyn A>() {
2020-04-12T14:53:58.6779418Z    |                  ^^^^^^^^^^^^^^ `&'static (dyn A + 'static)` doesn't derive both `PartialEq` and `Eq`
2020-04-12T14:53:58.6779934Z error: aborting due to previous error; 1 warning emitted
2020-04-12T14:53:58.6780884Z 
2020-04-12T14:53:58.6781449Z For more information about this error, try `rustc --explain E0741`.
2020-04-12T14:53:58.6781675Z 
---
2020-04-12T14:53:58.6783776Z test result: FAILED. 9829 passed; 1 failed; 61 ignored; 0 measured; 0 filtered out
2020-04-12T14:53:58.6784169Z 
2020-04-12T14:53:58.6788596Z 
2020-04-12T14:53:58.6788824Z 
2020-04-12T14:53:58.6795193Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-12T14:53:58.6797956Z 
2020-04-12T14:53:58.6798056Z 
2020-04-12T14:53:58.6798876Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-12T14:53:58.6799330Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-12T14:53:58.6799330Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-12T14:53:58.6800559Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-12T14:53:58.6801091Z Build completed unsuccessfully in 1:14:15
2020-04-12T14:53:58.6852313Z == clock drift check ==
2020-04-12T14:53:58.6876979Z   local time: Sun Apr 12 14:53:58 UTC 2020
2020-04-12T14:53:58.9015130Z   network time: Sun, 12 Apr 2020 14:53:58 GMT
2020-04-12T14:53:59.2574757Z 
2020-04-12T14:53:59.2574757Z 
2020-04-12T14:53:59.2652246Z ##[error]Bash exited with code '1'.
2020-04-12T14:53:59.2665775Z ##[section]Finishing: Run build
2020-04-12T14:53:59.2716015Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71038/merge to s
2020-04-12T14:53:59.2720888Z Task         : Get sources
2020-04-12T14:53:59.2721213Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T14:53:59.2721671Z Version      : 1.0.0
2020-04-12T14:53:59.2721888Z Author       : Microsoft
2020-04-12T14:53:59.2721888Z Author       : Microsoft
2020-04-12T14:53:59.2722246Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T14:53:59.2722624Z ==============================================================================
2020-04-12T14:53:59.6424063Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T14:53:59.6472544Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71038/merge to s
2020-04-12T14:53:59.6574888Z Cleaning up task key
2020-04-12T14:53:59.6576189Z Start cleaning up orphan processes.
2020-04-12T14:53:59.6796613Z Terminate orphan process: pid (3721) (python)
2020-04-12T14:53:59.6991865Z ##[section]Finishing: Finalize Job
