plain
2019-11-24T18:19:17.3022155Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T18:19:17.9264475Z ##[command]git config gc.auto 0
2019-11-24T18:19:17.9268041Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T18:19:17.9289068Z ##[command]git config --get-all http.proxy
2019-11-24T18:19:17.9297119Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66710/merge:refs/remotes/pull/66710/merge
---
2019-11-24T19:18:24.3036360Z .................................................................................................... 1600/9283
2019-11-24T19:18:28.9853351Z .................................................................................................... 1700/9283
2019-11-24T19:18:41.9432838Z .............................i...................................................................... 1800/9283
2019-11-24T19:18:48.6740212Z .................................................................................................... 1900/9283
2019-11-24T19:19:02.4797170Z ..............iiiii................................................................................. 2000/9283
2019-11-24T19:19:12.0759082Z .................................................................................................... 2200/9283
2019-11-24T19:19:14.5132691Z .................................................................................................... 2300/9283
2019-11-24T19:19:19.5806041Z .................................................................................................... 2400/9283
2019-11-24T19:19:40.4875444Z .................................................................................................... 2500/9283
---
2019-11-24T19:22:17.9613817Z ..............i...............i..................................................................... 4800/9283
2019-11-24T19:22:28.0799556Z .................................................................................................... 4900/9283
2019-11-24T19:22:33.5992879Z .................................................................................................... 5000/9283
2019-11-24T19:22:43.0067459Z .................................................................................................... 5100/9283
2019-11-24T19:22:49.0362380Z ...................ii.ii...........i................................................................ 5200/9283
2019-11-24T19:22:58.0192648Z .................................................................................................... 5400/9283
2019-11-24T19:23:08.9657225Z .................................................................................................... 5500/9283
2019-11-24T19:23:16.7424604Z .i.................................................................................................. 5600/9283
2019-11-24T19:23:22.0910548Z .................................................................................................... 5700/9283
2019-11-24T19:23:22.0910548Z .................................................................................................... 5700/9283
2019-11-24T19:23:32.3280934Z .......................................................................................ii...i..ii... 5800/9283
2019-11-24T19:23:54.8921057Z .................................................................................................... 6000/9283
2019-11-24T19:24:02.7070735Z .................................................................................................... 6100/9283
2019-11-24T19:24:06.7174243Z .................................................................................................... 6200/9283
2019-11-24T19:24:06.7174243Z .................................................................................................... 6200/9283
2019-11-24T19:24:20.2205429Z ..........i..ii..................................................................................... 6300/9283
2019-11-24T19:24:38.9436412Z ..............................................................................i..................... 6500/9283
2019-11-24T19:24:41.2300447Z .................................................................................................... 6600/9283
2019-11-24T19:24:43.4761479Z .....................................................................i.............................. 6700/9283
2019-11-24T19:24:46.4402149Z .................................................................................................... 6800/9283
---
2019-11-24T19:29:52.4953157Z  finished in 5.864
2019-11-24T19:29:52.5153086Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:29:52.7125160Z 
2019-11-24T19:29:52.7125731Z running 157 tests
2019-11-24T19:29:55.5045285Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/157
2019-11-24T19:29:57.4866390Z .i.i..iiii..............i.........iii.i..........ii......
2019-11-24T19:29:57.4868232Z 
2019-11-24T19:29:57.4872267Z  finished in 4.972
2019-11-24T19:29:57.5059957Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:29:57.6760689Z 
---
2019-11-24T19:29:59.6491716Z  finished in 2.143
2019-11-24T19:29:59.6673053Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:29:59.8200237Z 
2019-11-24T19:29:59.8200403Z running 9 tests
2019-11-24T19:29:59.8205126Z iiiiiiiii
2019-11-24T19:29:59.8205525Z 
2019-11-24T19:29:59.8205565Z  finished in 0.153
2019-11-24T19:29:59.8374078Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:30:00.0184575Z 
---
2019-11-24T19:30:19.1587450Z  finished in 19.321
2019-11-24T19:30:19.2043167Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:30:19.4099998Z 
2019-11-24T19:30:19.4100171Z running 124 tests
2019-11-24T19:30:43.3823670Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i....ii...i.......ii 100/124
2019-11-24T19:30:48.5609215Z .i.i.i......iii.i.....ii
2019-11-24T19:30:48.5610841Z 
2019-11-24T19:30:48.5611162Z  finished in 29.356
2019-11-24T19:30:48.5619366Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:30:48.5620560Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-24T19:42:48.8489726Z test result: ok. 991 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2019-11-24T19:42:48.8489846Z 
2019-11-24T19:42:53.8908583Z 
2019-11-24T19:42:53.8909573Z running 2421 tests
2019-11-24T19:43:04.2320889Z ......iiiii......................................................................................... 100/2421
2019-11-24T19:43:26.4319474Z .................................................................................................... 300/2421
2019-11-24T19:43:38.6256679Z ..i................................................................................................. 400/2421
2019-11-24T19:43:38.6256679Z ..i................................................................................................. 400/2421
2019-11-24T19:43:48.4981528Z ..................................................i..i..................iiii........................ 500/2421
2019-11-24T19:44:07.1763681Z .................................................................................................... 700/2421
2019-11-24T19:44:16.9856619Z .................................................................................................... 800/2421
2019-11-24T19:44:26.9468712Z .................................................................................................... 900/2421
2019-11-24T19:44:36.8433986Z .................................................................................................... 1000/2421
---
2019-11-24T19:48:24.8639396Z 
2019-11-24T19:48:24.8640006Z running 756 tests
2019-11-24T19:48:24.8887377Z .................................................................................................... 100/756
2019-11-24T19:48:25.2706010Z .................................................................................................... 200/756
2019-11-24T19:48:25.5305278Z ..............................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1406:17
2019-11-24T19:48:25.5424617Z ..................................................thread '<unnamed>' panicked at 'explicit panic.', src/libstd/io/stdio.rs.:854:13
2019-11-24T19:48:25.5431032Z .. 300/756
2019-11-24T19:48:27.6479562Z .................................................................................................... 500/756
2019-11-24T19:48:27.6791303Z .............thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-11-24T19:48:27.6820637Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-11-24T19:48:27.6835851Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1189:5
---
2019-11-24T19:48:39.3290861Z 
2019-11-24T19:48:39.3301151Z running 999 tests
2019-11-24T19:48:58.5592224Z i................................................................................................... 100/999
2019-11-24T19:49:09.4087232Z .................................................................................................... 200/999
2019-11-24T19:49:17.2354258Z ..................iii......i......i...i......i...................................................... 300/999
2019-11-24T19:49:22.4775641Z .................................................................................................... 400/999
2019-11-24T19:49:29.7859508Z ..........................................i..i.................................ii................... 500/999
2019-11-24T19:49:43.6730753Z .................................................................................................... 700/999
2019-11-24T19:49:43.6730753Z .................................................................................................... 700/999
2019-11-24T19:49:50.7871376Z .........................iiii....................................................................... 800/999
2019-11-24T19:50:05.3420964Z .................................................................................................... 900/999
2019-11-24T19:50:12.2980461Z ...............................................iiii................................................
2019-11-24T19:50:12.2983502Z 
2019-11-24T19:50:12.3086887Z  finished in 185.943
2019-11-24T19:50:12.3104103Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T19:50:12.5367231Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-11-24T20:05:47.0916191Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-11-24T20:06:06.0588671Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-11-24T20:06:06.0944287Z     Checking compiler_builtins v0.1.18
2019-11-24T20:06:07.1693668Z  Documenting alloc v0.0.0 (/checkout/src/liballoc)
2019-11-24T20:06:09.2296379Z error: `[from_raw]` cannot be resolved, ignoring it...
2019-11-24T20:06:09.2298131Z     --> src/liballoc/rc.rs:1708:52
2019-11-24T20:06:09.2298867Z      |
2019-11-24T20:06:09.2299654Z 1708 |     /// The pointer must have originated from the [`from_raw`] (or [`as_raw`], provided there was
2019-11-24T20:06:09.2301178Z      |
2019-11-24T20:06:09.2301791Z note: lint level defined here
2019-11-24T20:06:09.2302501Z     --> src/liballoc/lib.rs:70:9
2019-11-24T20:06:09.2303106Z      |
2019-11-24T20:06:09.2303106Z      |
2019-11-24T20:06:09.2303810Z 70   | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2019-11-24T20:06:09.2305204Z      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-11-24T20:06:09.2305533Z 
2019-11-24T20:06:09.3488816Z error: aborting due to previous error
2019-11-24T20:06:09.3489246Z 
2019-11-24T20:06:09.3489246Z 
2019-11-24T20:06:09.3741839Z error: Could not document `alloc`.
2019-11-24T20:06:09.3742514Z 
2019-11-24T20:06:09.3742761Z Caused by:
2019-11-24T20:06:09.3744944Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name alloc src/liballoc/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="compiler-builtins-c"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.41.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-470bf7eb02796b3e.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-bbe72a0febce0919.rmeta` (exit code: 1)
2019-11-24T20:06:09.3764552Z 
2019-11-24T20:06:09.3764552Z 
2019-11-24T20:06:09.3765810Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.41.0" "--index-page" "/checkout/src/doc/index.md"
2019-11-24T20:06:09.3766018Z 
2019-11-24T20:06:09.3766056Z 
2019-11-24T20:06:09.3777491Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-24T20:06:09.3778666Z Build completed unsuccessfully in 1:40:52
2019-11-24T20:06:09.3778666Z Build completed unsuccessfully in 1:40:52
2019-11-24T20:06:09.3839016Z == clock drift check ==
2019-11-24T20:06:09.3854074Z   local time: Sun Nov 24 20:06:09 UTC 2019
2019-11-24T20:06:09.5368504Z   network time: Sun, 24 Nov 2019 20:06:09 GMT
2019-11-24T20:06:09.5371571Z == end clock drift check ==
2019-11-24T20:06:12.3635734Z 
2019-11-24T20:06:12.3750594Z ##[error]Bash exited with code '1'.
2019-11-24T20:06:12.3783449Z ##[section]Starting: Checkout
2019-11-24T20:06:12.3785406Z ==============================================================================
2019-11-24T20:06:12.3785460Z Task         : Get sources
2019-11-24T20:06:12.3785523Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
