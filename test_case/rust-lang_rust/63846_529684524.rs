plain
2019-09-09T20:11:55.8582486Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T20:11:55.8596016Z ##[command]git config gc.auto 0
2019-09-09T20:11:55.8602493Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T20:11:55.8608961Z ##[command]git config --get-all http.proxy
2019-09-09T20:11:55.8612549Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63846/merge:refs/remotes/pull/63846/merge
---
2019-09-09T21:14:05.7269618Z .................................................................................................... 1500/9003
2019-09-09T21:14:11.4239841Z .................................................................................................... 1600/9003
2019-09-09T21:14:23.9632213Z .....................................................i...............i.............................. 1700/9003
2019-09-09T21:14:31.7793049Z .................................................................................................... 1800/9003
2019-09-09T21:14:46.3006074Z ............................................iiiii................................................... 1900/9003
2019-09-09T21:14:56.8500486Z .................................................................................................... 2100/9003
2019-09-09T21:14:59.2676114Z .................................................................................................... 2200/9003
2019-09-09T21:15:03.0553961Z .................................................................................................... 2300/9003
2019-09-09T21:15:10.3273964Z .................................................................................................... 2400/9003
---
2019-09-09T21:18:03.9018733Z ...............................i...............i.................................................... 4700/9003
2019-09-09T21:18:15.5131802Z .................................................................................................... 4800/9003
2019-09-09T21:18:21.6141973Z .................................................................................................... 4900/9003
2019-09-09T21:18:32.1289492Z .................................................................................................... 5000/9003
2019-09-09T21:18:37.8813974Z .............ii.ii.................................................................................. 5100/9003
2019-09-09T21:18:48.4919208Z .................................................................................................... 5300/9003
2019-09-09T21:18:58.3383539Z ............................................................................i....................... 5400/9003
2019-09-09T21:19:05.8195706Z .................................................................................................... 5500/9003
2019-09-09T21:19:11.9747791Z .................................................................................................... 5600/9003
2019-09-09T21:19:11.9747791Z .................................................................................................... 5600/9003
2019-09-09T21:19:22.4301395Z .......................................................................ii..i..ii...........i........ 5700/9003
2019-09-09T21:19:46.6896048Z .................................................................................................... 5900/9003
2019-09-09T21:19:55.7870462Z .................................................................................................... 6000/9003
2019-09-09T21:19:55.7870462Z .................................................................................................... 6000/9003
2019-09-09T21:20:01.1394336Z ........................................................................i..ii....................... 6100/9003
2019-09-09T21:20:30.3001086Z .................................................................................................... 6300/9003
2019-09-09T21:20:32.2993937Z ...............................i.................................................................... 6400/9003
2019-09-09T21:20:34.3117946Z .................................................................................................... 6500/9003
2019-09-09T21:20:36.7648721Z ...i................................................................................................ 6600/9003
---
2019-09-09T21:25:09.8271685Z  finished in 19.002
2019-09-09T21:25:09.8446327Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:25:09.9985110Z 
2019-09-09T21:25:09.9985355Z running 150 tests
2019-09-09T21:25:13.0710434Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-09T21:25:14.9629842Z ..iiii..............i.........iii.i.......ii......
2019-09-09T21:25:14.9632686Z 
2019-09-09T21:25:14.9633410Z  finished in 5.118
2019-09-09T21:25:14.9820533Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:25:16.0398426Z 
---
2019-09-09T21:25:17.1443999Z  finished in 2.162
2019-09-09T21:25:17.1641282Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:25:17.3353614Z 
2019-09-09T21:25:17.3354486Z running 9 tests
2019-09-09T21:25:17.3355449Z iiiiiiiii
2019-09-09T21:25:17.3356624Z 
2019-09-09T21:25:17.3356760Z  finished in 0.171
2019-09-09T21:25:17.3546174Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:25:17.5221094Z 
---
2019-09-09T21:25:35.0106674Z  finished in 17.655
2019-09-09T21:25:35.0276517Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:25:35.1834502Z 
2019-09-09T21:25:35.1834636Z running 123 tests
2019-09-09T21:25:57.8837568Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T21:26:02.2398795Z i.i.i......iii.i.....ii
2019-09-09T21:26:02.2401927Z 
2019-09-09T21:26:02.2405534Z  finished in 27.212
2019-09-09T21:26:02.2410661Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:26:02.2412268Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T21:39:16.1658124Z .................................................................................................... 100/977
2019-09-09T21:39:16.1828101Z .................................................................................................... 200/977
2019-09-09T21:39:16.1948049Z .................................................................................................... 300/977
2019-09-09T21:39:16.2150627Z .................................................................................................... 400/977
2019-09-09T21:39:16.3847840Z ..ii................................................................................................ 500/977
2019-09-09T21:39:16.4111824Z .................................................................................................... 700/977
2019-09-09T21:39:16.4298147Z .................................................................................................... 800/977
2019-09-09T21:39:16.9293963Z .................................................................................................... 900/977
2019-09-09T21:39:18.0522732Z .............................................................................
2019-09-09T21:39:18.0522732Z .............................................................................
2019-09-09T21:39:18.0522864Z test result: ok. 975 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2019-09-09T21:39:18.0522935Z 
2019-09-09T21:39:18.0523660Z    Doc-tests core
2019-09-09T21:39:22.8669037Z 
2019-09-09T21:39:22.8670016Z running 2400 tests
2019-09-09T21:39:33.1915441Z ......iiiii......................................................................................... 100/2400
2019-09-09T21:39:43.2173375Z ...........................................................................ii....................... 200/2400
2019-09-09T21:39:54.8742487Z .................................................................................................i.. 300/2400
2019-09-09T21:40:07.2171763Z .................................................................................................... 400/2400
2019-09-09T21:40:16.7914314Z ............................................i..i.................iiii............................... 500/2400
2019-09-09T21:40:35.6124096Z .................................................................................................... 700/2400
2019-09-09T21:40:45.2731714Z .................................................................................................... 800/2400
2019-09-09T21:40:54.9977390Z .................................................................................................... 900/2400
2019-09-09T21:41:04.7554581Z .................................................................................................... 1000/2400
---
2019-09-09T21:45:49.2403817Z 
2019-09-09T21:45:49.2404472Z running 991 tests
2019-09-09T21:46:09.0235766Z i................................................................................................... 100/991
2019-09-09T21:46:19.7789400Z .................................................................................................... 200/991
2019-09-09T21:46:27.8976697Z .................iii......i......i...i......i....................................................... 300/991
2019-09-09T21:46:32.8033042Z .................................................................................................... 400/991
2019-09-09T21:46:40.1272315Z ..................................i..i.................................ii........................... 500/991
2019-09-09T21:46:53.8607198Z .................................................................................................... 700/991
2019-09-09T21:46:53.8607198Z .................................................................................................... 700/991
2019-09-09T21:47:01.4592083Z .................iiii............................................................................... 800/991
2019-09-09T21:47:15.5424978Z .................................................................................................... 900/991
2019-09-09T21:47:22.5977234Z .......................................iiii................................................
2019-09-09T21:47:22.5978145Z 
2019-09-09T21:47:22.6070432Z  finished in 231.450
2019-09-09T21:47:22.6089205Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T21:47:22.8854766Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-09T22:01:05.6122089Z     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-09-09T22:01:05.6173638Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-09T22:01:05.6482082Z     Checking hashbrown v0.5.0
2019-09-09T22:01:06.4753962Z  Documenting std v0.0.0 (/checkout/src/libstd)
2019-09-09T22:01:11.6591003Z error: `[mach_absolute_time]` cannot be resolved, ignoring it...
2019-09-09T22:01:11.6592013Z   --> src/libstd/time.rs:71:20
2019-09-09T22:01:11.6592869Z    |
2019-09-09T22:01:11.6593800Z 71 | /// | Darwin    | [mach_absolute_time]                                                 |
2019-09-09T22:01:11.6595015Z    |
2019-09-09T22:01:11.6595568Z note: lint level defined here
2019-09-09T22:01:11.6596119Z   --> src/libstd/lib.rs:211:9
2019-09-09T22:01:11.6596761Z    |
2019-09-09T22:01:11.6596761Z    |
2019-09-09T22:01:11.6597261Z 211| #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
2019-09-09T22:01:11.6598308Z    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-09-09T22:01:11.6598536Z 
2019-09-09T22:01:11.6598536Z 
2019-09-09T22:01:11.6598963Z error: `[QueryPerformanceCounter]` cannot be resolved, ignoring it...
2019-09-09T22:01:11.6599437Z   --> src/libstd/time.rs:74:20
2019-09-09T22:01:11.6599868Z    |
2019-09-09T22:01:11.6600401Z 74 | /// | Windows   | [QueryPerformanceCounter]                                            |
2019-09-09T22:01:11.6603157Z    |
2019-09-09T22:01:11.6604039Z    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-09-09T22:01:11.6604324Z 
2019-09-09T22:01:11.6604324Z 
2019-09-09T22:01:11.6604865Z error: `[QueryPerformanceCounter]` cannot be resolved, ignoring it...
2019-09-09T22:01:11.6605441Z   --> src/libstd/time.rs:76:6
2019-09-09T22:01:11.6605941Z    |
2019-09-09T22:01:11.6607212Z 76 | /// [QueryPerformanceCounter]: https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter
2019-09-09T22:01:11.6608463Z    |
2019-09-09T22:01:11.6608987Z    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-09-09T22:01:11.6609479Z 
2019-09-09T22:01:11.6609479Z 
2019-09-09T22:01:11.6609961Z error: `[mach_absolute_time]` cannot be resolved, ignoring it...
2019-09-09T22:01:11.6610424Z   --> src/libstd/time.rs:81:6
2019-09-09T22:01:11.6610843Z    |
2019-09-09T22:01:11.6611424Z 81 | /// [mach_absolute_time]: https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/KernelProgramming/services/services.html
2019-09-09T22:01:11.6615179Z    |
2019-09-09T22:01:11.6616721Z    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2019-09-09T22:01:11.6617721Z 
2019-09-09T22:01:11.9880887Z error: aborting due to 4 previous errors
2019-09-09T22:01:11.9880887Z error: aborting due to 4 previous errors
2019-09-09T22:01:11.9882223Z 
2019-09-09T22:01:12.0293661Z error: Could not document `std`.
2019-09-09T22:01:12.0308372Z 
2019-09-09T22:01:12.0308725Z Caused by:
2019-09-09T22:01:12.0314910Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name std src/libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --color always --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.39.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-c9af94aec33c482c.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-c3e713a9dcf4c5a7.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-fc053e0be91a93ad.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-e105c0b9266fe10f.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-db383e97964fb507.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-b7c68b781374d024.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-ac0ff1b98d1f840a.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-88255ed6a45420e4.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-5b224b27d04f22d6.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-5eed2f40fc7685e2.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-614393d2bb72c85a.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-b1a836ada7f22c5f.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-e09a8fac343911cf.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-8234d9206d0add37.rmeta` (exit code: 1)
2019-09-09T22:01:12.0319625Z 
2019-09-09T22:01:12.0319625Z 
2019-09-09T22:01:12.0320655Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.39.0" "--index-page" "/checkout/src/doc/index.md"
2019-09-09T22:01:12.0321013Z 
2019-09-09T22:01:12.0321123Z 
2019-09-09T22:01:12.0327316Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T22:01:12.0327557Z Build completed unsuccessfully in 1:41:40
2019-09-09T22:01:12.0327557Z Build completed unsuccessfully in 1:41:40
2019-09-09T22:01:12.0386211Z == clock drift check ==
2019-09-09T22:01:12.0400724Z   local time: Mon Sep  9 22:01:12 UTC 2019
2019-09-09T22:01:12.3237356Z   network time: Mon, 09 Sep 2019 22:01:12 GMT
2019-09-09T22:01:12.3243617Z == end clock drift check ==
2019-09-09T22:01:17.0715724Z ##[error]Bash exited with code '1'.
2019-09-09T22:01:17.0757262Z ##[section]Starting: Checkout
2019-09-09T22:01:17.0761224Z ==============================================================================
2019-09-09T22:01:17.0761282Z Task         : Get sources
2019-09-09T22:01:17.0761326Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
