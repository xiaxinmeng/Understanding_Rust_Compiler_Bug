plain
2020-04-10T15:12:38.1237260Z ========================== Starting Command Output ===========================
2020-04-10T15:12:38.1241742Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7bec5dd8-3091-47c5-90ee-d0a2c39ff2e1.sh
2020-04-10T15:12:38.1242263Z 
2020-04-10T15:12:38.1248625Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T15:12:38.1282245Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T15:12:38.1286349Z Task         : Get sources
2020-04-10T15:12:38.1286656Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T15:12:38.1286956Z Version      : 1.0.0
2020-04-10T15:12:38.1287156Z Author       : Microsoft
---
2020-04-10T15:12:39.1261200Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T15:12:39.1298127Z ##[command]git config gc.auto 0
2020-04-10T15:12:39.1344239Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T15:12:39.1347345Z ##[command]git config --get-all http.proxy
2020-04-10T15:12:39.1353137Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-10T15:16:49.0475849Z  ---> 4ded3646833a
2020-04-10T15:16:49.0506561Z Successfully built 4ded3646833a
2020-04-10T15:16:49.0579845Z Successfully tagged rust-ci:latest
2020-04-10T15:16:49.0977522Z Built container sha256:4ded3646833aa7f8918cc2cf0e9f18316502d1d041901f583e69d3e5008d9398
2020-04-10T15:16:49.0989488Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/40ad68fcf1e6d3e4b28ddf6cb3482553d4f217d1519e4c5ec4af82426267c72d03aeca95c72e5d8f308fe2398c0fafdf7df9021f903dca2ba96064aead1382ea
2020-04-10T15:17:37.6447899Z upload failed: - to s3://rust-lang-ci-sccache2/docker/40ad68fcf1e6d3e4b28ddf6cb3482553d4f217d1519e4c5ec4af82426267c72d03aeca95c72e5d8f308fe2398c0fafdf7df9021f903dca2ba96064aead1382ea An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-10T15:17:38.2437456Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T15:17:38.2466091Z == clock drift check ==
2020-04-10T15:17:38.2466091Z == clock drift check ==
2020-04-10T15:17:38.2476563Z   local time: Fri Apr 10 15:17:38 UTC 2020
2020-04-10T15:17:38.3385718Z   network time: Fri, 10 Apr 2020 15:17:38 GMT
2020-04-10T15:17:38.3417694Z Starting sccache server...
2020-04-10T15:17:38.4184440Z configure: processing command line
2020-04-10T15:17:38.4184909Z configure: 
2020-04-10T15:17:38.4190819Z configure: rust.dist-src        := False
---
2020-04-10T15:22:18.8458197Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T15:22:20.1646202Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T15:22:21.5842942Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T15:22:22.1694266Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T15:22:30.3692287Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T15:22:32.1515321Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T15:22:36.1197875Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T15:22:39.7518710Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T15:22:48.5763015Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T15:42:35.1043053Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T15:42:36.6284868Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T15:42:38.4077371Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T15:42:39.2300642Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T15:42:49.4932060Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T15:42:51.4007369Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T15:42:56.2109510Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T15:43:01.0590727Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T15:43:11.7452236Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T16:06:05.8475347Z .................................................................................................... 1600/9883
2020-04-10T16:06:11.7040185Z .................................................................................................... 1700/9883
2020-04-10T16:06:15.7433526Z .................................................................................................... 1800/9883
2020-04-10T16:06:23.9520959Z .................................................................................................... 1900/9883
2020-04-10T16:06:31.5829316Z ..i................................................................................................. 2000/9883
2020-04-10T16:06:37.6365254Z ............................................................................................iiiii... 2100/9883
2020-04-10T16:06:57.4132576Z .................................................................................................... 2300/9883
2020-04-10T16:06:59.4727759Z .................................................................................................... 2400/9883
2020-04-10T16:07:01.5893594Z .................................................................................................... 2500/9883
2020-04-10T16:07:07.1376977Z .................................................................................................... 2600/9883
---
2020-04-10T16:09:59.6254739Z .................................................................................................... 5100/9883
2020-04-10T16:10:06.7963810Z .................................................................................................... 5200/9883
2020-04-10T16:10:11.6610629Z ...........i........................................................................................ 5300/9883
2020-04-10T16:10:21.2605008Z .................................................................................................... 5400/9883
2020-04-10T16:10:25.9007961Z ii.ii........i...i.................................................................................. 5500/9883
2020-04-10T16:10:33.3358551Z .............................................i...................................................... 5700/9883
2020-04-10T16:10:43.1178953Z .................................................................ii................................. 5800/9883
2020-04-10T16:10:49.4629210Z ....i............................................................................................... 5900/9883
2020-04-10T16:10:54.6402016Z .................................................................................................... 6000/9883
2020-04-10T16:10:54.6402016Z .................................................................................................... 6000/9883
2020-04-10T16:11:04.0237744Z ..................................................................................................ii 6100/9883
2020-04-10T16:11:15.0612699Z ...i..ii...........i................................................................................ 6200/9883
2020-04-10T16:11:28.5140527Z .................................................................................................... 6400/9883
2020-04-10T16:11:31.7123026Z .................................................................................................... 6500/9883
2020-04-10T16:11:31.7123026Z .................................................................................................... 6500/9883
2020-04-10T16:11:43.6525319Z ............................i..ii................................................................... 6600/9883
2020-04-10T16:12:03.5256123Z .................................................................................................... 6800/9883
2020-04-10T16:12:05.4911944Z ............................i....................................................................... 6900/9883
2020-04-10T16:12:07.4500915Z .................................................................................................... 7000/9883
2020-04-10T16:12:09.5375205Z ...................................................................i................................ 7100/9883
---
2020-04-10T16:13:41.4490295Z .................................................................................................... 7800/9883
2020-04-10T16:13:45.1444323Z .................................................................................................... 7900/9883
2020-04-10T16:13:51.2863673Z .................................................................................................... 8000/9883
2020-04-10T16:13:57.8710690Z ................................i................................................................... 8100/9883
2020-04-10T16:14:06.0145532Z ................................................................................iiiiii.iiii.i....... 8200/9883
2020-04-10T16:14:20.8845927Z .........................i......i................................................................... 8400/9883
2020-04-10T16:14:24.8402100Z .................................................................................................... 8500/9883
2020-04-10T16:14:35.1921590Z .................................................................................................... 8600/9883
2020-04-10T16:14:47.0666897Z .................................................................................................... 8700/9883
---
2020-04-10T16:17:02.2072859Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-10T16:17:02.2261054Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:17:02.4241760Z 
2020-04-10T16:17:02.4242500Z running 185 tests
2020-04-10T16:17:05.2304956Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-10T16:17:08.1813631Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-10T16:17:08.1816455Z 
2020-04-10T16:17:08.1816617Z  finished in 5.467
2020-04-10T16:17:08.1817231Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-10T16:17:08.1818021Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T16:17:09.7894578Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-10T16:17:09.8118000Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:17:09.9976798Z 
2020-04-10T16:17:09.9977707Z running 9 tests
2020-04-10T16:17:09.9979460Z iiiiiiiii
2020-04-10T16:17:09.9981170Z 
2020-04-10T16:17:09.9981447Z  finished in 0.186
2020-04-10T16:17:10.0080315Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-10T16:17:10.0202040Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T16:17:28.8257290Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-10T16:17:28.8461199Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:17:29.0245765Z 
2020-04-10T16:17:29.0247851Z running 115 tests
2020-04-10T16:17:41.6779859Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-10T16:17:43.3246075Z ...iiii.....ii.
2020-04-10T16:17:43.3247295Z 
2020-04-10T16:17:43.3249891Z  finished in 14.479
2020-04-10T16:17:43.3254368Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-10T16:17:43.3257548Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T16:29:21.2803306Z 
2020-04-10T16:29:21.2807431Z    Doc-tests core
2020-04-10T16:29:25.6466398Z 
2020-04-10T16:29:25.6467060Z running 2490 tests
2020-04-10T16:29:34.2651453Z ......iiiii......................................................................................... 100/2490
2020-04-10T16:29:42.9111439Z .....................................................................................ii............. 200/2490
2020-04-10T16:30:02.5660741Z ....................i............................................................................... 400/2490
2020-04-10T16:30:02.5660741Z ....................i............................................................................... 400/2490
2020-04-10T16:30:11.9880118Z ..........................................................................i..i..................iiii 500/2490
2020-04-10T16:30:27.6033325Z .................................................................................................... 700/2490
2020-04-10T16:30:35.5829859Z .................................................................................................... 800/2490
2020-04-10T16:30:43.6793241Z .................................................................................................... 900/2490
2020-04-10T16:30:51.5859827Z .................................................................................................... 1000/2490
---
2020-04-10T16:34:09.3442148Z 
2020-04-10T16:34:09.3442528Z running 1019 tests
2020-04-10T16:34:26.0054223Z i................................................................................................... 100/1019
2020-04-10T16:34:34.5856910Z .................................................................................................... 200/1019
2020-04-10T16:34:41.4130070Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-10T16:34:51.9731000Z ...................................................i....i......................................ii... 500/1019
2020-04-10T16:34:58.9669605Z .................................................................................................... 600/1019
2020-04-10T16:35:03.5074267Z .................................................................................................... 700/1019
2020-04-10T16:35:03.5074267Z .................................................................................................... 700/1019
2020-04-10T16:35:10.0168560Z .............................................iiii................................................... 800/1019
2020-04-10T16:35:22.6810222Z .................................................................................................... 900/1019
2020-04-10T16:35:28.2319926Z ...................................................................iiii............................. 1000/1019
2020-04-10T16:35:29.4107005Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-10T16:35:29.4107297Z 
2020-04-10T16:35:29.4227606Z  finished in 152.381
2020-04-10T16:35:29.4231130Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-10T16:38:26.7877357Z 
2020-04-10T16:38:26.7877849Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-10T16:38:26.7878273Z 
2020-04-10T16:38:26.7940938Z  finished in 0.910
2020-04-10T16:38:26.7942136Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-10T16:38:26.7956101Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:38:26.9764740Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T16:38:27.8758964Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-1ea1b43afe1f39bf
2020-04-10T16:38:27.8788841Z 
2020-04-10T16:38:27.8817988Z running 0 tests
2020-04-10T16:38:27.8818393Z 
---
2020-04-10T16:51:53.6385206Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6385980Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6386771Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6387576Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6388367Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6390053Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6390886Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6391668Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-10T16:51:53.6392518Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-10T16:52:52.5924505Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-10T16:52:52.6280883Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:52:52.8387913Z 
2020-04-10T16:52:52.8388505Z running 210 tests
2020-04-10T16:53:22.2066964Z ......................i...ii.......................................................................i 100/210
2020-04-10T16:53:58.1714097Z ........................................iiiiii......i..............iii.............................. 200/210
2020-04-10T16:54:03.1968140Z test result: ok. 195 passed; 0 failed; 15 ignored; 0 measured; 0 filtered out
2020-04-10T16:54:03.1968571Z 
2020-04-10T16:54:03.1975208Z  finished in 70.569
2020-04-10T16:54:03.1992330Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
2020-04-10T16:55:25.2316836Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-10T16:55:25.2511602Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:55:25.3988508Z 
2020-04-10T16:55:25.3989098Z running 13 tests
2020-04-10T16:55:25.8026780Z .iiiiiii.iii.
2020-04-10T16:55:25.8028017Z 
2020-04-10T16:55:25.8031526Z  finished in 0.551
2020-04-10T16:55:25.8091907Z Build completed successfully in 1:36:19
2020-04-10T16:55:25.8113054Z + python2.7 ../x.py test src/test/mir-opt --target=i686-unknown-linux-gnu
---
2020-04-10T16:55:48.4420777Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-04-10T16:55:53.3492097Z error: could not compile `compiler_builtins`.
2020-04-10T16:55:53.3492456Z 
2020-04-10T16:55:53.3492603Z Caused by:
2020-04-10T16:55:53.3510963Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiler_builtins /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C codegen-units=1 -C debuginfo=0 --cfg 'feature="c"' --cfg 'feature="cc"' --cfg 'feature="compiler-builtins"' --cfg 'feature="core"' --cfg 'feature="default"' --cfg 'feature="rustc-dep-of-std"' -C metadata=d29c7290fb7fc640 -C extra-filename=-d29c7290fb7fc640 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps --target i686-unknown-linux-gnu -C linker=cc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/deps/librustc_std_workspace_core-0d2a624f64cace19.rmeta --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-gnu/release/build/compiler_builtins-4fe3d01b7702c90b/out --cfg 'feature="unstable"' --cfg '__absvdi2="optimized-c"' --cfg '__absvsi2="optimized-c"' --cfg '__absvti2="optimized-c"' --cfg '__addvdi3="optimized-c"' --cfg '__addvsi3="optimized-c"' --cfg '__addvti3="optimized-c"' --cfg '__ashldi3="optimized-c"' --cfg '__ashrdi3="optimized-c"' --cfg '__clzdi2="optimized-c"' --cfg '__clzsi2="optimized-c"' --cfg '__clzti2="optimized-c"' --cfg '__cmpdi2="optimized-c"' --cfg '__cmpti2="optimized-c"' --cfg '__ctzdi2="optimized-c"' --cfg '__ctzsi2="optimized-c"' --cfg '__ctzti2="optimized-c"' --cfg '__divdc3="optimized-c"' --cfg '__divdi3="optimized-c"' --cfg '__divsc3="optimized-c"' --cfg '__divxc3="optimized-c"' --cfg '__extendhfsf2="optimized-c"' --cfg '__ffsti2="optimized-c"' --cfg '__floatdidf="optimized-c"' --cfg '__floatdisf="optimized-c"' --cfg '__floatdixf="optimized-c"' --cfg '__floatundidf="optimized-c"' --cfg '__floatundisf="optimized-c"' --cfg '__floatundixf="optimized-c"' --cfg '__int_util="optimized-c"' --cfg '__lshrdi3="optimized-c"' --cfg '__moddi3="optimized-c"' --cfg '__muldc3="optimized-c"' --cfg '__muldi3="optimized-c"' --cfg '__mulsc3="optimized-c"' --cfg '__mulvdi3="optimized-c"' --cfg '__mulvsi3="optimized-c"' --cfg '__mulvti3="optimized-c"' --cfg '__mulxc3="optimized-c"' --cfg '__negdf2="optimized-c"' --cfg '__negdi2="optimized-c"' --cfg '__negsf2="optimized-c"' --cfg '__negti2="optimized-c"' --cfg '__negvdi2="optimized-c"' --cfg '__negvsi2="optimized-c"' --cfg '__negvti2="optimized-c"' --cfg '__paritydi2="optimized-c"' --cfg '__paritysi2="optimized-c"' --cfg '__parityti2="optimized-c"' --cfg '__popcountdi2="optimized-c"' --cfg '__popcountsi2="optimized-c"' --cfg '__popcountti2="optimized-c"' --cfg '__powixf2="optimized-c"' --cfg '__subvdi3="optimized-c"' --cfg '__subvsi3="optimized-c"' --cfg '__subvti3="optimized-c"' --cfg '__truncdfhf2="optimized-c"' --cfg '__truncdfsf2="optimized-c"' --cfg '__truncsfhf2="optimized-c"' --cfg '__ucmpdi2="optimized-c"' --cfg '__ucmpti2="optimized-c"' --cfg '__udivdi3="optimized-c"' --cfg '__umoddi3="optimized-c"' --cfg 'apple_versioning="optimized-c"' -l static=compiler-rt` (signal: 11, SIGSEGV: invalid memory reference)
2020-04-10T16:55:53.9637596Z error: build failed
2020-04-10T16:55:53.9661885Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-10T16:55:53.9662765Z expected success, got: exit code: 101
2020-04-10T16:55:53.9675426Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --target=i686-unknown-linux-gnu
2020-04-10T16:55:53.9675426Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --target=i686-unknown-linux-gnu
2020-04-10T16:55:53.9676031Z Build completed unsuccessfully in 0:00:28
2020-04-10T16:55:53.9724699Z == clock drift check ==
2020-04-10T16:55:53.9737910Z   local time: Fri Apr 10 16:55:53 UTC 2020
2020-04-10T16:55:54.0309968Z   network time: Fri, 10 Apr 2020 16:55:54 GMT
2020-04-10T16:55:55.9301819Z 
2020-04-10T16:55:55.9301819Z 
2020-04-10T16:55:55.9372099Z ##[error]Bash exited with code '1'.
2020-04-10T16:55:55.9385725Z ##[section]Finishing: Run build
2020-04-10T16:55:55.9435655Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T16:55:55.9441030Z Task         : Get sources
2020-04-10T16:55:55.9441288Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T16:55:55.9441529Z Version      : 1.0.0
2020-04-10T16:55:55.9441687Z Author       : Microsoft
2020-04-10T16:55:55.9441687Z Author       : Microsoft
2020-04-10T16:55:55.9441936Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T16:55:55.9442238Z ==============================================================================
2020-04-10T16:55:56.2675237Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T16:55:56.2730938Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-10T16:55:56.2823308Z Cleaning up task key
2020-04-10T16:55:56.2824337Z Start cleaning up orphan processes.
2020-04-10T16:55:56.2994650Z Terminate orphan process: pid (4249) (python)
2020-04-10T16:55:56.5051397Z ##[section]Finishing: Finalize Job
