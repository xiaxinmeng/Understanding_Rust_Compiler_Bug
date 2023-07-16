plain
2019-11-06T08:00:39.6451398Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-06T08:00:39.6684409Z ##[command]git config gc.auto 0
2019-11-06T08:00:39.6748642Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-06T08:00:39.6798759Z ##[command]git config --get-all http.proxy
2019-11-06T08:00:39.6942132Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66114/merge:refs/remotes/pull/66114/merge
---
2019-11-06T08:57:50.7995963Z .................................................................................................... 1600/9279
2019-11-06T08:57:56.4732323Z .................................................................................................... 1700/9279
2019-11-06T08:58:08.8177909Z ...............................................................i...............i.................... 1800/9279
2019-11-06T08:58:16.1907398Z .................................................................................................... 1900/9279
2019-11-06T08:58:31.2732603Z .....................................................iiiii.......................................... 2000/9279
2019-11-06T08:58:41.7521692Z .................................................................................................... 2200/9279
2019-11-06T08:58:44.2828740Z .................................................................................................... 2300/9279
2019-11-06T08:58:47.5890321Z .................................................................................................... 2400/9279
2019-11-06T08:59:10.0843341Z .................................................................................................... 2500/9279
2019-11-06T08:59:10.0843341Z .................................................................................................... 2500/9279
2019-11-06T08:59:12.8816882Z .................................................................................................... 2600/9279
2019-11-06T08:59:20.5185251Z .................................................................................................... 2700/9279
2019-11-06T08:59:29.3758852Z .....................i.............................................................................. 2800/9279
2019-11-06T08:59:38.0221655Z .................................................................................................... 2900/9279
2019-11-06T08:59:42.3681590Z ....................i............................................................................... 3000/9279
2019-11-06T08:59:50.7301361Z .................................................................................................... 3100/9279
2019-11-06T08:59:56.1601159Z .................................................................................................... 3200/9279
2019-11-06T09:00:04.8477108Z ..ii................................................................................................ 3300/9279
2019-11-06T09:00:21.5940262Z ...............................................................................................i.... 3500/9279
2019-11-06T09:00:28.8812686Z ..........................................i......................................................... 3600/9279
2019-11-06T09:00:35.4607335Z .................................................................................................... 3700/9279
2019-11-06T09:00:41.8184658Z .................................................................................................... 3800/9279
---
2019-11-06T09:01:58.0962608Z .....................................................i...............i.............................. 4800/9279
2019-11-06T09:02:07.0516333Z .................................................................................................... 4900/9279
2019-11-06T09:02:15.3673330Z .................................................................................................... 5000/9279
2019-11-06T09:02:21.4837689Z .................................................................................................... 5100/9279
2019-11-06T09:02:31.6852971Z ......................................................ii.ii...........i............................. 5200/9279
2019-11-06T09:02:41.4604778Z .................................................................................................... 5400/9279
2019-11-06T09:02:51.4347647Z .................................................................................................... 5500/9279
2019-11-06T09:02:58.5635625Z ...........................i........................................................................ 5600/9279
2019-11-06T09:03:04.8803475Z .................................................................................................... 5700/9279
2019-11-06T09:03:04.8803475Z .................................................................................................... 5700/9279
2019-11-06T09:03:16.6156447Z .................................................................................................... 5800/9279
2019-11-06T09:03:28.1308171Z ............ii...i..ii...........i.................................................................. 5900/9279
2019-11-06T09:03:48.6648578Z .................................................................................................... 6100/9279
2019-11-06T09:03:57.1520541Z .................................................................................................... 6200/9279
2019-11-06T09:03:57.1520541Z .................................................................................................... 6200/9279
2019-11-06T09:04:11.9758341Z ...............................i..ii................................................................ 6300/9279
2019-11-06T09:04:32.1841350Z ..................................................................................................i. 6500/9279
2019-11-06T09:04:34.3287929Z .................................................................................................... 6600/9279
2019-11-06T09:04:36.5670376Z .............................................................................i...................... 6700/9279
2019-11-06T09:04:39.1890664Z .................................................................................................... 6800/9279
---
2019-11-06T09:09:50.9776878Z  finished in 5.406
2019-11-06T09:09:50.9962837Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-06T09:09:51.2088621Z 
2019-11-06T09:09:51.2088872Z running 158 tests
2019-11-06T09:09:54.2260258Z iiiiii....iii......iii..iiii...i.............................i..i..................i....i........... 100/158
2019-11-06T09:09:56.1779005Z ii.i.i..iiii..............i.........iii.i.........ii......
2019-11-06T09:09:56.1779749Z 
2019-11-06T09:09:56.1783616Z  finished in 5.182
2019-11-06T09:09:56.1972452Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-06T09:09:56.3689291Z 
---
2019-11-06T09:09:58.3441601Z  finished in 2.146
2019-11-06T09:09:58.3611031Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-06T09:09:58.5122409Z 
2019-11-06T09:09:58.5123254Z running 9 tests
2019-11-06T09:09:58.5124450Z iiiiiiiii
2019-11-06T09:09:58.5125336Z 
2019-11-06T09:09:58.5125386Z  finished in 0.151
2019-11-06T09:09:58.5319145Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-06T09:09:58.7183987Z 
---
2019-11-06T09:10:18.1277454Z  finished in 19.596
2019-11-06T09:10:18.1902000Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-06T09:10:18.4003310Z 
2019-11-06T09:10:18.4004078Z running 123 tests
2019-11-06T09:10:41.8509717Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-06T09:10:46.4359095Z i.i.i......iii.i.....ii
2019-11-06T09:10:46.4359564Z 
2019-11-06T09:10:46.4362105Z  finished in 28.246
2019-11-06T09:10:46.4371587Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-06T09:10:46.4372734Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-06T09:22:41.4583935Z 
2019-11-06T09:22:41.4592071Z    Doc-tests core
2019-11-06T09:22:46.5317568Z 
2019-11-06T09:22:57.5570494Z running 2417 tests
2019-11-06T09:22:57.5570710Z ......iiiii......................................................................................... 100/2417
2019-11-06T09:23:08.2078099Z ................................................................................ii.................. 200/2417
2019-11-06T09:23:32.8356953Z ..i................................................................................................. 400/2417
2019-11-06T09:23:32.8356953Z ..i................................................................................................. 400/2417
2019-11-06T09:23:43.4775708Z ..................................................i..i.................iiii......................... 500/2417
2019-11-06T09:24:03.0198833Z .................................................................................................... 700/2417
2019-11-06T09:24:13.2011964Z .................................................................................................... 800/2417
2019-11-06T09:24:23.3232134Z .................................................................................................... 900/2417
2019-11-06T09:24:33.5565905Z .................................................................................................... 1000/2417
---
2019-11-06T09:28:26.3422156Z ..............................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-11-06T09:28:26.3423405Z . 300/763
2019-11-06T09:28:26.3825717Z .................................................................................................... 400/763
2019-11-06T09:28:28.4661958Z .................................................................................................... 500/763
2019-11-06T09:28:28.4890056Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-06T09:28:28.4911792Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-11-06T09:28:28.4933171Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:.1165:5
2019-11-06T09:28:28.4964647Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-06T09:28:28.7438304Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-06T09:28:28.7486229Z ...........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-06T09:28:28.7499613Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-06T09:28:30.7959806Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-11-06T09:28:30.7966732Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-11-06T09:28:30.7978429Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-11-06T09:28:30.7996224Z ........thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
2019-11-06T09:28:30.7996224Z ........thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
2019-11-06T09:28:30.7996835Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2019-11-06T09:28:30.7996890Z   left: `1`,
2019-11-06T09:28:30.7997267Z  right: `2`', src/libstd/sync/mutex.rs:653:13
2019-11-06T09:28:30.8040748Z ......thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:791:13
2019-11-06T09:28:30.8052032Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison RwLock', src/libstd/sync/rwlock.rs:768:13
2019-11-06T09:28:30.8061590Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:705:13
2019-11-06T09:28:30.8069406Z thread '.<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:635:13
2019-11-06T09:28:30.8076059Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:646:13
2019-11-06T09:28:30.8084827Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:611:13
2019-11-06T09:28:30.8087465Z ...thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/rwlock.rs:623:13
2019-11-06T09:28:32.8641807Z .......................thread '<unnamed>' panicked at 'What the answer to my lifetimes dilemma is?', src/libstd/sys_common/remutex.rs:233:13
2019-11-06T09:28:32.8767068Z ...................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/thread/mod.rs:1544:13
2019-11-06T09:28:33.4842100Z ............thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1676:13
2019-11-06T09:28:33.4853549Z ....thread '<unnamed>' panicked at 'owned string', src/libstd/thread/mod.rs:1662:13
2019-11-06T09:28:33.4854028Z thread '<unnamed>' panicked at 'static string', src/libstd/thread/mod.rs:1648:13
---
2019-11-06T09:28:40.1271582Z 
2019-11-06T09:28:40.1277616Z running 1000 tests
2019-11-06T09:29:00.9236111Z i................................................................................................... 100/1000
2019-11-06T09:29:12.3363552Z .................................................................................................... 200/1000
2019-11-06T09:29:20.5590809Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-06T09:29:26.0512335Z .................................................................................................... 400/1000
2019-11-06T09:29:33.6081348Z ...........................................i..i.................................ii.................. 500/1000
2019-11-06T09:29:47.9307360Z .................................................................................................... 700/1000
2019-11-06T09:29:47.9307360Z .................................................................................................... 700/1000
2019-11-06T09:29:55.5812698Z ..........................iiii...................................................................... 800/1000
2019-11-06T09:30:11.4063274Z .................................................................................................... 900/1000
2019-11-06T09:30:19.1867535Z ................................................iiii................................................ 1000/1000
2019-11-06T09:30:19.1869798Z test result: ok. 980 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-06T09:30:19.1869941Z 
2019-11-06T09:30:19.1926739Z  finished in 189.251
2019-11-06T09:30:19.1941952Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-06T09:46:20.6265169Z  Documenting std v0.0.0 (/checkout/src/libstd)
2019-11-06T09:46:25.5920389Z error: `[std::error::Error]` cannot be resolved, ignoring it...
2019-11-06T09:46:25.5921741Z     --> src/libstd/thread/mod.rs:1278:19
2019-11-06T09:46:25.5922334Z      |
2019-11-06T09:46:25.5923355Z 1278 | /// the [`Error`](std::error::Error) trait.
2019-11-06T09:46:25.5924574Z      |
2019-11-06T09:46:25.5925438Z note: lint level defined here
2019-11-06T09:46:25.6005053Z     --> src/libstd/lib.rs:211:9
2019-11-06T09:46:25.6005268Z      |
2019-11-06T09:46:25.6005268Z      |
2019-11-06T09:46:25.6005519Z 211  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2019-11-06T09:46:25.6006477Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-11-06T09:46:25.6006511Z 
2019-11-06T09:46:25.9325320Z error: aborting due to previous error
2019-11-06T09:46:25.9325416Z 
2019-11-06T09:46:25.9325416Z 
2019-11-06T09:46:25.9742919Z error: Could not document `std`.
2019-11-06T09:46:25.9743086Z 
2019-11-06T09:46:25.9759160Z Caused by:
2019-11-06T09:46:25.9763661Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --color always --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.40.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-09eef9fc61c13fca.rmeta --extern backtrace_rs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-c9a54a91349fb46e.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-1d1c39dae3b9223d.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-8671f0ef9fb03c42.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-67faf12ddd846db2.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-654baceaa903a5f0.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-741b6f81304e4ea4.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-82147b962bb66078.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-dcef5f520f62138e.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-dd41ee797bc534ab.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-7aa22717cf97149e.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-92b604e6cc07ff11.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-e183b0e9e5a05a21.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-c92973a0ec57d75e.rmeta` (exit code: 1)
2019-11-06T09:46:25.9769401Z 
2019-11-06T09:46:25.9769401Z 
2019-11-06T09:46:25.9770564Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.40.0" "--index-page" "/checkout/src/doc/index.md"
2019-11-06T09:46:25.9770949Z 
2019-11-06T09:46:25.9770987Z 
2019-11-06T09:46:25.9784606Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-06T09:46:25.9784777Z Build completed unsuccessfully in 1:39:20
2019-11-06T09:46:25.9784777Z Build completed unsuccessfully in 1:39:20
2019-11-06T09:46:25.9845835Z == clock drift check ==
2019-11-06T09:46:25.9862635Z   local time: Wed Nov  6 09:46:25 UTC 2019
2019-11-06T09:46:26.2653636Z   network time: Wed, 06 Nov 2019 09:46:26 GMT
2019-11-06T09:46:26.2656564Z == end clock drift check ==
2019-11-06T09:46:31.8054284Z 
2019-11-06T09:46:31.8180327Z ##[error]Bash exited with code '1'.
2019-11-06T09:46:31.8220874Z ##[section]Starting: Checkout
2019-11-06T09:46:31.8222639Z ==============================================================================
2019-11-06T09:46:31.8222722Z Task         : Get sources
2019-11-06T09:46:31.8222761Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
