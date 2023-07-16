plain
2020-04-17T08:22:56.1118559Z ========================== Starting Command Output ===========================
2020-04-17T08:22:56.1121523Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3e93baa0-d79c-45f2-b468-b47a2e660a74.sh
2020-04-17T08:22:56.1121788Z 
2020-04-17T08:22:56.1126488Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T08:22:56.1146843Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71237/merge to s
2020-04-17T08:22:56.1150261Z Task         : Get sources
2020-04-17T08:22:56.1150563Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T08:22:56.1150853Z Version      : 1.0.0
2020-04-17T08:22:56.1151074Z Author       : Microsoft
---
2020-04-17T08:22:57.1089550Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T08:22:57.1102020Z ##[command]git config gc.auto 0
2020-04-17T08:22:57.1112330Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T08:22:57.1125460Z ##[command]git config --get-all http.proxy
2020-04-17T08:22:57.1132747Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71237/merge:refs/remotes/pull/71237/merge
---
2020-04-17T08:26:01.4469384Z  ---> f58a2bb1e753
2020-04-17T08:26:01.4471059Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-17T08:26:01.4475615Z  ---> Using cache
2020-04-17T08:26:01.4475994Z  ---> d079cc6b6db8
2020-04-17T08:26:01.4476864Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-17T08:26:01.4488721Z  ---> 4183ca46ee56
2020-04-17T08:26:01.4488949Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-17T08:26:01.4489298Z  ---> Using cache
2020-04-17T08:26:01.4489614Z  ---> 69e7f8a2a2fb
---
2020-04-17T08:26:01.4929320Z Looks like docker image is the same as before, not uploading
2020-04-17T08:26:09.1589935Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T08:26:09.1905491Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-17T08:26:09.1937094Z == clock drift check ==
2020-04-17T08:26:09.1945573Z   local time: Fri Apr 17 08:26:09 UTC 2020
2020-04-17T08:26:09.2658988Z   network time: Fri, 17 Apr 2020 08:26:09 GMT
2020-04-17T08:26:09.2681335Z Starting sccache server...
2020-04-17T08:26:09.3654025Z configure: processing command line
2020-04-17T08:26:09.3654339Z configure: 
2020-04-17T08:26:09.3655393Z configure: rust.dist-src        := False
---
2020-04-17T08:31:31.0782159Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T08:31:32.5553350Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T08:31:34.0856560Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T08:31:35.5869470Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T08:31:44.4474606Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T08:31:47.5564424Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T08:31:52.2335526Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T08:31:56.5537645Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T08:32:05.4704385Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T08:55:12.1154464Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T08:55:13.9296363Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T08:55:15.9758998Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T08:55:17.8302150Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T08:55:28.2840992Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T08:55:31.6751275Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T08:55:37.0691586Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T08:55:42.5886332Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T08:55:53.4543865Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T09:21:52.2073942Z .................................................................................................... 1700/9903
2020-04-17T09:21:56.8941843Z .................................................................................................... 1800/9903
2020-04-17T09:22:05.1285418Z .................................................................................................... 1900/9903
2020-04-17T09:22:14.1850497Z .........i.......................................................................................... 2000/9903
2020-04-17T09:22:20.7649605Z ...................................................................................................i 2100/9903
2020-04-17T09:22:35.3179720Z iiii................................................................................................ 2200/9903
2020-04-17T09:22:44.0540272Z .................................................................................................... 2400/9903
2020-04-17T09:22:46.2945384Z .................................................................................................... 2500/9903
2020-04-17T09:22:51.7181353Z .................................................................................................... 2600/9903
2020-04-17T09:23:11.2191657Z .................................................................................................... 2700/9903
---
2020-04-17T09:25:49.8900719Z .................................................................................................... 5100/9903
2020-04-17T09:25:57.5058577Z .................................................................................................... 5200/9903
2020-04-17T09:26:01.9361376Z .....................i.............................................................................. 5300/9903
2020-04-17T09:26:11.9751403Z ...........i........................................................................................ 5400/9903
2020-04-17T09:26:17.6308459Z ...........ii.ii........i...i....................................................................... 5500/9903
2020-04-17T09:26:25.2241760Z .........................................................i.......................................... 5700/9903
2020-04-17T09:26:34.4649141Z .............................................................................ii..................... 5800/9903
2020-04-17T09:26:41.1923279Z ................i................................................................................... 5900/9903
2020-04-17T09:26:46.7304405Z .................................................................................................... 6000/9903
2020-04-17T09:26:46.7304405Z .................................................................................................... 6000/9903
2020-04-17T09:26:57.1509259Z .................................................................................................... 6100/9903
2020-04-17T09:27:07.5323327Z ..........ii....i.ii...........i.................................................................... 6200/9903
2020-04-17T09:27:23.1596592Z .................................................................................................... 6400/9903
2020-04-17T09:27:29.4984275Z .................................................................................................... 6500/9903
2020-04-17T09:27:29.4984275Z .................................................................................................... 6500/9903
2020-04-17T09:27:46.1444402Z ........................................i..ii....................................................... 6600/9903
2020-04-17T09:28:08.1575237Z .................................................................................................... 6800/9903
2020-04-17T09:28:10.2258684Z .........................................i.......................................................... 6900/9903
2020-04-17T09:28:12.2334378Z .................................................................................................... 7000/9903
2020-04-17T09:28:14.3676229Z .................................................................................i.................. 7100/9903
---
2020-04-17T09:29:51.3883852Z .................................................................................................... 7800/9903
2020-04-17T09:29:55.9018393Z .................................................................................................... 7900/9903
2020-04-17T09:30:02.2824022Z .................................................................................................... 8000/9903
2020-04-17T09:30:08.2349071Z ...............................................i.................................................... 8100/9903
2020-04-17T09:30:18.0220282Z ...............................................................................................iiiii 8200/9903
2020-04-17T09:30:23.6384518Z i.iiiii.i........................................................................................... 8300/9903
2020-04-17T09:30:36.7486782Z .................................................................................................... 8500/9903
2020-04-17T09:30:44.9726870Z .................................................................................................... 8600/9903
2020-04-17T09:30:59.1520032Z .................................................................................................... 8700/9903
2020-04-17T09:31:05.8409813Z .................................................................................................... 8800/9903
---
2020-04-17T09:33:22.8557422Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-17T09:33:22.8771905Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T09:33:23.1054700Z 
2020-04-17T09:33:23.1055053Z running 185 tests
2020-04-17T09:33:25.7467551Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-17T09:33:28.3630568Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-17T09:33:28.3635350Z 
2020-04-17T09:33:28.3644547Z  finished in 5.487
2020-04-17T09:33:28.3724367Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-17T09:33:28.3817952Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T09:33:30.4523930Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-17T09:33:30.4702728Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T09:33:30.6101639Z 
2020-04-17T09:33:30.6101825Z running 9 tests
2020-04-17T09:33:30.6102749Z iiiiiiiii
2020-04-17T09:33:30.6104113Z 
2020-04-17T09:33:30.6104482Z  finished in 0.140
2020-04-17T09:33:30.6112569Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-17T09:33:30.6269666Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T09:33:50.8822803Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-17T09:33:50.9041043Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T09:33:51.0902887Z 
2020-04-17T09:33:51.0903257Z running 115 tests
2020-04-17T09:34:05.4219870Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-17T09:34:07.3358235Z ...iiii.....ii.
2020-04-17T09:34:07.3359250Z 
2020-04-17T09:34:07.3359384Z  finished in 16.432
2020-04-17T09:34:07.3359906Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-17T09:34:07.3360514Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-17T09:47:09.8733456Z 
2020-04-17T09:47:09.8740279Z    Doc-tests core
2020-04-17T09:47:14.5391665Z 
2020-04-17T09:47:14.5392593Z running 2496 tests
2020-04-17T09:47:23.4561799Z ......iiiii......................................................................................... 100/2496
2020-04-17T09:47:32.4403163Z .....................................................................................ii............. 200/2496
2020-04-17T09:47:52.7813679Z ....................i............................................................................... 400/2496
2020-04-17T09:47:52.7813679Z ....................i............................................................................... 400/2496
2020-04-17T09:48:02.4367528Z ..........................................................................i..i..................iiii 500/2496
2020-04-17T09:48:18.4690412Z .................................................................................................... 700/2496
2020-04-17T09:48:26.8476250Z .................................................................................................... 800/2496
2020-04-17T09:48:35.0232419Z .................................................................................................... 900/2496
2020-04-17T09:48:43.2190650Z .................................................................................................... 1000/2496
---
2020-04-17T09:52:18.9189720Z 
2020-04-17T09:52:18.9190260Z running 1020 tests
2020-04-17T09:52:36.4193382Z i................................................................................................... 100/1020
2020-04-17T09:52:46.5260910Z .................................................................................................... 200/1020
2020-04-17T09:52:54.0526546Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-17T09:52:58.8486579Z .................................................................................................... 400/1020
2020-04-17T09:53:05.6010284Z ....................................................i....i......................................ii.. 500/1020
2020-04-17T09:53:18.6641703Z .................................................................................................... 700/1020
2020-04-17T09:53:18.6641703Z .................................................................................................... 700/1020
2020-04-17T09:53:25.8673894Z ..............................................iiii.................................................. 800/1020
2020-04-17T09:53:40.2212241Z .................................................................................................... 900/1020
2020-04-17T09:53:46.3033644Z ....................................................................iiii............................ 1000/1020
2020-04-17T09:53:47.6222304Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-17T09:53:47.6222515Z 
2020-04-17T09:53:47.6341428Z  finished in 167.582
2020-04-17T09:53:47.6347626Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-17T09:57:05.4262826Z 
2020-04-17T09:57:05.4263148Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-17T09:57:05.4263393Z 
2020-04-17T09:57:05.4335089Z  finished in 1.027
2020-04-17T09:57:05.4336436Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-17T09:57:06.3789128Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T09:57:06.3789919Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T09:57:06.6358948Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-fb111e24749be64b
2020-04-17T09:57:06.6386008Z 
2020-04-17T09:57:06.6386243Z running 0 tests
2020-04-17T09:57:06.6386426Z 
---
2020-04-17T10:11:47.3219013Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3219720Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3220429Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3221137Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3222036Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3223478Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3226811Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3227570Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-17T10:11:47.3228505Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-17T10:12:49.9124632Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-17T10:12:49.9369669Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T10:12:50.1369161Z 
2020-04-17T10:12:50.1369558Z running 211 tests
2020-04-17T10:13:22.7126293Z ......................i...ii.......................................................................i 100/211
2020-04-17T10:14:00.5638479Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-17T10:14:05.3904672Z .......ii..
2020-04-17T10:14:05.3906511Z 
2020-04-17T10:14:05.3910285Z  finished in 75.454
2020-04-17T10:14:05.3925991Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-17T10:14:05.3937021Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-17T10:14:19.0002962Z doc tests for: /checkout/src/doc/rustc/src/targets/index.md
2020-04-17T10:14:19.0164432Z doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
2020-04-17T10:14:19.1545777Z  finished in 4.190
2020-04-17T10:14:19.1558583Z Set({"src/test/rustdoc-js-std"}) not skipped for "bootstrap::test::RustdocJSStd" -- not in ["src/tools/tidy"]
2020-04-17T10:14:19.9956359Z Checking "alias-1" ... OK
2020-04-17T10:14:20.0593286Z Checking "alias-2" ... OK
2020-04-17T10:14:20.1197970Z Checking "alias-3" ... OK
2020-04-17T10:14:20.1785339Z Checking "alias" ... OK
2020-04-17T10:14:20.2560905Z Checking "basic" ... OK
2020-04-17T10:14:20.3454795Z Checking "deduplication" ... OK
2020-04-17T10:14:20.4045853Z Checking "enum-option" ... OK
2020-04-17T10:14:20.4830770Z Checking "filter-crate" ... OK
2020-04-17T10:14:20.5615898Z Checking "fn-forget" ... OK
2020-04-17T10:14:20.6829733Z Checking "from_u" ... OK
2020-04-17T10:14:20.7734631Z Checking "keyword" ... OK
2020-04-17T10:14:20.8168253Z Checking "macro-check" ... OK
2020-04-17T10:14:20.8467336Z Checking "macro-print" ... OK
2020-04-17T10:14:20.9158199Z Checking "multi-query" ... OK
2020-04-17T10:14:20.9414068Z Checking "never" ... OK
2020-04-17T10:14:20.9561286Z Checking "quoted" ... OK
2020-04-17T10:14:20.9799322Z Checking "return-specific-literal" ... OK
2020-04-17T10:14:21.0455840Z Checking "return-specific" ... OK
2020-04-17T10:14:21.0805684Z Checking "should-fail" ... OK
2020-04-17T10:14:21.1406755Z Checking "string-from_ut" ... OK
2020-04-17T10:14:21.1925844Z Checking "struct-vec" ... OK
2020-04-17T10:14:21.2959684Z Checking "vec-new" ... OK
2020-04-17T10:14:21.3200305Z Check compiletest suite=rustdoc-js mode=js-doc-test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-17T10:14:21.4798043Z 
2020-04-17T10:14:21.4798304Z running 6 tests
2020-04-17T10:14:27.5126635Z ......
---
2020-04-17T10:14:27.6600164Z    Compiling rustdoc-themes v0.1.0 (/checkout/src/tools/rustdoc-themes)
2020-04-17T10:14:28.3587061Z     Finished release [optimized] target(s) in 0.84s
2020-04-17T10:14:28.3858369Z rustdoc: [check-theme] Starting tests! (Ignoring all other arguments)
2020-04-17T10:14:28.3861679Z  - Checking "/checkout/src/librustdoc/html/static/themes/dark.css"... OK
2020-04-17T10:14:28.3867209Z  - Checking "/checkout/src/librustdoc/html/static/themes/ayu.css"... FAILED
2020-04-17T10:14:28.3867571Z   Missing ".content .highlighted.mod,.content .highlighted.externcrate" rule
2020-04-17T10:14:28.3868034Z   Missing ".content span.attr,.content a.attr,.block a.current.attr,.content span.derive,.content a.derive,.block a.current.derive,.content span.macro,.content a.macro,.block a.current.macro" rule
2020-04-17T10:14:28.3868477Z   Missing ".content .highlighted.trait" rule
2020-04-17T10:14:28.3868744Z   Missing ".content span.struct,.content a.struct,.block a.current.struct" rule
2020-04-17T10:14:28.3869030Z   Missing "#titles>div:hover,#titles>div.selected" rule
2020-04-17T10:14:28.3869292Z   Missing ".content .highlighted.traitalias" rule
2020-04-17T10:14:28.3869567Z   Missing ".content span.type,.content a.type,.block a.current.type" rule
2020-04-17T10:14:28.3869864Z   Missing ".content span.union,.content a.union,.block a.current.union" rule
2020-04-17T10:14:28.3870587Z   Missing "::-webkit-scrollbar-track" rule
2020-04-17T10:14:28.3870815Z   Missing ".content .highlighted.foreigntype" rule
2020-04-17T10:14:28.3871243Z   Missing "#crate-search+.search-input:focus" rule
2020-04-17T10:14:28.3871481Z   Missing "pre.rust .lifetime" rule
2020-04-17T10:14:28.3871691Z   Missing ".content .highlighted.primitive" rule
2020-04-17T10:14:28.3871958Z   Missing ".content .highlighted.constant,.content .highlighted.static" rule
2020-04-17T10:14:28.3872219Z   Missing ".stab.unstable" rule
2020-04-17T10:14:28.3872499Z   Missing ".content .highlighted.fn,.content .highlighted.method,.content .highlighted.tymethod" rule
2020-04-17T10:14:28.3872934Z   Missing "h2,h3:not(.impl):not(.method):not(.type):not(.tymethod),h4:not(.method):not(.type):not(.tymethod)" rule
2020-04-17T10:14:28.3873468Z   Missing ".content span.enum,.content a.enum,.block a.current.enum" rule
2020-04-17T10:14:28.3873855Z   Missing ".content span.constant,.content a.constant,.block a.current.constant,.content span.static,.content a.static,.block a.current.static" rule
2020-04-17T10:14:28.3874264Z   Missing ".content span.keyword,.content a.keyword,.block a.current.keyword" rule
2020-04-17T10:14:28.3874533Z   Missing "pre.rust .comment" rule
2020-04-17T10:14:28.3874738Z   Missing ".content .highlighted.enum" rule
2020-04-17T10:14:28.3874956Z   Missing ".content .highlighted.struct" rule
2020-04-17T10:14:28.3875194Z   Missing ".content .highlighted.keyword" rule
2020-04-17T10:14:28.3875474Z   Missing ".content span.traitalias,.content a.traitalias,.block a.current.traitalias" rule
2020-04-17T10:14:28.3876181Z   Missing ".content span.fn,.content a.fn,.block a.current.fn,.content span.method,.content a.method,.block a.current.method,.content span.tymethod,.content a.tymethod,.block a.current.tymethod,.content .fnname" rule
2020-04-17T10:14:28.3876731Z   Missing "pre.rust .kw" rule
2020-04-17T10:14:28.3877371Z   Missing "pre.rust .self,pre.rust .bool-val,pre.rust .prelude-val,pre.rust .attribute,pre.rust .attribute .ident" rule
2020-04-17T10:14:28.3877827Z   Missing "*" rule
2020-04-17T10:14:28.3878081Z   Missing ".content span.foreigntype,.content a.foreigntype,.block a.current.foreigntype" rule
2020-04-17T10:14:28.3878472Z   Missing "pre.rust .doccomment" rule
2020-04-17T10:14:28.3878695Z   Missing ".stab.deprecated" rule
2020-04-17T10:14:28.3879144Z   Missing ".content .highlighted.attr,.content .highlighted.derive,.content .highlighted.macro" rule
2020-04-17T10:14:28.3879452Z   Missing ".stab.portability" rule
2020-04-17T10:14:28.3880188Z   Missing ".sidebar::-webkit-scrollbar-track" rule
2020-04-17T10:14:28.3880679Z   Missing ".sidebar::-webkit-scrollbar-thumb" rule
2020-04-17T10:14:28.3880935Z   Missing ".content .highlighted.union" rule
2020-04-17T10:14:28.3881514Z   Missing ".content span.primitive,.content a.primitive,.block a.current.primitive" rule
2020-04-17T10:14:28.3882486Z   Missing "::-webkit-scrollbar-thumb" rule
2020-04-17T10:14:28.3882801Z   Missing ".content span.externcrate,.content span.mod,.content a.mod,.block a.current.mod" rule
2020-04-17T10:14:28.3883259Z   Missing ".content .highlighted.type" rule
2020-04-17T10:14:28.3883945Z   Missing "pre.rust .kw-2,pre.rust .prelude-ty" rule
2020-04-17T10:14:28.3884463Z   Missing ".content span.trait,.content a.trait,.block a.current.trait" rule
2020-04-17T10:14:28.3884759Z   Missing ".stab.internal" rule
2020-04-17T10:14:28.3885773Z 
2020-04-17T10:14:28.3886716Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustdoc-themes" "/checkout/obj/build/bootstrap/debug/rustdoc" "/checkout/src/librustdoc/html/static/themes"
2020-04-17T10:14:28.3887339Z expected success, got: exit code: 1
2020-04-17T10:14:28.3887511Z 
2020-04-17T10:14:28.3887511Z 
2020-04-17T10:14:28.3887594Z 
2020-04-17T10:14:28.3897571Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-17T10:14:28.3897915Z Build completed unsuccessfully in 1:46:38
2020-04-17T10:14:28.3961233Z == clock drift check ==
2020-04-17T10:14:28.3989095Z   local time: Fri Apr 17 10:14:28 UTC 2020
2020-04-17T10:14:28.4665306Z   network time: Fri, 17 Apr 2020 10:14:28 GMT
2020-04-17T10:14:29.2451482Z 
2020-04-17T10:14:29.2451482Z 
2020-04-17T10:14:29.2549592Z ##[error]Bash exited with code '1'.
2020-04-17T10:14:29.2567228Z ##[section]Finishing: Run build
2020-04-17T10:14:29.2628316Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71237/merge to s
2020-04-17T10:14:29.2634763Z Task         : Get sources
2020-04-17T10:14:29.2635125Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T10:14:29.2635485Z Version      : 1.0.0
2020-04-17T10:14:29.2635728Z Author       : Microsoft
2020-04-17T10:14:29.2635728Z Author       : Microsoft
2020-04-17T10:14:29.2636110Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T10:14:29.2636559Z ==============================================================================
2020-04-17T10:14:29.6118906Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T10:14:29.6167881Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71237/merge to s
2020-04-17T10:14:29.6270859Z Cleaning up task key
2020-04-17T10:14:29.6272276Z Start cleaning up orphan processes.
2020-04-17T10:14:29.6493643Z Terminate orphan process: pid (4363) (python)
2020-04-17T10:14:29.6782773Z ##[section]Finishing: Finalize Job
