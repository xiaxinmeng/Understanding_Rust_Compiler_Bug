plain
2019-11-28T20:15:32.6319220Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T20:15:32.6527790Z ##[command]git config gc.auto 0
2019-11-28T20:15:32.6606640Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T20:15:32.6660183Z ##[command]git config --get-all http.proxy
2019-11-28T20:15:32.6803285Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66836/merge:refs/remotes/pull/66836/merge
---
2019-11-28T21:14:46.1187572Z .................................................................................................... 1600/9304
2019-11-28T21:14:50.8995741Z .................................................................................................... 1700/9304
2019-11-28T21:15:03.5195109Z ...................................i................................................................ 1800/9304
2019-11-28T21:15:11.4321076Z .................................................................................................... 1900/9304
2019-11-28T21:15:25.3429861Z ....................iiiii........................................................................... 2000/9304
2019-11-28T21:15:35.5613186Z .................................................................................................... 2200/9304
2019-11-28T21:15:38.1584248Z .................................................................................................... 2300/9304
2019-11-28T21:15:43.1198184Z .................................................................................................... 2400/9304
2019-11-28T21:16:04.3455714Z .................................................................................................... 2500/9304
---
2019-11-28T21:18:48.4495891Z .....................i...............i.............................................................. 4800/9304
2019-11-28T21:18:58.8300726Z .................................................................................................... 4900/9304
2019-11-28T21:19:04.4523104Z .................................................................................................... 5000/9304
2019-11-28T21:19:13.0483631Z .................................................................................................... 5100/9304
2019-11-28T21:19:20.5636027Z ..........................ii.ii...........i......................................................... 5200/9304
2019-11-28T21:19:30.1253059Z .................................................................................................... 5400/9304
2019-11-28T21:19:41.0981387Z .................................................................................................... 5500/9304
2019-11-28T21:19:48.2633655Z ........i........................................................................................... 5600/9304
2019-11-28T21:19:54.6753732Z .................................................................................................... 5700/9304
2019-11-28T21:19:54.6753732Z .................................................................................................... 5700/9304
2019-11-28T21:20:06.0448772Z ..............................................................................................ii...i 5800/9304
2019-11-28T21:20:18.2825237Z ..ii...........i.................................................................................... 5900/9304
2019-11-28T21:20:36.8832027Z .................................................................................................... 6100/9304
2019-11-28T21:20:41.0837986Z .................................................................................................... 6200/9304
2019-11-28T21:20:41.0837986Z .................................................................................................... 6200/9304
2019-11-28T21:20:54.6129016Z .................i..ii.............................................................................. 6300/9304
2019-11-28T21:21:13.6634756Z .....................................................................................i.............. 6500/9304
2019-11-28T21:21:15.9439306Z .................................................................................................... 6600/9304
2019-11-28T21:21:18.1196678Z ............................................................................i....................... 6700/9304
2019-11-28T21:21:20.8672957Z .................................................................................................... 6800/9304
---
2019-11-28T21:26:26.6805653Z  finished in 5.922
2019-11-28T21:26:26.6985377Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:26:26.8979609Z 
2019-11-28T21:26:26.8980536Z running 164 tests
2019-11-28T21:26:29.7190382Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/164
2019-11-28T21:26:31.6326867Z .i.i..iiii..iiiiiii............i.........iii.i..........ii......
2019-11-28T21:26:31.6328175Z 
2019-11-28T21:26:31.6332950Z  finished in 4.934
2019-11-28T21:26:31.6521359Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:26:32.6412204Z 
---
2019-11-28T21:26:33.7594860Z  finished in 2.107
2019-11-28T21:26:33.7770425Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:26:33.9253611Z 
2019-11-28T21:26:33.9254539Z running 9 tests
2019-11-28T21:26:33.9255264Z iiiiiiiii
2019-11-28T21:26:33.9256000Z 
2019-11-28T21:26:33.9260995Z  finished in 0.149
2019-11-28T21:26:33.9431691Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:26:34.1131127Z 
---
2019-11-28T21:26:53.1029426Z  finished in 19.159
2019-11-28T21:26:53.1240290Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:26:53.3100707Z 
2019-11-28T21:26:53.3101364Z running 124 tests
2019-11-28T21:27:17.4188089Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i....ii...i.......ii 100/124
2019-11-28T21:27:22.2429451Z .i.i.i......iii.i.....ii
2019-11-28T21:27:22.2430748Z 
2019-11-28T21:27:22.2434049Z  finished in 29.119
2019-11-28T21:27:22.2443234Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:27:22.2444082Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-28T21:38:30.3311599Z 
2019-11-28T21:38:30.3312938Z    Doc-tests core
2019-11-28T21:38:35.1756165Z 
2019-11-28T21:38:35.1757215Z running 2421 tests
2019-11-28T21:38:45.6029037Z ......iiiii......................................................................................... 100/2421
2019-11-28T21:39:07.5756025Z .................................................................................................... 300/2421
2019-11-28T21:39:20.3177687Z ..i................................................................................................. 400/2421
2019-11-28T21:39:20.3177687Z ..i................................................................................................. 400/2421
2019-11-28T21:39:30.8060551Z ..................................................i..i..................iiii........................ 500/2421
2019-11-28T21:39:49.6017400Z .................................................................................................... 700/2421
2019-11-28T21:39:59.4415591Z .................................................................................................... 800/2421
2019-11-28T21:40:09.3556743Z .................................................................................................... 900/2421
2019-11-28T21:40:19.2191072Z .................................................................................................... 1000/2421
---
2019-11-28T21:44:17.3053895Z 
2019-11-28T21:44:17.3055541Z running 999 tests
2019-11-28T21:44:36.1351350Z i................................................................................................... 100/999
2019-11-28T21:44:46.8204141Z .................................................................................................... 200/999
2019-11-28T21:44:54.4836859Z ..................iii......i......i...i......i...................................................... 300/999
2019-11-28T21:44:59.5513366Z .................................................................................................... 400/999
2019-11-28T21:45:06.7958422Z ..........................................i..i.................................ii................... 500/999
2019-11-28T21:45:20.5574230Z .................................................................................................... 700/999
2019-11-28T21:45:20.5574230Z .................................................................................................... 700/999
2019-11-28T21:45:27.5767026Z .........................iiii....................................................................... 800/999
2019-11-28T21:45:41.7228779Z .................................................................................................... 900/999
2019-11-28T21:45:48.5704864Z ...............................................iiii................................................
2019-11-28T21:45:48.5706269Z 
2019-11-28T21:45:48.5765974Z  finished in 180.185
2019-11-28T21:45:48.5783288Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-28T21:45:48.7784645Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-11-28T22:03:25.3509786Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0003::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 66) stdout ----
2019-11-28T22:03:25.3509864Z error[E0425]: cannot find value `number` in this scope
2019-11-28T22:03:25.3510125Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:67:7
2019-11-28T22:03:25.3510173Z   |
2019-11-28T22:03:25.3510332Z 3 | match number {
2019-11-28T22:03:25.3510417Z 
2019-11-28T22:03:25.3510470Z error: aborting due to previous error
2019-11-28T22:03:25.3510515Z 
2019-11-28T22:03:25.3510790Z For more information about this error, try `rustc --explain E0425`.
2019-11-28T22:03:25.3510790Z For more information about this error, try `rustc --explain E0425`.
2019-11-28T22:03:25.3510992Z Couldn't compile the test.
2019-11-28T22:03:25.3511535Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0018::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 467) stdout ----
2019-11-28T22:03:25.3511593Z error[E0658]: casting pointers to integers in statics is unstable
2019-11-28T22:03:25.3511818Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:469:32
2019-11-28T22:03:25.3511875Z   |
2019-11-28T22:03:25.3511916Z 4 | static MY_STATIC_ADDR: usize = &MY_STATIC as *const _ as usize;
2019-11-28T22:03:25.3512023Z   |
2019-11-28T22:03:25.3512023Z   |
2019-11-28T22:03:25.3512382Z   = note: for more information, see ***/issues/51910
2019-11-28T22:03:25.3512445Z   = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-11-28T22:03:25.3512528Z error: aborting due to previous error
2019-11-28T22:03:25.3512553Z 
2019-11-28T22:03:25.3512777Z For more information about this error, try `rustc --explain E0658`.
2019-11-28T22:03:25.3512777Z For more information about this error, try `rustc --explain E0658`.
2019-11-28T22:03:25.3512839Z Some expected error codes were not found: ["E0018"]
2019-11-28T22:03:25.3512902Z failures:
2019-11-28T22:03:25.3513213Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0003::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 66)
2019-11-28T22:03:25.3513552Z     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0018::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 467)
2019-11-28T22:03:25.3513597Z 
---
2019-11-28T22:03:25.3514304Z   local time: Thu Nov 28 22:03:25 UTC 2019
2019-11-28T22:03:25.6016473Z   network time: Thu, 28 Nov 2019 22:03:25 GMT
2019-11-28T22:03:25.6017601Z == end clock drift check ==
2019-11-28T22:03:26.4258362Z 
2019-11-28T22:03:26.4414970Z ##[error]Bash exited with code '1'.
2019-11-28T22:03:26.4452293Z ##[section]Starting: Checkout
2019-11-28T22:03:26.4453996Z ==============================================================================
2019-11-28T22:03:26.4454046Z Task         : Get sources
2019-11-28T22:03:26.4454122Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
