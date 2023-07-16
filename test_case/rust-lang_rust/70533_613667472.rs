plain
2020-04-14T18:44:08.7618147Z ========================== Starting Command Output ===========================
2020-04-14T18:44:08.7622568Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b36874eb-a987-47c8-b0ca-2f033d514e13.sh
2020-04-14T18:44:08.7622916Z 
2020-04-14T18:44:08.7629178Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T18:44:08.7647244Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70533/merge to s
2020-04-14T18:44:08.7650594Z Task         : Get sources
2020-04-14T18:44:08.7650818Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T18:44:08.7651037Z Version      : 1.0.0
2020-04-14T18:44:08.7651230Z Author       : Microsoft
---
2020-04-14T18:44:09.7532529Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T18:44:09.7537167Z ##[command]git config gc.auto 0
2020-04-14T18:44:09.7540001Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T18:44:09.7543393Z ##[command]git config --get-all http.proxy
2020-04-14T18:44:09.7548756Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70533/merge:refs/remotes/pull/70533/merge
---
2020-04-14T18:47:29.2443531Z  ---> f58a2bb1e753
2020-04-14T18:47:29.2444190Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-14T18:47:29.2454529Z  ---> Using cache
2020-04-14T18:47:29.2454878Z  ---> d079cc6b6db8
2020-04-14T18:47:29.2469001Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-14T18:47:29.2470670Z  ---> 4183ca46ee56
2020-04-14T18:47:29.2470859Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-14T18:47:29.2472328Z  ---> Using cache
2020-04-14T18:47:29.2472690Z  ---> 69e7f8a2a2fb
---
2020-04-14T18:47:29.2856151Z Looks like docker image is the same as before, not uploading
2020-04-14T18:47:36.0530211Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T18:47:36.0952192Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-14T18:47:36.0981746Z == clock drift check ==
2020-04-14T18:47:36.0997673Z   local time: Tue Apr 14 18:47:36 UTC 2020
2020-04-14T18:47:36.2958050Z   network time: Tue, 14 Apr 2020 18:47:36 GMT
2020-04-14T18:47:36.2984866Z Starting sccache server...
2020-04-14T18:47:36.3863691Z configure: processing command line
2020-04-14T18:47:36.3864330Z configure: 
2020-04-14T18:47:36.3865412Z configure: rust.dist-src        := False
---
2020-04-14T18:53:15.5638571Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T18:53:17.1890513Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T18:53:18.9133874Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T18:53:20.4024776Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T18:53:29.8833057Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T18:53:32.5888747Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T18:53:37.2486250Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T18:53:41.6892213Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T18:53:51.3314721Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T19:16:57.7592885Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T19:16:59.5926610Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T19:17:01.5963615Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T19:17:04.0454309Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-14T19:17:14.4327028Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T19:17:18.1269663Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T19:17:23.3060131Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-14T19:17:28.7107139Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T19:17:38.8633645Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-14T19:43:14.8278469Z .................................................................................................... 1700/9894
2020-04-14T19:43:19.1366803Z .................................................................................................... 1800/9894
2020-04-14T19:43:27.3657561Z .................................................................................................... 1900/9894
2020-04-14T19:43:35.1698312Z .....i.............................................................................................. 2000/9894
2020-04-14T19:43:41.7864465Z ...............................................................................................iiiii 2100/9894
2020-04-14T19:44:04.8612793Z .................................................................................................... 2300/9894
2020-04-14T19:44:07.2490541Z .................................................................................................... 2400/9894
2020-04-14T19:44:09.6743557Z .................................................................................................... 2500/9894
2020-04-14T19:44:15.9497368Z .................................................................................................... 2600/9894
---
2020-04-14T19:47:15.7110327Z .................................................................................................... 5100/9894
2020-04-14T19:47:22.9049100Z .................................................................................................... 5200/9894
2020-04-14T19:47:27.4364575Z ................i................................................................................... 5300/9894
2020-04-14T19:47:37.0232430Z ......i............................................................................................. 5400/9894
2020-04-14T19:47:42.0460388Z ......ii.ii........i...i............................................................................ 5500/9894
2020-04-14T19:47:49.2188224Z ....................................................i............................................... 5700/9894
2020-04-14T19:47:58.6164251Z ........................................................................ii.......................... 5800/9894
2020-04-14T19:48:04.8912451Z ...........i........................................................................................ 5900/9894
2020-04-14T19:48:10.0093682Z .................................................................................................... 6000/9894
2020-04-14T19:48:10.0093682Z .................................................................................................... 6000/9894
2020-04-14T19:48:19.6043853Z .................................................................................................... 6100/9894
2020-04-14T19:48:30.1268101Z .....ii...i..ii...........i......................................................................... 6200/9894
2020-04-14T19:48:44.1225857Z .................................................................................................... 6400/9894
2020-04-14T19:48:50.4091457Z .................................................................................................... 6500/9894
2020-04-14T19:48:50.4091457Z .................................................................................................... 6500/9894
2020-04-14T19:49:10.8554628Z ...................................i..ii............................................................ 6600/9894
2020-04-14T19:49:32.5686700Z .................................................................................................... 6800/9894
2020-04-14T19:49:34.5396219Z ...................................i................................................................ 6900/9894
2020-04-14T19:49:36.5093544Z .................................................................................................... 7000/9894
2020-04-14T19:49:38.6579868Z ..........................................................................i......................... 7100/9894
---
2020-04-14T19:51:18.3021962Z .................................................................................................... 7800/9894
2020-04-14T19:51:22.5376327Z .................................................................................................... 7900/9894
2020-04-14T19:51:29.4030250Z .................................................................................................... 8000/9894
2020-04-14T19:51:35.9651362Z ........................................i........................................................... 8100/9894
2020-04-14T19:51:45.9425290Z ........................................................................................iiiiii.iiiii 8200/9894
2020-04-14T19:51:52.4900950Z .i.................................................................................................. 8300/9894
2020-04-14T19:52:05.8027383Z .................................................................................................... 8500/9894
2020-04-14T19:52:16.3038060Z .................................................................................................... 8600/9894
2020-04-14T19:52:29.9337504Z .................................................................................................... 8700/9894
2020-04-14T19:52:35.9877239Z .................................................................................................... 8800/9894
---
2020-04-14T19:54:58.3523721Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-14T19:54:58.3704666Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T19:54:58.5724924Z 
2020-04-14T19:54:58.5725525Z running 185 tests
2020-04-14T19:55:01.2297870Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-14T19:55:03.7995509Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-14T19:55:03.8003053Z 
2020-04-14T19:55:03.8008779Z  finished in 5.430
2020-04-14T19:55:03.8014042Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-14T19:55:03.8201597Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T19:55:05.8461107Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-14T19:55:05.8613893Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T19:55:05.9996991Z 
2020-04-14T19:55:05.9997608Z running 9 tests
2020-04-14T19:55:05.9998750Z iiiiiiiii
2020-04-14T19:55:06.0000468Z 
2020-04-14T19:55:06.0000854Z  finished in 0.138
2020-04-14T19:55:06.0005474Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-14T19:55:06.0176354Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T19:55:24.8899180Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-14T19:55:24.9105273Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T19:55:25.0958806Z 
2020-04-14T19:55:25.0959561Z running 115 tests
2020-04-14T19:55:37.6412980Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-14T19:55:39.3924073Z ...iiii.....ii.
2020-04-14T19:55:39.3925105Z 
2020-04-14T19:55:39.3927506Z  finished in 14.482
2020-04-14T19:55:39.3933807Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-14T19:55:39.3940577Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-14T20:07:53.3079783Z 
2020-04-14T20:07:53.3080526Z    Doc-tests core
2020-04-14T20:07:58.5693664Z 
2020-04-14T20:07:58.5700184Z running 2494 tests
2020-04-14T20:08:06.9832732Z ......iiiii......................................................................................... 100/2494
2020-04-14T20:08:15.9135112Z .....................................................................................ii............. 200/2494
2020-04-14T20:08:36.7890758Z ....................i............................................................................... 400/2494
2020-04-14T20:08:36.7890758Z ....................i............................................................................... 400/2494
2020-04-14T20:08:47.5821179Z ..........................................................................i..i..................iiii 500/2494
2020-04-14T20:09:05.4076418Z .................................................................................................... 700/2494
2020-04-14T20:09:14.0615705Z .................................................................................................... 800/2494
2020-04-14T20:09:22.5222863Z .................................................................................................... 900/2494
2020-04-14T20:09:31.0007087Z .................................................................................................... 1000/2494
---
2020-04-14T20:13:01.5506503Z 
2020-04-14T20:13:01.5506783Z running 1020 tests
2020-04-14T20:13:19.0625141Z i................................................................................................... 100/1020
2020-04-14T20:13:29.1611105Z .................................................................................................... 200/1020
2020-04-14T20:13:36.3035705Z ...................iii......i......i...i......i..................................................... 300/1020
2020-04-14T20:13:40.7050241Z .................................................................................................... 400/1020
2020-04-14T20:13:46.7969159Z ....................................................i....i......................................ii.. 500/1020
2020-04-14T20:13:58.4815018Z .................................................................................................... 700/1020
2020-04-14T20:13:58.4815018Z .................................................................................................... 700/1020
2020-04-14T20:14:05.1913884Z ..............................................iiii.................................................. 800/1020
2020-04-14T20:14:18.0423849Z .................................................................................................... 900/1020
2020-04-14T20:14:23.9488054Z ....................................................................iiii............................ 1000/1020
2020-04-14T20:14:24.6677798Z test result: ok. 1000 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-14T20:14:24.6677983Z 
2020-04-14T20:14:24.6778892Z  finished in 157.060
2020-04-14T20:14:24.6784006Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-14T20:17:22.0061007Z 
2020-04-14T20:17:22.0061282Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-14T20:17:22.0061560Z 
2020-04-14T20:17:22.0061766Z  finished in 0.983
2020-04-14T20:17:22.0062385Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-14T20:17:22.0063276Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T20:17:22.0063914Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T20:17:22.8712295Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-70a868f9747b2669
2020-04-14T20:17:22.8733945Z 
2020-04-14T20:17:22.8734141Z running 0 tests
2020-04-14T20:17:22.8734269Z 
---
2020-04-14T20:31:43.4020085Z Set({"/checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4020823Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4021563Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4022601Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4023308Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4025193Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4025949Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4026660Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-14T20:31:43.4027543Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-14T20:32:47.9325391Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-14T20:32:47.9660843Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-14T20:32:48.2220849Z 
2020-04-14T20:32:48.2222185Z running 211 tests
2020-04-14T20:33:21.5166119Z ......................i...ii.......................................................................i 100/211
2020-04-14T20:34:01.3904637Z ........................................iiiiii......i..............iii.............................. 200/211
2020-04-14T20:34:06.4390637Z .......ii..
2020-04-14T20:34:06.4392730Z 
2020-04-14T20:34:06.4403956Z  finished in 78.474
2020-04-14T20:34:06.4418314Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
2020-04-14T20:34:06.4424233Z doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
---
2020-04-14T20:34:44.0587630Z .........................................
2020-04-14T20:34:44.0593024Z test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-14T20:34:44.0593333Z 
2020-04-14T20:34:44.0603609Z  finished in 13.815
2020-04-14T20:34:44.0604718Z Set({"src/test/rustdoc-gui"}) not skipped for "bootstrap::test::RustdocGUI" -- not in ["src/tools/tidy"]
2020-04-14T20:34:44.0650792Z Cloning into '/checkout/obj/build/browser-UI-test'...
2020-04-14T20:34:44.5675243Z 
2020-04-14T20:34:44.5676132Z You are in 'detached HEAD' state. You can look around, make experimental
2020-04-14T20:34:44.5676483Z changes and commit them, and you can discard any commits you make in this
2020-04-14T20:34:44.5676843Z state without impacting any branches by performing another checkout.
2020-04-14T20:34:44.5676843Z state without impacting any branches by performing another checkout.
2020-04-14T20:34:44.5677272Z 
2020-04-14T20:34:44.5677523Z If you want to create a new branch to retain commits you create, you may
2020-04-14T20:34:44.5678167Z do so (now or later) by using -b with the checkout command again. Example:
2020-04-14T20:34:44.5678416Z 
2020-04-14T20:34:44.5678777Z   git checkout -b <new-branch-name>
2020-04-14T20:34:44.5678929Z 
2020-04-14T20:34:44.5679365Z HEAD is now at 1de38b0 Merge pull request #150 from GuillaumeGomez/png-js
2020-04-14T20:34:44.5701206Z Cloning into '/checkout/obj/build/test-rust-docs-ui'...
2020-04-14T20:34:45.2559197Z Note: checking out '03ea8284c2ab62607b6260793f08ea34953b14be'.
2020-04-14T20:34:45.2559887Z You are in 'detached HEAD' state. You can look around, make experimental
2020-04-14T20:34:45.2560235Z changes and commit them, and you can discard any commits you make in this
2020-04-14T20:34:45.2560547Z state without impacting any branches by performing another checkout.
2020-04-14T20:34:45.2560963Z 
---
2020-04-14T20:34:45.2562512Z 
2020-04-14T20:34:45.2562673Z HEAD is now at 03ea828 Update README.md
2020-04-14T20:34:45.2624042Z 
2020-04-14T20:34:45.2624188Z 
2020-04-14T20:34:45.2624567Z failed to execute command: "npm" "install"
2020-04-14T20:34:45.2625317Z 
2020-04-14T20:34:45.2625403Z 
2020-04-14T20:34:45.2626260Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-14T20:34:45.2626599Z Build completed unsuccessfully in 1:45:23
2020-04-14T20:34:45.2626599Z Build completed unsuccessfully in 1:45:23
2020-04-14T20:34:45.2632662Z == clock drift check ==
2020-04-14T20:34:45.2651339Z   local time: Tue Apr 14 20:34:45 UTC 2020
2020-04-14T20:34:45.4603993Z   network time: Tue, 14 Apr 2020 20:34:45 GMT
2020-04-14T20:34:46.6192991Z 
2020-04-14T20:34:46.6192991Z 
2020-04-14T20:34:46.6308634Z ##[error]Bash exited with code '1'.
2020-04-14T20:34:46.6323098Z ##[section]Finishing: Run build
2020-04-14T20:34:46.6372294Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70533/merge to s
2020-04-14T20:34:46.6377387Z Task         : Get sources
2020-04-14T20:34:46.6377699Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T20:34:46.6377969Z Version      : 1.0.0
2020-04-14T20:34:46.6378159Z Author       : Microsoft
2020-04-14T20:34:46.6378159Z Author       : Microsoft
2020-04-14T20:34:46.6378489Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T20:34:46.6378840Z ==============================================================================
2020-04-14T20:34:47.0336150Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T20:34:47.0391028Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70533/merge to s
2020-04-14T20:34:47.0499551Z Cleaning up task key
2020-04-14T20:34:47.0507134Z Start cleaning up orphan processes.
2020-04-14T20:34:47.0749627Z Terminate orphan process: pid (3645) (python)
2020-04-14T20:34:47.1065175Z ##[section]Finishing: Finalize Job
