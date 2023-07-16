plain
2020-04-22T17:43:21.5336030Z ========================== Starting Command Output ===========================
2020-04-22T17:43:21.5355826Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c83c22bd-2368-426d-889b-a6ae4d63d9a7.sh
2020-04-22T17:43:21.5593456Z 
2020-04-22T17:43:21.5657614Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T17:43:21.5679339Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71435/merge to s
2020-04-22T17:43:21.5684386Z Task         : Get sources
2020-04-22T17:43:21.5684660Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T17:43:21.5684922Z Version      : 1.0.0
2020-04-22T17:43:21.5685698Z Author       : Microsoft
---
2020-04-22T17:43:22.3782061Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T17:43:22.3823966Z ##[command]git config gc.auto 0
2020-04-22T17:43:22.3857626Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T17:43:22.3925255Z ##[command]git config --get-all http.proxy
2020-04-22T17:43:22.4009286Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71435/merge:refs/remotes/pull/71435/merge
---
2020-04-22T17:46:42.3033858Z  ---> 318032b5f0e2
2020-04-22T17:46:42.3036320Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T17:46:42.3038855Z  ---> Using cache
2020-04-22T17:46:42.3039522Z  ---> d44a858fd1ce
2020-04-22T17:46:42.3040666Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T17:46:42.3042207Z  ---> 58b910f50f5a
2020-04-22T17:46:42.3042566Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T17:46:42.3043081Z  ---> Using cache
2020-04-22T17:46:42.3043608Z  ---> ee7702aadba1
---
2020-04-22T17:46:42.3904501Z Looks like docker image is the same as before, not uploading
2020-04-22T17:46:49.2800169Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T17:46:49.3105425Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T17:46:49.3136081Z == clock drift check ==
2020-04-22T17:46:49.3145601Z   local time: Wed Apr 22 17:46:49 UTC 2020
2020-04-22T17:46:49.4582919Z   network time: Wed, 22 Apr 2020 17:46:49 GMT
2020-04-22T17:46:49.4604307Z Starting sccache server...
2020-04-22T17:46:49.5423922Z configure: processing command line
2020-04-22T17:46:49.5424282Z configure: 
2020-04-22T17:46:49.5425591Z configure: rust.dist-src        := False
---
2020-04-22T17:52:24.0819187Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T17:52:25.6641336Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T17:52:27.3277414Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T17:52:29.2192188Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T17:52:38.1940872Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T17:52:41.9098743Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T17:52:46.6631822Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T17:52:51.1430156Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T17:53:00.1117617Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T18:16:53.9147093Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T18:16:55.6349355Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T18:16:57.5545452Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T18:16:58.7196578Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T18:17:09.8658224Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T18:17:12.7952984Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T18:17:17.7531551Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T18:17:22.6551101Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T18:17:32.9352475Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T18:41:18.8104934Z .................................................................................................... 1700/9915
2020-04-22T18:41:22.7515008Z .................................................................................................... 1800/9915
2020-04-22T18:41:30.7140455Z .................................................................................................... 1900/9915
2020-04-22T18:41:38.1542895Z ....i............................................................................................... 2000/9915
2020-04-22T18:41:44.0245908Z ..............................................................................................iiiii. 2100/9915
2020-04-22T18:42:03.0268396Z .................................................................................................... 2300/9915
2020-04-22T18:42:05.0893640Z .................................................................................................... 2400/9915
2020-04-22T18:42:07.1968876Z .................................................................................................... 2500/9915
2020-04-22T18:42:12.5335737Z .................................................................................................... 2600/9915
---
2020-04-22T18:45:06.6673366Z .................................................................................................... 5100/9915
2020-04-22T18:45:13.6155331Z .................................................................................................... 5200/9915
2020-04-22T18:45:17.9579316Z .................i.................................................................................. 5300/9915
2020-04-22T18:45:27.4312215Z .......i............................................................................................ 5400/9915
2020-04-22T18:45:32.5328363Z .......ii.ii........i...i........................................................................... 5500/9915
2020-04-22T18:45:39.6698974Z ......................................................i............................................. 5700/9915
2020-04-22T18:45:48.2112415Z .......................................................................................ii........... 5800/9915
2020-04-22T18:45:54.5219451Z ..........................i......................................................................... 5900/9915
2020-04-22T18:45:59.6345042Z .................................................................................................... 6000/9915
2020-04-22T18:45:59.6345042Z .................................................................................................... 6000/9915
2020-04-22T18:46:09.8610792Z .................................................................................................... 6100/9915
2020-04-22T18:46:19.7101092Z ....................ii...i..ii...........i.......................................................... 6200/9915
2020-04-22T18:46:35.0292747Z .................................................................................................... 6400/9915
2020-04-22T18:46:41.5328544Z .................................................................................................... 6500/9915
2020-04-22T18:46:41.5328544Z .................................................................................................... 6500/9915
2020-04-22T18:47:03.9617792Z ..................................................i..ii............................................. 6600/9915
2020-04-22T18:47:25.5732213Z .................................................................................................... 6800/9915
2020-04-22T18:47:28.3779919Z ...................................................i................................................ 6900/9915
2020-04-22T18:47:29.9577393Z .................................................................................................... 7000/9915
2020-04-22T18:47:31.9810306Z ...........................................................................................i........ 7100/9915
---
2020-04-22T18:49:10.8462301Z .................................................................................................... 7900/9915
2020-04-22T18:49:16.9780245Z .................................................................................................... 8000/9915
2020-04-22T18:49:22.8132865Z .........................................................i.......................................... 8100/9915
2020-04-22T18:49:32.7179700Z .................................................................................................... 8200/9915
2020-04-22T18:49:38.0866762Z ......iiiiii.iiiii.i................................................................................ 8300/9915
2020-04-22T18:49:51.3613595Z .................................................................................................... 8500/9915
2020-04-22T18:49:59.1729689Z .................................................................................................... 8600/9915
2020-04-22T18:50:12.6885104Z .................................................................................................... 8700/9915
2020-04-22T18:50:19.1880624Z .................................................................................................... 8800/9915
---
2020-04-22T18:51:21.5321645Z .................................................................................................... 9300/9915
2020-04-22T18:51:26.1958863Z .......................i............................................................................ 9400/9915
2020-04-22T18:51:32.1424018Z .................................................................................................... 9500/9915
2020-04-22T18:51:39.3360860Z .................................................................................................... 9600/9915
2020-04-22T18:51:48.4867970Z ...........................................................................................FFF....FF 9700/9915
2020-04-22T18:51:54.4714940Z .F.F.F.............................................................................................. 9800/9915
2020-04-22T18:52:09.0800598Z ...............
2020-04-22T18:52:09.0801764Z failures:
2020-04-22T18:52:09.0838105Z 
2020-04-22T18:52:09.0838930Z ---- [ui] ui/fn/dyn-fn-alignment.rs stdout ----
---
2020-04-22T18:52:09.0846000Z 
2020-04-22T18:52:09.0846088Z 
2020-04-22T18:52:09.0846176Z 
2020-04-22T18:52:09.0846363Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0847112Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/dyn-fn-alignment/dyn-fn-alignment.stderr
2020-04-22T18:52:09.0847724Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0848265Z To only update this specific test, also pass `--test-args fn/dyn-fn-alignment.rs`
2020-04-22T18:52:09.0848839Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0849053Z status: exit code: 0
2020-04-22T18:52:09.0849053Z status: exit code: 0
2020-04-22T18:52:09.0850747Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/dyn-fn-alignment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/dyn-fn-alignment/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/dyn-fn-alignment/auxiliary"
2020-04-22T18:52:09.0852105Z ------------------------------------------
2020-04-22T18:52:09.0852263Z 
2020-04-22T18:52:09.0852610Z ------------------------------------------
2020-04-22T18:52:09.0852798Z stderr:
---
2020-04-22T18:52:09.0859111Z 
2020-04-22T18:52:09.0859215Z 
2020-04-22T18:52:09.0859402Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0860015Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/autoderef.stderr
2020-04-22T18:52:09.0860768Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0861321Z To only update this specific test, also pass `--test-args unsized-locals/autoderef.rs`
2020-04-22T18:52:09.0861741Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0861956Z status: exit code: 0
2020-04-22T18:52:09.0861956Z status: exit code: 0
2020-04-22T18:52:09.0863632Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/auxiliary"
2020-04-22T18:52:09.0865000Z ------------------------------------------
2020-04-22T18:52:09.0865174Z 
2020-04-22T18:52:09.0865521Z ------------------------------------------
2020-04-22T18:52:09.0865708Z stderr:
---
2020-04-22T18:52:09.0872530Z 
2020-04-22T18:52:09.0872618Z 
2020-04-22T18:52:09.0872819Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0873595Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/by-value-trait-object-safety-rpass.stderr
2020-04-22T18:52:09.0874281Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0874904Z To only update this specific test, also pass `--test-args unsized-locals/by-value-trait-object-safety-rpass.rs`
2020-04-22T18:52:09.0875338Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0875566Z status: exit code: 0
2020-04-22T18:52:09.0875566Z status: exit code: 0
2020-04-22T18:52:09.0877420Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/by-value-trait-object-safety-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/auxiliary"
2020-04-22T18:52:09.0879044Z ------------------------------------------
2020-04-22T18:52:09.0879220Z 
2020-04-22T18:52:09.0879572Z ------------------------------------------
2020-04-22T18:52:09.0879759Z stderr:
---
2020-04-22T18:52:09.0886451Z 
2020-04-22T18:52:09.0886539Z 
2020-04-22T18:52:09.0886628Z 
2020-04-22T18:52:09.0886832Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0887597Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/by-value-trait-object-safety-withdefault.stderr
2020-04-22T18:52:09.0888289Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0888914Z To only update this specific test, also pass `--test-args unsized-locals/by-value-trait-object-safety-withdefault.rs`
2020-04-22T18:52:09.0889356Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0889585Z status: exit code: 0
2020-04-22T18:52:09.0889585Z status: exit code: 0
2020-04-22T18:52:09.0893453Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/by-value-trait-object-safety-withdefault.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/auxiliary"
2020-04-22T18:52:09.0895059Z ------------------------------------------
2020-04-22T18:52:09.0895236Z 
2020-04-22T18:52:09.0895589Z ------------------------------------------
2020-04-22T18:52:09.0895778Z stderr:
---
2020-04-22T18:52:09.0902595Z 
2020-04-22T18:52:09.0902690Z 
2020-04-22T18:52:09.0902890Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0903580Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/simple-unsized-locals/simple-unsized-locals.stderr
2020-04-22T18:52:09.0904195Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0904795Z To only update this specific test, also pass `--test-args unsized-locals/simple-unsized-locals.rs`
2020-04-22T18:52:09.0905212Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0905440Z status: exit code: 0
2020-04-22T18:52:09.0905440Z status: exit code: 0
2020-04-22T18:52:09.0907203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/simple-unsized-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/simple-unsized-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/simple-unsized-locals/auxiliary"
2020-04-22T18:52:09.0910482Z ------------------------------------------
2020-04-22T18:52:09.0910654Z 
2020-04-22T18:52:09.0911509Z ------------------------------------------
2020-04-22T18:52:09.0912222Z stderr:
---
2020-04-22T18:52:09.0919406Z 
2020-04-22T18:52:09.0919494Z 
2020-04-22T18:52:09.0919696Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0920411Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/reference-unsized-locals/reference-unsized-locals.stderr
2020-04-22T18:52:09.0921036Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0921646Z To only update this specific test, also pass `--test-args unsized-locals/reference-unsized-locals.rs`
2020-04-22T18:52:09.0922064Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0922501Z status: exit code: 0
2020-04-22T18:52:09.0922501Z status: exit code: 0
2020-04-22T18:52:09.0924822Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/reference-unsized-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/reference-unsized-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/reference-unsized-locals/auxiliary"
2020-04-22T18:52:09.0926357Z ------------------------------------------
2020-04-22T18:52:09.0926518Z 
2020-04-22T18:52:09.0926884Z ------------------------------------------
2020-04-22T18:52:09.0927072Z stderr:
---
2020-04-22T18:52:09.0933884Z 
2020-04-22T18:52:09.0933972Z 
2020-04-22T18:52:09.0934173Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0934834Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs-rpass/unsized-exprs-rpass.stderr
2020-04-22T18:52:09.0935442Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0936036Z To only update this specific test, also pass `--test-args unsized-locals/unsized-exprs-rpass.rs`
2020-04-22T18:52:09.0936552Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0936781Z status: exit code: 0
2020-04-22T18:52:09.0936781Z status: exit code: 0
2020-04-22T18:52:09.0938567Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/unsized-exprs-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs-rpass/auxiliary"
2020-04-22T18:52:09.0939986Z ------------------------------------------
2020-04-22T18:52:09.0940143Z 
2020-04-22T18:52:09.0940514Z ------------------------------------------
2020-04-22T18:52:09.0940700Z stderr:
---
2020-04-22T18:52:09.0948012Z 
2020-04-22T18:52:09.0948101Z 
2020-04-22T18:52:09.0948191Z 
2020-04-22T18:52:09.0948377Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0949022Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-index/unsized-index.stderr
2020-04-22T18:52:09.0949614Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0950197Z To only update this specific test, also pass `--test-args unsized-locals/unsized-index.rs`
2020-04-22T18:52:09.0950596Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0950815Z status: exit code: 0
2020-04-22T18:52:09.0950815Z status: exit code: 0
2020-04-22T18:52:09.0953642Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/unsized-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-index" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-index/auxiliary"
2020-04-22T18:52:09.0955257Z ------------------------------------------
2020-04-22T18:52:09.0955418Z 
2020-04-22T18:52:09.0955764Z ------------------------------------------
2020-04-22T18:52:09.0955968Z stderr:
---
2020-04-22T18:52:09.0962445Z 
2020-04-22T18:52:09.0962533Z 
2020-04-22T18:52:09.0962718Z The actual stderr differed from the expected stderr.
2020-04-22T18:52:09.0963411Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-parameters/unsized-parameters.stderr
2020-04-22T18:52:09.0964031Z To update references, rerun the tests and pass the `--bless` flag
2020-04-22T18:52:09.0964604Z To only update this specific test, also pass `--test-args unsized-locals/unsized-parameters.rs`
2020-04-22T18:52:09.0965035Z error: 1 errors occurred comparing output.
2020-04-22T18:52:09.0965250Z status: exit code: 0
2020-04-22T18:52:09.0965250Z status: exit code: 0
2020-04-22T18:52:09.0966992Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/unsized-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-parameters/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-parameters/auxiliary"
2020-04-22T18:52:09.0968395Z ------------------------------------------
2020-04-22T18:52:09.0968553Z 
2020-04-22T18:52:09.0968898Z ------------------------------------------
2020-04-22T18:52:09.0969101Z stderr:
---
2020-04-22T18:52:09.0980101Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T18:52:09.0980498Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T18:52:09.0980706Z 
2020-04-22T18:52:09.0980900Z 
2020-04-22T18:52:09.0984341Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T18:52:09.0986664Z 
2020-04-22T18:52:09.0986756Z 
2020-04-22T18:52:09.0987254Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T18:52:09.0987590Z Build completed unsuccessfully in 1:03:36
2020-04-22T18:52:09.0987590Z Build completed unsuccessfully in 1:03:36
2020-04-22T18:52:09.0987815Z == clock drift check ==
2020-04-22T18:52:09.1001713Z   local time: Wed Apr 22 18:52:09 UTC 2020
2020-04-22T18:52:09.1347669Z   network time: Wed, 22 Apr 2020 18:52:09 GMT
2020-04-22T18:52:09.6167887Z 
2020-04-22T18:52:09.6167887Z 
2020-04-22T18:52:09.6269764Z ##[error]Bash exited with code '1'.
2020-04-22T18:52:09.6286171Z ##[section]Finishing: Run build
2020-04-22T18:52:09.6334607Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71435/merge to s
2020-04-22T18:52:09.6339530Z Task         : Get sources
2020-04-22T18:52:09.6339855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T18:52:09.6340136Z Version      : 1.0.0
2020-04-22T18:52:09.6340339Z Author       : Microsoft
2020-04-22T18:52:09.6340339Z Author       : Microsoft
2020-04-22T18:52:09.6340673Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T18:52:09.6341035Z ==============================================================================
2020-04-22T18:52:09.9733218Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T18:52:09.9779192Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71435/merge to s
2020-04-22T18:52:09.9878321Z Cleaning up task key
2020-04-22T18:52:09.9879573Z Start cleaning up orphan processes.
2020-04-22T18:52:10.0125052Z Terminate orphan process: pid (5460) (python)
2020-04-22T18:52:10.0359317Z ##[section]Finishing: Finalize Job
