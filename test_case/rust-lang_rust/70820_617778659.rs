plain
2020-04-22T12:23:53.6827526Z ========================== Starting Command Output ===========================
2020-04-22T12:23:53.6829906Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0cac88b3-e10c-408d-86fa-711c8a14913d.sh
2020-04-22T12:23:53.6830155Z 
2020-04-22T12:23:53.6833766Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T12:23:53.6854364Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-22T12:23:53.6857416Z Task         : Get sources
2020-04-22T12:23:53.6857695Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T12:23:53.6857964Z Version      : 1.0.0
2020-04-22T12:23:53.6858162Z Author       : Microsoft
---
2020-04-22T12:23:54.6807072Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T12:23:54.6813312Z ##[command]git config gc.auto 0
2020-04-22T12:23:54.6818209Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T12:23:54.6832844Z ##[command]git config --get-all http.proxy
2020-04-22T12:23:54.6840769Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-22T12:27:21.5792668Z  ---> 318032b5f0e2
2020-04-22T12:27:21.5793703Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T12:27:21.5798167Z  ---> Using cache
2020-04-22T12:27:21.5798536Z  ---> d44a858fd1ce
2020-04-22T12:27:21.5799455Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T12:27:21.5806695Z  ---> 58b910f50f5a
2020-04-22T12:27:21.5806935Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T12:27:21.5810078Z  ---> Using cache
2020-04-22T12:27:21.5810749Z  ---> ee7702aadba1
---
2020-04-22T12:27:21.6328650Z Looks like docker image is the same as before, not uploading
2020-04-22T12:27:30.2274575Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T12:27:30.2560068Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T12:27:30.2592465Z == clock drift check ==
2020-04-22T12:27:30.2600030Z   local time: Wed Apr 22 12:27:30 UTC 2020
2020-04-22T12:27:30.5488609Z   network time: Wed, 22 Apr 2020 12:27:30 GMT
2020-04-22T12:27:30.5514070Z Starting sccache server...
2020-04-22T12:27:30.6361445Z configure: processing command line
2020-04-22T12:27:30.6365634Z configure: 
2020-04-22T12:27:30.6367741Z configure: rust.dist-src        := False
---
2020-04-22T12:32:30.4199375Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T12:32:31.7489907Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T12:32:33.2461838Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T12:32:33.9507810Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T12:32:42.8530512Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T12:32:44.5532851Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T12:32:48.6551987Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T12:32:52.4483827Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T12:33:02.2610529Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T12:54:08.4546901Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T12:54:09.9896880Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T12:54:11.7212149Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T12:54:11.9768321Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T12:54:21.8223297Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T12:54:24.2301057Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T12:54:28.6640042Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T12:54:32.9792305Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T12:54:42.6480086Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T13:16:35.2999051Z .................................................................................................... 1600/9913
2020-04-22T13:16:41.3108195Z .................................................................................................... 1700/9913
2020-04-22T13:16:45.4290686Z .................................................................................................... 1800/9913
2020-04-22T13:16:53.6360867Z .................................................................................................... 1900/9913
2020-04-22T13:17:01.1278111Z ..i................................................................................................. 2000/9913
2020-04-22T13:17:07.2223129Z ............................................................................................iiiii... 2100/9913
2020-04-22T13:17:25.9184148Z .................................................................................................... 2300/9913
2020-04-22T13:17:28.0794024Z .................................................................................................... 2400/9913
2020-04-22T13:17:30.2741923Z .................................................................................................... 2500/9913
2020-04-22T13:17:35.8886673Z .................................................................................................... 2600/9913
---
2020-04-22T13:20:24.2997342Z .................................................................................................... 5100/9913
2020-04-22T13:20:31.0188583Z .................................................................................................... 5200/9913
2020-04-22T13:20:35.7833359Z ...............i.................................................................................... 5300/9913
2020-04-22T13:20:44.8233490Z .....i.............................................................................................. 5400/9913
2020-04-22T13:20:49.8004424Z .....ii.ii........i...i............................................................................. 5500/9913
2020-04-22T13:20:57.4782359Z ....................................................i............................................... 5700/9913
2020-04-22T13:21:06.0111935Z .....................................................................................ii............. 5800/9913
2020-04-22T13:21:12.8008389Z ........................i........................................................................... 5900/9913
2020-04-22T13:21:18.2158677Z .................................................................................................... 6000/9913
2020-04-22T13:21:18.2158677Z .................................................................................................... 6000/9913
2020-04-22T13:21:28.4003158Z .................................................................................................... 6100/9913
2020-04-22T13:21:38.2210235Z ..................ii...i..ii...........i............................................................ 6200/9913
2020-04-22T13:21:52.3971537Z .................................................................................................... 6400/9913
2020-04-22T13:21:55.7201915Z .................................................................................................... 6500/9913
2020-04-22T13:21:55.7201915Z .................................................................................................... 6500/9913
2020-04-22T13:22:04.7590085Z ................................................i..ii............................................... 6600/9913
2020-04-22T13:22:26.5358787Z .................................................................................................... 6800/9913
2020-04-22T13:22:28.7383035Z .................................................i.................................................. 6900/9913
2020-04-22T13:22:30.7348200Z .................................................................................................... 7000/9913
2020-04-22T13:22:32.7481733Z .........................................................................................i.......... 7100/9913
---
2020-04-22T13:24:05.6413731Z .................................................................................................... 7900/9913
2020-04-22T13:24:11.7775660Z .................................................................................................... 8000/9913
2020-04-22T13:24:17.2956717Z .......................................................i............................................ 8100/9913
2020-04-22T13:24:26.8776716Z .................................................................................................... 8200/9913
2020-04-22T13:24:31.9797029Z ....iiiiii.iiiii.i.................................................................................. 8300/9913
2020-04-22T13:24:44.7195867Z .................................................................................................... 8500/9913
2020-04-22T13:24:52.0544055Z .................................................................................................... 8600/9913
2020-04-22T13:25:04.9331951Z .................................................................................................... 8700/9913
2020-04-22T13:25:11.0655166Z .................................................................................................... 8800/9913
---
2020-04-22T13:27:16.8868061Z 5     scope 1 {
2020-04-22T13:27:16.8868336Z 6     }
2020-04-22T13:27:16.8868762Z 7 
2020-04-22T13:27:16.8868971Z 
2020-04-22T13:27:16.8869235Z 8     bb0: {
2020-04-22T13:27:16.8870117Z -         unreachable;                     // bb0[0]: scope 0 at $DIR/remove-never-const.rs:19:13: 19:33
2020-04-22T13:27:16.8871047Z +         unreachable;                     // bb0[0]: scope 0 at $DIR/remove-never-const.rs:20:13: 20:33
2020-04-22T13:27:16.8871756Z 11 }
2020-04-22T13:27:16.8871983Z 12 
2020-04-22T13:27:16.8872200Z 
2020-04-22T13:27:16.8872200Z 
2020-04-22T13:27:16.8873179Z thread '[mir-opt] mir-opt/remove-never-const.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/remove-never-const/rustc.no_codegen.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3165:25
2020-04-22T13:27:16.8874238Z 
2020-04-22T13:27:16.8874457Z 
2020-04-22T13:27:16.8874697Z failures:
2020-04-22T13:27:16.8875218Z     [mir-opt] mir-opt/remove-never-const.rs
2020-04-22T13:27:16.8875218Z     [mir-opt] mir-opt/remove-never-const.rs
2020-04-22T13:27:16.8875494Z 
2020-04-22T13:27:16.8876099Z test result: FAILED. 89 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-22T13:27:16.8876463Z 
2020-04-22T13:27:16.8880404Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T13:27:16.8883294Z 
2020-04-22T13:27:16.8883567Z 
2020-04-22T13:27:16.8887457Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T13:27:16.8890567Z 
2020-04-22T13:27:16.8890769Z 
2020-04-22T13:27:16.8891430Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T13:27:16.8891898Z Build completed unsuccessfully in 0:58:15
2020-04-22T13:27:16.8891898Z Build completed unsuccessfully in 0:58:15
2020-04-22T13:27:16.8937428Z == clock drift check ==
2020-04-22T13:27:16.8953553Z   local time: Wed Apr 22 13:27:16 UTC 2020
2020-04-22T13:27:17.1909943Z   network time: Wed, 22 Apr 2020 13:27:17 GMT
2020-04-22T13:27:19.5590343Z 
2020-04-22T13:27:19.5590343Z 
2020-04-22T13:27:19.5664315Z ##[error]Bash exited with code '1'.
2020-04-22T13:27:19.5677409Z ##[section]Finishing: Run build
2020-04-22T13:27:19.5725866Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-22T13:27:19.5730727Z Task         : Get sources
2020-04-22T13:27:19.5731032Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T13:27:19.5731310Z Version      : 1.0.0
2020-04-22T13:27:19.5731662Z Author       : Microsoft
2020-04-22T13:27:19.5731662Z Author       : Microsoft
2020-04-22T13:27:19.5731985Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T13:27:19.5732339Z ==============================================================================
2020-04-22T13:27:19.9096547Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T13:27:19.9148686Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-22T13:27:19.9242093Z Cleaning up task key
2020-04-22T13:27:19.9243655Z Start cleaning up orphan processes.
2020-04-22T13:27:19.9421276Z Terminate orphan process: pid (4173) (python)
2020-04-22T13:27:19.9679576Z ##[section]Finishing: Finalize Job
