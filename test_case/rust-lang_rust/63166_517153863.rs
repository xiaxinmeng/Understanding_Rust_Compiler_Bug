plain
2019-08-01T05:41:01.5805058Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T05:41:01.5991533Z ##[command]git config gc.auto 0
2019-08-01T05:41:01.6063654Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T05:41:01.6118067Z ##[command]git config --get-all http.proxy
2019-08-01T05:41:01.6260509Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63166/merge:refs/remotes/pull/63166/merge
---
2019-08-01T05:41:38.5541369Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T05:41:38.5541401Z 
2019-08-01T05:41:38.5541616Z   git checkout -b <new-branch-name>
2019-08-01T05:41:38.5541646Z 
2019-08-01T05:41:38.5541716Z HEAD is now at 6f5885edc Merge 4b2f598986e135f0eb070a9be87a5a963e8ff3d4 into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-08-01T05:41:38.5739893Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T05:41:38.5743155Z ==============================================================================
2019-08-01T05:41:38.5743215Z Task         : Bash
2019-08-01T05:41:38.5743281Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T06:43:31.8407158Z .................................................................................................... 1400/8819
2019-08-01T06:43:38.0387496Z .................................................................................................... 1500/8819
2019-08-01T06:43:50.9728919Z .................................................................i...............i.................. 1600/8819
2019-08-01T06:43:58.7162908Z .................................................................................................... 1700/8819
2019-08-01T06:44:13.8229622Z ...................................................iiiii............................................ 1800/8819
2019-08-01T06:44:25.3439540Z .................................................................................................... 2000/8819
2019-08-01T06:44:27.9656878Z .................................................................................................... 2100/8819
2019-08-01T06:44:31.6981212Z .................................................................................................... 2200/8819
2019-08-01T06:44:38.4184096Z .................................................................................................... 2300/8819
---
2019-08-01T06:48:40.8539888Z .................................................................................................... 5300/8819
2019-08-01T06:48:48.5470504Z ..............i..................................................................................... 5400/8819
2019-08-01T06:48:54.4029788Z .................................................................................................... 5500/8819
2019-08-01T06:49:07.1684662Z .................................................................................................... 5600/8819
2019-08-01T06:49:21.1759546Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-08-01T06:49:38.3394331Z .................................................................................................... 5900/8819
2019-08-01T06:49:43.3073508Z .................................................................................................... 6000/8819
2019-08-01T06:49:43.3073508Z .................................................................................................... 6000/8819
2019-08-01T06:49:57.7674159Z ........i..ii....................................................................................... 6100/8819
2019-08-01T06:50:17.5063435Z ...................................................i................................................ 6300/8819
2019-08-01T06:50:19.7148232Z .................................................................................................... 6400/8819
2019-08-01T06:50:22.3643752Z .....................i.............................................................................. 6500/8819
2019-08-01T06:50:27.0864805Z .................................................................................................... 6600/8819
---
2019-08-01T06:55:13.5445957Z  finished in 23.422
2019-08-01T06:55:13.5629289Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T06:55:13.7238027Z 
2019-08-01T06:55:13.7238272Z running 146 tests
2019-08-01T06:55:16.9279404Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-01T06:55:18.7819349Z iii..............i.........iii.i......ii......
2019-08-01T06:55:18.7821156Z 
2019-08-01T06:55:18.7822670Z  finished in 5.219
2019-08-01T06:55:18.8002588Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T06:55:18.9558930Z 
---
2019-08-01T06:55:21.0984660Z  finished in 2.297
2019-08-01T06:55:21.1175128Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T06:55:21.2760161Z 
2019-08-01T06:55:21.2761262Z running 9 tests
2019-08-01T06:55:21.2762279Z iiiiiiiii
2019-08-01T06:55:21.2764027Z 
2019-08-01T06:55:21.2768878Z  finished in 0.159
2019-08-01T06:55:21.2950759Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T06:55:21.4587076Z 
---
2019-08-01T06:55:40.1263127Z  finished in 18.830
2019-08-01T06:55:40.1467217Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T06:55:40.3151300Z 
2019-08-01T06:55:40.3152015Z running 122 tests
2019-08-01T06:56:05.0218135Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-01T06:56:09.7801679Z .i.i......iii.i.....ii
2019-08-01T06:56:09.7803095Z 
2019-08-01T06:56:09.7803332Z  finished in 29.633
2019-08-01T06:56:09.7811574Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T06:56:09.7812134Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-01T07:10:27.5837249Z 
2019-08-01T07:10:27.5844393Z    Doc-tests core
2019-08-01T07:10:31.8517655Z 
2019-08-01T07:10:31.8518503Z running 2387 tests
2019-08-01T07:10:45.2888313Z ......iiiii......................................................................................... 100/2387
2019-08-01T07:10:58.2461277Z .........................................................................ii......................... 200/2387
2019-08-01T07:11:29.2815517Z .................................................................................................... 400/2387
2019-08-01T07:11:29.2815517Z .................................................................................................... 400/2387
2019-08-01T07:11:40.9088367Z ..............................i..i.................iiii............................................. 500/2387
2019-08-01T07:12:05.8520097Z .................................................................................................... 700/2387
2019-08-01T07:12:18.2845365Z .................................................................................................... 800/2387
2019-08-01T07:12:31.0374760Z .................................................................................................... 900/2387
2019-08-01T07:12:43.4311586Z .................................................................................................... 1000/2387
---
2019-08-01T07:14:11.1031221Z .................................................................................................... 1700/2387
2019-08-01T07:14:23.9107069Z .................................................................................................... 1800/2387
2019-08-01T07:14:36.6278422Z .................................................................................................... 1900/2387
2019-08-01T07:14:50.9214566Z .................................................................................................... 2000/2387
2019-08-01T07:15:03.9389969Z ...............................................................................................FFFFF 2100/2387
2019-08-01T07:15:19.3829753Z F.FF................................................................................................ 2200/2387
2019-08-01T07:15:36.5108001Z ................................................................................................i... 2300/2387
2019-08-01T07:15:49.1764438Z failures:
2019-08-01T07:15:49.1764505Z 
2019-08-01T07:15:49.1764505Z 
2019-08-01T07:15:49.1765620Z ---- result.rs - result::Result<&'_ T, E>::cloned (line 909) stdout ----
2019-08-01T07:15:49.1765686Z error[E0282]: type annotations needed for `std::result::Result<&i32, E>`
2019-08-01T07:15:49.1765975Z   |
2019-08-01T07:15:49.1765975Z   |
2019-08-01T07:15:49.1766018Z 6 | let x = Ok(&val);
2019-08-01T07:15:49.1766240Z   |     -   ^^ cannot infer type for `E`
2019-08-01T07:15:49.1766286Z   |     |
2019-08-01T07:15:49.1766357Z   |     consider giving `x` the explicit type `std::result::Result<&i32, E>`, where the type parameter `E` is specified
2019-08-01T07:15:49.1766485Z error: aborting due to previous error
2019-08-01T07:15:49.1766535Z 
2019-08-01T07:15:49.1766785Z For more information about this error, try `rustc --explain E0282`.
2019-08-01T07:15:49.1766987Z Couldn't compile the test.
2019-08-01T07:15:49.1766987Z Couldn't compile the test.
2019-08-01T07:15:49.1767243Z ---- result.rs - result::Result<&'_ T, E>::copied (line 829) stdout ----
2019-08-01T07:15:49.1767297Z error[E0282]: type annotations needed for `std::result::Result<&i32, E>`
2019-08-01T07:15:49.1767536Z   |
2019-08-01T07:15:49.1767536Z   |
2019-08-01T07:15:49.1767594Z 6 | let x = Ok(&val);
2019-08-01T07:15:49.1767807Z   |     -   ^^ cannot infer type for `E`
2019-08-01T07:15:49.1767852Z   |     |
2019-08-01T07:15:49.1767922Z   |     consider giving `x` the explicit type `std::result::Result<&i32, E>`, where the type parameter `E` is specified
2019-08-01T07:15:49.1767997Z error: aborting due to previous error
2019-08-01T07:15:49.1768025Z 
2019-08-01T07:15:49.1768289Z For more information about this error, try `rustc --explain E0282`.
2019-08-01T07:15:49.1768741Z Couldn't compile the test.
2019-08-01T07:15:49.1768741Z Couldn't compile the test.
2019-08-01T07:15:49.1768991Z ---- result.rs - result::Result<&'_ mut T, E>::cloned (line 929) stdout ----
2019-08-01T07:15:49.1769067Z error[E0282]: type annotations needed for `std::result::Result<&mut i32, E>`
2019-08-01T07:15:49.1769760Z   |
2019-08-01T07:15:49.1769760Z   |
2019-08-01T07:15:49.1769822Z 6 | let x = Ok(&mut val);
2019-08-01T07:15:49.1770044Z   |     -   ^^ cannot infer type for `E`
2019-08-01T07:15:49.1770089Z   |     |
2019-08-01T07:15:49.1770145Z   |     consider giving `x` the explicit type `std::result::Result<&mut i32, E>`, where the type parameter `E` is specified
2019-08-01T07:15:49.1770241Z error: aborting due to previous error
2019-08-01T07:15:49.1770269Z 
2019-08-01T07:15:49.1770513Z For more information about this error, try `rustc --explain E0282`.
2019-08-01T07:15:49.1770738Z Couldn't compile the test.
2019-08-01T07:15:49.1770738Z Couldn't compile the test.
2019-08-01T07:15:49.1771139Z ---- result.rs - result::Result<&'_ mut T, E>::copied (line 849) stdout ----
2019-08-01T07:15:49.1771217Z error[E0282]: type annotations needed for `std::result::Result<&mut i32, E>`
2019-08-01T07:15:49.1771522Z   |
2019-08-01T07:15:49.1771522Z   |
2019-08-01T07:15:49.1771564Z 6 | let x = Ok(&mut val);
2019-08-01T07:15:49.1771800Z   |     -   ^^ cannot infer type for `E`
2019-08-01T07:15:49.1771847Z   |     |
2019-08-01T07:15:49.1771899Z   |     consider giving `x` the explicit type `std::result::Result<&mut i32, E>`, where the type parameter `E` is specified
2019-08-01T07:15:49.1771997Z error: aborting due to previous error
2019-08-01T07:15:49.1772027Z 
2019-08-01T07:15:49.1772271Z For more information about this error, try `rustc --explain E0282`.
2019-08-01T07:15:49.1772495Z Couldn't compile the test.
2019-08-01T07:15:49.1772495Z Couldn't compile the test.
2019-08-01T07:15:49.1772917Z ---- result.rs - result::Result<T, &'_ E>::cloned_err (line 949) stdout ----
2019-08-01T07:15:49.1772976Z error[E0034]: multiple applicable items in scope
2019-08-01T07:15:49.1773232Z   |
2019-08-01T07:15:49.1773271Z 8 | let cloned = x.cloned();
2019-08-01T07:15:49.1773315Z   |                ^^^^^^ multiple `cloned` found
2019-08-01T07:15:49.1773373Z   |
2019-08-01T07:15:49.1773373Z   |
2019-08-01T07:15:49.1773419Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, _>`
2019-08-01T07:15:49.1773471Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, _>`
2019-08-01T07:15:49.1773560Z error: aborting due to previous error
2019-08-01T07:15:49.1773587Z 
2019-08-01T07:15:49.1773817Z For more information about this error, try `rustc --explain E0034`.
2019-08-01T07:15:49.1774021Z Couldn't compile the test.
2019-08-01T07:15:49.1774021Z Couldn't compile the test.
2019-08-01T07:15:49.1774256Z ---- result.rs - result::Result<T, &'_ E>::copied_err (line 869) stdout ----
2019-08-01T07:15:49.1774328Z error[E0282]: type annotations needed for `std::result::Result<T, &i32>`
2019-08-01T07:15:49.1774575Z   |
2019-08-01T07:15:49.1774575Z   |
2019-08-01T07:15:49.1774629Z 6 | let x = Err(&val);
2019-08-01T07:15:49.1774837Z   |     -   ^^^ cannot infer type for `T`
2019-08-01T07:15:49.1774879Z   |     |
2019-08-01T07:15:49.1774944Z   |     consider giving `x` the explicit type `std::result::Result<T, &i32>`, where the type parameter `T` is specified
2019-08-01T07:15:49.1775016Z error: aborting due to previous error
2019-08-01T07:15:49.1775042Z 
2019-08-01T07:15:49.1775287Z For more information about this error, try `rustc --explain E0282`.
2019-08-01T07:15:49.1775483Z Couldn't compile the test.
2019-08-01T07:15:49.1775483Z Couldn't compile the test.
2019-08-01T07:15:49.1775721Z ---- result.rs - result::Result<T, &'_ mut E>::cloned_err (line 969) stdout ----
2019-08-01T07:15:49.1775788Z error[E0034]: multiple applicable items in scope
2019-08-01T07:15:49.1776018Z   |
2019-08-01T07:15:49.1776056Z 8 | let cloned = x.cloned();
2019-08-01T07:15:49.1776123Z   |                ^^^^^^ multiple `cloned` found
2019-08-01T07:15:49.1776318Z   |
2019-08-01T07:15:49.1776318Z   |
2019-08-01T07:15:49.1776363Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, _>`
2019-08-01T07:15:49.1776430Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, _>`
2019-08-01T07:15:49.1776498Z error: aborting due to previous error
2019-08-01T07:15:49.1776525Z 
2019-08-01T07:15:49.1776801Z For more information about this error, try `rustc --explain E0034`.
2019-08-01T07:15:49.1776996Z Couldn't compile the test.
2019-08-01T07:15:49.1776996Z Couldn't compile the test.
2019-08-01T07:15:49.1777428Z ---- result.rs - result::Result<T, &'_ mut E>::copied_err (line 889) stdout ----
2019-08-01T07:15:49.1777497Z error[E0425]: cannot find value `cloned` in this scope
2019-08-01T07:15:49.1777739Z   |
2019-08-01T07:15:49.1777739Z   |
2019-08-01T07:15:49.1777796Z 9 | assert_eq!(cloned, Err(12));
2019-08-01T07:15:49.1777869Z 
2019-08-01T07:15:49.1777989Z error[E0034]: multiple applicable items in scope
2019-08-01T07:15:49.1778240Z  --> result.rs:894:16
2019-08-01T07:15:49.1778284Z   |
2019-08-01T07:15:49.1778284Z   |
2019-08-01T07:15:49.1778323Z 8 | let copied = x.copied();
2019-08-01T07:15:49.1778383Z   |                ^^^^^^ multiple `copied` found
2019-08-01T07:15:49.1778425Z   |
2019-08-01T07:15:49.1778514Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, _>`
2019-08-01T07:15:49.1778584Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, _>`
2019-08-01T07:15:49.1778829Z error: aborting due to 2 previous errors
2019-08-01T07:15:49.1778860Z 
2019-08-01T07:15:49.1778919Z Some errors have detailed explanations: E0034, E0425.
2019-08-01T07:15:49.1779860Z For more information about an error, try `rustc --explain E0034`.
2019-08-01T07:15:49.1779860Z For more information about an error, try `rustc --explain E0034`.
2019-08-01T07:15:49.1780076Z Couldn't compile the test.
2019-08-01T07:15:49.1780108Z 
2019-08-01T07:15:49.1780165Z failures:
2019-08-01T07:15:49.1780421Z     result.rs - result::Result<&'_ T, E>::cloned (line 909)
2019-08-01T07:15:49.1780678Z     result.rs - result::Result<&'_ T, E>::copied (line 829)
2019-08-01T07:15:49.1780941Z     result.rs - result::Result<&'_ mut T, E>::cloned (line 929)
2019-08-01T07:15:49.1781185Z     result.rs - result::Result<&'_ mut T, E>::copied (line 849)
2019-08-01T07:15:49.1781426Z     result.rs - result::Result<T, &'_ E>::cloned_err (line 949)
2019-08-01T07:15:49.1781686Z     result.rs - result::Result<T, &'_ E>::copied_err (line 869)
2019-08-01T07:15:49.1781935Z     result.rs - result::Result<T, &'_ mut E>::cloned_err (line 969)
2019-08-01T07:15:49.1782183Z     result.rs - result::Result<T, &'_ mut E>::copied_err (line 889)
2019-08-01T07:15:49.1782283Z test result: FAILED. 2364 passed; 8 failed; 15 ignored; 0 measured; 0 filtered out
2019-08-01T07:15:49.1782315Z 
2019-08-01T07:15:49.2076009Z error: test failed, to rerun pass '--doc'
2019-08-01T07:15:49.2112905Z 
2019-08-01T07:15:49.2112905Z 
2019-08-01T07:15:49.2114082Z 
2019-08-01T07:15:49.2116447Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
2019-08-01T07:15:49.2116627Z 
2019-08-01T07:15:49.2116659Z 
2019-08-01T07:15:49.2126742Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-01T07:15:49.2126838Z Build completed unsuccessfully in 1:27:55
2019-08-01T07:15:49.2126838Z Build completed unsuccessfully in 1:27:55
2019-08-01T07:15:49.6922925Z ##[error]Bash exited with code '1'.
2019-08-01T07:15:49.6958129Z ##[section]Starting: Checkout
2019-08-01T07:15:49.6960325Z ==============================================================================
2019-08-01T07:15:49.6960382Z Task         : Get sources
2019-08-01T07:15:49.6960595Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
