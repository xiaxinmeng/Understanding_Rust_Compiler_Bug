plain
2020-04-18T16:20:42.0217124Z ========================== Starting Command Output ===========================
2020-04-18T16:20:42.0220844Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f9fd2ed3-ce74-4436-8398-92ea8d717b63.sh
2020-04-18T16:20:42.0221202Z 
2020-04-18T16:20:42.0225009Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T16:20:42.0242014Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-18T16:20:42.0245245Z Task         : Get sources
2020-04-18T16:20:42.0245510Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T16:20:42.0245764Z Version      : 1.0.0
2020-04-18T16:20:42.0245957Z Author       : Microsoft
---
2020-04-18T16:20:43.0090631Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T16:20:43.0100581Z ##[command]git config gc.auto 0
2020-04-18T16:20:43.0106660Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T16:20:43.0112232Z ##[command]git config --get-all http.proxy
2020-04-18T16:20:43.0122488Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71289/merge:refs/remotes/pull/71289/merge
---
2020-04-18T16:22:42.5295170Z  ---> 318032b5f0e2
2020-04-18T16:22:42.5296558Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T16:22:42.5302085Z  ---> Using cache
2020-04-18T16:22:42.5302942Z  ---> d44a858fd1ce
2020-04-18T16:22:42.5356935Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T16:22:42.5359105Z  ---> 58b910f50f5a
2020-04-18T16:22:42.5359312Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T16:22:42.5359660Z  ---> Using cache
2020-04-18T16:22:42.5360002Z  ---> ee7702aadba1
---
2020-04-18T16:22:42.5732126Z Looks like docker image is the same as before, not uploading
2020-04-18T16:22:50.3043863Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T16:22:50.3292067Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T16:22:50.3319972Z == clock drift check ==
2020-04-18T16:22:50.3329086Z   local time: Sat Apr 18 16:22:50 UTC 2020
2020-04-18T16:22:50.6414029Z   network time: Sat, 18 Apr 2020 16:22:50 GMT
2020-04-18T16:22:50.6440548Z Starting sccache server...
2020-04-18T16:22:50.7278256Z configure: processing command line
2020-04-18T16:22:50.7278927Z configure: 
2020-04-18T16:22:50.7279953Z configure: rust.dist-src        := False
---
2020-04-18T16:27:56.6213261Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T16:27:58.0361855Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T16:27:59.5421950Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T16:28:00.6736345Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T16:28:09.1147049Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T16:28:11.6616836Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T16:28:15.9525069Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T16:28:19.9936619Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T16:28:28.7811169Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T16:48:59.7436676Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T16:49:01.1870644Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T16:49:02.8460698Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T16:49:02.8546236Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T16:49:12.3794577Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T16:49:14.2965160Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T16:49:18.5799407Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T16:49:22.5507604Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T16:49:32.3316073Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T17:10:14.4531577Z .................................................................................................... 1700/9905
2020-04-18T17:10:18.0720363Z .................................................................................................... 1800/9905
2020-04-18T17:10:25.8122069Z .................................................................................................i.. 1900/9905
2020-04-18T17:10:33.1356179Z .................................................................................................... 2000/9905
2020-04-18T17:10:39.0033228Z .......................................................................................iiiii........ 2100/9905
2020-04-18T17:10:57.0273520Z .................................................................................................... 2300/9905
2020-04-18T17:10:59.1098922Z .................................................................................................... 2400/9905
2020-04-18T17:11:01.2689833Z .................................................................................................... 2500/9905
2020-04-18T17:11:07.0886313Z .................................................................................................... 2600/9905
---
2020-04-18T17:13:35.4580782Z .................................................................................................... 4900/9905
2020-04-18T17:13:39.8585923Z ...............................................................i...............i.................... 5000/9905
2020-04-18T17:13:46.3426028Z .................................................................................................... 5100/9905
2020-04-18T17:13:52.5468779Z .................................................................................................... 5200/9905
2020-04-18T17:13:57.1087320Z .........i.........................................................................................i 5300/9905
2020-04-18T17:14:05.7379696Z ...................................................................................................i 5400/9905
2020-04-18T17:14:09.9862754Z i.ii........i...i................................................................................... 5500/9905
2020-04-18T17:14:16.9282371Z .............................................i...................................................... 5700/9905
2020-04-18T17:14:25.0515142Z .............................................................................ii..................... 5800/9905
2020-04-18T17:14:31.2002167Z ................i................................................................................... 5900/9905
2020-04-18T17:14:36.0875418Z .................................................................................................... 6000/9905
2020-04-18T17:14:36.0875418Z .................................................................................................... 6000/9905
2020-04-18T17:14:45.3207200Z .................................................................................................... 6100/9905
2020-04-18T17:14:55.0596777Z ..........ii...i..ii...........i.................................................................... 6200/9905
2020-04-18T17:15:06.8749700Z .................................................................................................... 6400/9905
2020-04-18T17:15:09.9161694Z .................................................................................................... 6500/9905
2020-04-18T17:15:09.9161694Z .................................................................................................... 6500/9905
2020-04-18T17:15:19.6297793Z ........................................i..ii....................................................... 6600/9905
2020-04-18T17:15:38.6715939Z .................................................................................................... 6800/9905
2020-04-18T17:15:40.5467132Z .........................................i.......................................................... 6900/9905
2020-04-18T17:15:42.3624027Z .................................................................................................... 7000/9905
2020-04-18T17:15:44.2335691Z .................................................................................i.................. 7100/9905
---
2020-04-18T17:17:05.5242956Z .................................................................................................... 7800/9905
2020-04-18T17:17:09.4066095Z .................................................................................................... 7900/9905
2020-04-18T17:17:15.1691635Z .................................................................................................... 8000/9905
2020-04-18T17:17:20.3170635Z ...............................................i.................................................... 8100/9905
2020-04-18T17:17:29.1267431Z ................................................................................................iiii 8200/9905
2020-04-18T17:17:34.0590056Z ii.iiiii.i.......................................................................................... 8300/9905
2020-04-18T17:17:45.7872588Z .................................................................................................... 8500/9905
2020-04-18T17:17:53.0407550Z .................................................................................................... 8600/9905
2020-04-18T17:18:05.2751488Z .................................................................................................... 8700/9905
2020-04-18T17:18:11.2472313Z .................................................................................................... 8800/9905
---
2020-04-18T17:20:09.8202414Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-18T17:20:09.8371697Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:20:10.0282429Z 
2020-04-18T17:20:10.0282664Z running 186 tests
2020-04-18T17:20:12.6087940Z iiii......i.............ii.i..........i.............................i..i..................i....i.... 100/186
2020-04-18T17:20:14.9042897Z ........i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-18T17:20:14.9045022Z 
2020-04-18T17:20:14.9052522Z  finished in 5.068
2020-04-18T17:20:14.9058173Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-18T17:20:14.9236009Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T17:20:16.8994003Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-18T17:20:16.9183074Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:20:17.0607699Z 
2020-04-18T17:20:17.0608133Z running 9 tests
2020-04-18T17:20:17.0609620Z iiiiiiiii
2020-04-18T17:20:17.0611583Z 
2020-04-18T17:20:17.0620769Z  finished in 0.143
2020-04-18T17:20:17.0621616Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-18T17:20:17.0790694Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T17:20:35.0629402Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-18T17:20:35.0845160Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-18T17:20:35.2633578Z 
2020-04-18T17:20:35.2633830Z running 115 tests
2020-04-18T17:20:47.5908452Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-18T17:20:49.1519911Z ...iiii.....ii.
2020-04-18T17:20:49.1521250Z 
2020-04-18T17:20:49.1523665Z  finished in 14.067
2020-04-18T17:20:49.1535452Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-18T17:20:49.1540035Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-18T17:27:01.2254735Z failures:
2020-04-18T17:27:01.2254966Z 
2020-04-18T17:27:01.2255956Z ---- [rustdoc] rustdoc/issue-23511.rs stdout ----
2020-04-18T17:27:01.2256260Z 
2020-04-18T17:27:01.2256536Z error: htmldocck failed!
2020-04-18T17:27:01.2256893Z status: exit code: 1
2020-04-18T17:27:01.2258198Z command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511" "/checkout/src/test/rustdoc/issue-23511.rs"
2020-04-18T17:27:01.2259844Z ------------------------------------------
2020-04-18T17:27:01.2260123Z 
2020-04-18T17:27:01.2260727Z ------------------------------------------
2020-04-18T17:27:01.2261075Z stderr:
2020-04-18T17:27:01.2261075Z stderr:
2020-04-18T17:27:01.2261707Z ------------------------------------------
2020-04-18T17:27:01.2262072Z 9: @has check failed
2020-04-18T17:27:01.2262408Z  `PATTERN` did not match
2020-04-18T17:27:01.2263193Z          // @has search-index.js foo
2020-04-18T17:27:01.2263718Z Encountered 1 errors
2020-04-18T17:27:01.2263930Z 
2020-04-18T17:27:01.2264579Z ------------------------------------------
2020-04-18T17:27:01.2264853Z 
---
2020-04-18T17:27:01.2267803Z 
2020-04-18T17:27:01.2267960Z 
2020-04-18T17:27:01.2268786Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T17:27:01.2269451Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-18T17:27:01.2276037Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T17:27:01.2280363Z 
2020-04-18T17:27:01.2280528Z 
2020-04-18T17:27:01.2285230Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T17:27:01.2285872Z Build completed unsuccessfully in 1:02:33
2020-04-18T17:27:01.2285872Z Build completed unsuccessfully in 1:02:33
2020-04-18T17:27:01.2312664Z == clock drift check ==
2020-04-18T17:27:01.2341863Z   local time: Sat Apr 18 17:27:01 UTC 2020
2020-04-18T17:27:01.7534094Z   network time: Sat, 18 Apr 2020 17:27:01 GMT
2020-04-18T17:27:03.1002657Z 
2020-04-18T17:27:03.1002657Z 
2020-04-18T17:27:03.1064360Z ##[error]Bash exited with code '1'.
2020-04-18T17:27:03.1077428Z ##[section]Finishing: Run build
2020-04-18T17:27:03.1121821Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-18T17:27:03.1126377Z Task         : Get sources
2020-04-18T17:27:03.1126661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T17:27:03.1126937Z Version      : 1.0.0
2020-04-18T17:27:03.1127126Z Author       : Microsoft
2020-04-18T17:27:03.1127126Z Author       : Microsoft
2020-04-18T17:27:03.1127423Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T17:27:03.1127894Z ==============================================================================
2020-04-18T17:27:03.4180163Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T17:27:03.4229807Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71289/merge to s
2020-04-18T17:27:03.4315774Z Cleaning up task key
2020-04-18T17:27:03.4316907Z Start cleaning up orphan processes.
2020-04-18T17:27:03.4477045Z Terminate orphan process: pid (3674) (python)
2020-04-18T17:27:03.4625398Z ##[section]Finishing: Finalize Job
