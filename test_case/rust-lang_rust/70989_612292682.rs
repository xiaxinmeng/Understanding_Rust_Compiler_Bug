plain
2020-04-11T00:01:51.5059353Z ========================== Starting Command Output ===========================
2020-04-11T00:01:51.5062088Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1ca758e5-8477-48a4-a8dd-4523b0f8035c.sh
2020-04-11T00:01:51.5062350Z 
2020-04-11T00:01:51.5066561Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T00:01:51.5086634Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T00:01:51.5091054Z Task         : Get sources
2020-04-11T00:01:51.5091550Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T00:01:51.5091815Z Version      : 1.0.0
2020-04-11T00:01:51.5091994Z Author       : Microsoft
---
2020-04-11T00:01:52.7921975Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T00:01:52.7926877Z ##[command]git config gc.auto 0
2020-04-11T00:01:52.7929869Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T00:01:52.7932333Z ##[command]git config --get-all http.proxy
2020-04-11T00:01:52.7937868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-11T00:04:42.0519365Z Successfully built 1b42fe7a2446
2020-04-11T00:04:42.0647985Z Successfully tagged rust-ci:latest
2020-04-11T00:04:42.0892339Z Built container sha256:1b42fe7a2446458f75515930ac3e4fe55f8fabb1d5449b7ca352db6a3447a33e
2020-04-11T00:04:42.0913043Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/4870776fe6050deeecf88d11c80bcab35cd9225192d232f8a44b9624b9eb93a3a2b9c1b875007d4c23016455da900b001d9635abb113d5fe469d39aa9788575f
2020-04-11T00:05:21.3300440Z upload failed: - to s3://rust-lang-ci-sccache2/docker/4870776fe6050deeecf88d11c80bcab35cd9225192d232f8a44b9624b9eb93a3a2b9c1b875007d4c23016455da900b001d9635abb113d5fe469d39aa9788575f An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-11T00:05:21.7409897Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-11T00:05:21.7433558Z == clock drift check ==
2020-04-11T00:05:21.7440192Z   local time: Sat Apr 11 00:05:21 UTC 2020
2020-04-11T00:05:21.7440192Z   local time: Sat Apr 11 00:05:21 UTC 2020
2020-04-11T00:05:22.0496966Z   network time: Sat, 11 Apr 2020 00:05:22 GMT
2020-04-11T00:05:22.0512915Z Starting sccache server...
2020-04-11T00:05:22.1214807Z configure: processing command line
2020-04-11T00:05:22.1215291Z configure: 
2020-04-11T00:05:22.1216293Z configure: rust.dist-src        := False
---
2020-04-11T00:09:49.4006089Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T00:09:50.7108918Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T00:09:52.0703787Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T00:09:53.6082290Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T00:10:00.7556679Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T00:10:03.3341721Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T00:10:07.1420717Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T00:10:10.7570348Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T00:10:18.3884784Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T00:29:46.9100886Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T00:29:48.5607045Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T00:29:50.2771184Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T00:29:52.3082310Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T00:30:01.2788182Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T00:30:04.3660866Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T00:30:08.8894966Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T00:30:13.5267192Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T00:30:22.8059983Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T00:52:14.3495056Z .................................................................................................... 1600/9884
2020-04-11T00:52:19.5989241Z .................................................................................................... 1700/9884
2020-04-11T00:52:23.2000991Z .................................................................................................... 1800/9884
2020-04-11T00:52:30.5029942Z .................................................................................................... 1900/9884
2020-04-11T00:52:37.3093514Z ..i................................................................................................. 2000/9884
2020-04-11T00:52:42.6836701Z ............................................................................................iiiii... 2100/9884
2020-04-11T00:53:00.8242237Z .................................................................................................... 2300/9884
2020-04-11T00:53:02.6790823Z .................................................................................................... 2400/9884
2020-04-11T00:53:04.6672394Z .................................................................................................... 2500/9884
2020-04-11T00:53:09.6739265Z .................................................................................................... 2600/9884
---
2020-04-11T00:55:36.1700715Z ..................................................................i...............i................. 5000/9884
2020-04-11T00:55:42.2961568Z .................................................................................................... 5100/9884
2020-04-11T00:55:48.7633664Z .................................................................................................... 5200/9884
2020-04-11T00:55:53.1040851Z ...........i........................................................................................ 5300/9884
2020-04-11T00:56:01.5865180Z i................................................................................................... 5400/9884
2020-04-11T00:56:05.5932717Z ii.ii........i...i.................................................................................. 5500/9884
2020-04-11T00:56:12.1743372Z .............................................i...................................................... 5700/9884
2020-04-11T00:56:20.8669870Z .................................................................ii................................. 5800/9884
2020-04-11T00:56:26.4800873Z ....i............................................................................................... 5900/9884
2020-04-11T00:56:31.0516717Z .................................................................................................... 6000/9884
2020-04-11T00:56:31.0516717Z .................................................................................................... 6000/9884
2020-04-11T00:56:39.1154421Z ..................................................................................................ii 6100/9884
2020-04-11T00:56:48.9930791Z ...i..ii...........i................................................................................ 6200/9884
2020-04-11T00:57:02.4641554Z .................................................................................................... 6400/9884
2020-04-11T00:57:07.2358105Z .................................................................................................... 6500/9884
2020-04-11T00:57:07.2358105Z .................................................................................................... 6500/9884
2020-04-11T00:57:19.4642975Z ............................i..ii................................................................... 6600/9884
2020-04-11T00:57:37.2511996Z .................................................................................................... 6800/9884
2020-04-11T00:57:38.9629144Z ............................i....................................................................... 6900/9884
2020-04-11T00:57:40.7420562Z .................................................................................................... 7000/9884
2020-04-11T00:57:42.5941977Z ...................................................................i................................ 7100/9884
---
2020-04-11T00:59:06.0791828Z .................................................................................................... 7800/9884
2020-04-11T00:59:09.4053515Z .................................................................................................... 7900/9884
2020-04-11T00:59:14.9448872Z .................................................................................................... 8000/9884
2020-04-11T00:59:20.8987400Z ................................i................................................................... 8100/9884
2020-04-11T00:59:28.1533863Z ................................................................................iiiiii.iiiii.i...... 8200/9884
2020-04-11T00:59:41.5181718Z ..........................i......i.................................................................. 8400/9884
2020-04-11T00:59:44.7357875Z .................................................................................................... 8500/9884
2020-04-11T00:59:54.3635576Z .................................................................................................... 8600/9884
2020-04-11T01:00:04.9939153Z .................................................................................................... 8700/9884
---
2020-04-11T01:02:07.6504512Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-11T01:02:07.6657187Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T01:02:07.8402849Z 
2020-04-11T01:02:07.8404494Z running 185 tests
2020-04-11T01:02:10.0921082Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-11T01:02:12.4516016Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-11T01:02:12.4522230Z 
2020-04-11T01:02:12.4529046Z  finished in 4.787
2020-04-11T01:02:12.4536544Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-11T01:02:12.4708016Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T01:02:14.3714485Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-11T01:02:14.3872939Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T01:02:14.5162121Z 
2020-04-11T01:02:14.5162749Z running 9 tests
2020-04-11T01:02:14.5163896Z iiiiiiiii
2020-04-11T01:02:14.5165016Z 
2020-04-11T01:02:14.5165167Z  finished in 0.128
2020-04-11T01:02:14.5170542Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-11T01:02:14.5335115Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T01:02:31.4668779Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-11T01:02:31.4867224Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T01:02:31.6518020Z 
2020-04-11T01:02:32.5269376Z running 115 tests
2020-04-11T01:02:42.8184043Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-11T01:02:44.2755116Z ...iiii.....ii.
2020-04-11T01:02:44.2756077Z 
2020-04-11T01:02:44.2756738Z  finished in 12.788
2020-04-11T01:02:44.2764648Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-11T01:02:44.2765586Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T01:13:30.7737261Z 
2020-04-11T01:13:30.7741036Z    Doc-tests core
2020-04-11T01:13:34.7182678Z 
2020-04-11T01:13:34.7203937Z running 2490 tests
2020-04-11T01:13:42.3320549Z ......iiiii......................................................................................... 100/2490
2020-04-11T01:13:49.7548557Z .....................................................................................ii............. 200/2490
2020-04-11T01:14:07.5834225Z ....................i............................................................................... 400/2490
2020-04-11T01:14:07.5834225Z ....................i............................................................................... 400/2490
2020-04-11T01:14:15.9569217Z ..........................................................................i..i..................iiii 500/2490
2020-04-11T01:14:29.6756901Z .................................................................................................... 700/2490
2020-04-11T01:14:36.7357943Z .................................................................................................... 800/2490
2020-04-11T01:14:43.8099082Z .................................................................................................... 900/2490
2020-04-11T01:14:50.8177052Z .................................................................................................... 1000/2490
---
2020-04-11T01:17:51.7175606Z 
2020-04-11T01:17:51.7176440Z running 1019 tests
2020-04-11T01:18:06.4002404Z i................................................................................................... 100/1019
2020-04-11T01:18:14.9627375Z .................................................................................................... 200/1019
2020-04-11T01:18:21.1230009Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-11T01:18:30.4855799Z ...................................................i....i......................................ii... 500/1019
2020-04-11T01:18:36.7488592Z .................................................................................................... 600/1019
2020-04-11T01:18:40.7829151Z .................................................................................................... 700/1019
2020-04-11T01:18:40.7829151Z .................................................................................................... 700/1019
2020-04-11T01:18:46.5114531Z .............................................iiii................................................... 800/1019
2020-04-11T01:18:58.2427817Z .................................................................................................... 900/1019
2020-04-11T01:19:03.3729937Z ...................................................................iiii............................. 1000/1019
2020-04-11T01:19:04.4464236Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-11T01:19:04.4464752Z 
2020-04-11T01:19:04.4557173Z  finished in 139.776
2020-04-11T01:19:04.4561229Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-11T01:21:48.1446004Z 
2020-04-11T01:21:48.1446322Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-11T01:21:48.1446572Z 
2020-04-11T01:21:48.1496054Z  finished in 0.779
2020-04-11T01:21:48.1497438Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-11T01:21:48.1511487Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T01:21:48.3109103Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T01:21:49.1585371Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-a21ea61a5d4ad7a1
2020-04-11T01:21:49.1608273Z 
2020-04-11T01:21:49.1608922Z running 0 tests
2020-04-11T01:21:49.1609079Z 
---
2020-04-11T01:34:20.2741922Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2742541Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2743150Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2743756Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2744392Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2745653Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2746265Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2746856Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T01:34:20.2747489Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-11T01:35:14.6760231Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-11T01:35:14.7035366Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T01:35:14.8865777Z 
2020-04-11T01:35:14.8865988Z running 210 tests
2020-04-11T01:35:42.7228818Z ......................i...ii.......................................................................i 100/210
2020-04-11T01:36:15.0120492Z ........................................iiiiii......i..............iii.............................. 200/210
2020-04-11T01:36:19.3219546Z test result: ok. 195 passed; 0 failed; 15 ignored; 0 measured; 0 filtered out
2020-04-11T01:36:19.3219762Z 
2020-04-11T01:36:19.3228117Z  finished in 64.619
2020-04-11T01:36:19.3233161Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
2020-04-11T01:37:35.7525096Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-11T01:37:35.7639946Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T01:37:35.8937485Z 
2020-04-11T01:37:35.8938217Z running 13 tests
2020-04-11T01:37:36.2694325Z .iiiiiii.iii.
2020-04-11T01:37:36.2695267Z 
2020-04-11T01:37:36.2701772Z  finished in 0.506
2020-04-11T01:37:36.2752217Z Build completed successfully in 1:30:54
2020-04-11T01:37:36.2762317Z + python2.7 ../x.py test src/test/mir-opt --target=i586-unknown-linux-gnu
---
2020-04-11T01:37:57.8627537Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-04-11T01:38:02.6475968Z error: could not compile `compiler_builtins`.
2020-04-11T01:38:02.6476262Z 
2020-04-11T01:38:02.6476369Z Caused by:
2020-04-11T01:38:02.6486035Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C codegen-units=1 -C debuginfo=0 --cfg 'feature="c"' --cfg 'feature="cc"' --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=8e492a363aba672b -C extra-filename=-8e492a363aba672b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps --target i586-unknown-linux-gnu -C linker=cc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/librustc_std_workspace_core-305f91493d64fcdb.rmeta --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/build/compiler_builtins-746b488ce1610eb5/out --cfg 'feature="unstable"' --cfg '__absvdi2="optimized-c"' --cfg '__absvsi2="optimized-c"' --cfg '__absvti2="optimized-c"' --cfg '__addvdi3="optimized-c"' --cfg '__addvsi3="optimized-c"' --cfg '__addvti3="optimized-c"' --cfg '__ashldi3="optimized-c"' --cfg '__ashrdi3="optimized-c"' --cfg '__clzdi2="optimized-c"' --cfg '__clzsi2="optimized-c"' --cfg '__clzti2="optimized-c"' --cfg '__cmpdi2="optimized-c"' --cfg '__cmpti2="optimized-c"' --cfg '__ctzdi2="optimized-c"' --cfg '__ctzsi2="optimized-c"' --cfg '__ctzti2="optimized-c"' --cfg '__divdc3="optimized-c"' --cfg '__divdi3="optimized-c"' --cfg '__divsc3="optimized-c"' --cfg '__divxc3="optimized-c"' --cfg '__extendhfsf2="optimized-c"' --cfg '__ffsti2="optimized-c"' --cfg '__floatdidf="optimized-c"' --cfg '__floatdisf="optimized-c"' --cfg '__floatdixf="optimized-c"' --cfg '__floatundidf="optimized-c"' --cfg '__floatundisf="optimized-c"' --cfg '__floatundixf="optimized-c"' --cfg '__int_util="optimized-c"' --cfg '__lshrdi3="optimized-c"' --cfg '__moddi3="optimized-c"' --cfg '__muldc3="optimized-c"' --cfg '__muldi3="optimized-c"' --cfg '__mulsc3="optimized-c"' --cfg '__mulvdi3="optimized-c"' --cfg '__mulvsi3="optimized-c"' --cfg '__mulvti3="optimized-c"' --cfg '__mulxc3="optimized-c"' --cfg '__negdf2="optimized-c"' --cfg '__negdi2="optimized-c"' --cfg '__negsf2="optimized-c"' --cfg '__negti2="optimized-c"' --cfg '__negvdi2="optimized-c"' --cfg '__negvsi2="optimized-c"' --cfg '__negvti2="optimized-c"' --cfg '__paritydi2="optimized-c"' --cfg '__paritysi2="optimized-c"' --cfg '__parityti2="optimized-c"' --cfg '__popcountdi2="optimized-c"' --cfg '__popcountsi2="optimized-c"' --cfg '__popcountti2="optimized-c"' --cfg '__powixf2="optimized-c"' --cfg '__subvdi3="optimized-c"' --cfg '__subvsi3="optimized-c"' --cfg '__subvti3="optimized-c"' --cfg '__truncdfhf2="optimized-c"' --cfg '__truncdfsf2="optimized-c"' --cfg '__truncsfhf2="optimized-c"' --cfg '__ucmpdi2="optimized-c"' --cfg '__ucmpti2="optimized-c"' --cfg '__udivdi3="optimized-c"' --cfg '__umoddi3="optimized-c"' --cfg 'apple_versioning="optimized-c"' -l static=compiler-rt` (signal: 11, SIGSEGV: invalid memory reference)
2020-04-11T01:38:03.0902137Z error: build failed
2020-04-11T01:38:03.0966926Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i586-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T01:38:03.0967693Z expected success, got: exit code: 101
2020-04-11T01:38:03.0968271Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --target=i586-unknown-linux-gnu
2020-04-11T01:38:03.0968271Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --target=i586-unknown-linux-gnu
2020-04-11T01:38:03.0968627Z Build completed unsuccessfully in 0:00:26
2020-04-11T01:38:03.1012843Z == clock drift check ==
2020-04-11T01:38:03.1027933Z   local time: Sat Apr 11 01:38:03 UTC 2020
2020-04-11T01:38:07.6307453Z   network time: Sat, 11 Apr 2020 01:38:07 GMT
2020-04-11T01:38:09.8291864Z 
2020-04-11T01:38:09.8291864Z 
2020-04-11T01:38:09.8381886Z ##[error]Bash exited with code '1'.
2020-04-11T01:38:09.8394284Z ##[section]Finishing: Run build
2020-04-11T01:38:09.8446292Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T01:38:09.8451922Z Task         : Get sources
2020-04-11T01:38:09.8452203Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T01:38:09.8452454Z Version      : 1.0.0
2020-04-11T01:38:09.8452647Z Author       : Microsoft
2020-04-11T01:38:09.8452647Z Author       : Microsoft
2020-04-11T01:38:09.8452928Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T01:38:09.8453255Z ==============================================================================
2020-04-11T01:38:10.1492398Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T01:38:10.1537153Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T01:38:10.1611794Z Cleaning up task key
2020-04-11T01:38:10.1613021Z Start cleaning up orphan processes.
2020-04-11T01:38:10.2224311Z Terminate orphan process: pid (6550) (python)
2020-04-11T01:38:10.2266895Z ##[section]Finishing: Finalize Job
