plain
2020-04-06T05:09:18.9321153Z ========================== Starting Command Output ===========================
2020-04-06T05:09:18.9323421Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/542c9d61-ae71-45c7-9757-d31aedc8ec3e.sh
2020-04-06T05:09:18.9323637Z 
2020-04-06T05:09:18.9328102Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T05:09:18.9345305Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-06T05:09:18.9349098Z Task         : Get sources
2020-04-06T05:09:18.9349334Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T05:09:18.9349560Z Version      : 1.0.0
2020-04-06T05:09:18.9349756Z Author       : Microsoft
---
2020-04-06T05:09:20.1536268Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T05:09:20.1543339Z ##[command]git config gc.auto 0
2020-04-06T05:09:20.1547842Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T05:09:20.1551211Z ##[command]git config --get-all http.proxy
2020-04-06T05:09:20.1557741Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-06T05:11:16.5441804Z Looks like docker image is the same as before, not uploading
2020-04-06T05:11:23.9696416Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T05:11:24.0085321Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T05:11:24.0128627Z == clock drift check ==
2020-04-06T05:11:24.1720927Z   local time: Mon Apr  6 05:11:24 UTC 2020
2020-04-06T05:11:24.1721211Z   network time: Mon, 06 Apr 2020 05:11:24 GMT
2020-04-06T05:11:24.1748518Z Starting sccache server...
2020-04-06T05:11:24.2473350Z configure: processing command line
2020-04-06T05:11:24.2473945Z configure: 
2020-04-06T05:11:24.2474856Z configure: rust.dist-src        := False
---
2020-04-06T05:15:57.9664114Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T05:15:59.7663893Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T05:16:00.7858161Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T05:16:01.9327437Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T05:16:09.8331199Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T05:16:11.9015724Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T05:16:15.8264447Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T05:16:19.5997098Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T05:16:28.0265270Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T05:39:43.3838527Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T05:39:45.0499930Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T05:39:46.8409514Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T05:39:48.7982393Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T05:39:58.1897616Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T05:40:01.2506008Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T05:40:06.3490697Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T05:40:11.2748555Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T05:40:20.5364753Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T06:03:37.1914167Z .................................................................................................... 1700/9877
2020-04-06T06:03:41.1566376Z .................................................................................................... 1800/9877
2020-04-06T06:03:50.0146500Z .................................................................................................i.. 1900/9877
2020-04-06T06:03:57.9214504Z .................................................................................................... 2000/9877
2020-04-06T06:04:04.3953070Z .......................................................................................iiiii........ 2100/9877
2020-04-06T06:04:25.8650091Z .................................................................................................... 2300/9877
2020-04-06T06:04:28.0809516Z .................................................................................................... 2400/9877
2020-04-06T06:04:30.3765786Z .................................................................................................... 2500/9877
2020-04-06T06:04:36.3871843Z .................................................................................................... 2600/9877
---
2020-04-06T06:07:34.2644009Z .............................................................i...............i...................... 5000/9877
2020-04-06T06:07:41.6733126Z .................................................................................................... 5100/9877
2020-04-06T06:07:49.2415621Z .................................................................................................... 5200/9877
2020-04-06T06:07:54.4967089Z ......i............................................................................................. 5300/9877
2020-04-06T06:08:04.4892033Z ...............................................................................................ii.ii 5400/9877
2020-04-06T06:08:09.4041934Z ........i...i....................................................................................... 5500/9877
2020-04-06T06:08:18.0528040Z ........................................i........................................................... 5700/9877
2020-04-06T06:08:18.0528040Z ........................................i........................................................... 5700/9877
2020-04-06T06:08:27.7497767Z ............................................................ii.....................................i 5800/9877
2020-04-06T06:08:40.0260973Z .................................................................................................... 6000/9877
2020-04-06T06:08:40.0260973Z .................................................................................................... 6000/9877
2020-04-06T06:08:49.8058573Z .............................................................................................ii...i. 6100/9877
2020-04-06T06:09:01.7249872Z .ii...........i..................................................................................... 6200/9877
2020-04-06T06:09:18.6722939Z .................................................................................................... 6400/9877
2020-04-06T06:09:24.1978896Z .................................................................................................... 6500/9877
2020-04-06T06:09:24.1978896Z .................................................................................................... 6500/9877
2020-04-06T06:09:38.8890929Z .......................i..ii........................................................................ 6600/9877
2020-04-06T06:09:59.1958278Z .................................................................................................... 6800/9877
2020-04-06T06:10:01.1725001Z .......................i............................................................................ 6900/9877
2020-04-06T06:10:03.1708673Z .................................................................................................... 7000/9877
2020-04-06T06:10:05.2902579Z ..............................................................i..................................... 7100/9877
---
2020-04-06T06:11:32.0524818Z .................................................................................................... 7800/9877
2020-04-06T06:11:35.8299891Z .................................................................................................... 7900/9877
2020-04-06T06:11:40.7481863Z .................................................................................................... 8000/9877
2020-04-06T06:11:47.2239666Z ..........................i......................................................................... 8100/9877
2020-04-06T06:11:54.2927972Z ...........................................................................iiiiiiiiii.i............. 8200/9877
2020-04-06T06:12:07.8172165Z ...................i......i......................................................................... 8400/9877
2020-04-06T06:12:11.8080107Z .................................................................................................... 8500/9877
2020-04-06T06:12:21.0062445Z .................................................................................................... 8600/9877
2020-04-06T06:12:31.7872834Z .................................................................................................... 8700/9877
---
2020-04-06T06:14:11.6265640Z 
2020-04-06T06:14:11.6266508Z ---- [ui] ui/consts/assoc_const_generic_impl.rs stdout ----
2020-04-06T06:14:11.6266924Z diff of stderr:
2020-04-06T06:14:11.6267129Z 
2020-04-06T06:14:11.6267401Z 18 LL |         let () = Self::I_AM_ZERO_SIZED;
2020-04-06T06:14:11.6268164Z 20 
2020-04-06T06:14:11.6268764Z - error: aborting due to previous error
2020-04-06T06:14:11.6269092Z + error: erroneous constant encountered
2020-04-06T06:14:11.6269559Z +   --> $DIR/assoc_const_generic_impl.rs:13:18
2020-04-06T06:14:11.6269559Z +   --> $DIR/assoc_const_generic_impl.rs:13:18
2020-04-06T06:14:11.6269832Z +    |
2020-04-06T06:14:11.6270114Z + LL |         let () = Self::I_AM_ZERO_SIZED;
2020-04-06T06:14:11.6270656Z + 
2020-04-06T06:14:11.6270905Z + error: aborting due to 2 previous errors
2020-04-06T06:14:11.6271133Z 22 
2020-04-06T06:14:11.6271307Z 23 
2020-04-06T06:14:11.6271307Z 23 
2020-04-06T06:14:11.6271485Z 
2020-04-06T06:14:11.6271640Z 
2020-04-06T06:14:11.6271877Z The actual stderr differed from the expected stderr.
2020-04-06T06:14:11.6272545Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assoc_const_generic_impl/assoc_const_generic_impl.stderr
2020-04-06T06:14:11.6273189Z To update references, rerun the tests and pass the `--bless` flag
2020-04-06T06:14:11.6273795Z To only update this specific test, also pass `--test-args consts/assoc_const_generic_impl.rs`
2020-04-06T06:14:11.6274325Z error: 1 errors occurred comparing output.
2020-04-06T06:14:11.6274600Z status: exit code: 1
2020-04-06T06:14:11.6274600Z status: exit code: 1
2020-04-06T06:14:11.6276327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/assoc_const_generic_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assoc_const_generic_impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assoc_const_generic_impl/auxiliary"
2020-04-06T06:14:11.6277906Z ------------------------------------------
2020-04-06T06:14:11.6278144Z 
2020-04-06T06:14:11.6278551Z ------------------------------------------
2020-04-06T06:14:11.6278811Z stderr:
2020-04-06T06:14:11.6278811Z stderr:
2020-04-06T06:14:11.6279208Z ------------------------------------------
2020-04-06T06:14:11.6279531Z warning: any use of this value will cause an error
2020-04-06T06:14:11.6280044Z   --> /checkout/src/test/ui/consts/assoc_const_generic_impl.rs:11:34
2020-04-06T06:14:11.6280378Z    |
2020-04-06T06:14:11.6280720Z LL |     const I_AM_ZERO_SIZED: ()  = [()][std::mem::size_of::<Self>()]; //~ WARN any use of this value
2020-04-06T06:14:11.6281675Z    |                                  |
2020-04-06T06:14:11.6282037Z    |                                  index out of bounds: the len is 1 but the index is 4
2020-04-06T06:14:11.6282370Z    |
2020-04-06T06:14:11.6282594Z note: the lint level is defined here
---
2020-04-06T06:14:11.6284063Z 
2020-04-06T06:14:11.6284283Z error: erroneous constant encountered
2020-04-06T06:14:11.6285053Z   --> /checkout/src/test/ui/consts/assoc_const_generic_impl.rs:13:18
2020-04-06T06:14:11.6285413Z    |
2020-04-06T06:14:11.6285764Z LL |         let () = Self::I_AM_ZERO_SIZED; //~ ERROR erroneous constant encountered
2020-04-06T06:14:11.6287829Z 
2020-04-06T06:14:11.6288240Z error: erroneous constant encountered
2020-04-06T06:14:11.6288868Z   --> /checkout/src/test/ui/consts/assoc_const_generic_impl.rs:13:18
2020-04-06T06:14:11.6289185Z    |
2020-04-06T06:14:11.6289185Z    |
2020-04-06T06:14:11.6289499Z LL |         let () = Self::I_AM_ZERO_SIZED; //~ ERROR erroneous constant encountered
2020-04-06T06:14:11.6290892Z 
2020-04-06T06:14:11.6291032Z error: aborting due to 2 previous errors
2020-04-06T06:14:11.6291175Z 
2020-04-06T06:14:11.6291248Z 
2020-04-06T06:14:11.6291248Z 
2020-04-06T06:14:11.6291597Z ------------------------------------------
2020-04-06T06:14:11.6291726Z 
2020-04-06T06:14:11.6291797Z 
2020-04-06T06:14:11.6292156Z ---- [ui] ui/consts/const-eval/index-out-of-bounds-never-type.rs stdout ----
2020-04-06T06:14:11.6292365Z diff of stderr:
2020-04-06T06:14:11.6292456Z 
2020-04-06T06:14:11.6292626Z 18 LL |     let _ = PrintName::<T>::VOID;
2020-04-06T06:14:11.6292971Z 20 
2020-04-06T06:14:11.6293259Z - error: aborting due to previous error
2020-04-06T06:14:11.6293452Z + error: erroneous constant encountered
2020-04-06T06:14:11.6293805Z +   --> $DIR/index-out-of-bounds-never-type.rs:15:13
2020-04-06T06:14:11.6293805Z +   --> $DIR/index-out-of-bounds-never-type.rs:15:13
2020-04-06T06:14:11.6293980Z +    |
2020-04-06T06:14:11.6294156Z + LL |     let _ = PrintName::<T>::VOID;
2020-04-06T06:14:11.6294491Z + 
2020-04-06T06:14:11.6294653Z + error: aborting due to 2 previous errors
2020-04-06T06:14:11.6294797Z 22 
2020-04-06T06:14:11.6294887Z 23 
2020-04-06T06:14:11.6294887Z 23 
2020-04-06T06:14:11.6294981Z 
2020-04-06T06:14:11.6295052Z 
2020-04-06T06:14:11.6295203Z The actual stderr differed from the expected stderr.
2020-04-06T06:14:11.6296070Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/index-out-of-bounds-never-type.stderr
2020-04-06T06:14:11.6296751Z To update references, rerun the tests and pass the `--bless` flag
2020-04-06T06:14:11.6297438Z To only update this specific test, also pass `--test-args consts/const-eval/index-out-of-bounds-never-type.rs`
2020-04-06T06:14:11.6297834Z error: 1 errors occurred comparing output.
2020-04-06T06:14:11.6298021Z status: exit code: 1
2020-04-06T06:14:11.6298021Z status: exit code: 1
2020-04-06T06:14:11.6299654Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/auxiliary"
2020-04-06T06:14:11.6301782Z ------------------------------------------
2020-04-06T06:14:11.6301916Z 
2020-04-06T06:14:11.6302183Z ------------------------------------------
2020-04-06T06:14:11.6302330Z stderr:
2020-04-06T06:14:11.6302330Z stderr:
2020-04-06T06:14:11.6304176Z ------------------------------------------
2020-04-06T06:14:11.6304446Z warning: any use of this value will cause an error
2020-04-06T06:14:11.6305147Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:10:61
2020-04-06T06:14:11.6305581Z    |
2020-04-06T06:14:11.6305809Z LL |     const VOID: ! = { let x = 0 * std::mem::size_of::<T>(); [][x] };
2020-04-06T06:14:11.6306598Z    |                                                             |
2020-04-06T06:14:11.6306975Z    |                                                             index out of bounds: the len is 0 but the index is 0
2020-04-06T06:14:11.6307278Z    |
2020-04-06T06:14:11.6307427Z note: the lint level is defined here
---
2020-04-06T06:14:11.6308505Z 
2020-04-06T06:14:11.6308837Z error: erroneous constant encountered
2020-04-06T06:14:11.6309259Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:15:13
2020-04-06T06:14:11.6309474Z    |
2020-04-06T06:14:11.6309645Z LL |     let _ = PrintName::<T>::VOID;
2020-04-06T06:14:11.6309959Z 
2020-04-06T06:14:11.6310096Z error: erroneous constant encountered
2020-04-06T06:14:11.6310542Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:15:13
2020-04-06T06:14:11.6310755Z    |
2020-04-06T06:14:11.6310755Z    |
2020-04-06T06:14:11.6310906Z LL |     let _ = PrintName::<T>::VOID;
2020-04-06T06:14:11.6311237Z 
2020-04-06T06:14:11.6311375Z error: aborting due to 2 previous errors
2020-04-06T06:14:11.6311501Z 
2020-04-06T06:14:11.6311589Z 
---
2020-04-06T06:14:11.6314655Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-06T06:14:11.6315045Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-06T06:14:11.6315217Z 
2020-04-06T06:14:11.6315288Z 
2020-04-06T06:14:11.6318114Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-06T06:14:11.6320122Z 
2020-04-06T06:14:11.6320211Z 
2020-04-06T06:14:11.6322566Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-06T06:14:11.6322858Z Build completed unsuccessfully in 1:01:23
2020-04-06T06:14:11.6322858Z Build completed unsuccessfully in 1:01:23
2020-04-06T06:14:11.6352104Z == clock drift check ==
2020-04-06T06:14:11.6368932Z   local time: Mon Apr  6 06:14:11 UTC 2020
2020-04-06T06:14:11.8045699Z   network time: Mon, 06 Apr 2020 06:14:11 GMT
2020-04-06T06:14:12.7134157Z 
2020-04-06T06:14:12.7134157Z 
2020-04-06T06:14:12.7206837Z ##[error]Bash exited with code '1'.
2020-04-06T06:14:12.7218126Z ##[section]Finishing: Run build
2020-04-06T06:14:12.7258448Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-06T06:14:12.7263000Z Task         : Get sources
2020-04-06T06:14:12.7263253Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T06:14:12.7263507Z Version      : 1.0.0
2020-04-06T06:14:12.7263670Z Author       : Microsoft
2020-04-06T06:14:12.7263670Z Author       : Microsoft
2020-04-06T06:14:12.7264117Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T06:14:12.7264452Z ==============================================================================
2020-04-06T06:14:13.0101789Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T06:14:13.0138531Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-06T06:14:13.0214013Z Cleaning up task key
2020-04-06T06:14:13.0214990Z Start cleaning up orphan processes.
2020-04-06T06:14:13.0360005Z Terminate orphan process: pid (3964) (python)
2020-04-06T06:14:13.0492971Z ##[section]Finishing: Finalize Job
