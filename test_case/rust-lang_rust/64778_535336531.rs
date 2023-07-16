plain
2019-09-26T03:28:46.1284444Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T03:28:46.1483646Z ##[command]git config gc.auto 0
2019-09-26T03:28:46.1564341Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T03:28:46.1631409Z ##[command]git config --get-all http.proxy
2019-09-26T03:28:46.1769900Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64778/merge:refs/remotes/pull/64778/merge
---
2019-09-26T04:27:03.4682763Z .................................................................................................... 1500/9044
2019-09-26T04:27:08.6902080Z .................................................................................................... 1600/9044
2019-09-26T04:27:19.7959085Z ..........................................................................i...............i......... 1700/9044
2019-09-26T04:27:25.8878398Z .................................................................................................... 1800/9044
2019-09-26T04:27:33.5169385Z .................................................................iiiii.............................. 1900/9044
2019-09-26T04:27:51.5060102Z .................................................................................................... 2100/9044
2019-09-26T04:27:53.7560735Z .................................................................................................... 2200/9044
2019-09-26T04:27:56.4533248Z .................................................................................................... 2300/9044
2019-09-26T04:28:04.0758713Z .................................................................................................... 2400/9044
---
2019-09-26T04:30:45.2115817Z .........................................................i..............i........................... 4700/9044
2019-09-26T04:30:53.4056342Z .................................................................................................... 4800/9044
2019-09-26T04:31:01.0325940Z .................................................................................................... 4900/9044
2019-09-26T04:31:07.9228212Z .................................................................................................... 5000/9044
2019-09-26T04:31:16.5739936Z ...........................................ii.ii.................................................... 5100/9044
2019-09-26T04:31:25.5342599Z .................................................................................................... 5300/9044
2019-09-26T04:31:34.7732921Z .................................................................................................... 5400/9044
2019-09-26T04:31:41.3549386Z ........i........................................................................................... 5500/9044
2019-09-26T04:31:46.4127346Z .................................................................................................... 5600/9044
2019-09-26T04:31:46.4127346Z .................................................................................................... 5600/9044
2019-09-26T04:31:57.0713790Z .................................................................................................... 5700/9044
2019-09-26T04:32:09.0203052Z ...ii...i..ii...........i........................................................................... 5800/9044
2019-09-26T04:32:28.9539029Z .................................................................................................... 6000/9044
2019-09-26T04:32:33.3784737Z .................................................................................................... 6100/9044
2019-09-26T04:32:33.3784737Z .................................................................................................... 6100/9044
2019-09-26T04:32:46.4591459Z .....i..ii.......................................................................................... 6200/9044
2019-09-26T04:33:03.0690719Z .................................................................i.................................. 6400/9044
2019-09-26T04:33:04.9049700Z .................................................................................................... 6500/9044
2019-09-26T04:33:07.1461890Z .....................................i.............................................................. 6600/9044
2019-09-26T04:33:10.7848767Z .................................................................................................... 6700/9044
---
2019-09-26T04:33:59.4322503Z .................................................................................................... 7200/9044
2019-09-26T04:34:04.1788090Z .................................................................................................... 7300/9044
2019-09-26T04:34:10.7376697Z .................................................................................................... 7400/9044
2019-09-26T04:34:20.1295935Z .................................................................................................... 7500/9044
2019-09-26T04:34:29.4315430Z .........................................................................................ii......i.. 7600/9044
2019-09-26T04:34:39.3461406Z .................................................................................................... 7800/9044
2019-09-26T04:34:54.7642976Z .................................................................................................... 7900/9044
2019-09-26T04:35:02.1413045Z .................................................................................................... 8000/9044
2019-09-26T04:35:11.6751024Z .................................................................................................... 8100/9044
---
2019-09-26T04:37:13.6505902Z  finished in 4.813
2019-09-26T04:37:13.6679285Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:37:13.8154242Z 
2019-09-26T04:37:13.8155794Z running 150 tests
2019-09-26T04:37:16.7540246Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-26T04:37:18.5151234Z ..iiii..............i.........iii.i.......ii......
2019-09-26T04:37:18.5151650Z 
2019-09-26T04:37:18.5158530Z  finished in 4.848
2019-09-26T04:37:18.5316292Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:37:18.6777546Z 
---
2019-09-26T04:37:20.5025852Z  finished in 1.971
2019-09-26T04:37:20.5187397Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:37:20.6701936Z 
2019-09-26T04:37:20.6702051Z running 9 tests
2019-09-26T04:37:20.6702990Z iiiiiiiii
2019-09-26T04:37:20.6703290Z 
2019-09-26T04:37:20.6707384Z  finished in 0.151
2019-09-26T04:37:20.6876347Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:37:20.8506682Z 
---
2019-09-26T04:37:37.8361434Z  finished in 17.147
2019-09-26T04:37:37.8523627Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:37:38.0000534Z 
2019-09-26T04:37:38.0000875Z running 123 tests
2019-09-26T04:38:00.2462496Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-26T04:38:04.5556744Z i.i.i......iii.i.....ii
2019-09-26T04:38:04.5557823Z 
2019-09-26T04:38:04.5558043Z  finished in 26.703
2019-09-26T04:38:04.5568757Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:38:04.5569733Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-26T04:50:01.8074027Z 
2019-09-26T04:50:01.8084490Z    Doc-tests core
2019-09-26T04:50:06.9868149Z 
2019-09-26T04:50:06.9869068Z running 2405 tests
2019-09-26T04:50:17.9370150Z ......iiiii......................................................................................... 100/2405
2019-09-26T04:50:28.3098475Z ...............................................................................ii................... 200/2405
2019-09-26T04:50:53.6218381Z .i.................................................................................................. 400/2405
2019-09-26T04:50:53.6218381Z .i.................................................................................................. 400/2405
2019-09-26T04:51:03.6325834Z ................................................i..i.................iiii........................... 500/2405
2019-09-26T04:51:23.9100256Z .................................................................................................... 700/2405
2019-09-26T04:51:34.3531221Z .................................................................................................... 800/2405
2019-09-26T04:51:44.7417561Z .................................................................................................... 900/2405
2019-09-26T04:51:54.9506452Z .................................................................................................... 1000/2405
---
2019-09-26T04:55:54.8657266Z .............................................. 300/763
2019-09-26T04:55:54.8707257Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-09-26T04:55:54.9167055Z .................................................................................................... 400/763
2019-09-26T04:55:56.9924854Z .................................................................................................... 500/763
2019-09-26T04:55:57.0132289Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.0148142Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.0166194Z .<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError..', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.0177991Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.2542016Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.2573986Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.2593062Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-26T04:55:57.2617736Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-26T04:55:59.3276835Z ......................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-09-26T04:55:59.3282711Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-09-26T04:55:59.3294210Z .....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-09-26T04:55:59.3298985Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-09-26T04:56:08.7975226Z 
2019-09-26T04:56:08.7975534Z running 992 tests
2019-09-26T04:56:31.5908565Z i................................................................................................... 100/992
2019-09-26T04:56:43.9231046Z .................................................................................................... 200/992
2019-09-26T04:56:52.7909072Z .................iii......i......i...i......i....................................................... 300/992
2019-09-26T04:56:58.8724706Z .................................................................................................... 400/992
2019-09-26T04:57:07.0667581Z ...................................i..i.................................ii.......................... 500/992
2019-09-26T04:57:23.0386860Z .................................................................................................... 700/992
2019-09-26T04:57:23.0386860Z .................................................................................................... 700/992
2019-09-26T04:57:31.9735469Z ..................iiii.............................................................................. 800/992
2019-09-26T04:57:47.8910962Z .................................................................................................... 900/992
2019-09-26T04:57:56.0234157Z ........................................iiii................................................
2019-09-26T04:57:56.0236399Z 
2019-09-26T04:57:56.0325857Z  finished in 203.739
2019-09-26T04:57:56.0342534Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T04:57:56.2551366Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-26T05:00:09.5431392Z  finished in 4.248
2019-09-26T05:00:09.5449854Z Testing rustc_index stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-26T05:00:09.7156290Z    Compiling rustc_index v0.0.0 (/checkout/src/librustc_index)
2019-09-26T05:00:09.8128511Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8129141Z    --> src/librustc_index/bit_set/tests.rs:288:3
2019-09-26T05:00:09.8129581Z 288 | #[bench]
2019-09-26T05:00:09.8129827Z     |   ^^^^^
2019-09-26T05:00:09.8130017Z     |
2019-09-26T05:00:09.8130017Z     |
2019-09-26T05:00:09.8130264Z     = note: `#[deny(soft_unstable)]` on by default
2019-09-26T05:00:09.8130617Z     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-09-26T05:00:09.8133615Z     = note: for more information, see issue #64266 <***/issues/64266>
2019-09-26T05:00:09.8134002Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8134002Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8134669Z    --> src/librustc_index/bit_set/tests.rs:303:3
2019-09-26T05:00:09.8135194Z 303 | #[bench]
2019-09-26T05:00:09.8135437Z     |   ^^^^^
2019-09-26T05:00:09.8135665Z     |
2019-09-26T05:00:09.8135665Z     |
2019-09-26T05:00:09.8136010Z     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-09-26T05:00:09.8143325Z     = note: for more information, see issue #64266 <***/issues/64266>
2019-09-26T05:00:09.8144724Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8144724Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8145219Z    --> src/librustc_index/bit_set/tests.rs:321:3
2019-09-26T05:00:09.8145723Z 321 | #[bench]
2019-09-26T05:00:09.8145969Z     |   ^^^^^
2019-09-26T05:00:09.8146198Z     |
2019-09-26T05:00:09.8146198Z     |
2019-09-26T05:00:09.8146552Z     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-09-26T05:00:09.8147178Z     = note: for more information, see issue #64266 <***/issues/64266>
2019-09-26T05:00:09.8147710Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8147710Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8148304Z    --> src/librustc_index/bit_set/tests.rs:339:3
2019-09-26T05:00:09.8148918Z 339 | #[bench]
2019-09-26T05:00:09.8149168Z     |   ^^^^^
2019-09-26T05:00:09.8149359Z     |
2019-09-26T05:00:09.8149359Z     |
2019-09-26T05:00:09.8149679Z     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-09-26T05:00:09.8150050Z     = note: for more information, see issue #64266 <***/issues/64266>
2019-09-26T05:00:09.8150402Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8150402Z error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-09-26T05:00:09.8150661Z    --> src/librustc_index/bit_set/tests.rs:354:3
2019-09-26T05:00:09.8151104Z 354 | #[bench]
2019-09-26T05:00:09.8151330Z     |   ^^^^^
2019-09-26T05:00:09.8151705Z     |
2019-09-26T05:00:09.8151705Z     |
2019-09-26T05:00:09.8152018Z     = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
2019-09-26T05:00:09.8152357Z     = note: for more information, see issue #64266 <***/issues/64266>
2019-09-26T05:00:09.8303601Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8303601Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8310858Z  --> src/librustc_index/bit_set/tests.rs:3:1
2019-09-26T05:00:09.8323820Z 3 | extern crate test;
2019-09-26T05:00:09.8325066Z   | ^^^^^^^^^^^^^^^^^^
2019-09-26T05:00:09.8325691Z   |
2019-09-26T05:00:09.8325691Z   |
2019-09-26T05:00:09.8326434Z   = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8328147Z 
2019-09-26T05:00:09.8328804Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8328804Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8329262Z  --> src/librustc_index/bit_set/tests.rs:4:5
2019-09-26T05:00:09.8330129Z 4 | use test::Bencher;
2019-09-26T05:00:09.8330551Z   |     ^^^^^^^^^^^^^
2019-09-26T05:00:09.8330930Z   |
2019-09-26T05:00:09.8330930Z   |
2019-09-26T05:00:09.8331479Z   = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8332161Z 
2019-09-26T05:00:09.8332557Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8332557Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8333204Z    --> src/librustc_index/bit_set/tests.rs:289:47
2019-09-26T05:00:09.8333723Z     |
2019-09-26T05:00:09.8334466Z 289 | fn union_hybrid_sparse_empty_to_dense(b: &mut Bencher) {
2019-09-26T05:00:09.8335713Z     |
2019-09-26T05:00:09.8335713Z     |
2019-09-26T05:00:09.8336264Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8336935Z 
2019-09-26T05:00:09.8337348Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8337348Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8337794Z    --> src/librustc_index/bit_set/tests.rs:304:46
2019-09-26T05:00:09.8338185Z     |
2019-09-26T05:00:09.8338668Z 304 | fn union_hybrid_sparse_full_to_dense(b: &mut Bencher) {
2019-09-26T05:00:09.8339549Z     |
2019-09-26T05:00:09.8339549Z     |
2019-09-26T05:00:09.8340055Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8340726Z 
2019-09-26T05:00:09.8341115Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8341115Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8341563Z    --> src/librustc_index/bit_set/tests.rs:322:48
2019-09-26T05:00:09.8341958Z     |
2019-09-26T05:00:09.8342428Z 322 | fn union_hybrid_sparse_domain_to_dense(b: &mut Bencher) {
2019-09-26T05:00:09.8343321Z     |
2019-09-26T05:00:09.8343321Z     |
2019-09-26T05:00:09.8343842Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8345188Z 
2019-09-26T05:00:09.8345695Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8345695Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8346138Z    --> src/librustc_index/bit_set/tests.rs:340:51
2019-09-26T05:00:09.8346561Z     |
2019-09-26T05:00:09.8347040Z 340 | fn union_hybrid_sparse_empty_small_domain(b: &mut Bencher) {
2019-09-26T05:00:09.8348130Z     |
2019-09-26T05:00:09.8348130Z     |
2019-09-26T05:00:09.8348678Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8349358Z 
2019-09-26T05:00:09.8350212Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8350212Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:09.8350745Z    --> src/librustc_index/bit_set/tests.rs:355:50
2019-09-26T05:00:09.8351133Z     |
2019-09-26T05:00:09.8351794Z 355 | fn union_hybrid_sparse_full_small_domain(b: &mut Bencher) {
2019-09-26T05:00:09.8352990Z     |
2019-09-26T05:00:09.8352990Z     |
2019-09-26T05:00:09.8353546Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:09.8354646Z 
2019-09-26T05:00:10.2152199Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2152199Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2153152Z    --> src/librustc_index/bit_set/tests.rs:295:7
2019-09-26T05:00:10.2154205Z 295 |     b.iter(|| {
2019-09-26T05:00:10.2155350Z     |       ^^^^
2019-09-26T05:00:10.2155938Z     |
2019-09-26T05:00:10.2155938Z     |
2019-09-26T05:00:10.2156701Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:10.2157631Z 
2019-09-26T05:00:10.2175328Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2175328Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2176163Z    --> src/librustc_index/bit_set/tests.rs:313:7
2019-09-26T05:00:10.2177323Z 313 |     b.iter(|| {
2019-09-26T05:00:10.2177900Z     |       ^^^^
2019-09-26T05:00:10.2178419Z     |
2019-09-26T05:00:10.2178419Z     |
2019-09-26T05:00:10.2179113Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:10.2180072Z 
2019-09-26T05:00:10.2217332Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2217332Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2218244Z    --> src/librustc_index/bit_set/tests.rs:331:7
2019-09-26T05:00:10.2219589Z 331 |     b.iter(|| {
2019-09-26T05:00:10.2220117Z     |       ^^^^
2019-09-26T05:00:10.2221336Z     |
2019-09-26T05:00:10.2221336Z     |
2019-09-26T05:00:10.2222141Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:10.2223064Z 
2019-09-26T05:00:10.2236143Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2236143Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2236796Z    --> src/librustc_index/bit_set/tests.rs:346:7
2019-09-26T05:00:10.2237636Z 346 |     b.iter(|| {
2019-09-26T05:00:10.2238072Z     |       ^^^^
2019-09-26T05:00:10.2238446Z     |
2019-09-26T05:00:10.2238446Z     |
2019-09-26T05:00:10.2239002Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:10.2239937Z 
2019-09-26T05:00:10.2255958Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2255958Z error[E0658]: use of unstable library feature 'test'
2019-09-26T05:00:10.2256839Z    --> src/librustc_index/bit_set/tests.rs:364:7
2019-09-26T05:00:10.2257915Z 364 |     b.iter(|| {
2019-09-26T05:00:10.2258316Z     |       ^^^^
2019-09-26T05:00:10.2258885Z     |
2019-09-26T05:00:10.2258885Z     |
2019-09-26T05:00:10.2259425Z     = note: for more information, see ***/issues/50297
2019-09-26T05:00:10.2260210Z 
2019-09-26T05:00:10.2268297Z error: aborting due to 17 previous errors
2019-09-26T05:00:10.2268980Z 
2019-09-26T05:00:10.2269512Z For more information about this error, try `rustc --explain E0658`.
2019-09-26T05:00:10.2269512Z For more information about this error, try `rustc --explain E0658`.
2019-09-26T05:00:10.2440345Z error: could not compile `rustc_index`.
2019-09-26T05:00:10.2440725Z 
2019-09-26T05:00:10.2441447Z To learn more, run the command again with --verbose.
2019-09-26T05:00:10.2460891Z 
2019-09-26T05:00:10.2468681Z 
2019-09-26T05:00:10.2470346Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_index" "--" "--quiet"
2019-09-26T05:00:10.2471232Z 
2019-09-26T05:00:10.2471392Z 
2019-09-26T05:00:10.2471800Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-26T05:00:10.2472013Z Build completed unsuccessfully in 1:24:21
2019-09-26T05:00:10.2472013Z Build completed unsuccessfully in 1:24:21
2019-09-26T05:00:10.2520129Z == clock drift check ==
2019-09-26T05:00:10.2533275Z   local time: Thu Sep 26 05:00:10 UTC 2019
2019-09-26T05:00:10.4030340Z   network time: Thu, 26 Sep 2019 05:00:10 GMT
2019-09-26T05:00:10.4031238Z == end clock drift check ==
2019-09-26T05:00:11.2672742Z ##[error]Bash exited with code '1'.
2019-09-26T05:00:11.2745778Z ##[section]Starting: Checkout
2019-09-26T05:00:11.2747812Z ==============================================================================
2019-09-26T05:00:11.2747888Z Task         : Get sources
2019-09-26T05:00:11.2747936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
