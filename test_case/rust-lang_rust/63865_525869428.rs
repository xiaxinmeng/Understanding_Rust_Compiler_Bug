plain
2019-08-28T16:40:52.5525111Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-28T16:40:52.5715396Z ##[command]git config gc.auto 0
2019-08-28T16:40:52.5787278Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-28T16:40:52.5846461Z ##[command]git config --get-all http.proxy
2019-08-28T16:40:52.6021667Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63865/merge:refs/remotes/pull/63865/merge
---
2019-08-28T17:44:29.5760419Z .................................................................................................... 1500/8969
2019-08-28T17:44:35.3830404Z .................................................................................................... 1600/8969
2019-08-28T17:44:48.3196975Z .............................................i...............i...................................... 1700/8969
2019-08-28T17:44:56.5276988Z .................................................................................................... 1800/8969
2019-08-28T17:45:11.0008546Z ....................................iiiii........................................................... 1900/8969
2019-08-28T17:45:21.8022628Z .................................................................................................... 2100/8969
2019-08-28T17:45:24.4244351Z .................................................................................................... 2200/8969
2019-08-28T17:45:28.8214036Z .................................................................................................... 2300/8969
2019-08-28T17:45:36.2110044Z .................................................................................................... 2400/8969
---
2019-08-28T17:48:36.9246687Z .......................i...............i............................................................ 4700/8969
2019-08-28T17:48:48.9350142Z .................................................................................................... 4800/8969
2019-08-28T17:48:55.2569322Z .................................................................................................... 4900/8969
2019-08-28T17:49:06.4514159Z .................................................................................................... 5000/8969
2019-08-28T17:49:11.8367066Z ....ii.ii........................................................................................... 5100/8969
2019-08-28T17:49:26.0019750Z .................................................................................................... 5300/8969
2019-08-28T17:49:33.3573056Z ...................................................................i................................ 5400/8969
2019-08-28T17:49:40.7784263Z .................................................................................................... 5500/8969
2019-08-28T17:49:47.8187565Z .................................................................................................... 5600/8969
2019-08-28T17:49:47.8187565Z .................................................................................................... 5600/8969
2019-08-28T17:49:58.3002188Z .............................................................ii...i..ii............i................ 5700/8969
2019-08-28T17:50:24.6546875Z .................................................................................................... 5900/8969
2019-08-28T17:50:33.3120334Z .................................................................................................... 6000/8969
2019-08-28T17:50:33.3120334Z .................................................................................................... 6000/8969
2019-08-28T17:50:40.0788595Z ..............................................................i..ii................................. 6100/8969
2019-08-28T17:51:08.5883096Z .................................................................................................... 6300/8969
2019-08-28T17:51:11.2923020Z .................i.................................................................................. 6400/8969
2019-08-28T17:51:12.8996772Z .........................................................................................i.......... 6500/8969
2019-08-28T17:51:15.6452749Z .................................................................................................... 6600/8969
---
2019-08-28T17:55:59.0418153Z  finished in 20.874
2019-08-28T17:55:59.0609935Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T17:55:59.2229796Z 
2019-08-28T17:55:59.2230842Z running 149 tests
2019-08-28T17:56:02.5244563Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/149
2019-08-28T17:56:04.5414698Z ..iiii..............i.........iii.i......ii......
2019-08-28T17:56:04.5416414Z 
2019-08-28T17:56:04.5418311Z  finished in 5.480
2019-08-28T17:56:04.5604037Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T17:56:04.7229760Z 
---
2019-08-28T17:56:06.8174750Z  finished in 2.257
2019-08-28T17:56:06.8350061Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T17:56:06.9878425Z 
2019-08-28T17:56:06.9878553Z running 9 tests
2019-08-28T17:56:06.9880210Z iiiiiiiii
2019-08-28T17:56:06.9880650Z 
2019-08-28T17:56:06.9881136Z  finished in 0.153
2019-08-28T17:56:07.0079729Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T17:56:07.1894996Z 
---
2019-08-28T17:56:25.4745727Z  finished in 18.466
2019-08-28T17:56:25.4935802Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T17:56:25.6557042Z 
2019-08-28T17:56:25.6557309Z running 122 tests
2019-08-28T17:56:50.1763688Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-28T17:56:54.9838467Z .i.i......iii.i.....ii
2019-08-28T17:56:54.9840939Z 
2019-08-28T17:56:54.9842735Z  finished in 29.490
2019-08-28T17:56:54.9849318Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T17:56:54.9850058Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-28T18:11:12.9189293Z 
2019-08-28T18:11:12.9191068Z    Doc-tests core
2019-08-28T18:11:18.3202721Z 
2019-08-28T18:11:18.3210583Z running 2379 tests
2019-08-28T18:11:31.0389497Z ......iiiii......................................................................................... 100/2379
2019-08-28T18:11:43.7369636Z .........................................................................ii......................... 200/2379
2019-08-28T18:12:13.7044291Z .................................................................................................... 400/2379
2019-08-28T18:12:13.7044291Z .................................................................................................... 400/2379
2019-08-28T18:12:25.2287399Z ..............................i..i.................iiii............................................. 500/2379
2019-08-28T18:12:49.1431990Z .................................................................................................... 700/2379
2019-08-28T18:13:01.4939989Z .................................................................................................... 800/2379
2019-08-28T18:13:13.7779182Z .................................................................................................... 900/2379
2019-08-28T18:13:25.9683306Z .................................................................................................... 1000/2379
---
2019-08-28T18:18:44.5337830Z ............................................... 300/763
2019-08-28T18:18:44.5358387Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-08-28T18:18:44.5788993Z .................................................................................................... 400/763
2019-08-28T18:18:46.6566810Z .................................................................................................... 500/763
2019-08-28T18:18:46.6821867Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-28T18:18:46.6843582Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1084:5
2019-08-28T18:18:46.6848642Z 1084:5.
2019-08-28T18:18:46.6883086Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-28T18:18:46.9154638Z ..........................................thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-28T18:18:46.9182845Z ....thread 'thread '..<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-28T18:18:46.9200152Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-28T18:18:46.9281895Z ............... 600/763
2019-08-28T18:18:48.9767903Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-08-28T18:18:48.9769285Z ......thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
---
2019-08-28T18:18:58.4443747Z 
2019-08-28T18:18:58.4444090Z running 991 tests
2019-08-28T18:19:22.6820216Z i................................................................................................... 100/991
2019-08-28T18:19:36.0230405Z .................................................................................................... 200/991
2019-08-28T18:19:45.0277409Z .................iii......i......i...i......i....................................................... 300/991
2019-08-28T18:19:50.1740976Z .................................................................................................... 400/991
2019-08-28T18:19:58.2859537Z ..................................i..i.................................ii........................... 500/991
2019-08-28T18:20:13.3330403Z .................................................................................................... 700/991
2019-08-28T18:20:13.3330403Z .................................................................................................... 700/991
2019-08-28T18:20:21.8058318Z .................iiii............................................................................... 800/991
2019-08-28T18:20:36.9658401Z .................................................................................................... 900/991
2019-08-28T18:20:44.7739292Z .......................................iiii................................................
2019-08-28T18:20:44.7741557Z 
2019-08-28T18:20:44.7902241Z  finished in 257.272
2019-08-28T18:20:44.7920119Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-28T18:20:44.9904385Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-08-28T18:34:46.3220273Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-08-28T18:35:07.3897006Z     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-08-28T18:35:07.4266183Z     Checking compiler_builtins v0.1.18
2019-08-28T18:35:08.6310845Z  Documenting alloc v0.0.0 (/checkout/src/liballoc)
2019-08-28T18:35:11.0709790Z error: `[into_bytes]` cannot be resolved, ignoring it...
2019-08-28T18:35:11.0710542Z     |
2019-08-28T18:35:11.0710542Z     |
2019-08-28T18:35:11.0710875Z 449 |     /// The inverse of this method is [`into_bytes`].
2019-08-28T18:35:11.0711556Z     |
2019-08-28T18:35:11.0711851Z note: lint level defined here
2019-08-28T18:35:11.0712137Z    --> src/liballoc/lib.rs:70:9
2019-08-28T18:35:11.0712376Z     |
2019-08-28T18:35:11.0712376Z     |
2019-08-28T18:35:11.0713151Z 70  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2019-08-28T18:35:11.0714144Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-08-28T18:35:11.0714221Z 
2019-08-28T18:35:11.1994484Z error: aborting due to previous error
2019-08-28T18:35:11.1994597Z 
2019-08-28T18:35:11.1994597Z 
2019-08-28T18:35:11.2216468Z error: Could not document `alloc`.
2019-08-28T18:35:11.2216566Z 
2019-08-28T18:35:11.2216615Z Caused by:
2019-08-28T18:35:11.2217985Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name alloc src/liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --color always --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.39.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-e105c0b9266fe10f.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-db383e97964fb507.rmeta` (exit code: 1)
2019-08-28T18:35:11.2224984Z 
2019-08-28T18:35:11.2224984Z 
2019-08-28T18:35:11.2226086Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.39.0" "--index-page" "/checkout/src/doc/index.md"
2019-08-28T18:35:11.2227032Z 
2019-08-28T18:35:11.2227271Z 
2019-08-28T18:35:11.2232482Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-28T18:35:11.2233110Z Build completed unsuccessfully in 1:46:58
2019-08-28T18:35:11.2233110Z Build completed unsuccessfully in 1:46:58
2019-08-28T18:35:11.2297907Z == clock drift check ==
2019-08-28T18:35:11.2309965Z   local time: Wed Aug 28 18:35:11 UTC 2019
2019-08-28T18:35:11.3248659Z   network time: Wed, 28 Aug 2019 18:35:11 GMT
2019-08-28T18:35:11.3254332Z == end clock drift check ==
2019-08-28T18:35:13.9520537Z ##[error]Bash exited with code '1'.
2019-08-28T18:35:13.9562287Z ##[section]Starting: Checkout
2019-08-28T18:35:13.9564464Z ==============================================================================
2019-08-28T18:35:13.9564529Z Task         : Get sources
2019-08-28T18:35:13.9564584Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
