plain
2020-04-06T12:42:52.2001666Z ========================== Starting Command Output ===========================
2020-04-06T12:42:52.2005583Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7e8569c7-4cac-4720-bd0a-ded8199c70b8.sh
2020-04-06T12:42:52.2005913Z 
2020-04-06T12:42:52.2011081Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T12:42:52.2030642Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70834/merge to s
2020-04-06T12:42:52.2033071Z Task         : Get sources
2020-04-06T12:42:52.2033294Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T12:42:52.2033505Z Version      : 1.0.0
2020-04-06T12:42:52.2033664Z Author       : Microsoft
---
2020-04-06T12:42:53.1994026Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T12:42:53.2000143Z ##[command]git config gc.auto 0
2020-04-06T12:42:53.2004314Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T12:42:53.2008231Z ##[command]git config --get-all http.proxy
2020-04-06T12:42:53.2015688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70834/merge:refs/remotes/pull/70834/merge
---
2020-04-06T12:45:14.0884511Z Looks like docker image is the same as before, not uploading
2020-04-06T12:45:21.6983812Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T12:45:21.7195524Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-06T12:45:21.7215792Z == clock drift check ==
2020-04-06T12:45:21.7234023Z   local time: Mon Apr  6 12:45:21 UTC 2020
2020-04-06T12:45:21.8823001Z   network time: Mon, 06 Apr 2020 12:45:21 GMT
2020-04-06T12:45:21.8841874Z Starting sccache server...
2020-04-06T12:45:21.9509675Z configure: processing command line
2020-04-06T12:45:21.9510394Z configure: 
2020-04-06T12:45:21.9511518Z configure: rust.dist-src        := False
---
2020-04-06T12:49:23.6460911Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T12:49:24.7883532Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T12:49:26.0019572Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T12:49:26.1476308Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T12:49:33.6983494Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T12:49:34.9944587Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T12:49:38.4265747Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T12:49:41.5323781Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T12:49:49.2570751Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T13:07:19.5152829Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T13:07:20.9875936Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T13:07:22.5551393Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T13:07:23.4632376Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T13:07:32.2190236Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T13:07:33.9652598Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T13:07:38.2614130Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T13:07:42.4639152Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T13:07:51.4043319Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T13:27:49.1905932Z .................................................................................................... 1700/9878
2020-04-06T13:27:52.4418840Z .................................................................................................... 1800/9878
2020-04-06T13:27:59.3948509Z .................................................................................................i.. 1900/9878
2020-04-06T13:28:05.5721892Z .................................................................................................... 2000/9878
2020-04-06T13:28:10.5887266Z .......................................................................................iiiii........ 2100/9878
2020-04-06T13:28:27.1463710Z .................................................................................................... 2300/9878
2020-04-06T13:28:28.8335916Z .................................................................................................... 2400/9878
2020-04-06T13:28:30.6244349Z .................................................................................................... 2500/9878
2020-04-06T13:28:35.4051377Z .................................................................................................... 2600/9878
---
2020-04-06T13:30:55.2195490Z .............................................................i...............i...................... 5000/9878
2020-04-06T13:31:01.2032777Z .................................................................................................... 5100/9878
2020-04-06T13:31:07.3071572Z .................................................................................................... 5200/9878
2020-04-06T13:31:11.6339806Z ......i............................................................................................. 5300/9878
2020-04-06T13:31:19.6382532Z ...............................................................................................ii.ii 5400/9878
2020-04-06T13:31:23.5985442Z ........i...i....................................................................................... 5500/9878
2020-04-06T13:31:30.6961040Z ........................................i........................................................... 5700/9878
2020-04-06T13:31:30.6961040Z ........................................i........................................................... 5700/9878
2020-04-06T13:31:38.3886529Z ............................................................ii.....................................i 5800/9878
2020-04-06T13:31:48.2775943Z .................................................................................................... 6000/9878
2020-04-06T13:31:48.2775943Z .................................................................................................... 6000/9878
2020-04-06T13:31:56.0601297Z .............................................................................................ii...i. 6100/9878
2020-04-06T13:32:06.0569681Z .ii...........i..................................................................................... 6200/9878
2020-04-06T13:32:18.4376237Z .................................................................................................... 6400/9878
2020-04-06T13:32:20.6226101Z .................................................................................................... 6500/9878
2020-04-06T13:32:20.6226101Z .................................................................................................... 6500/9878
2020-04-06T13:32:30.6242457Z .......................i..ii........................................................................ 6600/9878
2020-04-06T13:32:47.0840645Z .................................................................................................... 6800/9878
2020-04-06T13:32:48.7114201Z .......................i............................................................................ 6900/9878
2020-04-06T13:32:50.3300549Z .................................................................................................... 7000/9878
2020-04-06T13:32:52.0945353Z ..............................................................i..................................... 7100/9878
---
2020-04-06T13:34:08.9419512Z .................................................................................................... 7800/9878
2020-04-06T13:34:12.6291548Z .................................................................................................... 7900/9878
2020-04-06T13:34:17.3587296Z .................................................................................................... 8000/9878
2020-04-06T13:34:23.3489807Z ...........................i........................................................................ 8100/9878
2020-04-06T13:34:29.7978929Z ............................................................................iiiiiiiiii.i............ 8200/9878
2020-04-06T13:34:41.9315300Z ....................i......i........................................................................ 8400/9878
2020-04-06T13:34:45.7304444Z .................................................................................................... 8500/9878
2020-04-06T13:34:54.2600136Z .................................................................................................... 8600/9878
2020-04-06T13:35:03.8521018Z .................................................................................................... 8700/9878
---
2020-04-06T13:36:56.0514853Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-06T13:36:56.0686668Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T13:36:56.2401715Z 
2020-04-06T13:36:56.2401990Z running 185 tests
2020-04-06T13:36:58.7201028Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-06T13:37:00.8099149Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-06T13:37:00.8101088Z 
2020-04-06T13:37:00.8101203Z  finished in 4.741
2020-04-06T13:37:00.8107301Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-06T13:37:00.8256893Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T13:37:02.7017884Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-06T13:37:02.7170392Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T13:37:02.8347907Z 
2020-04-06T13:37:02.8348241Z running 9 tests
2020-04-06T13:37:02.8354814Z iiiiiiiii
2020-04-06T13:37:02.8356025Z 
2020-04-06T13:37:02.8356211Z  finished in 0.117
2020-04-06T13:37:02.8361238Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-06T13:37:02.8512082Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T13:37:19.5050591Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-06T13:37:19.5227926Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T13:37:19.6730484Z 
2020-04-06T13:37:19.6730864Z running 115 tests
2020-04-06T13:37:30.7915077Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-06T13:37:32.1595154Z ...iiii.....ii.
2020-04-06T13:37:32.1600045Z 
2020-04-06T13:37:32.1600821Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-06T13:37:32.1601327Z  finished in 12.636
2020-04-06T13:37:32.1601996Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-06T13:47:10.5515695Z 
2020-04-06T13:47:10.5516527Z    Doc-tests core
2020-04-06T13:47:14.1586961Z 
2020-04-06T13:47:14.1587691Z running 2492 tests
2020-04-06T13:47:21.2299182Z ......iiiii......................................................................................... 100/2492
2020-04-06T13:47:27.9957081Z .....................................................................................ii............. 200/2492
2020-04-06T13:47:43.7631428Z ......................i............................................................................. 400/2492
2020-04-06T13:47:51.3401326Z ............................................................................i..i..................ii 500/2492
2020-04-06T13:47:57.2285726Z ii.................................................................................................. 600/2492
2020-04-06T13:48:03.6790689Z .................................................................................................... 700/2492
---
2020-04-06T13:51:07.6688128Z 
2020-04-06T13:51:07.6688497Z running 1018 tests
2020-04-06T13:51:20.9270058Z i................................................................................................... 100/1018
2020-04-06T13:51:28.6419781Z .................................................................................................... 200/1018
2020-04-06T13:51:34.4039841Z ..................iii......i......i...i......i...................................................... 300/1018
2020-04-06T13:51:43.0155468Z ..................................................i....i......................................ii.... 500/1018
2020-04-06T13:51:48.7884898Z .................................................................................................... 600/1018
2020-04-06T13:51:52.5795529Z .................................................................................................... 700/1018
2020-04-06T13:51:52.5795529Z .................................................................................................... 700/1018
2020-04-06T13:51:57.9595432Z ............................................iiii.................................................... 800/1018
2020-04-06T13:52:08.7140334Z .................................................................................................... 900/1018
2020-04-06T13:52:13.5853064Z ..................................................................iiii.............................. 1000/1018
2020-04-06T13:52:14.3405834Z test result: ok. 998 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-06T13:52:14.3406038Z 
2020-04-06T13:52:14.3508257Z  finished in 126.554
2020-04-06T13:52:14.3510206Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-06T13:54:35.2865318Z 
2020-04-06T13:54:35.2865626Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-06T13:54:35.2865908Z 
2020-04-06T13:54:35.2866117Z  finished in 0.761
2020-04-06T13:54:35.2866889Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-06T13:54:35.2867972Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-06T13:54:35.2868808Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T13:54:35.6839020Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-2ec9520e787c69a9
2020-04-06T13:54:35.6860088Z 
2020-04-06T13:54:35.6860268Z running 0 tests
2020-04-06T13:54:35.6860393Z 
---
2020-04-06T14:05:58.9458293Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9459056Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9459861Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9460746Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9461552Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9463195Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9464014Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9464782Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-06T14:05:58.9465574Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-06T14:06:08.8850312Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-04-06T14:06:09.1939460Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-04-06T14:06:09.4660878Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-04-06T14:06:10.5217013Z     Finished release [optimized] target(s) in 1.32s
2020-04-06T14:06:10.7406889Z core/future/fn.pending.html:3: broken link - core/task/struct.Poll.html
2020-04-06T14:06:12.3941221Z std/future/fn.pending.html:3: broken link - core/task/struct.Poll.html
2020-04-06T14:06:15.8194099Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-04-06T14:06:15.8218333Z 
2020-04-06T14:06:15.8218768Z 
2020-04-06T14:06:15.8219986Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-04-06T14:06:15.8220942Z expected success, got: exit code: 101
2020-04-06T14:06:15.8220942Z expected success, got: exit code: 101
2020-04-06T14:06:15.8221254Z 
2020-04-06T14:06:15.8221469Z 
2020-04-06T14:06:15.8226423Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-06T14:06:15.8227007Z Build completed unsuccessfully in 1:19:37
2020-04-06T14:06:15.8275937Z == clock drift check ==
2020-04-06T14:06:15.8292444Z   local time: Mon Apr  6 14:06:15 UTC 2020
2020-04-06T14:06:15.8953758Z   network time: Mon, 06 Apr 2020 14:06:15 GMT
2020-04-06T14:06:18.5828818Z 
2020-04-06T14:06:18.5828818Z 
2020-04-06T14:06:18.5875426Z ##[error]Bash exited with code '1'.
2020-04-06T14:06:18.5897224Z ##[section]Finishing: Run build
2020-04-06T14:06:18.5940977Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70834/merge to s
2020-04-06T14:06:18.5946055Z Task         : Get sources
2020-04-06T14:06:18.5946411Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T14:06:18.5946748Z Version      : 1.0.0
2020-04-06T14:06:18.5946981Z Author       : Microsoft
2020-04-06T14:06:18.5946981Z Author       : Microsoft
2020-04-06T14:06:18.5947340Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T14:06:18.5947790Z ==============================================================================
2020-04-06T14:06:18.8556500Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T14:06:18.8594972Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70834/merge to s
2020-04-06T14:06:18.8676274Z Cleaning up task key
2020-04-06T14:06:18.8681510Z Start cleaning up orphan processes.
2020-04-06T14:06:18.8831463Z Terminate orphan process: pid (3554) (python)
2020-04-06T14:06:18.9065221Z ##[section]Finishing: Finalize Job
