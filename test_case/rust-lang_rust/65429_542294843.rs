plain
2019-10-15T14:42:42.4268132Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T14:42:42.4321209Z ##[command]git config gc.auto 0
2019-10-15T14:42:42.4418587Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T14:42:42.4490560Z ##[command]git config --get-all http.proxy
2019-10-15T14:42:42.4653981Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65429/merge:refs/remotes/pull/65429/merge
---
2019-10-15T15:46:39.3428784Z .................................................................................................... 1600/9182
2019-10-15T15:46:44.8588501Z .................................................................................................... 1700/9182
2019-10-15T15:46:58.4061692Z ............................i...............i....................................................... 1800/9182
2019-10-15T15:47:06.2491724Z .................................................................................................... 1900/9182
2019-10-15T15:47:21.3286414Z ...................iiiii............................................................................ 2000/9182
2019-10-15T15:47:32.2184999Z .................................................................................................... 2200/9182
2019-10-15T15:47:34.8915288Z .................................................................................................... 2300/9182
2019-10-15T15:47:40.3373196Z .................................................................................................... 2400/9182
2019-10-15T15:48:03.2948882Z .................................................................................................... 2500/9182
---
2019-10-15T15:51:10.0476301Z ...........................i...............i........................................................ 4800/9182
2019-10-15T15:51:22.9516381Z .................................................................................................... 4900/9182
2019-10-15T15:51:29.7397966Z .................................................................................................... 5000/9182
2019-10-15T15:51:39.5067232Z .................................................................................................... 5100/9182
2019-10-15T15:51:47.5738940Z ...........................ii.ii.................................................................... 5200/9182
2019-10-15T15:51:57.7228671Z .................................................................................................... 5400/9182
2019-10-15T15:52:09.2171246Z .............................................................................................i...... 5500/9182
2019-10-15T15:52:17.8965620Z .................................................................................................... 5600/9182
2019-10-15T15:52:23.0432316Z .................................................................................................... 5700/9182
2019-10-15T15:52:23.0432316Z .................................................................................................... 5700/9182
2019-10-15T15:52:34.3219632Z ..........................................................................................ii...i..ii 5800/9182
2019-10-15T15:53:00.8181754Z .................................................................................................... 6000/9182
2019-10-15T15:53:11.1072161Z .................................................................................................... 6100/9182
2019-10-15T15:53:20.7125357Z .................................................................................................i.. 6200/9182
2019-10-15T15:53:35.8781333Z ii.................................................................................................. 6300/9182
---
2019-10-15T15:58:49.9132189Z  finished in 6.095
2019-10-15T15:58:49.9374821Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T15:58:50.1096455Z 
2019-10-15T15:58:50.1097768Z running 153 tests
2019-10-15T15:58:53.5879378Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-15T15:58:55.7296021Z i..iiii..............i.........iii.i.........ii......
2019-10-15T15:58:55.7298205Z 
2019-10-15T15:58:55.7299893Z  finished in 5.792
2019-10-15T15:58:55.7498193Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T15:58:55.9138479Z 
---
2019-10-15T15:58:58.1645956Z  finished in 2.415
2019-10-15T15:58:58.1875492Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T15:58:58.3590350Z 
2019-10-15T15:58:58.3590986Z running 9 tests
2019-10-15T15:58:58.3592566Z iiiiiiiii
2019-10-15T15:58:58.3593433Z 
2019-10-15T15:58:58.3599980Z  finished in 0.172
2019-10-15T15:58:58.3883364Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T15:58:58.5783480Z 
---
2019-10-15T15:59:17.9075266Z  finished in 19.519
2019-10-15T15:59:17.9311602Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T15:59:18.1137476Z 
2019-10-15T15:59:18.1138336Z running 123 tests
2019-10-15T15:59:45.3078138Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-15T15:59:50.3091721Z i.i.i......iii.i.....ii
2019-10-15T15:59:50.3093169Z 
2019-10-15T15:59:50.3093379Z  finished in 32.378
2019-10-15T15:59:50.3105271Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-15T15:59:50.3107068Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-15T16:13:39.9336320Z 
2019-10-15T16:13:39.9340822Z    Doc-tests core
2019-10-15T16:13:45.2838525Z 
2019-10-15T16:13:45.2843071Z running 2405 tests
2019-10-15T16:13:57.6505158Z ......iiiii......................................................................................... 100/2405
2019-10-15T16:14:09.4492066Z ...............................................................................ii................... 200/2405
2019-10-15T16:14:37.4470280Z .i.................................................................................................. 400/2405
2019-10-15T16:14:37.4470280Z .i.................................................................................................. 400/2405
2019-10-15T16:14:49.1851923Z ................................................i..i.................iiii........................... 500/2405
2019-10-15T16:15:11.5870175Z .................................................................................................... 700/2405
2019-10-15T16:15:23.2094309Z .................................................................................................... 800/2405
2019-10-15T16:15:34.6505168Z .................................................................................................... 900/2405
2019-10-15T16:15:46.1752945Z .................................................................................................... 1000/2405
---
2019-10-15T16:20:09.3270620Z ............................................... 300/763
2019-10-15T16:20:09.3300794Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-10-15T16:20:09.4065323Z .................................................................................................... 400/763
2019-10-15T16:20:11.4871071Z .................................................................................................... 500/763
2019-10-15T16:20:11.5141254Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-15T16:20:11.5164538Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-10-15T16:20:11.5173770Z .thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-15T16:20:11.5221067Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-15T16:20:11.7760739Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-15T16:20:11.7808672Z ............thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-10-15T16:20:11.7922238Z ............... 600/763
2019-10-15T16:20:13.8258333Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-10-15T16:20:13.8269055Z ..thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-10-15T16:20:13.8280508Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
---
2019-10-15T16:20:23.4298086Z 
2019-10-15T16:20:23.4317616Z running 995 tests
2019-10-15T16:20:47.5272970Z i................................................................................................... 100/995
2019-10-15T16:21:00.4298543Z .................................................................................................... 200/995
2019-10-15T16:21:09.7232661Z ...................iii......i......i...i......i..........................................F.......... 300/995
2019-10-15T16:21:16.1462371Z .................................................................................................... 400/995
2019-10-15T16:21:24.7562121Z ......................................i..i.................................ii....................... 500/995
2019-10-15T16:21:41.6102242Z .................................................................................................... 700/995
2019-10-15T16:21:41.6102242Z .................................................................................................... 700/995
2019-10-15T16:21:50.4196209Z .....................iiii........................................................................... 800/995
2019-10-15T16:22:07.4011068Z .................................................................................................... 900/995
2019-10-15T16:22:15.8084275Z ...........................................iiii................................................
2019-10-15T16:22:15.8085314Z 
2019-10-15T16:22:15.8086312Z ---- fs.rs - fs::File::with_options (line 417) stdout ----
2019-10-15T16:22:15.8086861Z error[E0658]: use of unstable library feature 'with_options'
2019-10-15T16:22:15.8087280Z  --> fs.rs:421:17
2019-10-15T16:22:15.8087280Z  --> fs.rs:421:17
2019-10-15T16:22:15.8087467Z   |
2019-10-15T16:22:15.8087693Z 6 |     let mut f = File::with_options().read(true).open("foo.txt")?;
2019-10-15T16:22:15.8088336Z   |
2019-10-15T16:22:15.8088336Z   |
2019-10-15T16:22:15.8089040Z   = note: for more information, see ***/issues/65439
2019-10-15T16:22:15.8089277Z   = help: add `#![feature(with_options)]` to the crate attributes to enable
2019-10-15T16:22:15.8089584Z error: aborting due to previous error
2019-10-15T16:22:15.8089708Z 
2019-10-15T16:22:15.8090152Z For more information about this error, try `rustc --explain E0658`.
2019-10-15T16:22:15.8090592Z Couldn't compile the test.
---
2019-10-15T16:22:15.8194001Z == clock drift check ==
2019-10-15T16:22:15.8209921Z   local time: Tue Oct 15 16:22:15 UTC 2019
2019-10-15T16:22:16.1006154Z   network time: Tue, 15 Oct 2019 16:22:16 GMT
2019-10-15T16:22:16.1018228Z == end clock drift check ==
2019-10-15T16:22:17.3519182Z ##[error]Bash exited with code '1'.
2019-10-15T16:22:17.3572740Z ##[section]Starting: Checkout
2019-10-15T16:22:17.3575110Z ==============================================================================
2019-10-15T16:22:17.3575177Z Task         : Get sources
2019-10-15T16:22:17.3575229Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
