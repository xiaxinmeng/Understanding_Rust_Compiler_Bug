plain
2020-02-09T09:56:00.4967866Z ========================== Starting Command Output ===========================
2020-02-09T09:56:00.4968994Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/682bbab0-19b1-4743-9935-ae18a35bcd99.sh
2020-02-09T09:56:00.4969021Z 
2020-02-09T09:56:00.4971335Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-09T09:56:00.4977511Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67695/merge to s
2020-02-09T09:56:00.4979474Z Task         : Get sources
2020-02-09T09:56:00.4979501Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T09:56:00.4979527Z Version      : 1.0.0
2020-02-09T09:56:00.4979552Z Author       : Microsoft
---
2020-02-09T09:56:01.2684608Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-09T09:56:01.2797651Z ##[command]git config gc.auto 0
2020-02-09T09:56:01.2883955Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-09T09:56:01.3139848Z ##[command]git config --get-all http.proxy
2020-02-09T09:56:01.3149786Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67695/merge:refs/remotes/pull/67695/merge
---
2020-02-09T10:48:21.1669345Z .................................................................................................... 1700/9619
2020-02-09T10:48:25.7902060Z .................................................................................................... 1800/9619
2020-02-09T10:48:36.7580779Z ............................i....................................................................... 1900/9619
2020-02-09T10:48:43.0559119Z .................................................................................................... 2000/9619
2020-02-09T10:48:55.5887647Z ..................iiiii............................................................................. 2100/9619
2020-02-09T10:49:04.2947735Z .................................................................................................... 2300/9619
2020-02-09T10:49:06.4304947Z .................................................................................................... 2400/9619
2020-02-09T10:49:10.7332095Z .................................................................................................... 2500/9619
2020-02-09T10:49:29.0088549Z .................................................................................................... 2600/9619
---
2020-02-09T10:51:47.9048646Z .....................................................................i...............i.............. 4900/9619
2020-02-09T10:51:54.9191075Z .................................................................................................... 5000/9619
2020-02-09T10:52:02.1496224Z .................................................................................................... 5100/9619
2020-02-09T10:52:06.7301351Z ...........i........................................................................................ 5200/9619
2020-02-09T10:52:17.2186548Z .....................................................................................ii.ii........i. 5300/9619
2020-02-09T10:52:20.8535128Z ..i................................................................................................. 5400/9619
2020-02-09T10:52:32.1053513Z .................................................................................................... 5600/9619
2020-02-09T10:52:39.9562181Z .........................................................................i.......................... 5700/9619
2020-02-09T10:52:46.7309000Z .................................................................................................... 5800/9619
2020-02-09T10:52:52.7602781Z .................................................................................................... 5900/9619
2020-02-09T10:52:52.7602781Z .................................................................................................... 5900/9619
2020-02-09T10:53:01.5722014Z .................................................................ii...i..ii...........i............. 6000/9619
2020-02-09T10:53:21.0245996Z .................................................................................................... 6200/9619
2020-02-09T10:53:27.9736122Z .................................................................................................... 6300/9619
2020-02-09T10:53:27.9736122Z .................................................................................................... 6300/9619
2020-02-09T10:53:34.5958737Z .............................................................................................i..ii.. 6400/9619
2020-02-09T10:53:54.8356771Z .................................................................................................... 6600/9619
2020-02-09T10:54:03.7098752Z ................................................................................i................... 6700/9619
2020-02-09T10:54:05.6596020Z .................................................................................................... 6800/9619
2020-02-09T10:54:07.6577756Z .......................................................................................i............ 6900/9619
---
2020-02-09T10:55:36.0310287Z .................................................................................................... 7600/9619
2020-02-09T10:55:39.6546128Z .................................................................................................... 7700/9619
2020-02-09T10:55:44.7680343Z .................................................................................................... 7800/9619
2020-02-09T10:55:52.3766455Z .................................................................................................... 7900/9619
2020-02-09T10:56:00.5741513Z ...............................................................iiiiiii.i............................ 8000/9619
2020-02-09T10:56:14.2087062Z ...i......i......................................................................................... 8200/9619
2020-02-09T10:56:19.0657414Z .................................................................................................... 8300/9619
2020-02-09T10:56:32.6375779Z .................................................................................................... 8400/9619
2020-02-09T10:56:40.4750523Z .................................................................................................... 8500/9619
---
2020-02-09T10:58:49.9603536Z  finished in 7.018
2020-02-09T10:58:49.9761720Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T10:58:50.1694877Z 
2020-02-09T10:58:50.1695084Z running 178 tests
2020-02-09T10:58:53.1216052Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-09T10:58:55.3474729Z ...i.i.i...iii..iiiii.iiiiiiiiiii......................iii............ii......
2020-02-09T10:58:55.3476356Z 
2020-02-09T10:58:55.3477742Z  finished in 5.371
2020-02-09T10:58:55.3645325Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T10:58:55.5097508Z 
---
2020-02-09T10:58:57.3308933Z  finished in 1.966
2020-02-09T10:58:57.3503844Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T10:58:57.4881360Z 
2020-02-09T10:58:57.4881592Z running 9 tests
2020-02-09T10:58:57.4882382Z iiiiiiiii
2020-02-09T10:58:57.4882729Z 
2020-02-09T10:58:57.4882764Z  finished in 0.137
2020-02-09T10:58:57.5043252Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T10:58:57.6739085Z 
---
2020-02-09T10:59:16.4532755Z  finished in 18.948
2020-02-09T10:59:16.4726746Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T10:59:16.6492169Z 
2020-02-09T10:59:16.6492391Z running 116 tests
2020-02-09T10:59:29.8719047Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-09T10:59:31.6343693Z ....iiii.....ii.
2020-02-09T10:59:31.6345413Z 
2020-02-09T10:59:31.6346731Z  finished in 15.162
2020-02-09T10:59:31.6351788Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-09T10:59:31.6352436Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-09T11:11:50.5913603Z 
2020-02-09T11:11:50.5924219Z    Doc-tests core
2020-02-09T11:11:54.6305763Z 
2020-02-09T11:11:54.6306426Z running 2471 tests
2020-02-09T11:12:03.2700990Z ......iiiii......................................................................................... 100/2471
2020-02-09T11:12:11.7241411Z ..................................................................................ii................ 200/2471
2020-02-09T11:12:31.2754959Z .................i.................................................................................. 400/2471
2020-02-09T11:12:31.2754959Z .................i.................................................................................. 400/2471
2020-02-09T11:12:40.1075993Z ......................................................................i..i..................iiii.... 500/2471
2020-02-09T11:12:55.6032597Z .................................................................................................... 700/2471
2020-02-09T11:13:03.6264236Z .................................................................................................... 800/2471
2020-02-09T11:13:11.6253585Z .................................................................................................... 900/2471
2020-02-09T11:13:19.7272823Z .................................................................................................... 1000/2471
---
2020-02-09T11:16:22.5495176Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2020-02-09T11:16:22.5507364Z ... 300/760
2020-02-09T11:16:22.6613188Z .................................................................................................... 400/760
2020-02-09T11:16:24.7378002Z .................................................................................................... 500/760
2020-02-09T11:16:24.7624459Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-02-09T11:16:24.7638599Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-02-09T11:16:24.7661399Z .thread '.<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', .src/libstd/sync/mpsc/mod.rs:2778.:.21
2020-02-09T11:16:24.7666114Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-02-09T11:16:24.9725526Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-02-09T11:16:24.9753759Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:.21
2020-02-09T11:16:24.9764577Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-09T11:16:27.0179449Z ..................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-09T11:16:27.0182896Z ......thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-02-09T11:16:27.0183678Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-02-09T11:16:27.0189462Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-02-09T11:16:35.9032905Z 
2020-02-09T11:16:35.9087479Z running 1010 tests
2020-02-09T11:16:52.1573585Z i................................................................................................... 100/1010
2020-02-09T11:17:01.1773316Z .................................................................................................... 200/1010
2020-02-09T11:17:07.4812609Z ..................iii......i......i...i......i...................................................... 300/1010
2020-02-09T11:17:18.0230957Z ............................................i...i.....................................ii............ 500/1010
2020-02-09T11:17:24.5514529Z .................................................................................................... 600/1010
2020-02-09T11:17:29.2331564Z .................................................................................................... 700/1010
2020-02-09T11:17:29.2331564Z .................................................................................................... 700/1010
2020-02-09T11:17:35.2370763Z ....................................iiii............................................................ 800/1010
2020-02-09T11:17:47.8282708Z .................................................................................................... 900/1010
2020-02-09T11:17:53.7547736Z ..........................................................iiii...................................... 1000/1010
2020-02-09T11:17:54.1481396Z test result: ok. 990 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-09T11:17:54.1481423Z 
2020-02-09T11:17:54.1574813Z  finished in 151.760
2020-02-09T11:17:54.1588170Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-09T11:34:24.3503056Z     Checking backtrace v0.3.40
2020-02-09T11:34:26.3965766Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-02-09T11:34:26.3980419Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-02-09T11:34:27.2164512Z  Documenting std v0.0.0 (/checkout/src/libstd)
2020-02-09T11:34:31.6496839Z error: `[erased]` cannot be resolved, ignoring it.
2020-02-09T11:34:31.6497890Z     --> src/libstd/keyword_docs.rs:1216:50
2020-02-09T11:34:31.6498689Z 1216 | /// is being passed. That is, the type has been [erased].
2020-02-09T11:34:31.6499176Z      |                                                  ^^^^^^ cannot be resolved, ignoring
2020-02-09T11:34:31.6499519Z      |
2020-02-09T11:34:31.6499943Z note: the lint level is defined here
2020-02-09T11:34:31.6499943Z note: the lint level is defined here
2020-02-09T11:34:31.6500299Z     --> src/libstd/lib.rs:211:9
2020-02-09T11:34:31.6500816Z      |
2020-02-09T11:34:31.6501205Z 211  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2020-02-09T11:34:31.6502177Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-09T11:34:31.6502534Z 
2020-02-09T11:34:31.6502534Z 
2020-02-09T11:34:31.6518975Z error: `[erased]` cannot be resolved, ignoring it.
2020-02-09T11:34:31.6519432Z     --> src/libstd/keyword_docs.rs:1216:50
2020-02-09T11:34:31.6520173Z 1216 | /// is being passed. That is, the type has been [erased].
2020-02-09T11:34:31.6520583Z      |                                                  ^^^^^^ cannot be resolved, ignoring
2020-02-09T11:34:31.6520909Z      |
2020-02-09T11:34:31.6521445Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-09T11:34:31.6521445Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-09T11:34:31.6521624Z 
2020-02-09T11:34:31.8862237Z error: aborting due to 2 previous errors
2020-02-09T11:34:32.5228062Z 
2020-02-09T11:34:32.5236096Z error: Could not document `std`.
2020-02-09T11:34:32.5236421Z 
2020-02-09T11:34:32.5236581Z Caused by:
2020-02-09T11:34:32.5239829Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.43.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-a50b2a4892bda60b.rmeta --extern backtrace_rs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-8b55be6b06fe0dc1.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-c0a46bd54532630c.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-3743784da0d08c17.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-97219ecb6ce2be37.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-44ec2c826e36804f.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-329aec82b1077588.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-ee264506271ae68f.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-c0bddf96caad2a82.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-eadd12f141b9ce87.rmeta` (exit code: 1)
2020-02-09T11:34:32.5240432Z 
2020-02-09T11:34:32.5240432Z 
2020-02-09T11:34:32.5241336Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.43.0" "--index-page" "/checkout/src/doc/index.md"
2020-02-09T11:34:32.5241698Z 
2020-02-09T11:34:32.5241801Z 
2020-02-09T11:34:32.5241941Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-09T11:34:32.5242061Z Build completed unsuccessfully in 1:32:42
2020-02-09T11:34:32.5242061Z Build completed unsuccessfully in 1:32:42
2020-02-09T11:34:32.5242193Z == clock drift check ==
2020-02-09T11:34:32.5242310Z   local time: Sun Feb  9 11:34:31 UTC 2020
2020-02-09T11:34:32.5242424Z   network time: Sun, 09 Feb 2020 11:34:32 GMT
2020-02-09T11:34:32.5242553Z == end clock drift check ==
2020-02-09T11:34:35.4050031Z 
2020-02-09T11:34:35.4150648Z ##[error]Bash exited with code '1'.
2020-02-09T11:34:35.4159887Z ##[section]Finishing: Run build
2020-02-09T11:34:35.4178675Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67695/merge to s
2020-02-09T11:34:35.4180259Z Task         : Get sources
2020-02-09T11:34:35.4180294Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-09T11:34:35.4180476Z Version      : 1.0.0
2020-02-09T11:34:35.4180508Z Author       : Microsoft
2020-02-09T11:34:35.4180508Z Author       : Microsoft
2020-02-09T11:34:35.4180541Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-09T11:34:35.4180595Z ==============================================================================
2020-02-09T11:34:35.8216663Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-09T11:34:35.8254865Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67695/merge to s
2020-02-09T11:34:35.8354877Z Cleaning up task key
2020-02-09T11:34:35.8355514Z Start cleaning up orphan processes.
2020-02-09T11:34:35.8450561Z Terminate orphan process: pid (4877) (python)
2020-02-09T11:34:35.8656661Z ##[section]Finishing: Finalize Job
