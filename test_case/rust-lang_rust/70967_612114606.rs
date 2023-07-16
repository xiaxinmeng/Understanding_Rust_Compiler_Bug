plain
2020-04-10T14:53:22.1391532Z ========================== Starting Command Output ===========================
2020-04-10T14:53:22.1395913Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b8b0c5ac-5847-48df-b8d6-36abdef49c13.sh
2020-04-10T14:53:22.1396372Z 
2020-04-10T14:53:22.1400394Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T14:53:22.1417858Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T14:53:22.1421142Z Task         : Get sources
2020-04-10T14:53:22.1421434Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T14:53:22.1421942Z Version      : 1.0.0
2020-04-10T14:53:22.1422120Z Author       : Microsoft
---
2020-04-10T14:53:23.3127561Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T14:53:23.3138024Z ##[command]git config gc.auto 0
2020-04-10T14:53:23.3145201Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T14:53:23.3151466Z ##[command]git config --get-all http.proxy
2020-04-10T14:53:23.3162335Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70967/merge:refs/remotes/pull/70967/merge
---
2020-04-10T14:54:58.3884788Z Looks like docker image is the same as before, not uploading
2020-04-10T14:55:06.0480455Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T14:55:06.0806933Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-10T14:55:06.0835563Z == clock drift check ==
2020-04-10T14:55:06.0844863Z   local time: Fri Apr 10 14:55:06 UTC 2020
2020-04-10T14:55:06.2782723Z   network time: Fri, 10 Apr 2020 14:55:06 GMT
2020-04-10T14:55:06.2813971Z Starting sccache server...
2020-04-10T14:55:06.3625853Z configure: processing command line
2020-04-10T14:55:06.3626450Z configure: 
2020-04-10T14:55:06.3627510Z configure: rust.dist-src        := False
---
2020-04-10T15:00:37.2477865Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T15:00:38.7500980Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T15:00:40.4459013Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T15:00:42.0565076Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T15:00:51.4383018Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T15:00:54.3021023Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T15:00:58.9618357Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T15:01:03.3566947Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T15:01:13.7407726Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T15:25:18.3322884Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-10T15:25:20.2920936Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-10T15:25:22.3834315Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-10T15:25:24.4634891Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-10T15:25:35.4030144Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T15:25:39.5617376Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-10T15:25:45.1356256Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-10T15:25:50.7825363Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-10T15:26:01.7536635Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-10T15:52:52.7497555Z .................................................................................................... 1600/9883
2020-04-10T15:52:59.2945861Z .................................................................................................... 1700/9883
2020-04-10T15:53:03.7381372Z .................................................................................................... 1800/9883
2020-04-10T15:53:12.6674894Z .................................................................................................... 1900/9883
2020-04-10T15:53:20.9776653Z ..i................................................................................................. 2000/9883
2020-04-10T15:53:27.5492918Z .............................................................................................iiiii.. 2100/9883
2020-04-10T15:53:49.1388686Z .................................................................................................... 2300/9883
2020-04-10T15:53:51.3346699Z .................................................................................................... 2400/9883
2020-04-10T15:53:53.5197691Z .................................................................................................... 2500/9883
2020-04-10T15:53:59.3140519Z .................................................................................................... 2600/9883
---
2020-04-10T15:57:03.7813876Z .................................................................................................... 5100/9883
2020-04-10T15:57:12.0266571Z .................................................................................................... 5200/9883
2020-04-10T15:57:17.2001740Z ...........i........................................................................................ 5300/9883
2020-04-10T15:57:27.5508601Z .................................................................................................... 5400/9883
2020-04-10T15:57:32.4134441Z ii.ii........i...i.................................................................................. 5500/9883
2020-04-10T15:57:40.2324751Z .............................................i...................................................... 5700/9883
2020-04-10T15:57:50.7470606Z .................................................................ii................................. 5800/9883
2020-04-10T15:57:57.2153498Z ....i............................................................................................... 5900/9883
2020-04-10T15:58:02.9539064Z .................................................................................................... 6000/9883
2020-04-10T15:58:02.9539064Z .................................................................................................... 6000/9883
2020-04-10T15:58:12.7676456Z ..................................................................................................ii 6100/9883
2020-04-10T15:58:24.2402249Z ...i..ii...........i................................................................................ 6200/9883
2020-04-10T15:58:40.2930792Z .................................................................................................... 6400/9883
2020-04-10T15:58:46.2000125Z .................................................................................................... 6500/9883
2020-04-10T15:58:46.2000125Z .................................................................................................... 6500/9883
2020-04-10T15:59:00.2082131Z ............................i..ii................................................................... 6600/9883
2020-04-10T15:59:21.6729332Z .................................................................................................... 6800/9883
2020-04-10T15:59:23.7652801Z ............................i....................................................................... 6900/9883
2020-04-10T15:59:25.8420873Z .................................................................................................... 7000/9883
2020-04-10T15:59:28.0260776Z ...................................................................i................................ 7100/9883
---
2020-04-10T16:01:08.7849991Z .................................................................................................... 7800/9883
2020-04-10T16:01:12.7665809Z .................................................................................................... 7900/9883
2020-04-10T16:01:19.2896295Z .................................................................................................... 8000/9883
2020-04-10T16:01:26.3684117Z ................................i................................................................... 8100/9883
2020-04-10T16:01:35.0650251Z ................................................................................iiiiii.iiii.i....... 8200/9883
2020-04-10T16:01:51.0514021Z .........................i......i................................................................... 8400/9883
2020-04-10T16:01:54.9299565Z .................................................................................................... 8500/9883
2020-04-10T16:02:06.1604608Z .................................................................................................... 8600/9883
2020-04-10T16:02:18.6394352Z .................................................................................................... 8700/9883
---
2020-04-10T16:04:45.0979206Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-10T16:04:45.1148126Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:04:45.3388620Z 
2020-04-10T16:04:45.3389377Z running 185 tests
2020-04-10T16:04:48.1961481Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-10T16:04:51.5216361Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-10T16:04:51.5291211Z 
2020-04-10T16:04:51.5291368Z  finished in 5.743
2020-04-10T16:04:51.5292009Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-10T16:04:51.5292772Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T16:04:53.0513748Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-10T16:04:53.0691547Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:04:53.2162522Z 
2020-04-10T16:04:53.2162858Z running 9 tests
2020-04-10T16:04:53.2163849Z iiiiiiiii
2020-04-10T16:04:53.2164769Z 
2020-04-10T16:04:53.2168451Z  finished in 0.147
2020-04-10T16:04:53.2174329Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-10T16:04:53.2339697Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T16:05:13.9499440Z ebuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-10T16:05:13.9733349Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:05:14.1651603Z 
2020-04-10T16:05:14.1652387Z running 115 tests
2020-04-10T16:05:27.9615926Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-10T16:05:29.7350817Z ...iiii.....ii.
2020-04-10T16:05:29.7352924Z 
2020-04-10T16:05:29.7353240Z  finished in 15.762
2020-04-10T16:05:29.7360602Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-10T16:05:29.7362881Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-10T16:18:56.9010525Z 
2020-04-10T16:18:56.9014722Z    Doc-tests core
2020-04-10T16:19:01.7211556Z 
2020-04-10T16:19:01.7212684Z running 2490 tests
2020-04-10T16:19:10.9405870Z ......iiiii......................................................................................... 100/2490
2020-04-10T16:19:20.0268519Z .....................................................................................ii............. 200/2490
2020-04-10T16:19:40.8380006Z ....................i............................................................................... 400/2490
2020-04-10T16:19:40.8380006Z ....................i............................................................................... 400/2490
2020-04-10T16:19:50.7649291Z ..........................................................................i..i..................iiii 500/2490
2020-04-10T16:20:06.7979975Z .................................................................................................... 700/2490
2020-04-10T16:20:15.0298881Z .................................................................................................... 800/2490
2020-04-10T16:20:23.2880706Z .................................................................................................... 900/2490
2020-04-10T16:20:31.5700153Z .................................................................................................... 1000/2490
---
2020-04-10T16:23:55.0334581Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:888:13
2020-04-10T16:23:55.0338799Z . 300/764
2020-04-10T16:23:55.1784114Z .................................................................................................... 400/764
2020-04-10T16:23:57.2731981Z .................................................................................................... 500/764
2020-04-10T16:23:57.3042795Z ......................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError.', src/libstd/sync/mpsc/mod.rs:2741:22
2020-04-10T16:23:57.3067222Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvErrorthread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-04-10T16:23:57.3068388Z ', src/libstd/sync/mpsc/mod.rs:2778:21
2020-04-10T16:23:57.3089684Z .......thread '<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-04-10T16:23:57.5448219Z ........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-04-10T16:23:57.5464586Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-04-10T16:23:57.5488930Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-04-10T16:23:59.5885992Z ............................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-04-10T16:23:59.5888503Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-04-10T16:23:59.5899071Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-04-10T16:23:59.5901078Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-04-10T16:24:08.6244131Z 
2020-04-10T16:24:08.6244440Z running 1019 tests
2020-04-10T16:24:26.4441841Z i................................................................................................... 100/1019
2020-04-10T16:24:36.4985869Z .................................................................................................... 200/1019
2020-04-10T16:24:43.8717590Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-10T16:24:55.1197933Z ...................................................i....i......................................ii... 500/1019
2020-04-10T16:25:02.5764040Z .................................................................................................... 600/1019
2020-04-10T16:25:07.4197048Z .................................................................................................... 700/1019
2020-04-10T16:25:07.4197048Z .................................................................................................... 700/1019
2020-04-10T16:25:14.3662244Z .............................................iiii................................................... 800/1019
2020-04-10T16:25:28.3112954Z .................................................................................................... 900/1019
2020-04-10T16:25:34.3816341Z ...................................................................iiii............................. 1000/1019
2020-04-10T16:25:35.6320735Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-10T16:25:35.6321119Z 
2020-04-10T16:25:35.6437523Z  finished in 169.109
2020-04-10T16:25:35.6442233Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-10T16:28:59.6548721Z 
2020-04-10T16:28:59.6548964Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-10T16:28:59.6549237Z 
2020-04-10T16:28:59.6606151Z  finished in 1.036
2020-04-10T16:28:59.6611174Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-10T16:28:59.6632122Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-10T16:28:59.8485459Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-10T16:29:00.8472165Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-6ca30117392aa36d
2020-04-10T16:29:00.8500003Z 
2020-04-10T16:29:00.8500703Z running 0 tests
2020-04-10T16:29:00.8500979Z 
---
2020-04-10T16:43:08.3866241Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-04-10T16:43:14.3714542Z    Compiling compiler_builtins v0.1.25
2020-04-10T16:43:29.8936525Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-04-10T16:43:31.0925994Z  Documenting alloc v0.0.0 (/checkout/src/liballoc)
2020-04-10T16:43:33.6586576Z error: `[set_len]` cannot be resolved, ignoring it.
2020-04-10T16:43:33.6588621Z     |
2020-04-10T16:43:33.6588621Z     |
2020-04-10T16:43:33.6589677Z 864 |     /// *[undefined behavior]* to call other methods or drop the vector before using [`set_len`]
2020-04-10T16:43:33.6591954Z     |
2020-04-10T16:43:33.6592626Z note: the lint level is defined here
2020-04-10T16:43:33.6593401Z    --> src/liballoc/lib.rs:72:9
2020-04-10T16:43:33.6594043Z     |
2020-04-10T16:43:33.6594043Z     |
2020-04-10T16:43:33.6595011Z 72  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2020-04-10T16:43:33.6597533Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-04-10T16:43:33.6598050Z 
2020-04-10T16:43:33.6598050Z 
2020-04-10T16:43:33.6598694Z error: `[set_len]` cannot be resolved, ignoring it.
2020-04-10T16:43:33.6600349Z     |
2020-04-10T16:43:33.6600349Z     |
2020-04-10T16:43:33.6601231Z 866 |     /// You can have a memory leak if you forget to use [`set_len`] to make the index in bounds
2020-04-10T16:43:33.6603171Z     |
2020-04-10T16:43:33.6604284Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-04-10T16:43:33.6604765Z 
2020-04-10T16:43:33.7710190Z error: aborting due to 2 previous errors
2020-04-10T16:43:33.7710190Z error: aborting due to 2 previous errors
2020-04-10T16:43:33.7711564Z 
2020-04-10T16:43:33.7892683Z error: Could not document `alloc`.
2020-04-10T16:43:33.7892937Z 
2020-04-10T16:43:33.7893494Z Caused by:
2020-04-10T16:43:33.7897002Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc src/liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages -Z unstable-options --resource-suffix 1.44.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-22a504ac3e2c9700.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-631ae9803e600fc3.rmeta` (exit code: 1)
2020-04-10T16:43:33.7909345Z 
2020-04-10T16:43:33.7909345Z 
2020-04-10T16:43:33.7911186Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "-Z" "unstable-options" "--resource-suffix" "1.44.0" "--index-page" "/checkout/src/doc/index.md"
2020-04-10T16:43:33.7912835Z 
2020-04-10T16:43:33.7912936Z 
2020-04-10T16:43:33.7919813Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-10T16:43:33.7920176Z Build completed unsuccessfully in 1:46:47
2020-04-10T16:43:33.7920176Z Build completed unsuccessfully in 1:46:47
2020-04-10T16:43:33.7972214Z == clock drift check ==
2020-04-10T16:43:33.7990560Z   local time: Fri Apr 10 16:43:33 UTC 2020
2020-04-10T16:43:33.8694741Z   network time: Fri, 10 Apr 2020 16:43:33 GMT
2020-04-10T16:43:35.4514034Z 
2020-04-10T16:43:35.4514034Z 
2020-04-10T16:43:35.4585907Z ##[error]Bash exited with code '1'.
2020-04-10T16:43:35.4601015Z ##[section]Finishing: Run build
2020-04-10T16:43:35.4649902Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T16:43:35.4654787Z Task         : Get sources
2020-04-10T16:43:35.4655128Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T16:43:35.4655429Z Version      : 1.0.0
2020-04-10T16:43:35.4655641Z Author       : Microsoft
2020-04-10T16:43:35.4655641Z Author       : Microsoft
2020-04-10T16:43:35.4656135Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T16:43:35.4656713Z ==============================================================================
2020-04-10T16:43:35.8046847Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T16:43:35.8090412Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T16:43:35.8176483Z Cleaning up task key
2020-04-10T16:43:35.8177795Z Start cleaning up orphan processes.
2020-04-10T16:43:35.8381165Z Terminate orphan process: pid (3870) (python)
2020-04-10T16:43:35.8628213Z ##[section]Finishing: Finalize Job
