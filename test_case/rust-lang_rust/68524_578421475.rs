plain
2020-01-25T15:05:23.6556918Z ========================== Starting Command Output ===========================
2020-01-25T15:05:23.6573838Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4392cf68-94e4-4c2b-996d-662b945002ef.sh
2020-01-25T15:05:23.6807084Z 
2020-01-25T15:05:23.6859259Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T15:05:23.6864993Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T15:05:23.6866698Z Task         : Get sources
2020-01-25T15:05:23.6866733Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T15:05:23.6866769Z Version      : 1.0.0
2020-01-25T15:05:23.6866852Z Author       : Microsoft
---
2020-01-25T15:05:24.8443832Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T15:05:24.8458173Z ##[command]git config gc.auto 0
2020-01-25T15:05:24.8461132Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T15:05:24.8463831Z ##[command]git config --get-all http.proxy
2020-01-25T15:05:24.8471203Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-25T15:56:08.9037122Z .................................................................................................... 1700/9556
2020-01-25T15:56:14.5051765Z .................................................................................................... 1800/9556
2020-01-25T15:56:25.2979117Z .......................i............................................................................ 1900/9556
2020-01-25T15:56:31.7231783Z .................................................................................................... 2000/9556
2020-01-25T15:56:44.9496709Z .............iiiii.................................................................................. 2100/9556
2020-01-25T15:56:53.9558874Z .................................................................................................... 2300/9556
2020-01-25T15:56:56.1589896Z .................................................................................................... 2400/9556
2020-01-25T15:57:00.9643703Z .................................................................................................... 2500/9556
2020-01-25T15:57:19.5040019Z .................................................................................................... 2600/9556
---
2020-01-25T15:59:45.8022016Z ..............................................................i...............i..................... 4900/9556
2020-01-25T15:59:52.7550597Z .................................................................................................... 5000/9556
2020-01-25T16:00:00.2606212Z .................................................................................................... 5100/9556
2020-01-25T16:00:04.4947183Z .....i.............................................................................................. 5200/9556
2020-01-25T16:00:14.8788400Z .............................................................................ii.ii........i...i..... 5300/9556
2020-01-25T16:00:22.8845562Z ...............i.................................................................................... 5500/9556
2020-01-25T16:00:31.5878851Z .................................................................................................... 5600/9556
2020-01-25T16:00:37.6471994Z ................................................................i................................... 5700/9556
2020-01-25T16:00:44.2398497Z .................................................................................................... 5800/9556
2020-01-25T16:00:44.2398497Z .................................................................................................... 5800/9556
2020-01-25T16:00:51.1257399Z .................................................................................................... 5900/9556
2020-01-25T16:00:59.3884151Z .......................................................ii...i..ii...........i....................... 6000/9556
2020-01-25T16:01:19.8700580Z .................................................................................................... 6200/9556
2020-01-25T16:01:24.0502467Z .................................................................................................... 6300/9556
2020-01-25T16:01:24.0502467Z .................................................................................................... 6300/9556
2020-01-25T16:01:27.9468642Z ...................................................................................i..ii............ 6400/9556
2020-01-25T16:01:49.3459547Z .................................................................................................... 6600/9556
2020-01-25T16:01:57.1421167Z ...........................................................i........................................ 6700/9556
2020-01-25T16:01:59.1280040Z .................................................................................................... 6800/9556
2020-01-25T16:02:01.2498416Z ..........................................................i......................................... 6900/9556
---
2020-01-25T16:03:32.3661012Z .................................................................................................... 7600/9556
2020-01-25T16:03:37.3668687Z .................................................................................................... 7700/9556
2020-01-25T16:03:43.4166676Z .................................................................................................... 7800/9556
2020-01-25T16:03:53.0301763Z .................................................................................................... 7900/9556
2020-01-25T16:03:58.4759999Z ..............iiiiiii............................................................................... 8000/9556
2020-01-25T16:04:11.6482505Z .................................................................................................... 8200/9556
2020-01-25T16:04:21.2431804Z .................................................................................................... 8300/9556
2020-01-25T16:04:33.8065789Z .................................................................................................... 8400/9556
2020-01-25T16:04:40.0613110Z .................................................................................................... 8500/9556
---
2020-01-25T16:06:47.7276721Z  finished in 6.988
2020-01-25T16:06:47.7458525Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T16:06:47.9589709Z 
2020-01-25T16:06:47.9591317Z running 169 tests
2020-01-25T16:06:50.7284756Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-25T16:06:52.7044123Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-25T16:06:52.7044685Z 
2020-01-25T16:06:52.7049331Z  finished in 4.959
2020-01-25T16:06:52.7229413Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T16:06:52.8737042Z 
---
2020-01-25T16:06:54.6637683Z  finished in 1.940
2020-01-25T16:06:54.6830907Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T16:06:54.8363429Z 
2020-01-25T16:06:54.8364770Z running 9 tests
2020-01-25T16:06:54.8365564Z iiiiiiiii
2020-01-25T16:06:54.8365902Z 
2020-01-25T16:06:54.8366100Z  finished in 0.153
2020-01-25T16:06:54.8548076Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T16:06:55.0352912Z 
---
2020-01-25T16:07:13.0837209Z  finished in 18.229
2020-01-25T16:07:13.1069200Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T16:07:13.3116086Z 
2020-01-25T16:07:13.3116468Z running 116 tests
2020-01-25T16:07:25.7044304Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-01-25T16:07:27.4072550Z ....iiii.....ii.
2020-01-25T16:07:27.4078426Z 
2020-01-25T16:07:27.4088503Z  finished in 14.301
2020-01-25T16:07:27.4090137Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T16:07:27.4090641Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-25T16:19:07.3973838Z 
2020-01-25T16:19:07.3978675Z    Doc-tests core
2020-01-25T16:19:11.5992123Z 
2020-01-25T16:19:11.5993352Z running 2443 tests
2020-01-25T16:19:19.8975642Z ......iiiii......................................................................................... 100/2443
2020-01-25T16:19:28.4562056Z ..................................................................................ii................ 200/2443
2020-01-25T16:19:47.3262892Z .................i.................................................................................. 400/2443
2020-01-25T16:19:47.3262892Z .................i.................................................................................. 400/2443
2020-01-25T16:19:55.8827392Z ..................................................................i..i..................iiii........ 500/2443
2020-01-25T16:20:10.7614166Z .................................................................................................... 700/2443
2020-01-25T16:20:18.5028282Z .................................................................................................... 800/2443
2020-01-25T16:20:26.3056728Z .................................................................................................... 900/2443
2020-01-25T16:20:34.0342414Z .................................................................................................... 1000/2443
---
2020-01-25T16:23:41.1559902Z .................................................................................................... 500/760
2020-01-25T16:23:41.1961857Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-01-25T16:23:41.1976950Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-01-25T16:23:41.1979186Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-01-25T16:23:41.1999583Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-01-25T16:23:41.5489001Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-01-25T16:23:41.5556752Z .....thread '<unnamed>.' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:.21.
2020-01-25T16:23:41.5569657Z ..thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-01-25T16:23:43.6499553Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-25T16:23:43.6501491Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-01-25T16:23:43.6504819Z ....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-01-25T16:23:43.6509948Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-01-25T16:23:52.8986004Z 
2020-01-25T16:23:52.8987731Z running 1003 tests
2020-01-25T16:24:08.4000627Z i................................................................................................... 100/1003
2020-01-25T16:24:17.4241571Z .................................................................................................... 200/1003
2020-01-25T16:24:23.8501994Z ..................i.ii.....i......i...i......i...................................................... 300/1003
2020-01-25T16:24:28.4138645Z .................................................................................................... 400/1003
2020-01-25T16:24:34.5695603Z ..........................................i..i.....................................ii............... 500/1003
2020-01-25T16:24:46.1792492Z .................................................................................................... 700/1003
2020-01-25T16:24:46.1792492Z .................................................................................................... 700/1003
2020-01-25T16:24:51.9841991Z .............................iiii................................................................... 800/1003
2020-01-25T16:25:04.4446240Z .................................................................................................... 900/1003
2020-01-25T16:25:10.9241183Z ...................................................iiii............................................. 1000/1003
2020-01-25T16:25:10.9248683Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-25T16:25:10.9248900Z 
2020-01-25T16:25:10.9249072Z  finished in 151.830
2020-01-25T16:25:10.9249973Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-25T16:41:48.3770468Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 11891) stdout ----
2020-01-25T16:41:48.3770534Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T16:41:48.3770781Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11900:18
2020-01-25T16:41:48.3770894Z    |
2020-01-25T16:41:48.3770941Z 11 | Pin::new(&mut b).resume();
2020-01-25T16:41:48.3771026Z    |
2020-01-25T16:41:48.3771026Z    |
2020-01-25T16:41:48.3771092Z help: expected the unit value `()`; create it with empty parentheses
2020-01-25T16:41:48.3771135Z    |
2020-01-25T16:41:48.3771179Z 11 | Pin::new(&mut b).resume(());
2020-01-25T16:41:48.3771268Z 
2020-01-25T16:41:48.3771311Z error: aborting due to previous error
2020-01-25T16:41:48.3771339Z 
2020-01-25T16:41:48.3771589Z For more information about this error, try `rustc --explain E0061`.
2020-01-25T16:41:48.3771589Z For more information about this error, try `rustc --explain E0061`.
2020-01-25T16:41:48.3771640Z Some expected error codes were not found: ["E0626"]
2020-01-25T16:41:48.3772062Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T16:41:48.3772315Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11919:18
2020-01-25T16:41:48.3772362Z    |
2020-01-25T16:41:48.3772362Z    |
2020-01-25T16:41:48.3772419Z 11 | Pin::new(&mut b).resume();
2020-01-25T16:41:48.3772503Z    |
2020-01-25T16:41:48.3772503Z    |
2020-01-25T16:41:48.3772548Z help: expected the unit value `()`; create it with empty parentheses
2020-01-25T16:41:48.3772609Z    |
2020-01-25T16:41:48.3772650Z 11 | Pin::new(&mut b).resume(());
2020-01-25T16:41:48.3772738Z 
2020-01-25T16:41:48.3772783Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3773028Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11915:9
2020-01-25T16:41:48.3773096Z   |
---
2020-01-25T16:41:48.3774569Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 11928) stdout ----
2020-01-25T16:41:48.3774633Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T16:41:48.3774892Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11938:18
2020-01-25T16:41:48.3774940Z    |
2020-01-25T16:41:48.3774988Z 12 | Pin::new(&mut b).resume();
2020-01-25T16:41:48.3775086Z    |
2020-01-25T16:41:48.3775086Z    |
2020-01-25T16:41:48.3775132Z help: expected the unit value `()`; create it with empty parentheses
2020-01-25T16:41:48.3775174Z    |
2020-01-25T16:41:48.3775231Z 12 | Pin::new(&mut b).resume(());
2020-01-25T16:41:48.3775301Z 
2020-01-25T16:41:48.3775345Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3775606Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11933:7
2020-01-25T16:41:48.3775651Z   |
2020-01-25T16:41:48.3775651Z   |
2020-01-25T16:41:48.3775692Z 7 |   let v = vec![1,2,3];
2020-01-25T16:41:48.3775761Z   |       ^ cannot infer type for type `{integer}`
2020-01-25T16:41:48.3775803Z   |
2020-01-25T16:41:48.3775848Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3776157Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11935:5
2020-01-25T16:41:48.3776206Z   |
2020-01-25T16:41:48.3776249Z 9 |     yield x; // ...when this yield occurs.
2020-01-25T16:41:48.3776336Z 
2020-01-25T16:41:48.3776380Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3776632Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11934:14
2020-01-25T16:41:48.3776693Z   |
2020-01-25T16:41:48.3776693Z   |
2020-01-25T16:41:48.3776915Z 8 |   for &x in &v { // <-- borrow of `v` is still in scope...
2020-01-25T16:41:48.3777023Z   |
2020-01-25T16:41:48.3777067Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3777363Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11935:5
2020-01-25T16:41:48.3777425Z   |
2020-01-25T16:41:48.3777425Z   |
2020-01-25T16:41:48.3777468Z 9 |     yield x; // ...when this yield occurs.
2020-01-25T16:41:48.3777544Z 
2020-01-25T16:41:48.3777605Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3778026Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11934:13
2020-01-25T16:41:48.3778075Z   |
2020-01-25T16:41:48.3778075Z   |
2020-01-25T16:41:48.3778328Z 8 |   for &x in &v { // <-- borrow of `v` is still in scope...
2020-01-25T16:41:48.3778428Z   |
2020-01-25T16:41:48.3778476Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3778756Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11935:5
2020-01-25T16:41:48.3778803Z   |
2020-01-25T16:41:48.3778803Z   |
2020-01-25T16:41:48.3778857Z 9 |     yield x; // ...when this yield occurs.
2020-01-25T16:41:48.3778949Z 
2020-01-25T16:41:48.3778998Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3779283Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11934:8
2020-01-25T16:41:48.3779333Z   |
2020-01-25T16:41:48.3779333Z   |
2020-01-25T16:41:48.3780792Z 8 |   for &x in &v { // <-- borrow of `v` is still in scope...
2020-01-25T16:41:48.3780920Z   |
2020-01-25T16:41:48.3780968Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3781235Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11935:5
2020-01-25T16:41:48.3781305Z   |
2020-01-25T16:41:48.3781305Z   |
2020-01-25T16:41:48.3781351Z 9 |     yield x; // ...when this yield occurs.
2020-01-25T16:41:48.3781425Z 
2020-01-25T16:41:48.3781489Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3781768Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11935:11
2020-01-25T16:41:48.3781816Z   |
2020-01-25T16:41:48.3781816Z   |
2020-01-25T16:41:48.3781879Z 9 |     yield x; // ...when this yield occurs.
2020-01-25T16:41:48.3781984Z   |
2020-01-25T16:41:48.3782047Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3782311Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11935:5
2020-01-25T16:41:48.3782360Z   |
2020-01-25T16:41:48.3782360Z   |
2020-01-25T16:41:48.3782422Z 9 |     yield x; // ...when this yield occurs.
2020-01-25T16:41:48.3782498Z 
2020-01-25T16:41:48.3782544Z error: aborting due to 6 previous errors
2020-01-25T16:41:48.3782590Z 
2020-01-25T16:41:48.3782637Z Some errors have detailed explanations: E0061, E0698.
2020-01-25T16:41:48.3782637Z Some errors have detailed explanations: E0061, E0698.
2020-01-25T16:41:48.3782887Z For more information about an error, try `rustc --explain E0061`.
2020-01-25T16:41:48.3782949Z Some expected error codes were not found: ["E0626"]
2020-01-25T16:41:48.3783446Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T16:41:48.3783740Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11954:18
2020-01-25T16:41:48.3783792Z    |
2020-01-25T16:41:48.3783792Z    |
2020-01-25T16:41:48.3783838Z 12 | Pin::new(&mut b).resume();
2020-01-25T16:41:48.3783945Z    |
2020-01-25T16:41:48.3783945Z    |
2020-01-25T16:41:48.3783995Z help: expected the unit value `()`; create it with empty parentheses
2020-01-25T16:41:48.3784040Z    |
2020-01-25T16:41:48.3784105Z 12 | Pin::new(&mut b).resume(());
2020-01-25T16:41:48.3784180Z 
2020-01-25T16:41:48.3784228Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3784572Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11949:7
2020-01-25T16:41:48.3784620Z   |
2020-01-25T16:41:48.3784620Z   |
2020-01-25T16:41:48.3784775Z 7 |   let v = vec![1,2,3];
2020-01-25T16:41:48.3784837Z   |       ^ cannot infer type for type `{integer}`
2020-01-25T16:41:48.3784879Z   |
2020-01-25T16:41:48.3784931Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3785360Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11951:5
2020-01-25T16:41:48.3785409Z   |
2020-01-25T16:41:48.3785631Z 9 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3785724Z 
2020-01-25T16:41:48.3785772Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3786037Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11950:12
2020-01-25T16:41:48.3786102Z   |
2020-01-25T16:41:48.3786102Z   |
2020-01-25T16:41:48.3786342Z 8 |   for x in v { // <-- Take ownership of the values instead!
2020-01-25T16:41:48.3786466Z   |
2020-01-25T16:41:48.3786514Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3786774Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11951:5
2020-01-25T16:41:48.3786830Z   |
2020-01-25T16:41:48.3786830Z   |
2020-01-25T16:41:48.3787067Z 9 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3787142Z 
2020-01-25T16:41:48.3787207Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3787468Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11950:7
2020-01-25T16:41:48.3787515Z   |
2020-01-25T16:41:48.3787515Z   |
2020-01-25T16:41:48.3787770Z 8 |   for x in v { // <-- Take ownership of the values instead!
2020-01-25T16:41:48.3787869Z   |
2020-01-25T16:41:48.3787915Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3788203Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11951:5
2020-01-25T16:41:48.3788250Z   |
2020-01-25T16:41:48.3788250Z   |
2020-01-25T16:41:48.3788463Z 9 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3788555Z 
2020-01-25T16:41:48.3788684Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3788948Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11951:11
2020-01-25T16:41:48.3789012Z   |
2020-01-25T16:41:48.3789012Z   |
2020-01-25T16:41:48.3789229Z 9 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3789344Z   |
2020-01-25T16:41:48.3789391Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3789651Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11951:5
2020-01-25T16:41:48.3789714Z   |
2020-01-25T16:41:48.3789714Z   |
2020-01-25T16:41:48.3789930Z 9 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3790013Z 
2020-01-25T16:41:48.3790077Z error: aborting due to 5 previous errors
2020-01-25T16:41:48.3790106Z 
2020-01-25T16:41:48.3790155Z Some errors have detailed explanations: E0061, E0698.
2020-01-25T16:41:48.3790155Z Some errors have detailed explanations: E0061, E0698.
2020-01-25T16:41:48.3790472Z For more information about an error, try `rustc --explain E0061`.
2020-01-25T16:41:48.3790692Z Couldn't compile the test.
2020-01-25T16:41:48.3791005Z ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0626 (line 11959) stdout ----
2020-01-25T16:41:48.3791085Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-25T16:41:48.3791350Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11971:18
2020-01-25T16:41:48.3791400Z    |
2020-01-25T16:41:48.3791445Z 14 | Pin::new(&mut b).resume();
2020-01-25T16:41:48.3791552Z    |
2020-01-25T16:41:48.3791552Z    |
2020-01-25T16:41:48.3791601Z help: expected the unit value `()`; create it with empty parentheses
2020-01-25T16:41:48.3791717Z    |
2020-01-25T16:41:48.3791760Z 14 | Pin::new(&mut b).resume(());
2020-01-25T16:41:48.3791835Z 
2020-01-25T16:41:48.3791898Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3792179Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11964:7
2020-01-25T16:41:48.3792228Z    |
2020-01-25T16:41:48.3792228Z    |
2020-01-25T16:41:48.3792289Z 7  |   let v = vec![1,2,3];
2020-01-25T16:41:48.3792339Z    |       ^ cannot infer type for type `{integer}`
2020-01-25T16:41:48.3792384Z    |
2020-01-25T16:41:48.3792448Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3792711Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11968:5
2020-01-25T16:41:48.3792761Z    |
2020-01-25T16:41:48.3792998Z 11 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3793077Z 
2020-01-25T16:41:48.3793134Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3793412Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11967:9
2020-01-25T16:41:48.3793462Z    |
2020-01-25T16:41:48.3793462Z    |
2020-01-25T16:41:48.3793505Z 10 |     let x = v[i]; // (*)
2020-01-25T16:41:48.3793625Z    |
2020-01-25T16:41:48.3793672Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3793951Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11968:5
2020-01-25T16:41:48.3794000Z    |
2020-01-25T16:41:48.3794000Z    |
2020-01-25T16:41:48.3794219Z 11 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3794313Z 
2020-01-25T16:41:48.3794360Z error[E0698]: type inside generator must be known in this context
2020-01-25T16:41:48.3794624Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11968:11
2020-01-25T16:41:48.3794690Z    |
2020-01-25T16:41:48.3794690Z    |
2020-01-25T16:41:48.3794918Z 11 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3795015Z    |
2020-01-25T16:41:48.3795079Z note: the type is part of the generator because of this `yield`
2020-01-25T16:41:48.3795348Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11968:5
2020-01-25T16:41:48.3795396Z    |
2020-01-25T16:41:48.3795396Z    |
2020-01-25T16:41:48.3795633Z 11 |     yield x; // <-- Now yield is OK.
2020-01-25T16:41:48.3795710Z 
2020-01-25T16:41:48.3795754Z error: aborting due to 4 previous errors
2020-01-25T16:41:48.3795800Z 
2020-01-25T16:41:48.3795847Z Some errors have detailed explanations: E0061, E0698.
---
2020-01-25T16:41:49.8162489Z   local time: Sat Jan 25 16:41:48 UTC 2020
2020-01-25T16:41:49.8162637Z   network time: Sat, 25 Jan 2020 16:41:48 GMT
2020-01-25T16:41:49.8162728Z == end clock drift check ==
2020-01-25T16:41:49.8162769Z 
2020-01-25T16:41:49.8216587Z ##[error]Bash exited with code '1'.
2020-01-25T16:41:49.8228215Z ##[section]Finishing: Run build
2020-01-25T16:41:49.8251254Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T16:41:49.8253024Z Task         : Get sources
2020-01-25T16:41:49.8253090Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T16:41:49.8253135Z Version      : 1.0.0
2020-01-25T16:41:49.8253180Z Author       : Microsoft
2020-01-25T16:41:49.8253180Z Author       : Microsoft
2020-01-25T16:41:49.8253258Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T16:41:49.8253308Z ==============================================================================
2020-01-25T16:41:50.2200877Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T16:41:50.2238684Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-25T16:41:50.2343057Z Cleaning up task key
2020-01-25T16:41:50.2343802Z Start cleaning up orphan processes.
2020-01-25T16:41:50.2441085Z Terminate orphan process: pid (3531) (python)
2020-01-25T16:41:50.2720646Z ##[section]Finishing: Finalize Job
