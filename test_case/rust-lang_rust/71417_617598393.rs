plain
2020-04-22T06:20:32.7263240Z ========================== Starting Command Output ===========================
2020-04-22T06:20:32.7265421Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cc7a0f50-d13e-4110-897d-5c16c864da7a.sh
2020-04-22T06:20:32.7265610Z 
2020-04-22T06:20:32.7268386Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T06:20:32.7283031Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71417/merge to s
2020-04-22T06:20:32.7285833Z Task         : Get sources
2020-04-22T06:20:32.7286050Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T06:20:32.7286258Z Version      : 1.0.0
2020-04-22T06:20:32.7286403Z Author       : Microsoft
---
2020-04-22T06:20:34.0361861Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T06:20:34.0367739Z ##[command]git config gc.auto 0
2020-04-22T06:20:34.0370458Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T06:20:34.0373831Z ##[command]git config --get-all http.proxy
2020-04-22T06:20:34.0379890Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71417/merge:refs/remotes/pull/71417/merge
---
2020-04-22T06:22:23.6727566Z  ---> 318032b5f0e2
2020-04-22T06:22:23.6728186Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-22T06:22:23.6733613Z  ---> Using cache
2020-04-22T06:22:23.6733949Z  ---> d44a858fd1ce
2020-04-22T06:22:23.6734883Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-22T06:22:23.6742535Z  ---> 58b910f50f5a
2020-04-22T06:22:23.6742714Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-22T06:22:23.6746793Z  ---> Using cache
2020-04-22T06:22:23.6747116Z  ---> ee7702aadba1
---
2020-04-22T06:22:23.7093206Z Looks like docker image is the same as before, not uploading
2020-04-22T06:22:31.1595019Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T06:22:31.1850351Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-22T06:22:31.1873661Z == clock drift check ==
2020-04-22T06:22:31.1884117Z   local time: Wed Apr 22 06:22:31 UTC 2020
2020-04-22T06:22:31.4520286Z   network time: Wed, 22 Apr 2020 06:22:31 GMT
2020-04-22T06:22:31.4545138Z Starting sccache server...
2020-04-22T06:22:31.5233467Z configure: processing command line
2020-04-22T06:22:31.5233703Z configure: 
2020-04-22T06:22:31.5234478Z configure: rust.dist-src        := False
---
2020-04-22T06:27:32.1167915Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T06:27:33.5542497Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T06:27:34.9829129Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T06:27:36.6599865Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T06:27:44.2298715Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T06:27:47.4394107Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T06:27:51.5316741Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T06:27:55.4199289Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T06:28:02.6607352Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T06:48:12.7351694Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T06:48:14.2395552Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T06:48:15.8828163Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T06:48:17.4495569Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T06:48:25.3215290Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T06:48:29.0852491Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T06:48:33.2888999Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T06:48:37.2308005Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T06:48:44.9711477Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T07:09:03.0813288Z .................................................................................................... 1600/9912
2020-04-22T07:09:08.9871217Z .................................................................................................... 1700/9912
2020-04-22T07:09:12.6035626Z .................................................................................................... 1800/9912
2020-04-22T07:09:19.9292654Z .................................................................................................... 1900/9912
2020-04-22T07:09:26.6870712Z ..i................................................................................................. 2000/9912
2020-04-22T07:09:32.1153165Z ............................................................................................iiiii... 2100/9912
2020-04-22T07:09:49.1345489Z .................................................................................................... 2300/9912
2020-04-22T07:09:51.0288680Z .................................................................................................... 2400/9912
2020-04-22T07:09:52.9931783Z .................................................................................................... 2500/9912
2020-04-22T07:09:57.8891228Z .................................................................................................... 2600/9912
---
2020-04-22T07:12:29.8544373Z .................................................................................................... 5100/9912
2020-04-22T07:12:35.9616443Z .................................................................................................... 5200/9912
2020-04-22T07:12:40.2028362Z ...............i.................................................................................... 5300/9912
2020-04-22T07:12:48.7134614Z .....i.............................................................................................. 5400/9912
2020-04-22T07:12:53.2809954Z .....ii.ii........i...i............................................................................. 5500/9912
2020-04-22T07:13:00.0638710Z ....................................................i............................................... 5700/9912
2020-04-22T07:13:07.7666336Z ....................................................................................ii.............. 5800/9912
2020-04-22T07:13:13.6693164Z .......................i............................................................................ 5900/9912
2020-04-22T07:13:18.3603369Z .................................................................................................... 6000/9912
2020-04-22T07:13:18.3603369Z .................................................................................................... 6000/9912
2020-04-22T07:13:27.4676849Z .................................................................................................... 6100/9912
2020-04-22T07:13:36.1116396Z .................ii...i..ii...........i............................................................. 6200/9912
2020-04-22T07:13:49.4471033Z .................................................................................................... 6400/9912
2020-04-22T07:13:55.1780438Z .................................................................................................... 6500/9912
2020-04-22T07:13:55.1780438Z .................................................................................................... 6500/9912
2020-04-22T07:14:12.9660155Z ...............................................i..ii................................................ 6600/9912
2020-04-22T07:14:30.9941597Z .................................................................................................... 6800/9912
2020-04-22T07:14:33.0087855Z ................................................i................................................... 6900/9912
2020-04-22T07:14:34.6956350Z .................................................................................................... 7000/9912
2020-04-22T07:14:36.3601209Z ........................................................................................i........... 7100/9912
---
2020-04-22T07:16:00.0029420Z .................................................................................................... 7900/9912
2020-04-22T07:16:05.5068885Z .................................................................................................... 8000/9912
2020-04-22T07:16:10.4632632Z .......................................................i............................................ 8100/9912
2020-04-22T07:16:18.8501650Z .................................................................................................... 8200/9912
2020-04-22T07:16:23.3062842Z ...iiiiii.iiiii.i................................................................................... 8300/9912
2020-04-22T07:16:34.8517824Z .................................................................................................... 8500/9912
2020-04-22T07:16:41.7239975Z .................................................................................................... 8600/9912
2020-04-22T07:16:53.4813228Z .................................................................................................... 8700/9912
2020-04-22T07:16:59.1603108Z .................................................................................................... 8800/9912
---
2020-04-22T07:17:52.5029712Z .................................................................................................... 9300/9912
2020-04-22T07:17:56.6195969Z ....................i............................................................................... 9400/9912
2020-04-22T07:18:02.0316404Z .................................................................................................... 9500/9912
2020-04-22T07:18:07.9593010Z .................................................................................................... 9600/9912
2020-04-22T07:18:14.9605872Z .......................................................................................FFF.....FFF.. 9700/9912
2020-04-22T07:18:26.9022708Z ................i................................................................................... 9900/9912
2020-04-22T07:18:33.3873556Z ............
2020-04-22T07:18:33.3874342Z failures:
2020-04-22T07:18:33.3904050Z 
2020-04-22T07:18:33.3904050Z 
2020-04-22T07:18:33.3904828Z ---- [ui] ui/unsized-locals/autoderef.rs stdout ----
2020-04-22T07:18:33.3905131Z 
2020-04-22T07:18:33.3905620Z error: test compilation failed although it shouldn't!
2020-04-22T07:18:33.3905940Z status: exit code: 101
2020-04-22T07:18:33.3907490Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/auxiliary"
2020-04-22T07:18:33.3908840Z ------------------------------------------
2020-04-22T07:18:33.3909089Z 
2020-04-22T07:18:33.3909493Z ------------------------------------------
2020-04-22T07:18:33.3909784Z stderr:
---
2020-04-22T07:18:33.3911860Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3912138Z 
2020-04-22T07:18:33.3912387Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-22T07:18:33.3912614Z 
2020-04-22T07:18:33.3913307Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-22T07:18:33.3914179Z note: rustc 1.44.0-nightly (8093d37d4 2020-04-22) running on x86_64-unknown-linux-gnu
2020-04-22T07:18:33.3914477Z 
2020-04-22T07:18:33.3914477Z 
2020-04-22T07:18:33.3915118Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-22T07:18:33.3915749Z error: aborting due to previous error
2020-04-22T07:18:33.3915970Z 
2020-04-22T07:18:33.3916132Z 
2020-04-22T07:18:33.3916555Z ------------------------------------------
2020-04-22T07:18:33.3916555Z ------------------------------------------
2020-04-22T07:18:33.3916803Z 
2020-04-22T07:18:33.3916976Z 
2020-04-22T07:18:33.3917442Z ---- [ui] ui/unsized-locals/by-value-trait-object-safety-rpass.rs stdout ----
2020-04-22T07:18:33.3917746Z 
2020-04-22T07:18:33.3918276Z error: test compilation failed although it shouldn't!
2020-04-22T07:18:33.3918605Z status: exit code: 101
2020-04-22T07:18:33.3920157Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/by-value-trait-object-safety-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/auxiliary"
2020-04-22T07:18:33.3921916Z ------------------------------------------
2020-04-22T07:18:33.3922184Z 
2020-04-22T07:18:33.3922590Z ------------------------------------------
2020-04-22T07:18:33.3922881Z stderr:
---
2020-04-22T07:18:33.3924877Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3925128Z 
2020-04-22T07:18:33.3925397Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-22T07:18:33.3925621Z 
2020-04-22T07:18:33.3926190Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-22T07:18:33.3927036Z note: rustc 1.44.0-nightly (8093d37d4 2020-04-22) running on x86_64-unknown-linux-gnu
2020-04-22T07:18:33.3927350Z 
2020-04-22T07:18:33.3927350Z 
2020-04-22T07:18:33.3927941Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-22T07:18:33.3928547Z error: aborting due to previous error
2020-04-22T07:18:33.3928749Z 
2020-04-22T07:18:33.3928924Z 
2020-04-22T07:18:33.3929312Z ------------------------------------------
2020-04-22T07:18:33.3929312Z ------------------------------------------
2020-04-22T07:18:33.3929551Z 
2020-04-22T07:18:33.3929712Z 
2020-04-22T07:18:33.3930168Z ---- [ui] ui/unsized-locals/by-value-trait-object-safety-withdefault.rs stdout ----
2020-04-22T07:18:33.3930453Z 
2020-04-22T07:18:33.3930887Z error: test compilation failed although it shouldn't!
2020-04-22T07:18:33.3931211Z status: exit code: 101
2020-04-22T07:18:33.3932890Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/by-value-trait-object-safety-withdefault.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/auxiliary"
2020-04-22T07:18:33.3934324Z ------------------------------------------
2020-04-22T07:18:33.3934583Z 
2020-04-22T07:18:33.3934973Z ------------------------------------------
2020-04-22T07:18:33.3935238Z stderr:
---
2020-04-22T07:18:33.3940074Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3940393Z 
2020-04-22T07:18:33.3940684Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-22T07:18:33.3941108Z 
2020-04-22T07:18:33.3942055Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-22T07:18:33.3942888Z note: rustc 1.44.0-nightly (8093d37d4 2020-04-22) running on x86_64-unknown-linux-gnu
2020-04-22T07:18:33.3944374Z 
2020-04-22T07:18:33.3944374Z 
2020-04-22T07:18:33.3945369Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-22T07:18:33.3947243Z error: aborting due to previous error
2020-04-22T07:18:33.3947507Z 
2020-04-22T07:18:33.3947575Z 
2020-04-22T07:18:33.3947941Z ------------------------------------------
2020-04-22T07:18:33.3947941Z ------------------------------------------
2020-04-22T07:18:33.3948138Z 
2020-04-22T07:18:33.3948213Z 
2020-04-22T07:18:33.3948560Z ---- [ui] ui/unsized-locals/simple-unsized-locals.rs stdout ----
2020-04-22T07:18:33.3948703Z 
2020-04-22T07:18:33.3948978Z error: test compilation failed although it shouldn't!
2020-04-22T07:18:33.3949175Z status: exit code: 101
2020-04-22T07:18:33.3950503Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/simple-unsized-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/simple-unsized-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/simple-unsized-locals/auxiliary"
2020-04-22T07:18:33.3951764Z ------------------------------------------
2020-04-22T07:18:33.3951890Z 
2020-04-22T07:18:33.3952187Z ------------------------------------------
2020-04-22T07:18:33.3956017Z stderr:
---
2020-04-22T07:18:33.3957583Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3957741Z 
2020-04-22T07:18:33.3957890Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-22T07:18:33.3958024Z 
2020-04-22T07:18:33.3958487Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-22T07:18:33.3959042Z note: rustc 1.44.0-nightly (8093d37d4 2020-04-22) running on x86_64-unknown-linux-gnu
2020-04-22T07:18:33.3959390Z 
2020-04-22T07:18:33.3959390Z 
2020-04-22T07:18:33.3959869Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-22T07:18:33.3960236Z error: aborting due to previous error
2020-04-22T07:18:33.3960369Z 
2020-04-22T07:18:33.3960437Z 
2020-04-22T07:18:33.3960689Z ------------------------------------------
2020-04-22T07:18:33.3960689Z ------------------------------------------
2020-04-22T07:18:33.3960807Z 
2020-04-22T07:18:33.3960874Z 
2020-04-22T07:18:33.3961188Z ---- [ui] ui/unsized-locals/reference-unsized-locals.rs stdout ----
2020-04-22T07:18:33.3961335Z 
2020-04-22T07:18:33.3961605Z error: test compilation failed although it shouldn't!
2020-04-22T07:18:33.3961804Z status: exit code: 101
2020-04-22T07:18:33.3963149Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/reference-unsized-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/reference-unsized-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/reference-unsized-locals/auxiliary"
2020-04-22T07:18:33.3965897Z ------------------------------------------
2020-04-22T07:18:33.3966024Z 
2020-04-22T07:18:33.3966283Z ------------------------------------------
2020-04-22T07:18:33.3966425Z stderr:
---
2020-04-22T07:18:33.3968068Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3968232Z 
2020-04-22T07:18:33.3968396Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-22T07:18:33.3968532Z 
2020-04-22T07:18:33.3968990Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-22T07:18:33.3969539Z note: rustc 1.44.0-nightly (8093d37d4 2020-04-22) running on x86_64-unknown-linux-gnu
2020-04-22T07:18:33.3969708Z 
2020-04-22T07:18:33.3969708Z 
2020-04-22T07:18:33.3970144Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-22T07:18:33.3970525Z error: aborting due to previous error
2020-04-22T07:18:33.3970639Z 
2020-04-22T07:18:33.3970711Z 
2020-04-22T07:18:33.3970983Z ------------------------------------------
2020-04-22T07:18:33.3970983Z ------------------------------------------
2020-04-22T07:18:33.3971100Z 
2020-04-22T07:18:33.3971167Z 
2020-04-22T07:18:33.3971459Z ---- [ui] ui/unsized-locals/unsized-exprs-rpass.rs stdout ----
2020-04-22T07:18:33.3971599Z 
2020-04-22T07:18:33.3971889Z error: test compilation failed although it shouldn't!
2020-04-22T07:18:33.3972068Z status: exit code: 101
2020-04-22T07:18:33.3973374Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/unsized-exprs-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs-rpass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/unsized-exprs-rpass/auxiliary"
2020-04-22T07:18:33.3974463Z ------------------------------------------
2020-04-22T07:18:33.3974583Z 
2020-04-22T07:18:33.3974837Z ------------------------------------------
2020-04-22T07:18:33.3974978Z stderr:
---
2020-04-22T07:18:33.3976363Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3976521Z 
2020-04-22T07:18:33.3976686Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-22T07:18:33.3976822Z 
2020-04-22T07:18:33.3977421Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-22T07:18:33.3977998Z note: rustc 1.44.0-nightly (8093d37d4 2020-04-22) running on x86_64-unknown-linux-gnu
2020-04-22T07:18:33.3978177Z 
2020-04-22T07:18:33.3978177Z 
2020-04-22T07:18:33.3978639Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-22T07:18:33.3979038Z error: aborting due to previous error
2020-04-22T07:18:33.3979157Z 
2020-04-22T07:18:33.3979228Z 
2020-04-22T07:18:33.3979509Z ------------------------------------------
---
2020-04-22T07:18:33.3983620Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-22T07:18:33.3983919Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-22T07:18:33.3986776Z 
2020-04-22T07:18:33.3987097Z 
2020-04-22T07:18:33.3990923Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-22T07:18:33.3993723Z 
2020-04-22T07:18:33.3993907Z 
2020-04-22T07:18:33.3994513Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-22T07:18:33.3994954Z Build completed unsuccessfully in 0:54:30
2020-04-22T07:18:33.3994954Z Build completed unsuccessfully in 0:54:30
2020-04-22T07:18:33.4012902Z == clock drift check ==
2020-04-22T07:18:33.4036400Z   local time: Wed Apr 22 07:18:33 UTC 2020
2020-04-22T07:18:33.4396890Z   network time: Wed, 22 Apr 2020 07:18:33 GMT
2020-04-22T07:18:33.8676395Z 
2020-04-22T07:18:33.8676395Z 
2020-04-22T07:18:33.8756329Z ##[error]Bash exited with code '1'.
2020-04-22T07:18:33.8767741Z ##[section]Finishing: Run build
2020-04-22T07:18:33.8813545Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71417/merge to s
2020-04-22T07:18:33.8817969Z Task         : Get sources
2020-04-22T07:18:33.8818211Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T07:18:33.8818439Z Version      : 1.0.0
2020-04-22T07:18:33.8818605Z Author       : Microsoft
2020-04-22T07:18:33.8818605Z Author       : Microsoft
2020-04-22T07:18:33.8818851Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T07:18:33.8819128Z ==============================================================================
2020-04-22T07:18:34.1756684Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T07:18:34.1795464Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71417/merge to s
2020-04-22T07:18:34.1878283Z Cleaning up task key
2020-04-22T07:18:34.1879272Z Start cleaning up orphan processes.
2020-04-22T07:18:34.2151848Z Terminate orphan process: pid (7184) (python)
2020-04-22T07:18:34.2204812Z ##[section]Finishing: Finalize Job
