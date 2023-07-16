plain
2020-03-27T06:50:35.3716382Z ========================== Starting Command Output ===========================
2020-03-27T06:50:35.3718784Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/155fca45-d1a7-4bcc-9f01-d8b7c2eed593.sh
2020-03-27T06:50:35.3719076Z 
2020-03-27T06:50:35.3722939Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T06:50:35.3783253Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-27T06:50:35.3786214Z Task         : Get sources
2020-03-27T06:50:35.3786502Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T06:50:35.3786796Z Version      : 1.0.0
2020-03-27T06:50:35.3786983Z Author       : Microsoft
---
2020-03-27T06:50:36.3749234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T06:50:36.3754346Z ##[command]git config gc.auto 0
2020-03-27T06:50:36.3757852Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T06:50:36.3761165Z ##[command]git config --get-all http.proxy
2020-03-27T06:50:36.3766883Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70458/merge:refs/remotes/pull/70458/merge
---
2020-03-27T06:57:35.7390130Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T06:57:37.2143845Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T06:57:37.4425001Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T06:57:46.6337555Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T06:57:47.6802025Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T06:57:48.9859186Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T06:57:57.0096611Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T06:58:26.6852419Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T06:58:58.6602312Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T07:00:40.2469613Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T07:19:00.1225249Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T07:19:02.0685487Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T07:19:03.0935861Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T07:19:14.0000932Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T07:19:16.1113065Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T07:19:17.5890135Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T07:19:28.0029936Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T07:20:05.7967448Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T07:20:48.5747886Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T07:22:54.0450851Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T07:44:13.2274224Z .................................................................................................... 1700/9849
2020-03-27T07:44:17.1390292Z .................................................................................................... 1800/9849
2020-03-27T07:44:26.8549543Z .........................................................................................i.......... 1900/9849
2020-03-27T07:44:32.4174208Z ......F............................................................................................. 2000/9849
2020-03-27T07:44:38.5169959Z ...............................................................................iiiii................ 2100/9849
2020-03-27T07:44:58.6249936Z .................................................................................................... 2300/9849
2020-03-27T07:45:00.7192561Z .................................................................................................... 2400/9849
2020-03-27T07:45:03.0507230Z .................................................................................................... 2500/9849
2020-03-27T07:45:09.3471731Z ..................................................................................................F. 2600/9849
2020-03-27T07:45:09.3471731Z ..................................................................................................F. 2600/9849
2020-03-27T07:45:14.7860896Z FFFFF...F.FF.F.....F................................................................................ 2700/9849
2020-03-27T07:45:28.1513536Z ................................................i................................................... 2900/9849
2020-03-27T07:45:33.9346919Z .................................................................................................... 3000/9849
2020-03-27T07:45:41.0578043Z .................................................................................................... 3100/9849
2020-03-27T07:45:45.7034778Z i................................................................................................... 3200/9849
---
2020-03-27T07:47:40.6852957Z .....................................................i...............i.............................. 5000/9849
2020-03-27T07:47:48.0145578Z .................................................................................................... 5100/9849
2020-03-27T07:47:55.0647733Z ..................................................................................................i. 5200/9849
2020-03-27T07:47:59.8910541Z .................................................................................................... 5300/9849
2020-03-27T07:48:10.0021272Z ...................................................................................ii.ii........i... 5400/9849
2020-03-27T07:48:13.5614834Z i................................................................................................... 5500/9849
2020-03-27T07:48:22.3329407Z ............................i......................................................................F 5700/9849
2020-03-27T07:48:22.3329407Z ............................i......................................................................F 5700/9849
2020-03-27T07:48:29.5903571Z F.F..........................................ii....................................i................ 5800/9849
2020-03-27T07:48:41.5362513Z .................................................................................................... 6000/9849
2020-03-27T07:48:41.5362513Z .................................................................................................... 6000/9849
2020-03-27T07:48:50.3798431Z .............................................................................ii...i..ii...........i. 6100/9849
2020-03-27T07:49:10.2304112Z .................................................................................................... 6300/9849
2020-03-27T07:49:15.6177402Z .................................................................................................... 6400/9849
2020-03-27T07:49:19.0061232Z .................................................................................................... 6500/9849
2020-03-27T07:49:19.0061232Z .................................................................................................... 6500/9849
2020-03-27T07:49:30.6298418Z .......i..ii........................................................................................ 6600/9849
2020-03-27T07:49:47.1695480Z ..........................................F.F....................................................... 6800/9849
2020-03-27T07:49:49.1770305Z .......i............................................................................................ 6900/9849
2020-03-27T07:49:51.1848517Z .................................................................................................... 7000/9849
2020-03-27T07:49:53.3943209Z ...........................................i........................................................ 7100/9849
---
2020-03-27T07:51:25.3682281Z .................................................................................................... 7700/9849
2020-03-27T07:51:30.3888345Z .................................................................................................... 7800/9849
2020-03-27T07:51:35.2921912Z .................................................................................................... 7900/9849
2020-03-27T07:51:42.3215273Z .................................................................................................... 8000/9849
2020-03-27T07:51:49.6442310Z i................................................................................................... 8100/9849
2020-03-27T07:51:57.4779277Z .................................................iiiiiiiiii.i....................................... 8200/9849
2020-03-27T07:52:10.2962963Z i................................................................................................... 8400/9849
2020-03-27T07:52:15.3813049Z .................................................................................................... 8500/9849
2020-03-27T07:52:28.1850843Z .................................................................................................... 8600/9849
2020-03-27T07:52:37.4585590Z .................................................................................................... 8700/9849
---
2020-03-27T07:54:28.4071959Z ---- [ui] ui/abi/stack-probes-lto.rs stdout ----
2020-03-27T07:54:28.4072355Z 
2020-03-27T07:54:28.4073004Z error: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4073462Z status: exit code: 1
2020-03-27T07:54:28.4075495Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/auxiliary"
2020-03-27T07:54:28.4077366Z ------------------------------------------
2020-03-27T07:54:28.4077725Z 
2020-03-27T07:54:28.4078306Z ------------------------------------------
2020-03-27T07:54:28.4078721Z stderr:
2020-03-27T07:54:28.4078721Z stderr:
2020-03-27T07:54:28.4079274Z ------------------------------------------
2020-03-27T07:54:28.4080048Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fbada4b1f38, 2611768)
2020-03-27T07:54:28.4080972Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fbb02b6514a, 20224)
2020-03-27T07:54:28.4081879Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fbb02ac42b0, 12728)
2020-03-27T07:54:28.4082885Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fbacc013280, 1936)
2020-03-27T07:54:28.4083825Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fbb02b4189e, 85408)
2020-03-27T07:54:28.4084765Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fbb02b512f2, 1896)
2020-03-27T07:54:28.4085335Z extract_bitcode alloc.o (0x7fbb02b51a96, 4184)
2020-03-27T07:54:28.4086027Z findBitcodeInObject failed
2020-03-27T07:54:28.4086501Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4087717Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4088350Z error: aborting due to previous error
2020-03-27T07:54:28.4088992Z 
2020-03-27T07:54:28.4089193Z 
2020-03-27T07:54:28.4089767Z ------------------------------------------
2020-03-27T07:54:28.4089767Z ------------------------------------------
2020-03-27T07:54:28.4090043Z 
2020-03-27T07:54:28.4090253Z 
2020-03-27T07:54:28.4090732Z ---- [ui] ui/debuginfo-lto.rs stdout ----
2020-03-27T07:54:28.4091006Z 
2020-03-27T07:54:28.4091707Z error: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4092096Z status: exit code: 1
2020-03-27T07:54:28.4093932Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/debuginfo-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo-lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/debuginfo-lto/auxiliary"
2020-03-27T07:54:28.4095650Z ------------------------------------------
2020-03-27T07:54:28.4096111Z 
2020-03-27T07:54:28.4096621Z ------------------------------------------
2020-03-27T07:54:28.4096965Z stderr:
2020-03-27T07:54:28.4096965Z stderr:
2020-03-27T07:54:28.4097464Z ------------------------------------------
2020-03-27T07:54:28.4098192Z extract_bitcode debuginfo-lto-aux.debuginfo_lto_aux.3a1fbbbh-cgu.0.rcgu.o (0x7f51e7080238, 12024)
2020-03-27T07:54:28.4099326Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f51be9c8f38, 2611768)
2020-03-27T07:54:28.4100787Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f51e707914a, 20224)
2020-03-27T07:54:28.4101586Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f51e6fd82b0, 12728)
2020-03-27T07:54:28.4102368Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f51b0009b30, 1936)
2020-03-27T07:54:28.4103500Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f51e705589e, 85408)
2020-03-27T07:54:28.4104329Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f51e70652f2, 1896)
2020-03-27T07:54:28.4104686Z extract_bitcode alloc.o (0x7f51e7065a96, 4184)
2020-03-27T07:54:28.4104925Z findBitcodeInObject failed
2020-03-27T07:54:28.4105176Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4105495Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4106074Z error: aborting due to previous error
2020-03-27T07:54:28.4106240Z 
2020-03-27T07:54:28.4106337Z 
2020-03-27T07:54:28.4106725Z ------------------------------------------
2020-03-27T07:54:28.4106725Z ------------------------------------------
2020-03-27T07:54:28.4106898Z 
2020-03-27T07:54:28.4106995Z 
2020-03-27T07:54:28.4107488Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat stdout ----
2020-03-27T07:54:28.4107746Z 
2020-03-27T07:54:28.4108205Z error in revision `fat`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4108503Z status: exit code: 1
2020-03-27T07:54:28.4110848Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.fat/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.fat/auxiliary"
2020-03-27T07:54:28.4112665Z ------------------------------------------
2020-03-27T07:54:28.4112841Z 
2020-03-27T07:54:28.4113198Z ------------------------------------------
2020-03-27T07:54:28.4113416Z stderr:
2020-03-27T07:54:28.4113416Z stderr:
2020-03-27T07:54:28.4113783Z ------------------------------------------
2020-03-27T07:54:28.4114350Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f6d745bef38, 2611768)
2020-03-27T07:54:28.4115055Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f6d9cc7214a, 20224)
2020-03-27T07:54:28.4115758Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f6d9cbd12b0, 12728)
2020-03-27T07:54:28.4116525Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f6d68010050, 1936)
2020-03-27T07:54:28.4117283Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f6d9cc4e89e, 85408)
2020-03-27T07:54:28.4117995Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f6d9cc5e2f2, 1896)
2020-03-27T07:54:28.4118391Z extract_bitcode alloc.o (0x7f6d9cc5ea96, 4184)
2020-03-27T07:54:28.4118633Z findBitcodeInObject failed
2020-03-27T07:54:28.4118903Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4119244Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4119659Z error: aborting due to previous error
2020-03-27T07:54:28.4119823Z 
2020-03-27T07:54:28.4119921Z 
2020-03-27T07:54:28.4120292Z ------------------------------------------
2020-03-27T07:54:28.4120292Z ------------------------------------------
2020-03-27T07:54:28.4120463Z 
2020-03-27T07:54:28.4120561Z 
2020-03-27T07:54:28.4121049Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin stdout ----
2020-03-27T07:54:28.4121323Z 
2020-03-27T07:54:28.4121773Z error in revision `thin`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4122076Z status: exit code: 1
2020-03-27T07:54:28.4124250Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thin" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.thin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto=thin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.thin/auxiliary"
2020-03-27T07:54:28.4125971Z ------------------------------------------
2020-03-27T07:54:28.4126145Z 
2020-03-27T07:54:28.4126505Z ------------------------------------------
2020-03-27T07:54:28.4126727Z stderr:
2020-03-27T07:54:28.4126727Z stderr:
2020-03-27T07:54:28.4127094Z ------------------------------------------
2020-03-27T07:54:28.4127660Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fd02e73af38, 2611768)
2020-03-27T07:54:28.4128366Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fd056dee14a, 20224)
2020-03-27T07:54:28.4130155Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fd056d4d2b0, 12728)
2020-03-27T07:54:28.4130938Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fd02000f080, 1936)
2020-03-27T07:54:28.4131706Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fd056dca89e, 85408)
2020-03-27T07:54:28.4132417Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fd056dda2f2, 1896)
2020-03-27T07:54:28.4132815Z extract_bitcode alloc.o (0x7fd056ddaa96, 4184)
2020-03-27T07:54:28.4133228Z findBitcodeInObject failed
2020-03-27T07:54:28.4133503Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4133862Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4134265Z error: aborting due to previous error
2020-03-27T07:54:28.4134428Z 
2020-03-27T07:54:28.4134542Z 
2020-03-27T07:54:28.4134910Z ------------------------------------------
2020-03-27T07:54:28.4134910Z ------------------------------------------
2020-03-27T07:54:28.4135082Z 
2020-03-27T07:54:28.4135179Z 
2020-03-27T07:54:28.4135643Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----
2020-03-27T07:54:28.4135899Z 
2020-03-27T07:54:28.4136347Z error in revision `fat0`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4136646Z status: exit code: 1
2020-03-27T07:54:28.4138770Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat0" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/auxiliary"
2020-03-27T07:54:28.4140464Z ------------------------------------------
2020-03-27T07:54:28.4140640Z 
2020-03-27T07:54:28.4140995Z ------------------------------------------
2020-03-27T07:54:28.4141211Z stderr:
2020-03-27T07:54:28.4141211Z stderr:
2020-03-27T07:54:28.4141578Z ------------------------------------------
2020-03-27T07:54:28.4142143Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f374d411f38, 2611768)
2020-03-27T07:54:28.4142853Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f3775ac514a, 20224)
2020-03-27T07:54:28.4143563Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f3775a242b0, 12728)
2020-03-27T07:54:28.4144345Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f374000ad50, 1936)
2020-03-27T07:54:28.4145793Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f3775aa189e, 85408)
2020-03-27T07:54:28.4146457Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f3775ab12f2, 1896)
2020-03-27T07:54:28.4146827Z extract_bitcode alloc.o (0x7f3775ab1a96, 4184)
2020-03-27T07:54:28.4147049Z findBitcodeInObject failed
2020-03-27T07:54:28.4147297Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4147629Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4148006Z error: aborting due to previous error
2020-03-27T07:54:28.4148162Z 
2020-03-27T07:54:28.4148267Z 
2020-03-27T07:54:28.4148599Z ------------------------------------------
2020-03-27T07:54:28.4148599Z ------------------------------------------
2020-03-27T07:54:28.4148759Z 
2020-03-27T07:54:28.4148850Z 
2020-03-27T07:54:28.4149294Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 stdout ----
2020-03-27T07:54:28.4149519Z 
2020-03-27T07:54:28.4150147Z error in revision `fat1`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4150440Z status: exit code: 1
2020-03-27T07:54:28.4152630Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat1" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat1/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=1" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat1/auxiliary"
2020-03-27T07:54:28.4154422Z ------------------------------------------
2020-03-27T07:54:28.4154585Z 
2020-03-27T07:54:28.4154929Z ------------------------------------------
2020-03-27T07:54:28.4155116Z stderr:
2020-03-27T07:54:28.4155116Z stderr:
2020-03-27T07:54:28.4155454Z ------------------------------------------
2020-03-27T07:54:28.4155978Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fa3d28c4f38, 2611768)
2020-03-27T07:54:28.4156632Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fa3faf7814a, 20224)
2020-03-27T07:54:28.4157283Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fa3faed72b0, 12728)
2020-03-27T07:54:28.4158014Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fa3c8010fa0, 1936)
2020-03-27T07:54:28.4158703Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fa3faf5489e, 85408)
2020-03-27T07:54:28.4159374Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fa3faf642f2, 1896)
2020-03-27T07:54:28.4159727Z extract_bitcode alloc.o (0x7fa3faf64a96, 4184)
2020-03-27T07:54:28.4159951Z findBitcodeInObject failed
2020-03-27T07:54:28.4160201Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4160536Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4160903Z error: aborting due to previous error
2020-03-27T07:54:28.4161068Z 
2020-03-27T07:54:28.4161159Z 
2020-03-27T07:54:28.4161488Z ------------------------------------------
2020-03-27T07:54:28.4161488Z ------------------------------------------
2020-03-27T07:54:28.4161648Z 
2020-03-27T07:54:28.4161738Z 
2020-03-27T07:54:28.4162182Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat2 stdout ----
2020-03-27T07:54:28.4162413Z 
2020-03-27T07:54:28.4163014Z error in revision `fat2`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4163326Z status: exit code: 1
2020-03-27T07:54:28.4165426Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat2" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat2/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=2" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat2/auxiliary"
2020-03-27T07:54:28.4167122Z ------------------------------------------
2020-03-27T07:54:28.4167297Z 
2020-03-27T07:54:28.4167678Z ------------------------------------------
2020-03-27T07:54:28.4167881Z stderr:
2020-03-27T07:54:28.4167881Z stderr:
2020-03-27T07:54:28.4168246Z ------------------------------------------
2020-03-27T07:54:28.4176522Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7ff96864ef38, 2611768)
2020-03-27T07:54:28.4177289Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7ff990d0214a, 20224)
2020-03-27T07:54:28.4177999Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7ff990c612b0, 12728)
2020-03-27T07:54:28.4179626Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7ff95c010f60, 1936)
2020-03-27T07:54:28.4180388Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7ff990cde89e, 85408)
2020-03-27T07:54:28.4181114Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7ff990cee2f2, 1896)
2020-03-27T07:54:28.4181966Z extract_bitcode alloc.o (0x7ff990ceea96, 4184)
2020-03-27T07:54:28.4182277Z findBitcodeInObject failed
2020-03-27T07:54:28.4182564Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4182906Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4183314Z error: aborting due to previous error
2020-03-27T07:54:28.4183494Z 
2020-03-27T07:54:28.4183592Z 
2020-03-27T07:54:28.4183985Z ------------------------------------------
2020-03-27T07:54:28.4183985Z ------------------------------------------
2020-03-27T07:54:28.4184159Z 
2020-03-27T07:54:28.4184258Z 
2020-03-27T07:54:28.4184742Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat3 stdout ----
2020-03-27T07:54:28.4184984Z 
2020-03-27T07:54:28.4185432Z error in revision `fat3`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4185745Z status: exit code: 1
2020-03-27T07:54:28.4187891Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fat3" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat3/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=3" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat3/auxiliary"
2020-03-27T07:54:28.4189591Z ------------------------------------------
2020-03-27T07:54:28.4189766Z 
2020-03-27T07:54:28.4190139Z ------------------------------------------
2020-03-27T07:54:28.4190340Z stderr:
2020-03-27T07:54:28.4190340Z stderr:
2020-03-27T07:54:28.4190704Z ------------------------------------------
2020-03-27T07:54:28.4192178Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f8f4e7d3f38, 2611768)
2020-03-27T07:54:28.4192909Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f8f76e8714a, 20224)
2020-03-27T07:54:28.4193622Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f8f76de62b0, 12728)
2020-03-27T07:54:28.4194417Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f8f40010a40, 1936)
2020-03-27T07:54:28.4195162Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f8f76e6389e, 85408)
2020-03-27T07:54:28.4195891Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f8f76e732f2, 1896)
2020-03-27T07:54:28.4196273Z extract_bitcode alloc.o (0x7f8f76e73a96, 4184)
2020-03-27T07:54:28.4196514Z findBitcodeInObject failed
2020-03-27T07:54:28.4196798Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4197141Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4197565Z error: aborting due to previous error
2020-03-27T07:54:28.4197731Z 
2020-03-27T07:54:28.4197829Z 
2020-03-27T07:54:28.4198187Z ------------------------------------------
2020-03-27T07:54:28.4198187Z ------------------------------------------
2020-03-27T07:54:28.4198359Z 
2020-03-27T07:54:28.4198475Z 
2020-03-27T07:54:28.4198941Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin0 stdout ----
2020-03-27T07:54:28.4199186Z 
2020-03-27T07:54:28.4199638Z error in revision `thin0`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4199955Z status: exit code: 1
2020-03-27T07:54:28.4202876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thin0" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin0/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-C" "lto=thin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin0/auxiliary"
2020-03-27T07:54:28.4205140Z ------------------------------------------
2020-03-27T07:54:28.4205339Z 
2020-03-27T07:54:28.4205701Z ------------------------------------------
2020-03-27T07:54:28.4205903Z stderr:
2020-03-27T07:54:28.4205903Z stderr:
2020-03-27T07:54:28.4206268Z ------------------------------------------
2020-03-27T07:54:28.4206855Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f6e44924f38, 2611768)
2020-03-27T07:54:28.4207548Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f6e6cfd814a, 20224)
2020-03-27T07:54:28.4208265Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f6e6cf372b0, 12728)
2020-03-27T07:54:28.4209300Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f6e3800b430, 1936)
2020-03-27T07:54:28.4211535Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f6e6cfb489e, 85408)
2020-03-27T07:54:28.4212305Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f6e6cfc42f2, 1896)
2020-03-27T07:54:28.4212692Z extract_bitcode alloc.o (0x7f6e6cfc4a96, 4184)
2020-03-27T07:54:28.4212935Z findBitcodeInObject failed
2020-03-27T07:54:28.4213223Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4213567Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4213989Z error: aborting due to previous error
2020-03-27T07:54:28.4214152Z 
2020-03-27T07:54:28.4214249Z 
2020-03-27T07:54:28.4214609Z ------------------------------------------
2020-03-27T07:54:28.4214609Z ------------------------------------------
2020-03-27T07:54:28.4214796Z 
2020-03-27T07:54:28.4214894Z 
2020-03-27T07:54:28.4215372Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin1 stdout ----
2020-03-27T07:54:28.4215835Z 
2020-03-27T07:54:28.4217055Z error in revision `thin1`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4217372Z status: exit code: 1
2020-03-27T07:54:28.4220070Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thin1" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin1/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=1" "-C" "lto=thin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin1/auxiliary"
2020-03-27T07:54:28.4221820Z ------------------------------------------
2020-03-27T07:54:28.4222016Z 
2020-03-27T07:54:28.4222380Z ------------------------------------------
2020-03-27T07:54:28.4222583Z stderr:
2020-03-27T07:54:28.4222583Z stderr:
2020-03-27T07:54:28.4222959Z ------------------------------------------
2020-03-27T07:54:28.4223526Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f7aa6d49f38, 2611768)
2020-03-27T07:54:28.4224218Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f7acf3fd14a, 20224)
2020-03-27T07:54:28.4224934Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f7acf35c2b0, 12728)
2020-03-27T07:54:28.4225701Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f7a98010ee0, 1936)
2020-03-27T07:54:28.4226457Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f7acf3d989e, 85408)
2020-03-27T07:54:28.4227169Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f7acf3e92f2, 1896)
2020-03-27T07:54:28.4227758Z extract_bitcode alloc.o (0x7f7acf3e9a96, 4184)
2020-03-27T07:54:28.4228017Z findBitcodeInObject failed
2020-03-27T07:54:28.4228285Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4228629Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4229049Z error: aborting due to previous error
2020-03-27T07:54:28.4229213Z 
2020-03-27T07:54:28.4229310Z 
2020-03-27T07:54:28.4229677Z ------------------------------------------
2020-03-27T07:54:28.4229677Z ------------------------------------------
2020-03-27T07:54:28.4229864Z 
2020-03-27T07:54:28.4229961Z 
2020-03-27T07:54:28.4230425Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin2 stdout ----
2020-03-27T07:54:28.4230665Z 
2020-03-27T07:54:28.4231130Z error in revision `thin2`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4231432Z status: exit code: 1
2020-03-27T07:54:28.4233551Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thin2" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin2/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=2" "-C" "lto=thin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin2/auxiliary"
2020-03-27T07:54:28.4235275Z ------------------------------------------
2020-03-27T07:54:28.4235450Z 
2020-03-27T07:54:28.4235805Z ------------------------------------------
2020-03-27T07:54:28.4236008Z stderr:
2020-03-27T07:54:28.4236008Z stderr:
2020-03-27T07:54:28.4236385Z ------------------------------------------
2020-03-27T07:54:28.4236952Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fd48513df38, 2611768)
2020-03-27T07:54:28.4237646Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fd4ad7f114a, 20224)
2020-03-27T07:54:28.4238361Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fd4ad7502b0, 12728)
2020-03-27T07:54:28.4239127Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fd470010910, 1936)
2020-03-27T07:54:28.4239881Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fd4ad7cd89e, 85408)
2020-03-27T07:54:28.4240592Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fd4ad7dd2f2, 1896)
2020-03-27T07:54:28.4240973Z extract_bitcode alloc.o (0x7fd4ad7dda96, 4184)
2020-03-27T07:54:28.4241228Z findBitcodeInObject failed
2020-03-27T07:54:28.4241499Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4241840Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4242262Z error: aborting due to previous error
2020-03-27T07:54:28.4242425Z 
2020-03-27T07:54:28.4242523Z 
2020-03-27T07:54:28.4247555Z ------------------------------------------
2020-03-27T07:54:28.4247555Z ------------------------------------------
2020-03-27T07:54:28.4247750Z 
2020-03-27T07:54:28.4247850Z 
2020-03-27T07:54:28.4248336Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin3 stdout ----
2020-03-27T07:54:28.4248915Z 
2020-03-27T07:54:28.4249415Z error in revision `thin3`: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4249717Z status: exit code: 1
2020-03-27T07:54:28.4251987Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thin3" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin3/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=3" "-C" "lto=thin" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin3/auxiliary"
2020-03-27T07:54:28.4255489Z ------------------------------------------
2020-03-27T07:54:28.4255666Z 
2020-03-27T07:54:28.4256028Z ------------------------------------------
2020-03-27T07:54:28.4256247Z stderr:
2020-03-27T07:54:28.4256247Z stderr:
2020-03-27T07:54:28.4256613Z ------------------------------------------
2020-03-27T07:54:28.4257180Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f2688fc7f38, 2611768)
2020-03-27T07:54:28.4257886Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f26b167b14a, 20224)
2020-03-27T07:54:28.4258591Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f26b15da2b0, 12728)
2020-03-27T07:54:28.4259656Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f2674005480, 1936)
2020-03-27T07:54:28.4260423Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f26b165789e, 85408)
2020-03-27T07:54:28.4261134Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f26b16672f2, 1896)
2020-03-27T07:54:28.4261531Z extract_bitcode alloc.o (0x7f26b1667a96, 4184)
2020-03-27T07:54:28.4261773Z findBitcodeInObject failed
2020-03-27T07:54:28.4262043Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4262399Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4262801Z error: aborting due to previous error
2020-03-27T07:54:28.4262964Z 
2020-03-27T07:54:28.4263062Z 
2020-03-27T07:54:28.4263434Z ------------------------------------------
2020-03-27T07:54:28.4263434Z ------------------------------------------
2020-03-27T07:54:28.4263607Z 
2020-03-27T07:54:28.4263705Z 
2020-03-27T07:54:28.4264049Z ---- [ui] ui/fat-lto.rs stdout ----
2020-03-27T07:54:28.4264228Z 
2020-03-27T07:54:28.4264621Z error: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4264877Z status: exit code: 1
2020-03-27T07:54:28.4266556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fat-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-lto/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Clto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fat-lto/auxiliary"
2020-03-27T07:54:28.4268392Z ------------------------------------------
2020-03-27T07:54:28.4268566Z 
2020-03-27T07:54:28.4268924Z ------------------------------------------
2020-03-27T07:54:28.4269125Z stderr:
2020-03-27T07:54:28.4269125Z stderr:
2020-03-27T07:54:28.4269512Z ------------------------------------------
2020-03-27T07:54:28.4270086Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f79ddc19f38, 2611768)
2020-03-27T07:54:28.4270856Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f7a062c414a, 20224)
2020-03-27T07:54:28.4271521Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f7a062232b0, 12728)
2020-03-27T07:54:28.4272232Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f79d00080c0, 1936)
2020-03-27T07:54:28.4272935Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f7a062a089e, 85408)
2020-03-27T07:54:28.4273761Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f7a062b02f2, 1896)
2020-03-27T07:54:28.4274143Z extract_bitcode alloc.o (0x7f7a062b0a96, 4184)
2020-03-27T07:54:28.4274399Z findBitcodeInObject failed
2020-03-27T07:54:28.4274777Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4275163Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4275579Z error: aborting due to previous error
2020-03-27T07:54:28.4275742Z 
2020-03-27T07:54:28.4275840Z 
2020-03-27T07:54:28.4276222Z ------------------------------------------
2020-03-27T07:54:28.4276222Z ------------------------------------------
2020-03-27T07:54:28.4276394Z 
2020-03-27T07:54:28.4276490Z 
2020-03-27T07:54:28.4276859Z ---- [ui] ui/issues/issue-44056.rs stdout ----
2020-03-27T07:54:28.4277143Z 
2020-03-27T07:54:28.4277692Z error: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4277948Z status: exit code: 1
2020-03-27T07:54:28.4286859Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44056.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Ctarget-feature=+avx" "-Clto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44056/auxiliary"
2020-03-27T07:54:28.4289065Z ------------------------------------------
2020-03-27T07:54:28.4290480Z 
2020-03-27T07:54:28.4290945Z ------------------------------------------
2020-03-27T07:54:28.4291149Z stderr:
2020-03-27T07:54:28.4291149Z stderr:
2020-03-27T07:54:28.4291537Z ------------------------------------------
2020-03-27T07:54:28.4292109Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fa661ecff38, 2611768)
2020-03-27T07:54:28.4292800Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fa68a58014a, 20224)
2020-03-27T07:54:28.4294191Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fa68a4df2b0, 12728)
2020-03-27T07:54:28.4294983Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fa654007af0, 1936)
2020-03-27T07:54:28.4295752Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fa68a55c89e, 85408)
2020-03-27T07:54:28.4296460Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fa68a56c2f2, 1896)
2020-03-27T07:54:28.4296847Z extract_bitcode alloc.o (0x7fa68a56ca96, 4184)
2020-03-27T07:54:28.4297268Z findBitcodeInObject failed
2020-03-27T07:54:28.4297538Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4297879Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4298295Z error: aborting due to previous error
2020-03-27T07:54:28.4298539Z 
2020-03-27T07:54:28.4298638Z 
2020-03-27T07:54:28.4299029Z ------------------------------------------
2020-03-27T07:54:28.4299029Z ------------------------------------------
2020-03-27T07:54:28.4299203Z 
2020-03-27T07:54:28.4299301Z 
2020-03-27T07:54:28.4299686Z ---- [ui] ui/lto-many-codegen-units.rs stdout ----
2020-03-27T07:54:28.4299876Z 
2020-03-27T07:54:28.4300282Z error: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4300539Z status: exit code: 1
2020-03-27T07:54:28.4303454Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-many-codegen-units.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-many-codegen-units/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "codegen-units=8" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-many-codegen-units/auxiliary"
2020-03-27T07:54:28.4304983Z ------------------------------------------
2020-03-27T07:54:28.4305157Z 
2020-03-27T07:54:28.4305516Z ------------------------------------------
2020-03-27T07:54:28.4305932Z stderr:
2020-03-27T07:54:28.4305932Z stderr:
2020-03-27T07:54:28.4306328Z ------------------------------------------
2020-03-27T07:54:28.4306898Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f4314f74f38, 2611768)
2020-03-27T07:54:28.4307590Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f433d62514a, 20224)
2020-03-27T07:54:28.4308312Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f433d5842b0, 12728)
2020-03-27T07:54:28.4309083Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f43000079a0, 1936)
2020-03-27T07:54:28.4309840Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f433d60189e, 85408)
2020-03-27T07:54:28.4310551Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f433d6112f2, 1896)
2020-03-27T07:54:28.4310933Z extract_bitcode alloc.o (0x7f433d611a96, 4184)
2020-03-27T07:54:28.4311194Z findBitcodeInObject failed
2020-03-27T07:54:28.4311467Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4311808Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4312223Z error: aborting due to previous error
2020-03-27T07:54:28.4312387Z 
2020-03-27T07:54:28.4312484Z 
2020-03-27T07:54:28.4312854Z ------------------------------------------
2020-03-27T07:54:28.4312854Z ------------------------------------------
2020-03-27T07:54:28.4313026Z 
2020-03-27T07:54:28.4313123Z 
2020-03-27T07:54:28.4313498Z ---- [ui] ui/lto-duplicate-symbols.rs stdout ----
2020-03-27T07:54:28.4313743Z diff of stderr:
2020-03-27T07:54:28.4313869Z 
2020-03-27T07:54:28.4314309Z - warning: Linking globals named 'foo': symbol multiply defined!
2020-03-27T07:54:28.4314687Z - 
2020-03-27T07:54:28.4315178Z - error: failed to load bc of "lto_duplicate_symbols2.3a1fbbbh-cgu.0": 
2020-03-27T07:54:28.4316091Z + extract_bitcode lto-duplicate-symbols2.lto_duplicate_symbols2.3a1fbbbh-cgu.0.rcgu.o (0x7fa494002c5e, 2504)
2020-03-27T07:54:28.4316829Z + extract_bitcode lto-duplicate-symbols1.lto_duplicate_symbols1.3a1fbbbh-cgu.0.rcgu.o (0x7fa494002c5e, 2504)
2020-03-27T07:54:28.4317527Z + extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fa49f2c9f38, 2611768)
2020-03-27T07:54:28.4318219Z + extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fa4c797a14a, 20224)
2020-03-27T07:54:28.4318928Z + extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fa4c78d92b0, 12728)
2020-03-27T07:54:28.4319721Z + extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fa494002ee0, 1936)
2020-03-27T07:54:28.4320467Z + extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fa4c795689e, 85408)
2020-03-27T07:54:28.4321199Z + extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fa4c79662f2, 1896)
2020-03-27T07:54:28.4321588Z + extract_bitcode alloc.o (0x7fa4c7966a96, 4184)
2020-03-27T07:54:28.4321844Z + findBitcodeInObject failed
2020-03-27T07:54:28.4322271Z + llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4322624Z + error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4323074Z 5 error: aborting due to previous error
2020-03-27T07:54:28.4323262Z 6 
2020-03-27T07:54:28.4323363Z 
2020-03-27T07:54:28.4323461Z 
2020-03-27T07:54:28.4323461Z 
2020-03-27T07:54:28.4323680Z The actual stderr differed from the expected stderr.
2020-03-27T07:54:28.4324368Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
2020-03-27T07:54:28.4324999Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T07:54:28.4325588Z To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`
2020-03-27T07:54:28.4326024Z error: 1 errors occurred comparing output.
2020-03-27T07:54:28.4326330Z status: exit code: 1
2020-03-27T07:54:28.4326330Z status: exit code: 1
2020-03-27T07:54:28.4328365Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary"
2020-03-27T07:54:28.4330026Z ------------------------------------------
2020-03-27T07:54:28.4330202Z 
2020-03-27T07:54:28.4330559Z ------------------------------------------
2020-03-27T07:54:28.4330905Z stderr:
2020-03-27T07:54:28.4330905Z stderr:
2020-03-27T07:54:28.4331284Z ------------------------------------------
2020-03-27T07:54:28.4331897Z extract_bitcode lto-duplicate-symbols2.lto_duplicate_symbols2.3a1fbbbh-cgu.0.rcgu.o (0x7fa494002c5e, 2504)
2020-03-27T07:54:28.4332635Z extract_bitcode lto-duplicate-symbols1.lto_duplicate_symbols1.3a1fbbbh-cgu.0.rcgu.o (0x7fa494002c5e, 2504)
2020-03-27T07:54:28.4333311Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7fa49f2c9f38, 2611768)
2020-03-27T07:54:28.4334010Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7fa4c797a14a, 20224)
2020-03-27T07:54:28.4334840Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7fa4c78d92b0, 12728)
2020-03-27T07:54:28.4335609Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7fa494002ee0, 1936)
2020-03-27T07:54:28.4336458Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7fa4c795689e, 85408)
2020-03-27T07:54:28.4337300Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7fa4c79662f2, 1896)
2020-03-27T07:54:28.4337691Z extract_bitcode alloc.o (0x7fa4c7966a96, 4184)
2020-03-27T07:54:28.4338063Z findBitcodeInObject failed
2020-03-27T07:54:28.4338431Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4338751Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4339138Z error: aborting due to previous error
2020-03-27T07:54:28.4339291Z 
2020-03-27T07:54:28.4339381Z 
2020-03-27T07:54:28.4339737Z ------------------------------------------
2020-03-27T07:54:28.4339737Z ------------------------------------------
2020-03-27T07:54:28.4339896Z 
2020-03-27T07:54:28.4339987Z 
2020-03-27T07:54:28.4340554Z ---- [ui] ui/lto-still-runs-thread-dtors.rs stdout ----
2020-03-27T07:54:28.4340749Z 
2020-03-27T07:54:28.4341152Z error: test compilation failed although it shouldn't!
2020-03-27T07:54:28.4341409Z status: exit code: 1
2020-03-27T07:54:28.4343351Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-still-runs-thread-dtors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-still-runs-thread-dtors/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-still-runs-thread-dtors/auxiliary"
2020-03-27T07:54:28.4344885Z ------------------------------------------
2020-03-27T07:54:28.4345184Z 
2020-03-27T07:54:28.4345554Z ------------------------------------------
2020-03-27T07:54:28.4345756Z stderr:
2020-03-27T07:54:28.4345756Z stderr:
2020-03-27T07:54:28.4346136Z ------------------------------------------
2020-03-27T07:54:28.4346699Z extract_bitcode std-0bc6d41fddd2f7ce.std.84bs9ali-cgu.0.rcgu.o (0x7f190dd0ef38, 2611768)
2020-03-27T07:54:28.4347386Z extract_bitcode panic_unwind-31ab1f37cc7fbcf0.panic_unwind.1bwjg4nk-cgu.0.rcgu.o (0x7f19363c214a, 20224)
2020-03-27T07:54:28.4348296Z extract_bitcode hashbrown-1c500a2819dcae4a.hashbrown.akdfvttc-cgu.0.rcgu.o (0x7f19363212b0, 12728)
2020-03-27T07:54:28.4349120Z extract_bitcode rustc_std_workspace_alloc-cc7c8bb5cd024239.rustc_std_workspace_alloc.7m72glyb-cgu.0.rcgu.o (0x7f190000a4c0, 1936)
2020-03-27T07:54:28.4349879Z extract_bitcode backtrace-706fb6c46eadc49e.backtrace.pf6hyhs1-cgu.0.rcgu.o (0x7f193639e89e, 85408)
2020-03-27T07:54:28.4350591Z extract_bitcode backtrace_sys-d91e36c724ec230c.backtrace_sys.6f5mhp24-cgu.0.rcgu.o (0x7f19363ae2f2, 1896)
2020-03-27T07:54:28.4350970Z extract_bitcode alloc.o (0x7f19363aea96, 4184)
2020-03-27T07:54:28.4351226Z findBitcodeInObject failed
2020-03-27T07:54:28.4351495Z llvm lastError Some("Bitcode section not found in object file")
2020-03-27T07:54:28.4351838Z error: failed to extract bitcode from the object file for LTO module
2020-03-27T07:54:28.4352253Z error: aborting due to previous error
2020-03-27T07:54:28.4352418Z 
---
2020-03-27T07:54:28.4433584Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T07:54:28.4434000Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T07:54:28.4434234Z 
2020-03-27T07:54:28.4434346Z 
2020-03-27T07:54:28.4438049Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T07:54:28.4440754Z 
2020-03-27T07:54:28.4440845Z 
2020-03-27T07:54:28.4441308Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T07:54:28.4441674Z Build completed unsuccessfully in 1:00:13
2020-03-27T07:54:28.4441674Z Build completed unsuccessfully in 1:00:13
2020-03-27T07:54:28.4441905Z == clock drift check ==
2020-03-27T07:54:28.4442133Z   local time: Fri Mar 27 07:54:28 UTC 2020
2020-03-27T07:54:28.6998145Z   network time: Fri, 27 Mar 2020 07:54:28 GMT
2020-03-27T07:54:28.7006522Z == end clock drift check ==
2020-03-27T07:54:29.1313716Z 
2020-03-27T07:54:29.1377372Z ##[error]Bash exited with code '1'.
2020-03-27T07:54:29.1393159Z ##[section]Finishing: Run build
2020-03-27T07:54:29.1442735Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-27T07:54:29.1448270Z Task         : Get sources
2020-03-27T07:54:29.1448667Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T07:54:29.1449043Z Version      : 1.0.0
2020-03-27T07:54:29.1449297Z Author       : Microsoft
2020-03-27T07:54:29.1449297Z Author       : Microsoft
2020-03-27T07:54:29.1449695Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T07:54:29.1450188Z ==============================================================================
2020-03-27T07:54:29.4597902Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T07:54:29.4652269Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-27T07:54:29.4737657Z Cleaning up task key
2020-03-27T07:54:29.4738905Z Start cleaning up orphan processes.
2020-03-27T07:54:29.4906923Z Terminate orphan process: pid (3440) (python)
2020-03-27T07:54:29.5051353Z ##[section]Finishing: Finalize Job
