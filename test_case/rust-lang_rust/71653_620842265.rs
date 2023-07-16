plain
2020-04-28T19:35:45.7712323Z ========================== Starting Command Output ===========================
2020-04-28T19:35:45.7728855Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7cf79fce-c644-4287-baf4-89ea6fb95d6b.sh
2020-04-28T19:35:45.9804953Z 
2020-04-28T19:35:45.9857764Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T19:35:45.9879357Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71653/merge to s
2020-04-28T19:35:45.9885212Z Task         : Get sources
2020-04-28T19:35:45.9885442Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T19:35:45.9885695Z Version      : 1.0.0
2020-04-28T19:35:45.9885848Z Author       : Microsoft
---
2020-04-28T19:35:48.0592913Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T19:35:48.0749034Z ##[command]git config gc.auto 0
2020-04-28T19:35:48.0787688Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T19:35:48.0808770Z ##[command]git config --get-all http.proxy
2020-04-28T19:35:48.0896827Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71653/merge:refs/remotes/pull/71653/merge
---
2020-04-28T19:39:09.6327879Z  ---> cb2676f08729
2020-04-28T19:39:09.6328734Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-28T19:39:09.6343126Z  ---> Using cache
2020-04-28T19:39:09.6353354Z  ---> df25ce111862
2020-04-28T19:39:09.6355728Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-28T19:39:09.6361275Z  ---> 599b9ac96b27
2020-04-28T19:39:09.6361494Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-28T19:39:09.6368585Z  ---> Using cache
2020-04-28T19:39:09.6369139Z  ---> 091087e35a36
---
2020-04-28T19:39:09.6855007Z Looks like docker image is the same as before, not uploading
2020-04-28T19:39:17.7304433Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T19:39:17.7631662Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-28T19:39:17.7660920Z == clock drift check ==
2020-04-28T19:39:17.7672192Z   local time: Tue Apr 28 19:39:17 UTC 2020
2020-04-28T19:39:17.7928484Z   network time: Tue, 28 Apr 2020 19:39:17 GMT
2020-04-28T19:39:17.7956026Z Starting sccache server...
2020-04-28T19:39:17.8795519Z configure: processing command line
2020-04-28T19:39:17.8796285Z configure: 
2020-04-28T19:39:17.8797290Z configure: rust.dist-src        := False
---
2020-04-28T19:44:34.9129748Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T19:44:36.3205352Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T19:44:37.8839688Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T19:44:39.0112082Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T19:44:47.3415789Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T19:44:49.4951620Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T19:44:53.6287879Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T19:44:57.4862005Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T19:45:07.1529724Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T20:06:39.6548480Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-28T20:06:41.3261265Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-28T20:06:43.2016823Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-28T20:06:44.6761987Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-28T20:06:54.1088062Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-28T20:06:57.8295499Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-28T20:07:02.6431343Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-28T20:07:07.0492109Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-28T20:07:16.1288931Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-28T20:29:11.2678439Z .................................................................................................... 1800/9944
2020-04-28T20:29:17.1849938Z .................................................................................................... 1900/9944
2020-04-28T20:29:25.8643412Z ................i................................................................................... 2000/9944
2020-04-28T20:29:31.8313102Z .................................................................................................... 2100/9944
2020-04-28T20:29:43.3835014Z ......iiiii......................................................................................... 2200/9944
2020-04-28T20:29:51.7550993Z .................................................................................................... 2400/9944
2020-04-28T20:29:53.7913218Z .................................................................................................... 2500/9944
2020-04-28T20:29:58.3127492Z .................................................................................................... 2600/9944
2020-04-28T20:30:16.3096440Z .................................................................................................... 2700/9944
---
2020-04-28T20:32:42.7841646Z ....i............................................................................................... 5100/9944
2020-04-28T20:32:50.4242524Z .................................................................................................... 5200/9944
2020-04-28T20:32:55.0412054Z ..................................i................................................................. 5300/9944
2020-04-28T20:33:03.1461443Z .........................i.......................................................................... 5400/9944
2020-04-28T20:33:09.5775945Z ..........................ii.ii........i...i........................................................ 5500/9944
2020-04-28T20:33:17.1022496Z .........................................................................i.......................... 5700/9944
2020-04-28T20:33:25.3706550Z .................................................................................................... 5800/9944
2020-04-28T20:33:31.4040896Z ......ii.....................................i...................................................... 5900/9944
2020-04-28T20:33:37.3614118Z .................................................................................................... 6000/9944
2020-04-28T20:33:37.3614118Z .................................................................................................... 6000/9944
2020-04-28T20:33:46.6033000Z .................................................................................................... 6100/9944
2020-04-28T20:33:52.6353725Z ........................................ii...i..ii...........i...................................... 6200/9944
2020-04-28T20:34:11.1031994Z .................................................................................................... 6400/9944
2020-04-28T20:34:17.5392684Z .................................................................................................... 6500/9944
2020-04-28T20:34:17.5392684Z .................................................................................................... 6500/9944
2020-04-28T20:34:26.1507679Z ......................................................................i..ii......................... 6600/9944
2020-04-28T20:34:49.8547026Z .................................................................................................... 6800/9944
2020-04-28T20:34:58.1712831Z .......................................................................i............................ 6900/9944
2020-04-28T20:35:00.0681052Z .................................................................................................... 7000/9944
2020-04-28T20:35:02.2127401Z .................................................................................................... 7100/9944
---
2020-04-28T20:36:35.5271503Z .................................................................................................... 7900/9944
2020-04-28T20:36:39.8904409Z .................................................................................................... 8000/9944
2020-04-28T20:36:45.9730200Z ....................................................................................i............... 8100/9944
2020-04-28T20:36:53.9145339Z .................................................................................................... 8200/9944
2020-04-28T20:36:59.8884201Z .................................iiiiiiiiiii.i...................................................... 8300/9944
2020-04-28T20:37:13.4963467Z .................................................................................................... 8500/9944
2020-04-28T20:37:18.6204801Z .................................................................................................... 8600/9944
2020-04-28T20:37:32.3843145Z .................................................................................................... 8700/9944
2020-04-28T20:37:40.1375263Z .................................................................................................... 8800/9944
---
2020-04-28T20:39:23.8204325Z 6    |
2020-04-28T20:39:23.8204617Z 7    = note: `#[warn(incomplete_features)]` on by default
2020-04-28T20:39:23.8205058Z 8 
2020-04-28T20:39:23.8205774Z - error[E0741]: the types of const generic parameters must derive `PartialEq` and `Eq`
2020-04-28T20:39:23.8206553Z + error[E0741]: `&'static (dyn A + 'static)` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
2020-04-28T20:39:23.8207206Z 10   --> $DIR/issue-63322-forbid-dyn.rs:8:18
2020-04-28T20:39:23.8207534Z 11    |
2020-04-28T20:39:23.8207986Z 12 LL | fn test<const T: &'static dyn A>() {
2020-04-28T20:39:23.8208433Z 
2020-04-28T20:39:23.8209220Z The actual stderr differed from the expected stderr.
2020-04-28T20:39:23.8209220Z The actual stderr differed from the expected stderr.
2020-04-28T20:39:23.8209906Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn/issue-63322-forbid-dyn.stderr
2020-04-28T20:39:23.8210542Z To update references, rerun the tests and pass the `--bless` flag
2020-04-28T20:39:23.8211137Z To only update this specific test, also pass `--test-args const-generics/issues/issue-63322-forbid-dyn.rs`
2020-04-28T20:39:23.8211680Z error: 1 errors occurred comparing output.
2020-04-28T20:39:23.8211926Z status: exit code: 1
2020-04-28T20:39:23.8211926Z status: exit code: 1
2020-04-28T20:39:23.8213986Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn/auxiliary"
2020-04-28T20:39:23.8215789Z ------------------------------------------
2020-04-28T20:39:23.8216048Z 
2020-04-28T20:39:23.8216498Z ------------------------------------------
2020-04-28T20:39:23.8216952Z stderr:
---
2020-04-28T20:39:23.8219394Z    |            ^^^^^^^^^^^^^^
2020-04-28T20:39:23.8219616Z    |
2020-04-28T20:39:23.8219870Z    = note: `#[warn(incomplete_features)]` on by default
2020-04-28T20:39:23.8220280Z 
2020-04-28T20:39:23.8220849Z error[E0741]: `&'static (dyn A + 'static)` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
2020-04-28T20:39:23.8221543Z   --> /checkout/src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs:8:18
2020-04-28T20:39:23.8221856Z    |
2020-04-28T20:39:23.8222252Z LL | fn test<const T: &'static dyn A>() {
2020-04-28T20:39:23.8222834Z    |                  ^^^^^^^^^^^^^^ `&'static (dyn A + 'static)` doesn't derive both `PartialEq` and `Eq`
2020-04-28T20:39:23.8223418Z error: aborting due to previous error; 1 warning emitted
2020-04-28T20:39:23.8223633Z 
2020-04-28T20:39:23.8224068Z For more information about this error, try `rustc --explain E0741`.
2020-04-28T20:39:23.8224332Z 
---
2020-04-28T20:39:23.8231964Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-28T20:39:23.8232691Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-28T20:39:23.8241553Z 
2020-04-28T20:39:23.8242062Z 
2020-04-28T20:39:23.8245841Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-28T20:39:23.8251776Z 
2020-04-28T20:39:23.8251876Z 
2020-04-28T20:39:23.8253555Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-28T20:39:23.8253844Z Build completed unsuccessfully in 0:58:23
2020-04-28T20:39:23.8253844Z Build completed unsuccessfully in 0:58:23
2020-04-28T20:39:23.8306233Z == clock drift check ==
2020-04-28T20:39:23.8321347Z   local time: Tue Apr 28 20:39:23 UTC 2020
2020-04-28T20:39:23.9089317Z   network time: Tue, 28 Apr 2020 20:39:23 GMT
2020-04-28T20:39:24.4824740Z 
2020-04-28T20:39:24.4824740Z 
2020-04-28T20:39:24.4897588Z ##[error]Bash exited with code '1'.
2020-04-28T20:39:24.4909806Z ##[section]Finishing: Run build
2020-04-28T20:39:24.4956758Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71653/merge to s
2020-04-28T20:39:24.4962722Z Task         : Get sources
2020-04-28T20:39:24.4962993Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T20:39:24.4963244Z Version      : 1.0.0
2020-04-28T20:39:24.4963410Z Author       : Microsoft
2020-04-28T20:39:24.4963410Z Author       : Microsoft
2020-04-28T20:39:24.4963693Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T20:39:24.4963997Z ==============================================================================
2020-04-28T20:39:24.8262585Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T20:39:24.8306816Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71653/merge to s
2020-04-28T20:39:24.8390367Z Cleaning up task key
2020-04-28T20:39:24.8391326Z Start cleaning up orphan processes.
2020-04-28T20:39:24.8555770Z Terminate orphan process: pid (6442) (python)
2020-04-28T20:39:24.8912562Z ##[section]Finishing: Finalize Job
