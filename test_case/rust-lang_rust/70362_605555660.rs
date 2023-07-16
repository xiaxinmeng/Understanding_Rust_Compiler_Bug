plain
2020-03-29T02:17:34.8102794Z ========================== Starting Command Output ===========================
2020-03-29T02:17:34.8108737Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4ae209ac-fac7-4c43-8cd1-6b60e57214d3.sh
2020-03-29T02:17:34.8109293Z 
2020-03-29T02:17:34.8114724Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-29T02:17:34.8134516Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70362/merge to s
2020-03-29T02:17:34.8137804Z Task         : Get sources
2020-03-29T02:17:34.8138060Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-29T02:17:34.8138329Z Version      : 1.0.0
2020-03-29T02:17:34.8138498Z Author       : Microsoft
---
2020-03-29T02:17:35.7981048Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-29T02:17:35.7986899Z ##[command]git config gc.auto 0
2020-03-29T02:17:35.7991027Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-29T02:17:35.7994797Z ##[command]git config --get-all http.proxy
2020-03-29T02:17:35.8001818Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70362/merge:refs/remotes/pull/70362/merge
---
2020-03-29T02:25:29.0848276Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-29T02:25:30.5959323Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-29T02:25:32.3485341Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-29T02:25:34.3374954Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-29T02:25:42.9227562Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T02:25:46.4349574Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-29T02:25:50.9728830Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-29T02:25:55.3779061Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-29T02:26:30.6823586Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-29T02:49:54.4026388Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-29T02:49:56.2895112Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-29T02:49:58.3447568Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-29T02:50:01.3072714Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-29T02:50:11.3233952Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T02:50:15.8196157Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-29T02:50:21.4998197Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-29T02:50:27.3009269Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-29T02:51:09.9533031Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-29T03:17:33.9516739Z .................................................................................................... 1700/9855
2020-03-29T03:17:38.0487510Z .................................................................................................... 1800/9855
2020-03-29T03:17:48.4377746Z .........................................................................................i.......... 1900/9855
2020-03-29T03:17:55.3178376Z .................................................................................................... 2000/9855
2020-03-29T03:18:01.9543960Z ...............................................................................iiiii................ 2100/9855
2020-03-29T03:18:23.6502584Z .................................................................................................... 2300/9855
2020-03-29T03:18:25.8245857Z .................................................................................................... 2400/9855
2020-03-29T03:18:28.2445123Z .................................................................................................... 2500/9855
2020-03-29T03:18:37.6886137Z .................................................................................................... 2600/9855
---
2020-03-29T03:21:33.6658983Z .....................................................i...............i.............................. 5000/9855
2020-03-29T03:21:41.7411355Z .................................................................................................... 5100/9855
2020-03-29T03:21:49.3039963Z ..................................................................................................i. 5200/9855
2020-03-29T03:21:54.3695360Z .................................................................................................... 5300/9855
2020-03-29T03:22:05.5036183Z ....................................................................................ii.ii........i.. 5400/9855
2020-03-29T03:22:09.1384180Z .i.................................................................................................. 5500/9855
2020-03-29T03:22:18.3383966Z .............................i...................................................................... 5700/9855
2020-03-29T03:22:28.3552499Z ...............................................ii....................................i.............. 5800/9855
2020-03-29T03:22:35.8530203Z .................................................................................................... 5900/9855
2020-03-29T03:22:41.0395440Z .................................................................................................... 6000/9855
2020-03-29T03:22:41.0395440Z .................................................................................................... 6000/9855
2020-03-29T03:22:50.2694885Z ...............................................................................ii...i..ii........... 6100/9855
2020-03-29T03:23:02.4819810Z i................................................................................................... 6200/9855
2020-03-29T03:23:18.0457791Z .................................................................................................... 6400/9855
2020-03-29T03:23:25.0144808Z .................................................................................................... 6500/9855
2020-03-29T03:23:25.0144808Z .................................................................................................... 6500/9855
2020-03-29T03:23:46.6087142Z .........i..ii...................................................................................... 6600/9855
2020-03-29T03:24:06.7298971Z .................................................................................................... 6800/9855
2020-03-29T03:24:08.8632349Z .........i.......................................................................................... 6900/9855
2020-03-29T03:24:11.0582134Z .................................................................................................... 7000/9855
2020-03-29T03:24:13.2073070Z ..............................................i..................................................... 7100/9855
---
2020-03-29T03:25:57.0406009Z .................................................................................................... 7800/9855
2020-03-29T03:26:02.3270388Z .................................................................................................... 7900/9855
2020-03-29T03:26:08.8950564Z .................................................................................................... 8000/9855
2020-03-29T03:26:17.3815705Z ......i............................................................................................. 8100/9855
2020-03-29T03:26:25.5421960Z .......................................................iiiiiiiiii.i................................. 8200/9855
2020-03-29T03:26:34.8848869Z ...................................................................................................i 8300/9855
2020-03-29T03:26:45.2173361Z .................................................................................................... 8500/9855
2020-03-29T03:26:58.4013851Z .................................................................................................... 8600/9855
2020-03-29T03:27:08.3139892Z .................................................................................................... 8700/9855
2020-03-29T03:27:13.6308832Z .................................................................................................... 8800/9855
---
2020-03-29T03:29:33.4408911Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-03-29T03:29:33.4617020Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T03:29:33.6800331Z 
2020-03-29T03:29:33.6801473Z running 183 tests
2020-03-29T03:29:36.4325905Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/183
2020-03-29T03:29:39.0750169Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii.............ii......
2020-03-29T03:29:39.0754448Z 
2020-03-29T03:29:39.0757970Z  finished in 5.614
2020-03-29T03:29:39.0776610Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-03-29T03:29:39.0962337Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-29T03:29:41.4363700Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-03-29T03:29:41.4364830Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T03:29:41.4365302Z 
2020-03-29T03:29:41.4365576Z running 9 tests
2020-03-29T03:29:41.4366298Z iiiiiiiii
2020-03-29T03:29:41.4367529Z 
2020-03-29T03:29:41.4367784Z  finished in 0.147
2020-03-29T03:29:41.4368494Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-03-29T03:29:41.4369356Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-29T03:30:01.1377028Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-03-29T03:30:01.1611862Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T03:30:01.9399605Z 
2020-03-29T03:30:01.9399943Z running 115 tests
2020-03-29T03:30:15.9431117Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-03-29T03:30:17.5187851Z ...iiii.....ii.
2020-03-29T03:30:17.5192888Z 
2020-03-29T03:30:17.5198246Z  finished in 16.358
2020-03-29T03:30:17.5203411Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-03-29T03:30:17.5207978Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-03-29T03:43:24.2862327Z 
2020-03-29T03:43:24.2863341Z    Doc-tests core
2020-03-29T03:43:29.0307182Z 
2020-03-29T03:43:29.0308021Z running 2487 tests
2020-03-29T03:43:38.4736425Z ......iiiii......................................................................................... 100/2487
2020-03-29T03:43:47.7012383Z .....................................................................................ii............. 200/2487
2020-03-29T03:44:09.2859255Z ....................i............................................................................... 400/2487
2020-03-29T03:44:09.2859255Z ....................i............................................................................... 400/2487
2020-03-29T03:44:19.7871853Z ..........................................................................i..i..................iiii 500/2487
2020-03-29T03:44:36.6330238Z .................................................................................................... 700/2487
2020-03-29T03:44:45.2267443Z .................................................................................................... 800/2487
2020-03-29T03:44:53.7344973Z .................................................................................................... 900/2487
2020-03-29T03:45:02.4057473Z .................................................................................................... 1000/2487
---
2020-03-29T03:48:27.4451020Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-03-29T03:48:27.4459281Z .. 300/760
2020-03-29T03:48:27.6184783Z .................................................................................................... 400/760
2020-03-29T03:48:29.7044009Z .................................................................................................... 500/760
2020-03-29T03:48:29.7348546Z ..................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-03-29T03:48:29.7379974Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778.:21
2020-03-29T03:48:29.7393586Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs.:2766:.17.
2020-03-29T03:48:29.7406100Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-03-29T03:48:29.9571586Z .......................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-03-29T03:48:29.9590172Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-03-29T03:48:29.9610257Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-03-29T03:48:32.0352128Z .....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-03-29T03:48:32.0356374Z .thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-03-29T03:48:32.0364934Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-03-29T03:48:32.0371850Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-03-29T03:48:40.9610111Z 
2020-03-29T03:48:40.9610479Z running 1012 tests
2020-03-29T03:48:59.1450737Z i................................................................................................... 100/1012
2020-03-29T03:49:09.3075040Z .................................................................................................... 200/1012
2020-03-29T03:49:16.4686313Z ..................iii......i......i...i......i...................................................... 300/1012
2020-03-29T03:49:28.1839675Z ............................................i....i......................................ii.......... 500/1012
2020-03-29T03:49:35.4313464Z .................................................................................................... 600/1012
2020-03-29T03:49:40.8352450Z .................................................................................................... 700/1012
2020-03-29T03:49:40.8352450Z .................................................................................................... 700/1012
2020-03-29T03:49:47.6074827Z ......................................iiii.......................................................... 800/1012
2020-03-29T03:50:01.4293498Z .................................................................................................... 900/1012
2020-03-29T03:50:08.1481103Z ............................................................iiii.................................... 1000/1012
2020-03-29T03:50:08.6203560Z test result: ok. 992 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-03-29T03:50:08.6203809Z 
2020-03-29T03:50:08.6295588Z  finished in 166.109
2020-03-29T03:50:08.6302122Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-03-29T03:53:28.4170502Z 
2020-03-29T03:53:28.4170796Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-29T03:53:28.4171069Z 
2020-03-29T03:53:28.4234760Z  finished in 1.004
2020-03-29T03:53:28.4240762Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-03-29T03:53:28.4257158Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-03-29T03:53:28.6373962Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-29T03:53:29.6646077Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-dbd0500e6cd8ca1c
2020-03-29T03:53:29.6673738Z 
2020-03-29T03:53:29.6674284Z running 0 tests
2020-03-29T03:53:29.6674658Z 
---
2020-03-29T04:08:38.5492191Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5493072Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5493963Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5494852Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5495766Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5497573Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5498466Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5499329Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-03-29T04:08:38.5500242Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-03-29T04:08:51.5681169Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-03-29T04:08:51.9997751Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-03-29T04:08:52.1698161Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-03-29T04:08:53.7613752Z     Finished release [optimized] target(s) in 1.75s
2020-03-29T04:08:54.0573887Z core/alloc/enum.AllocInit.html:14: broken link - ptr/index.html
2020-03-29T04:08:54.0575406Z core/alloc/enum.AllocInit.html:21: broken link - ptr/index.html
2020-03-29T04:08:56.3440179Z std/alloc/enum.AllocInit.html:14: broken link - ptr/index.html
2020-03-29T04:08:56.3441863Z std/alloc/enum.AllocInit.html:21: broken link - ptr/index.html
2020-03-29T04:08:58.4529886Z alloc/alloc/enum.AllocInit.html:14: broken link - ptr/index.html
2020-03-29T04:08:58.4532778Z alloc/alloc/enum.AllocInit.html:21: broken link - ptr/index.html
2020-03-29T04:09:01.0520172Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-03-29T04:09:01.0565626Z 
2020-03-29T04:09:01.0565802Z 
2020-03-29T04:09:01.0566796Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-03-29T04:09:01.0567332Z expected success, got: exit code: 101
---
2020-03-29T04:09:01.0650212Z   local time: Sun Mar 29 04:09:01 UTC 2020
2020-03-29T04:09:01.1689162Z   network time: Sun, 29 Mar 2020 04:09:01 GMT
2020-03-29T04:09:01.1690925Z == end clock drift check ==
2020-03-29T04:09:03.7251098Z 
2020-03-29T04:09:03.7351292Z ##[error]Bash exited with code '1'.
2020-03-29T04:09:03.7367110Z ##[section]Finishing: Run build
2020-03-29T04:09:03.7443507Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70362/merge to s
2020-03-29T04:09:03.7449178Z Task         : Get sources
2020-03-29T04:09:03.7450907Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-29T04:09:03.7451800Z Version      : 1.0.0
2020-03-29T04:09:03.7452308Z Author       : Microsoft
2020-03-29T04:09:03.7452308Z Author       : Microsoft
2020-03-29T04:09:03.7452732Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-29T04:09:03.7453212Z ==============================================================================
2020-03-29T04:09:04.1448790Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-29T04:09:04.1499000Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70362/merge to s
2020-03-29T04:09:04.1600199Z Cleaning up task key
2020-03-29T04:09:04.1601668Z Start cleaning up orphan processes.
2020-03-29T04:09:04.1828007Z Terminate orphan process: pid (3364) (python)
2020-03-29T04:09:04.2108767Z ##[section]Finishing: Finalize Job
