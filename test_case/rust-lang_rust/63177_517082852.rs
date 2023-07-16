plain
2019-08-01T00:08:29.1243892Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-01T00:08:29.1423352Z ##[command]git config gc.auto 0
2019-08-01T00:08:29.1473539Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-01T00:08:29.1517023Z ##[command]git config --get-all http.proxy
2019-08-01T00:08:29.1641956Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63177/merge:refs/remotes/pull/63177/merge
---
2019-08-01T00:09:03.9396430Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-01T00:09:03.9396456Z 
2019-08-01T00:09:03.9396620Z   git checkout -b <new-branch-name>
2019-08-01T00:09:03.9396644Z 
2019-08-01T00:09:03.9396695Z HEAD is now at 91e130838 Merge f851f29352aaa9d33fdd08397a297fcfb6c418d8 into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-08-01T00:09:03.9593590Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-01T00:09:03.9595940Z ==============================================================================
2019-08-01T00:09:03.9595986Z Task         : Bash
2019-08-01T00:09:03.9596037Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T01:08:36.9698737Z .................................................................................................... 1400/8819
2019-08-01T01:08:42.8103162Z .................................................................................................... 1500/8819
2019-08-01T01:08:55.5173414Z .................................................................i...............i.................. 1600/8819
2019-08-01T01:09:02.8500228Z .................................................................................................... 1700/8819
2019-08-01T01:09:17.3263892Z ...................................................iiiii............................................ 1800/8819
2019-08-01T01:09:28.3308992Z .................................................................................................... 2000/8819
2019-08-01T01:09:30.7032436Z .................................................................................................... 2100/8819
2019-08-01T01:09:34.2948831Z .................................................................................................... 2200/8819
2019-08-01T01:09:41.1424125Z .................................................................................................... 2300/8819
---
2019-08-01T01:13:34.7135879Z .................................................................................................... 5300/8819
2019-08-01T01:13:42.1100280Z ..............i..................................................................................... 5400/8819
2019-08-01T01:13:47.7255558Z .................................................................................................... 5500/8819
2019-08-01T01:14:00.2290628Z .................................................................................................... 5600/8819
2019-08-01T01:14:13.6168452Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-08-01T01:14:31.0965989Z .................................................................................................... 5900/8819
2019-08-01T01:14:36.0580848Z .................................................................................................... 6000/8819
2019-08-01T01:14:36.0580848Z .................................................................................................... 6000/8819
2019-08-01T01:14:50.1004519Z ........i..ii....................................................................................... 6100/8819
2019-08-01T01:15:09.4352515Z ...................................................i................................................ 6300/8819
2019-08-01T01:15:11.5098524Z .................................................................................................... 6400/8819
2019-08-01T01:15:13.9784184Z .....................i.............................................................................. 6500/8819
2019-08-01T01:15:18.5059526Z .................................................................................................... 6600/8819
---
2019-08-01T01:19:55.0634585Z  finished in 20.485
2019-08-01T01:19:55.0807889Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:19:55.2364015Z 
2019-08-01T01:19:55.2364818Z running 146 tests
2019-08-01T01:19:58.4538831Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-08-01T01:20:00.2778030Z iii..............i.........iii.i......ii......
2019-08-01T01:20:00.2781739Z 
2019-08-01T01:20:00.2783118Z  finished in 5.197
2019-08-01T01:20:00.2956021Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:20:00.4517342Z 
---
2019-08-01T01:20:02.4759891Z  finished in 2.180
2019-08-01T01:20:02.4935453Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:20:02.6419283Z 
2019-08-01T01:20:02.6419869Z running 9 tests
2019-08-01T01:20:02.6421064Z iiiiiiiii
2019-08-01T01:20:02.6421456Z 
2019-08-01T01:20:02.6421502Z  finished in 0.148
2019-08-01T01:20:02.6598460Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:20:02.8148703Z 
---
2019-08-01T01:20:20.7166270Z  finished in 18.056
2019-08-01T01:20:20.7357841Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:20:20.8999427Z 
2019-08-01T01:20:20.8999773Z running 122 tests
2019-08-01T01:20:43.6670459Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-01T01:20:48.1862250Z .i.i......iii.i.....ii
2019-08-01T01:20:48.1865961Z 
2019-08-01T01:20:48.1870288Z  finished in 27.451
2019-08-01T01:20:48.1877922Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:20:48.1878347Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-01T01:33:15.5752170Z 
2019-08-01T01:33:15.5876215Z  finished in 161.372
2019-08-01T01:33:15.5891763Z Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-01T01:33:15.7765974Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-08-01T01:33:21.0310751Z error[E0658]: use of unstable library feature 'find_result': new API
2019-08-01T01:33:21.0312405Z     --> src/libcore/../libcore/tests/iter.rs:1264:26
2019-08-01T01:33:21.0312985Z      |
2019-08-01T01:33:21.0313573Z 1264 |     assert_eq!(xs.iter().find_result(testfn), Ok(None));
2019-08-01T01:33:21.0314964Z      |
2019-08-01T01:33:21.0314964Z      |
2019-08-01T01:33:21.0315738Z      = note: for more information, see ***/issues/63178
2019-08-01T01:33:21.0316304Z      = help: add `#![feature(find_result)]` to the crate attributes to enable
2019-08-01T01:33:21.0316497Z 
2019-08-01T01:33:21.0328362Z error[E0658]: use of unstable library feature 'find_result': new API
2019-08-01T01:33:21.0328667Z     --> src/libcore/../libcore/tests/iter.rs:1266:26
2019-08-01T01:33:21.0329028Z      |
2019-08-01T01:33:21.0329272Z 1266 |     assert_eq!(xs.iter().find_result(testfn), Ok(Some(&2)));
2019-08-01T01:33:21.0329719Z      |
2019-08-01T01:33:21.0329719Z      |
2019-08-01T01:33:21.0330211Z      = note: for more information, see ***/issues/63178
2019-08-01T01:33:21.0330465Z      = help: add `#![feature(find_result)]` to the crate attributes to enable
2019-08-01T01:33:21.0330498Z 
2019-08-01T01:33:21.0342397Z error[E0658]: use of unstable library feature 'find_result': new API
2019-08-01T01:33:21.0342935Z     --> src/libcore/../libcore/tests/iter.rs:1268:26
2019-08-01T01:33:21.0343291Z      |
2019-08-01T01:33:21.0343669Z 1268 |     assert_eq!(xs.iter().find_result(testfn), Err(()));
2019-08-01T01:33:21.0345149Z      |
2019-08-01T01:33:21.0345149Z      |
2019-08-01T01:33:21.0345741Z      = note: for more information, see ***/issues/63178
2019-08-01T01:33:21.0346455Z      = help: add `#![feature(find_result)]` to the crate attributes to enable
2019-08-01T01:33:21.0346682Z 
2019-08-01T01:33:21.0358742Z error[E0658]: use of unstable library feature 'find_result': new API
2019-08-01T01:33:21.0359392Z     --> src/libcore/../libcore/tests/iter.rs:1272:21
2019-08-01T01:33:21.0359817Z      |
2019-08-01T01:33:21.0360439Z 1272 |     assert_eq!(iter.find_result(testfn), Ok(Some(&2)));
2019-08-01T01:33:21.0361994Z      |
2019-08-01T01:33:21.0361994Z      |
2019-08-01T01:33:21.0362831Z      = note: for more information, see ***/issues/63178
2019-08-01T01:33:21.0363557Z      = help: add `#![feature(find_result)]` to the crate attributes to enable
2019-08-01T01:33:21.0363724Z 
2019-08-01T01:33:21.0376001Z error[E0658]: use of unstable library feature 'find_result': new API
2019-08-01T01:33:21.0376567Z     --> src/libcore/../libcore/tests/iter.rs:1273:21
2019-08-01T01:33:21.0377066Z      |
2019-08-01T01:33:21.0377745Z 1273 |     assert_eq!(iter.find_result(testfn), Err(()));
2019-08-01T01:33:21.0378525Z      |
2019-08-01T01:33:21.0378525Z      |
2019-08-01T01:33:21.0379024Z      = note: for more information, see ***/issues/63178
2019-08-01T01:33:21.0379659Z      = help: add `#![feature(find_result)]` to the crate attributes to enable
2019-08-01T01:33:27.3009518Z error: aborting due to 5 previous errors
2019-08-01T01:33:27.3010100Z 
2019-08-01T01:33:27.3010483Z For more information about this error, try `rustc --explain E0658`.
2019-08-01T01:33:27.4120842Z error: Could not compile `core`.
2019-08-01T01:33:27.4120842Z error: Could not compile `core`.
2019-08-01T01:33:27.4121441Z 
2019-08-01T01:33:27.4121823Z To learn more, run the command again with --verbose.
2019-08-01T01:33:27.4140405Z 
2019-08-01T01:33:27.4140639Z 
2019-08-01T01:33:27.4141341Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
2019-08-01T01:33:27.4141742Z 
2019-08-01T01:33:27.4141851Z 
2019-08-01T01:33:27.4151992Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-01T01:33:27.4152242Z Build completed unsuccessfully in 1:18:01
2019-08-01T01:33:27.4152242Z Build completed unsuccessfully in 1:18:01
2019-08-01T01:33:27.9374229Z ##[error]Bash exited with code '1'.
2019-08-01T01:33:27.9420049Z ##[section]Starting: Checkout
2019-08-01T01:33:27.9421543Z ==============================================================================
2019-08-01T01:33:27.9421603Z Task         : Get sources
2019-08-01T01:33:27.9421640Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
