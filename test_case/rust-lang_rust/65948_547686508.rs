plain
2019-10-29T22:56:50.5049929Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T22:56:50.5301126Z ##[command]git config gc.auto 0
2019-10-29T22:56:50.5379245Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T22:56:50.5438680Z ##[command]git config --get-all http.proxy
2019-10-29T22:56:50.5569159Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65948/merge:refs/remotes/pull/65948/merge
---
2019-10-29T23:55:43.1024760Z .................................................................................................... 1600/9262
2019-10-29T23:55:48.7086090Z .................................................................................................... 1700/9262
2019-10-29T23:56:00.6792342Z ..........................................................i...............i......................... 1800/9262
2019-10-29T23:56:08.5297607Z .................................................................................................... 1900/9262
2019-10-29T23:56:22.8836858Z ................................................iiiii............................................... 2000/9262
2019-10-29T23:56:33.4430450Z .................................................................................................... 2200/9262
2019-10-29T23:56:35.9464105Z .................................................................................................... 2300/9262
2019-10-29T23:56:39.5810360Z .................................................................................................... 2400/9262
2019-10-29T23:57:02.4169700Z .................................................................................................... 2500/9262
---
2019-10-29T23:59:49.4279932Z .................................................i...............i.................................. 4800/9262
2019-10-29T23:59:58.1158151Z .................................................................................................... 4900/9262
2019-10-30T00:00:06.4192688Z .................................................................................................... 5000/9262
2019-10-30T00:00:12.4937107Z .................................................................................................... 5100/9262
2019-10-30T00:00:22.6539267Z ..................................................ii.ii...........i................................. 5200/9262
2019-10-30T00:00:32.4555559Z .................................................................................................... 5400/9262
2019-10-30T00:00:42.2827110Z .................................................................................................... 5500/9262
2019-10-30T00:00:49.6749597Z ......................i............................................................................. 5600/9262
2019-10-30T00:00:56.1454852Z .................................................................................................... 5700/9262
2019-10-30T00:00:56.1454852Z .................................................................................................... 5700/9262
2019-10-30T00:01:07.5787590Z .................................................................................................... 5800/9262
2019-10-30T00:01:19.4429588Z .......ii...i..ii...........i....................................................................... 5900/9262
2019-10-30T00:01:40.6015552Z .................................................................................................... 6100/9262
2019-10-30T00:01:44.7536468Z .................................................................................................... 6200/9262
2019-10-30T00:01:44.7536468Z .................................................................................................... 6200/9262
2019-10-30T00:01:58.4253355Z ..........................i..ii..................................................................... 6300/9262
2019-10-30T00:02:18.1975914Z ............................................................................................i....... 6500/9262
2019-10-30T00:02:20.4441729Z .................................................................................................... 6600/9262
2019-10-30T00:02:22.7179570Z ...................................................................i................................ 6700/9262
2019-10-30T00:02:25.6304675Z .................................................................................................... 6800/9262
---
2019-10-30T00:07:36.0585139Z  finished in 5.483
2019-10-30T00:07:36.0777717Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-30T00:07:36.2611035Z 
2019-10-30T00:07:36.2611147Z running 158 tests
2019-10-30T00:07:39.3205027Z iiiiii....iii......iii..iiii...i.............................i..i..................i....i........... 100/158
2019-10-30T00:07:41.2812454Z ii.i.i..iiii..............i.........iii.i.........ii......
2019-10-30T00:07:41.2813110Z 
2019-10-30T00:07:41.2817642Z  finished in 5.204
2019-10-30T00:07:41.3029821Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-30T00:07:41.4650926Z 
---
2019-10-30T00:07:43.4188174Z  finished in 2.115
2019-10-30T00:07:43.4372682Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-30T00:07:43.5967010Z 
2019-10-30T00:07:43.5967320Z running 9 tests
2019-10-30T00:07:43.5968411Z iiiiiiiii
2019-10-30T00:07:43.5968838Z 
2019-10-30T00:07:43.5983508Z  finished in 0.160
2019-10-30T00:07:43.6164007Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-30T00:07:43.7944309Z 
---
2019-10-30T00:08:03.3787668Z  finished in 19.762
2019-10-30T00:08:03.4003972Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-30T00:08:03.5787621Z 
2019-10-30T00:08:03.5787925Z running 123 tests
2019-10-30T00:08:27.2344061Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-30T00:08:31.8025330Z i.i.i......iii.i.....ii
2019-10-30T00:08:31.8028029Z 
2019-10-30T00:08:31.8028510Z  finished in 28.402
2019-10-30T00:08:31.8037375Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-30T00:08:31.8038118Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-30T00:20:47.3948749Z 
2019-10-30T00:20:47.3956104Z    Doc-tests core
2019-10-30T00:20:52.4845975Z 
2019-10-30T00:20:52.4876875Z running 2417 tests
2019-10-30T00:21:03.3161754Z ......iiiii......................................................................................... 100/2417
2019-10-30T00:21:13.8845061Z ................................................................................ii.................. 200/2417
2019-10-30T00:21:38.6374974Z ..i................................................................................................. 400/2417
2019-10-30T00:21:38.6374974Z ..i................................................................................................. 400/2417
2019-10-30T00:21:49.0634367Z ..................................................i..i.................iiii......................... 500/2417
2019-10-30T00:21:58.1752381Z ........................F.FFFFFF.................................................................... 600/2417
2019-10-30T00:22:18.5971277Z .................................................................................................... 800/2417
2019-10-30T00:22:28.6973178Z .................................................................................................... 900/2417
2019-10-30T00:22:38.8834572Z .................................................................................................... 1000/2417
2019-10-30T00:22:49.0659157Z .................................................................................................... 1100/2417
---
2019-10-30T00:25:07.7979736Z ..........................i......................................................................... 2400/2417
2019-10-30T00:25:09.6435294Z .................
2019-10-30T00:25:09.6436798Z failures:
2019-10-30T00:25:09.6436863Z 
2019-10-30T00:25:09.6437728Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_mut (line 591) stdout ----
2019-10-30T00:25:09.6438770Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6439017Z   --> mem/maybe_uninit.rs:612:9
2019-10-30T00:25:09.6439128Z 24 |     buf.get_mut()
2019-10-30T00:25:09.6439171Z    |         ^^^^^^^
2019-10-30T00:25:09.6439226Z    |
2019-10-30T00:25:09.6439226Z    |
2019-10-30T00:25:09.6439629Z    = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6439722Z    = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6439993Z 
2019-10-30T00:25:09.6440313Z error[E0658]: use of unstable library feature 'is_sorted': new API
2019-10-30T00:25:09.6440544Z   --> mem/maybe_uninit.rs:616:13
2019-10-30T00:25:09.6440589Z    |
2019-10-30T00:25:09.6440646Z 28 | assert!(buf.is_sorted());
2019-10-30T00:25:09.6440728Z    |
2019-10-30T00:25:09.6440728Z    |
2019-10-30T00:25:09.6441050Z    = note: for more information, see ***/issues/53485
2019-10-30T00:25:09.6441107Z    = help: add `#![feature(is_sorted)]` to the crate attributes to enable
2019-10-30T00:25:09.6441196Z error: aborting due to 2 previous errors
2019-10-30T00:25:09.6441224Z 
2019-10-30T00:25:09.6441493Z For more information about this error, try `rustc --explain E0658`.
2019-10-30T00:25:09.6441712Z Couldn't compile the test.
2019-10-30T00:25:09.6441712Z Couldn't compile the test.
2019-10-30T00:25:09.6442134Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_mut (line 623) stdout ----
2019-10-30T00:25:09.6442407Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6442626Z  --> mem/maybe_uninit.rs:628:8
2019-10-30T00:25:09.6442685Z   |
2019-10-30T00:25:09.6442726Z 8 |     *b.get_mut() = true;
2019-10-30T00:25:09.6442819Z   |
2019-10-30T00:25:09.6442819Z   |
2019-10-30T00:25:09.6443227Z   = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6443282Z   = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6443367Z error: aborting due to previous error
2019-10-30T00:25:09.6443393Z 
2019-10-30T00:25:09.6443641Z For more information about this error, try `rustc --explain E0658`.
2019-10-30T00:25:09.6443857Z Couldn't compile the test.
2019-10-30T00:25:09.6443857Z Couldn't compile the test.
2019-10-30T00:25:09.6444120Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_mut (line 638) stdout ----
2019-10-30T00:25:09.6444359Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6444798Z  --> mem/maybe_uninit.rs:644:39
2019-10-30T00:25:09.6444855Z   |
2019-10-30T00:25:09.6444894Z 9 |     reader.read_exact(unsafe { buffer.get_mut() })?;
2019-10-30T00:25:09.6444987Z   |
2019-10-30T00:25:09.6444987Z   |
2019-10-30T00:25:09.6445289Z   = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6445471Z   = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6445544Z error: aborting due to previous error
2019-10-30T00:25:09.6445570Z 
2019-10-30T00:25:09.6445844Z For more information about this error, try `rustc --explain E0658`.
2019-10-30T00:25:09.6446061Z Couldn't compile the test.
2019-10-30T00:25:09.6446061Z Couldn't compile the test.
2019-10-30T00:25:09.6446330Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_mut (line 654) stdout ----
2019-10-30T00:25:09.6446398Z error[E0433]: failed to resolve: use of undeclared type or module `ptr`
2019-10-30T00:25:09.6446616Z   --> mem/maybe_uninit.rs:664:5
2019-10-30T00:25:09.6446674Z    |
2019-10-30T00:25:09.6446719Z 13 |     ptr::write(&mut foo.get_mut().a as *mut u32, 1337);
2019-10-30T00:25:09.6446779Z    |     ^^^ use of undeclared type or module `ptr`
2019-10-30T00:25:09.6446852Z error[E0433]: failed to resolve: use of undeclared type or module `ptr`
2019-10-30T00:25:09.6447092Z   --> mem/maybe_uninit.rs:668:5
2019-10-30T00:25:09.6447134Z    |
2019-10-30T00:25:09.6447134Z    |
2019-10-30T00:25:09.6447178Z 17 |     ptr::write(&mut foo.get_mut().b as *mut u8, 42);
2019-10-30T00:25:09.6447224Z    |     ^^^ use of undeclared type or module `ptr`
2019-10-30T00:25:09.6447385Z 
2019-10-30T00:25:09.6447639Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6448117Z   --> mem/maybe_uninit.rs:664:25
2019-10-30T00:25:09.6448190Z    |
2019-10-30T00:25:09.6448236Z 13 |     ptr::write(&mut foo.get_mut().a as *mut u32, 1337);
2019-10-30T00:25:09.6448336Z    |
2019-10-30T00:25:09.6448336Z    |
2019-10-30T00:25:09.6448703Z    = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6448888Z    = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6448920Z 
2019-10-30T00:25:09.6449236Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6449465Z   --> mem/maybe_uninit.rs:668:25
2019-10-30T00:25:09.6449510Z    |
2019-10-30T00:25:09.6449569Z 17 |     ptr::write(&mut foo.get_mut().b as *mut u8, 42);
2019-10-30T00:25:09.6449655Z    |
2019-10-30T00:25:09.6449655Z    |
2019-10-30T00:25:09.6449969Z    = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6450027Z    = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6450115Z error: aborting due to 4 previous errors
2019-10-30T00:25:09.6450143Z 
2019-10-30T00:25:09.6450187Z Some errors have detailed explanations: E0433, E0658.
2019-10-30T00:25:09.6450463Z For more information about an error, try `rustc --explain E0433`.
2019-10-30T00:25:09.6450463Z For more information about an error, try `rustc --explain E0433`.
2019-10-30T00:25:09.6450710Z Couldn't compile the test.
2019-10-30T00:25:09.6450989Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_ref (line 549) stdout ----
2019-10-30T00:25:09.6451368Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6451597Z  --> mem/maybe_uninit.rs:553:35
2019-10-30T00:25:09.6451639Z   |
2019-10-30T00:25:09.6451680Z 7 | let x_vec: &Vec<u32> = unsafe { x.get_ref() };
2019-10-30T00:25:09.6451778Z   |
2019-10-30T00:25:09.6451778Z   |
2019-10-30T00:25:09.6452052Z   = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6452122Z   = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6452198Z error: aborting due to previous error
2019-10-30T00:25:09.6452226Z 
2019-10-30T00:25:09.6452526Z For more information about this error, try `rustc --explain E0658`.
2019-10-30T00:25:09.6452854Z Couldn't compile the test.
2019-10-30T00:25:09.6452854Z Couldn't compile the test.
2019-10-30T00:25:09.6453174Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_ref (line 528) stdout ----
2019-10-30T00:25:09.6453457Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6453695Z   --> mem/maybe_uninit.rs:542:7
2019-10-30T00:25:09.6453794Z 17 |     x.get_ref()
2019-10-30T00:25:09.6453837Z    |       ^^^^^^^
2019-10-30T00:25:09.6453877Z    |
2019-10-30T00:25:09.6453877Z    |
2019-10-30T00:25:09.6454203Z    = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6454259Z    = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6454333Z error: aborting due to previous error
2019-10-30T00:25:09.6454375Z 
2019-10-30T00:25:09.6454647Z For more information about this error, try `rustc --explain E0658`.
2019-10-30T00:25:09.6454874Z Couldn't compile the test.
2019-10-30T00:25:09.6454874Z Couldn't compile the test.
2019-10-30T00:25:09.6455175Z ---- mem/maybe_uninit.rs - mem::maybe_uninit::MaybeUninit<T>::get_ref (line 557) stdout ----
2019-10-30T00:25:09.6455451Z error[E0658]: use of unstable library feature 'maybe_uninit_ref'
2019-10-30T00:25:09.6455680Z  --> mem/maybe_uninit.rs:563:7
2019-10-30T00:25:09.6455723Z   |
2019-10-30T00:25:09.6455778Z 9 |     b.get_ref().set(true);
2019-10-30T00:25:09.6455859Z   |
2019-10-30T00:25:09.6455859Z   |
2019-10-30T00:25:09.6456173Z   = note: for more information, see ***/issues/63568
2019-10-30T00:25:09.6456229Z   = help: add `#![feature(maybe_uninit_ref)]` to the crate attributes to enable
2019-10-30T00:25:09.6456318Z error: aborting due to previous error
2019-10-30T00:25:09.6456347Z 
2019-10-30T00:25:09.6456613Z For more information about this error, try `rustc --explain E0658`.
2019-10-30T00:25:09.6456837Z Couldn't compile the test.
---
2019-10-30T00:25:09.6459693Z 
2019-10-30T00:25:09.6585912Z error: test failed, to rerun pass '--doc'
2019-10-30T00:25:09.6594847Z 
2019-10-30T00:25:09.6594915Z 
2019-10-30T00:25:09.6596402Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "core" "--" "--quiet"
2019-10-30T00:25:09.6596565Z 
2019-10-30T00:25:09.6596812Z 
2019-10-30T00:25:09.6609634Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-30T00:25:09.6609710Z Build completed unsuccessfully in 1:21:39
2019-10-30T00:25:09.6609710Z Build completed unsuccessfully in 1:21:39
2019-10-30T00:25:09.6662399Z == clock drift check ==
2019-10-30T00:25:09.6677310Z   local time: Wed Oct 30 00:25:09 UTC 2019
2019-10-30T00:25:09.8213417Z   network time: Wed, 30 Oct 2019 00:25:09 GMT
2019-10-30T00:25:09.8215838Z == end clock drift check ==
2019-10-30T00:25:10.3908820Z 
2019-10-30T00:25:10.4035636Z ##[error]Bash exited with code '1'.
2019-10-30T00:25:10.4074016Z ##[section]Starting: Checkout
2019-10-30T00:25:10.4075798Z ==============================================================================
2019-10-30T00:25:10.4075856Z Task         : Get sources
2019-10-30T00:25:10.4075905Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
