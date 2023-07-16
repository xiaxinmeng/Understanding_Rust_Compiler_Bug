plain
2019-11-12T22:32:32.6439114Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T22:32:32.6630682Z ##[command]git config gc.auto 0
2019-11-12T22:32:32.6716065Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T22:32:33.6643606Z ##[command]git config --get-all http.proxy
2019-11-12T22:32:33.6648861Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66314/merge:refs/remotes/pull/66314/merge
---
2019-11-12T23:32:10.6280341Z .................................................................................................... 1500/9234
2019-11-12T23:32:16.9507230Z .................................................................................................... 1600/9234
2019-11-12T23:32:26.5038794Z .................................................................................................... 1700/9234
2019-11-12T23:32:35.2952625Z ...i................................................................................................ 1800/9234
2019-11-12T23:32:42.1635039Z .......................................................................................iiiii........ 1900/9234
2019-11-12T23:33:03.7809283Z .................................................................................................... 2100/9234
2019-11-12T23:33:06.1861163Z .................................................................................................... 2200/9234
2019-11-12T23:33:08.6904262Z .................................................................................................... 2300/9234
2019-11-12T23:33:18.3820201Z .................................................................................................... 2400/9234
---
2019-11-12T23:36:14.8339327Z ....................................................................................i............... 4700/9234
2019-11-12T23:36:21.9573905Z i................................................................................................... 4800/9234
2019-11-12T23:36:31.3560573Z .................................................................................................... 4900/9234
2019-11-12T23:36:36.7499198Z .................................................................................................... 5000/9234
2019-11-12T23:36:48.2805615Z .......................................................................................ii.ii........ 5100/9234
2019-11-12T23:36:56.9251675Z ......................i............................................................................. 5300/9234
2019-11-12T23:37:05.2817738Z .................................................................................................... 5400/9234
2019-11-12T23:37:14.0976926Z .....................................................................i.............................. 5500/9234
2019-11-12T23:37:21.6574234Z .................................................................................................... 5600/9234
2019-11-12T23:37:21.6574234Z .................................................................................................... 5600/9234
2019-11-12T23:37:29.3095898Z .................................................................................................... 5700/9234
2019-11-12T23:37:39.4730900Z .......................................................ii...i..ii...........i....................... 5800/9234
2019-11-12T23:38:02.5594338Z .................................................................................................... 6000/9234
2019-11-12T23:38:09.7422184Z .................................................................................................... 6100/9234
2019-11-12T23:38:09.7422184Z .................................................................................................... 6100/9234
2019-11-12T23:38:14.8320059Z ..........................................................................i..ii..................... 6200/9234
2019-11-12T23:38:44.7148653Z .................................................................................................... 6400/9234
2019-11-12T23:38:47.0220407Z ..........................................i......................................................... 6500/9234
2019-11-12T23:38:49.3089179Z .................................................................................................... 6600/9234
2019-11-12T23:38:51.7414591Z ..........................i......................................................................... 6700/9234
---
2019-11-12T23:44:09.5601243Z  finished in 5.807
2019-11-12T23:44:09.5800486Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T23:44:09.7465067Z 
2019-11-12T23:44:09.7468189Z running 156 tests
2019-11-12T23:44:12.6712043Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-12T23:44:14.5914527Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-12T23:44:14.5915146Z 
2019-11-12T23:44:14.5919984Z  finished in 5.012
2019-11-12T23:44:14.6130939Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T23:44:14.7755010Z 
---
2019-11-12T23:44:16.7414438Z  finished in 2.129
2019-11-12T23:44:16.7596962Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T23:44:16.9226426Z 
2019-11-12T23:44:16.9227855Z running 9 tests
2019-11-12T23:44:16.9228904Z iiiiiiiii
2019-11-12T23:44:16.9229268Z 
2019-11-12T23:44:16.9229345Z  finished in 0.162
2019-11-12T23:44:16.9410037Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T23:44:17.1074318Z 
---
2019-11-12T23:44:36.6659968Z  finished in 19.724
2019-11-12T23:44:36.6856202Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T23:44:36.8557599Z 
2019-11-12T23:44:36.8558191Z running 123 tests
2019-11-12T23:45:00.9578385Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-12T23:45:05.8201194Z i.i.i......iii.i.....ii
2019-11-12T23:45:05.8203452Z 
2019-11-12T23:45:05.8204543Z  finished in 29.134
2019-11-12T23:45:05.8212757Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T23:45:05.8213646Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-12T23:57:00.3472878Z 
2019-11-12T23:57:00.3473620Z    Doc-tests core
2019-11-12T23:57:05.2785934Z 
2019-11-12T23:57:15.9586357Z running 2418 tests
2019-11-12T23:57:15.9586547Z ......iiiii......................................................................................... 100/2418
2019-11-12T23:57:26.5326836Z ................................................................................ii.................. 200/2418
2019-11-12T23:57:51.0384846Z ..i................................................................................................. 400/2418
2019-11-12T23:57:51.0384846Z ..i................................................................................................. 400/2418
2019-11-12T23:58:01.2613522Z ..................................................i..i.................iiii......................... 500/2418
2019-11-12T23:58:20.6262003Z .................................................................................................... 700/2418
2019-11-12T23:58:30.7311007Z .................................................................................................... 800/2418
2019-11-12T23:58:40.8398896Z .................................................................................................... 900/2418
2019-11-12T23:58:50.8807421Z .................................................................................................... 1000/2418
---
2019-11-13T00:03:02.1770976Z 
2019-11-13T00:03:02.1777371Z running 1000 tests
2019-11-13T00:03:23.2343893Z i................................................................................................... 100/1000
2019-11-13T00:03:34.8060078Z .................................................................................................... 200/1000
2019-11-13T00:03:43.0094302Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-13T00:03:48.3971887Z .................................................................................................... 400/1000
2019-11-13T00:03:55.9561225Z ...........................................i..i.................................ii.................. 500/1000
2019-11-13T00:04:10.1215833Z .................................................................................................... 700/1000
2019-11-13T00:04:10.1215833Z .................................................................................................... 700/1000
2019-11-13T00:04:17.6726336Z ..........................iiii...................................................................... 800/1000
2019-11-13T00:04:33.2515372Z .................................................................................................... 900/1000
2019-11-13T00:04:40.9504958Z ................................................iiii................................................ 1000/1000
2019-11-13T00:04:40.9511157Z test result: ok. 980 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-13T00:04:40.9511222Z 
2019-11-13T00:04:40.9621815Z  finished in 194.423
2019-11-13T00:04:40.9636331Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-13T00:21:27.9584870Z Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
2019-11-13T00:21:27.9585824Z    Compiling same-file v1.0.4
2019-11-13T00:21:27.9586250Z    Compiling walkdir v2.2.7
2019-11-13T00:21:28.9573811Z    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
2019-11-13T00:21:30.0011108Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0001.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0012521Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:8:8
2019-11-13T00:21:30.0013051Z   |
2019-11-13T00:21:30.0013607Z 8 | E0001: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0001.md"),
2019-11-13T00:21:30.0018197Z 
2019-11-13T00:21:30.0018197Z 
2019-11-13T00:21:30.0024566Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0002.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0025114Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:9:8
2019-11-13T00:21:30.0025540Z   |
2019-11-13T00:21:30.0026020Z 9 | E0002: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0002.md"),
2019-11-13T00:21:30.0030095Z 
2019-11-13T00:21:30.0030095Z 
2019-11-13T00:21:30.0036355Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0004.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0036750Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:10:8
2019-11-13T00:21:30.0036980Z    |
2019-11-13T00:21:30.0037287Z 10 | E0004: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0004.md"),
2019-11-13T00:21:30.0042942Z 
2019-11-13T00:21:30.0042942Z 
2019-11-13T00:21:30.0047748Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0005.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0048203Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:11:8
2019-11-13T00:21:30.0048485Z    |
2019-11-13T00:21:30.0048831Z 11 | E0005: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0005.md"),
2019-11-13T00:21:30.0059301Z 
2019-11-13T00:21:30.0059301Z 
2019-11-13T00:21:30.0059955Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0007.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0060374Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:12:8
2019-11-13T00:21:30.0060773Z    |
2019-11-13T00:21:30.0061296Z 12 | E0007: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0007.md"),
2019-11-13T00:21:30.0062597Z 
2019-11-13T00:21:30.0062597Z 
2019-11-13T00:21:30.0063261Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0009.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0063759Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:13:8
2019-11-13T00:21:30.0064011Z    |
2019-11-13T00:21:30.0064397Z 13 | E0009: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0009.md"),
2019-11-13T00:21:30.0064824Z 
2019-11-13T00:21:30.0064824Z 
2019-11-13T00:21:30.0095724Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0010.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0096226Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:14:8
2019-11-13T00:21:30.0096493Z    |
2019-11-13T00:21:30.0097000Z 14 | E0010: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0010.md"),
2019-11-13T00:21:30.0097410Z 
2019-11-13T00:21:30.0097410Z 
2019-11-13T00:21:30.0097882Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0013.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0098446Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:15:8
2019-11-13T00:21:30.0098667Z    |
2019-11-13T00:21:30.0098979Z 15 | E0013: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0013.md"),
2019-11-13T00:21:30.0099371Z 
2019-11-13T00:21:30.0099371Z 
2019-11-13T00:21:30.0100019Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0014.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0100427Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:16:8
2019-11-13T00:21:30.0100653Z    |
2019-11-13T00:21:30.0100996Z 16 | E0014: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0014.md"),
2019-11-13T00:21:30.0101964Z 
2019-11-13T00:21:30.0101964Z 
2019-11-13T00:21:30.0102646Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0015.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0103123Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:17:8
2019-11-13T00:21:30.0103373Z    |
2019-11-13T00:21:30.0103760Z 17 | E0015: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0015.md"),
2019-11-13T00:21:30.0104187Z 
2019-11-13T00:21:30.0104187Z 
2019-11-13T00:21:30.0104724Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0017.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0105162Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:18:8
2019-11-13T00:21:30.0105579Z    |
2019-11-13T00:21:30.0106073Z 18 | E0017: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0017.md"),
2019-11-13T00:21:30.0106474Z 
2019-11-13T00:21:30.0106474Z 
2019-11-13T00:21:30.0107100Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0019.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0107477Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:19:8
2019-11-13T00:21:30.0107714Z    |
2019-11-13T00:21:30.0108032Z 19 | E0019: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0019.md"),
2019-11-13T00:21:30.0108423Z 
2019-11-13T00:21:30.0108423Z 
2019-11-13T00:21:30.0108877Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0023.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0109272Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:20:8
2019-11-13T00:21:30.0109488Z    |
2019-11-13T00:21:30.0109802Z 20 | E0023: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0023.md"),
2019-11-13T00:21:30.0110295Z 
2019-11-13T00:21:30.0110295Z 
2019-11-13T00:21:30.0110982Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0025.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0111427Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:21:8
2019-11-13T00:21:30.0111823Z    |
2019-11-13T00:21:30.0112889Z 21 | E0025: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0025.md"),
2019-11-13T00:21:30.0113440Z 
2019-11-13T00:21:30.0113440Z 
2019-11-13T00:21:30.0113981Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0026.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0114946Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:22:8
2019-11-13T00:21:30.0115203Z    |
2019-11-13T00:21:30.0115585Z 22 | E0026: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0026.md"),
2019-11-13T00:21:30.0116010Z 
2019-11-13T00:21:30.0116010Z 
2019-11-13T00:21:30.0116545Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0027.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0116983Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:23:8
2019-11-13T00:21:30.0117250Z    |
2019-11-13T00:21:30.0117616Z 23 | E0027: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0027.md"),
2019-11-13T00:21:30.0118205Z 
2019-11-13T00:21:30.0118205Z 
2019-11-13T00:21:30.0118707Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0029.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0119122Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:24:8
2019-11-13T00:21:30.0119588Z    |
2019-11-13T00:21:30.0120074Z 24 | E0029: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0029.md"),
2019-11-13T00:21:30.0120763Z 
2019-11-13T00:21:30.0120763Z 
2019-11-13T00:21:30.0121282Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0030.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0121894Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:25:8
2019-11-13T00:21:30.0122575Z    |
2019-11-13T00:21:30.0122947Z 25 | E0030: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0030.md"),
2019-11-13T00:21:30.0123394Z 
2019-11-13T00:21:30.0123394Z 
2019-11-13T00:21:30.0123913Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0033.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0124367Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:26:8
2019-11-13T00:21:30.0124614Z    |
2019-11-13T00:21:30.0124993Z 26 | E0033: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0033.md"),
2019-11-13T00:21:30.0125622Z 
2019-11-13T00:21:30.0125622Z 
2019-11-13T00:21:30.0126108Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0034.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0126535Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:27:8
2019-11-13T00:21:30.0126779Z    |
2019-11-13T00:21:30.0127150Z 27 | E0034: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0034.md"),
2019-11-13T00:21:30.0127557Z 
2019-11-13T00:21:30.0127557Z 
2019-11-13T00:21:30.0128064Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0038.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0128477Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:28:8
2019-11-13T00:21:30.0128736Z    |
2019-11-13T00:21:30.0129088Z 28 | E0038: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0038.md"),
2019-11-13T00:21:30.0129631Z 
2019-11-13T00:21:30.0129631Z 
2019-11-13T00:21:30.0130193Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0040.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0130640Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:29:8
2019-11-13T00:21:30.0130901Z    |
2019-11-13T00:21:30.0131250Z 29 | E0040: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0040.md"),
2019-11-13T00:21:30.0131847Z 
2019-11-13T00:21:30.0131847Z 
2019-11-13T00:21:30.0132726Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0044.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0133190Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:30:8
2019-11-13T00:21:30.0133445Z    |
2019-11-13T00:21:30.0133808Z 30 | E0044: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0044.md"),
2019-11-13T00:21:30.0134254Z 
2019-11-13T00:21:30.0134254Z 
2019-11-13T00:21:30.0134772Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0045.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0135226Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:31:8
2019-11-13T00:21:30.0135908Z    |
2019-11-13T00:21:30.0136231Z 31 | E0045: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0045.md"),
2019-11-13T00:21:30.0136625Z 
2019-11-13T00:21:30.0136625Z 
2019-11-13T00:21:30.0137074Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0046.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0137472Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:32:8
2019-11-13T00:21:30.0137687Z    |
2019-11-13T00:21:30.0138021Z 32 | E0046: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0046.md"),
2019-11-13T00:21:30.0138526Z 
2019-11-13T00:21:30.0138526Z 
2019-11-13T00:21:30.0139061Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0049.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0139472Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:33:8
2019-11-13T00:21:30.0139705Z    |
2019-11-13T00:21:30.0140023Z 33 | E0049: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0049.md"),
2019-11-13T00:21:30.0140406Z 
2019-11-13T00:21:30.0140406Z 
2019-11-13T00:21:30.0140860Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0050.md: No such file or directory (os error 2)
2019-11-13T00:21:30.0141237Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:34:8
2019-11-13T00:21:30.0141471Z    |
2019-11-13T00:21:30.0141785Z 34 | E0050: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0050.md"),
2019-11-13T00:21:30.0142620Z 
2019-11-13T00:21:30.0142620Z 
2019-11-13T00:21:30.9586711Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0053.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9595598Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:35:8
2019-11-13T00:21:30.9596582Z    |
2019-11-13T00:21:30.9597442Z 35 | E0053: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0053.md"),
2019-11-13T00:21:30.9598294Z 
2019-11-13T00:21:30.9598294Z 
2019-11-13T00:21:30.9598841Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0054.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9599363Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:36:8
2019-11-13T00:21:30.9599715Z    |
2019-11-13T00:21:30.9600434Z 36 | E0054: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0054.md"),
2019-11-13T00:21:30.9601409Z 
2019-11-13T00:21:30.9601409Z 
2019-11-13T00:21:30.9602437Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0055.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9603202Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:37:8
2019-11-13T00:21:30.9603728Z    |
2019-11-13T00:21:30.9604324Z 37 | E0055: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0055.md"),
2019-11-13T00:21:30.9605150Z 
2019-11-13T00:21:30.9605150Z 
2019-11-13T00:21:30.9606106Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0057.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9607028Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:38:8
2019-11-13T00:21:30.9608591Z    |
2019-11-13T00:21:30.9609573Z 38 | E0057: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0057.md"),
2019-11-13T00:21:30.9611392Z 
2019-11-13T00:21:30.9611392Z 
2019-11-13T00:21:30.9612062Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0059.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9612910Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:39:8
2019-11-13T00:21:30.9613345Z    |
2019-11-13T00:21:30.9614225Z 39 | E0059: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0059.md"),
2019-11-13T00:21:30.9615007Z 
2019-11-13T00:21:30.9615007Z 
2019-11-13T00:21:30.9616270Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0060.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9616830Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:40:8
2019-11-13T00:21:30.9617226Z    |
2019-11-13T00:21:30.9617867Z 40 | E0060: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0060.md"),
2019-11-13T00:21:30.9623156Z 
2019-11-13T00:21:30.9623156Z 
2019-11-13T00:21:30.9623840Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0061.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9624290Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:41:8
2019-11-13T00:21:30.9624563Z    |
2019-11-13T00:21:30.9624956Z 41 | E0061: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0061.md"),
2019-11-13T00:21:30.9625404Z 
2019-11-13T00:21:30.9625404Z 
2019-11-13T00:21:30.9625925Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0062.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9626463Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:42:8
2019-11-13T00:21:30.9626666Z    |
2019-11-13T00:21:30.9626969Z 42 | E0062: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0062.md"),
2019-11-13T00:21:30.9627324Z 
2019-11-13T00:21:30.9627324Z 
2019-11-13T00:21:30.9627733Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0063.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9628097Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:43:8
2019-11-13T00:21:30.9628472Z    |
2019-11-13T00:21:30.9628809Z 43 | E0063: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0063.md"),
2019-11-13T00:21:30.9629335Z 
2019-11-13T00:21:30.9629335Z 
2019-11-13T00:21:30.9630022Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0067.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9630379Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:44:8
2019-11-13T00:21:30.9630602Z    |
2019-11-13T00:21:30.9631033Z 44 | E0067: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0067.md"),
2019-11-13T00:21:30.9631614Z 
2019-11-13T00:21:30.9631614Z 
2019-11-13T00:21:30.9632058Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0069.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9632495Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:45:8
2019-11-13T00:21:30.9632883Z    |
2019-11-13T00:21:30.9633290Z 45 | E0069: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0069.md"),
2019-11-13T00:21:30.9633716Z 
2019-11-13T00:21:30.9633716Z 
2019-11-13T00:21:30.9634248Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0070.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9634688Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:46:8
2019-11-13T00:21:30.9634955Z    |
2019-11-13T00:21:30.9635339Z 46 | E0070: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0070.md"),
2019-11-13T00:21:30.9635784Z 
2019-11-13T00:21:30.9635784Z 
2019-11-13T00:21:30.9636405Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0071.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9636773Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:47:8
2019-11-13T00:21:30.9636979Z    |
2019-11-13T00:21:30.9637283Z 47 | E0071: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0071.md"),
2019-11-13T00:21:30.9637645Z 
2019-11-13T00:21:30.9637645Z 
2019-11-13T00:21:30.9638054Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0072.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9638417Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:48:8
2019-11-13T00:21:30.9638618Z    |
2019-11-13T00:21:30.9639053Z 48 | E0072: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0072.md"),
2019-11-13T00:21:30.9639460Z 
2019-11-13T00:21:30.9639460Z 
2019-11-13T00:21:30.9639916Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0073.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9640448Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:49:8
2019-11-13T00:21:30.9640841Z    |
2019-11-13T00:21:30.9641151Z 49 | E0073: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0073.md"),
2019-11-13T00:21:30.9641506Z 
2019-11-13T00:21:30.9641506Z 
2019-11-13T00:21:30.9641915Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0074.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9642355Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:50:8
2019-11-13T00:21:30.9643272Z    |
2019-11-13T00:21:30.9643790Z 50 | E0074: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0074.md"),
2019-11-13T00:21:30.9644240Z 
2019-11-13T00:21:30.9644240Z 
2019-11-13T00:21:30.9644783Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0075.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9645232Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:51:8
2019-11-13T00:21:30.9645505Z    |
2019-11-13T00:21:30.9646013Z 51 | E0075: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0075.md"),
2019-11-13T00:21:30.9646551Z 
2019-11-13T00:21:30.9646551Z 
2019-11-13T00:21:30.9646977Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0076.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9647351Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:52:8
2019-11-13T00:21:30.9647937Z    |
2019-11-13T00:21:30.9648236Z 52 | E0076: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0076.md"),
2019-11-13T00:21:30.9648675Z 
2019-11-13T00:21:30.9648675Z 
2019-11-13T00:21:30.9649108Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0077.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9649482Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:53:8
2019-11-13T00:21:30.9649697Z    |
2019-11-13T00:21:30.9650011Z 53 | E0077: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0077.md"),
2019-11-13T00:21:30.9650390Z 
2019-11-13T00:21:30.9650390Z 
2019-11-13T00:21:30.9650824Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0080.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9651171Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:54:8
2019-11-13T00:21:30.9651405Z    |
2019-11-13T00:21:30.9652049Z 54 | E0080: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0080.md"),
2019-11-13T00:21:30.9652792Z 
2019-11-13T00:21:30.9652792Z 
2019-11-13T00:21:30.9653352Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0081.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9653814Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:55:8
2019-11-13T00:21:30.9654083Z    |
2019-11-13T00:21:30.9654471Z 55 | E0081: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0081.md"),
2019-11-13T00:21:30.9654898Z 
2019-11-13T00:21:30.9654898Z 
2019-11-13T00:21:30.9655429Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0084.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9655866Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:56:8
2019-11-13T00:21:30.9656477Z    |
2019-11-13T00:21:30.9656805Z 56 | E0084: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0084.md"),
2019-11-13T00:21:30.9657273Z 
2019-11-13T00:21:30.9657273Z 
2019-11-13T00:21:30.9657749Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0087.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9658304Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:57:8
2019-11-13T00:21:30.9658710Z    |
2019-11-13T00:21:30.9659170Z 57 | E0087: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0087.md"),
2019-11-13T00:21:30.9681042Z 
2019-11-13T00:21:30.9681042Z 
2019-11-13T00:21:30.9681679Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0088.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9683121Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:58:8
2019-11-13T00:21:30.9683478Z    |
2019-11-13T00:21:30.9683860Z 58 | E0088: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0088.md"),
2019-11-13T00:21:30.9687123Z 
2019-11-13T00:21:30.9687123Z 
2019-11-13T00:21:30.9687552Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0089.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9687921Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:59:8
2019-11-13T00:21:30.9688135Z    |
2019-11-13T00:21:30.9688440Z 59 | E0089: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0089.md"),
2019-11-13T00:21:30.9688778Z 
2019-11-13T00:21:30.9688778Z 
2019-11-13T00:21:30.9689195Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0090.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9689539Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:60:8
2019-11-13T00:21:30.9689946Z    |
2019-11-13T00:21:30.9690242Z 60 | E0090: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0090.md"),
2019-11-13T00:21:30.9690665Z 
2019-11-13T00:21:30.9690665Z 
2019-11-13T00:21:30.9691102Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0091.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9691457Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:61:8
2019-11-13T00:21:30.9691669Z    |
2019-11-13T00:21:30.9691966Z 61 | E0091: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0091.md"),
2019-11-13T00:21:30.9693244Z 
2019-11-13T00:21:30.9693244Z 
2019-11-13T00:21:30.9693826Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0092.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9694268Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:62:8
2019-11-13T00:21:30.9694543Z    |
2019-11-13T00:21:30.9694914Z 62 | E0092: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0092.md"),
2019-11-13T00:21:30.9695559Z 
2019-11-13T00:21:30.9695559Z 
2019-11-13T00:21:30.9696618Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0093.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9697322Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:63:8
2019-11-13T00:21:30.9697551Z    |
2019-11-13T00:21:30.9697851Z 63 | E0093: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0093.md"),
2019-11-13T00:21:30.9698528Z 
2019-11-13T00:21:30.9698528Z 
2019-11-13T00:21:30.9699152Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0094.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9699525Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:64:8
2019-11-13T00:21:30.9699893Z    |
2019-11-13T00:21:30.9700211Z 64 | E0094: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0094.md"),
2019-11-13T00:21:30.9700628Z 
2019-11-13T00:21:30.9700628Z 
2019-11-13T00:21:30.9701081Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0106.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9701441Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:65:8
2019-11-13T00:21:30.9701674Z    |
2019-11-13T00:21:30.9702399Z 65 | E0106: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0106.md"),
2019-11-13T00:21:30.9702849Z 
2019-11-13T00:21:30.9702849Z 
2019-11-13T00:21:30.9703371Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0107.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9703814Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:66:8
2019-11-13T00:21:30.9704076Z    |
2019-11-13T00:21:30.9704443Z 66 | E0107: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0107.md"),
2019-11-13T00:21:30.9704875Z 
2019-11-13T00:21:30.9704875Z 
2019-11-13T00:21:30.9705553Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0109.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9706274Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:67:8
2019-11-13T00:21:30.9706666Z    |
2019-11-13T00:21:30.9707136Z 67 | E0109: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0109.md"),
2019-11-13T00:21:30.9707645Z 
2019-11-13T00:21:30.9707645Z 
2019-11-13T00:21:30.9708210Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0110.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9708544Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:68:8
2019-11-13T00:21:30.9708874Z    |
2019-11-13T00:21:30.9709160Z 68 | E0110: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0110.md"),
2019-11-13T00:21:30.9709566Z 
2019-11-13T00:21:30.9709566Z 
2019-11-13T00:21:30.9709987Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0116.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9710345Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:69:8
2019-11-13T00:21:30.9710550Z    |
2019-11-13T00:21:30.9710846Z 69 | E0116: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0116.md"),
2019-11-13T00:21:30.9711169Z 
2019-11-13T00:21:30.9711169Z 
2019-11-13T00:21:30.9711572Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0117.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9711906Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:70:8
2019-11-13T00:21:30.9712124Z    |
2019-11-13T00:21:30.9712855Z 70 | E0117: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0117.md"),
2019-11-13T00:21:30.9713292Z 
2019-11-13T00:21:30.9713292Z 
2019-11-13T00:21:30.9713815Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0118.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9714260Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:71:8
2019-11-13T00:21:30.9714525Z    |
2019-11-13T00:21:30.9714889Z 71 | E0118: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0118.md"),
2019-11-13T00:21:30.9715324Z 
2019-11-13T00:21:30.9715324Z 
2019-11-13T00:21:30.9715978Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0119.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9716533Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:72:8
2019-11-13T00:21:30.9716893Z    |
2019-11-13T00:21:30.9717390Z 72 | E0119: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0119.md"),
2019-11-13T00:21:30.9718004Z 
2019-11-13T00:21:30.9718004Z 
2019-11-13T00:21:30.9718644Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0120.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9719481Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:73:8
2019-11-13T00:21:30.9719874Z    |
2019-11-13T00:21:30.9720148Z 73 | E0120: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0120.md"),
2019-11-13T00:21:30.9720474Z 
2019-11-13T00:21:30.9720474Z 
2019-11-13T00:21:30.9720860Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0121.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9721199Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:74:8
2019-11-13T00:21:30.9721396Z    |
2019-11-13T00:21:30.9721677Z 74 | E0121: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0121.md"),
2019-11-13T00:21:30.9721992Z 
2019-11-13T00:21:30.9721992Z 
2019-11-13T00:21:30.9722797Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0124.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9723285Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:75:8
2019-11-13T00:21:30.9723570Z    |
2019-11-13T00:21:30.9723941Z 75 | E0124: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0124.md"),
2019-11-13T00:21:30.9724377Z 
2019-11-13T00:21:30.9724377Z 
2019-11-13T00:21:30.9724893Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0128.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9725333Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:76:8
2019-11-13T00:21:30.9725871Z    |
2019-11-13T00:21:30.9726390Z 76 | E0128: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0128.md"),
2019-11-13T00:21:30.9726938Z 
2019-11-13T00:21:30.9726938Z 
2019-11-13T00:21:30.9727331Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0130.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9727859Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:77:8
2019-11-13T00:21:30.9728073Z    |
2019-11-13T00:21:30.9728383Z 77 | E0130: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0130.md"),
2019-11-13T00:21:30.9728716Z 
2019-11-13T00:21:30.9728716Z 
2019-11-13T00:21:30.9729353Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0131.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9729865Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:78:8
2019-11-13T00:21:30.9730247Z    |
2019-11-13T00:21:30.9730535Z 78 | E0131: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0131.md"),
2019-11-13T00:21:30.9731044Z 
2019-11-13T00:21:30.9731044Z 
2019-11-13T00:21:30.9731426Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0132.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9731761Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:79:8
2019-11-13T00:21:30.9732141Z    |
2019-11-13T00:21:30.9733239Z 79 | E0132: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0132.md"),
2019-11-13T00:21:30.9733666Z 
2019-11-13T00:21:30.9733666Z 
2019-11-13T00:21:30.9734186Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0133.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9734619Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:80:8
2019-11-13T00:21:30.9735026Z    |
2019-11-13T00:21:30.9735618Z 80 | E0133: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0133.md"),
2019-11-13T00:21:30.9736104Z 
2019-11-13T00:21:30.9736104Z 
2019-11-13T00:21:30.9736837Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0136.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9737176Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:81:8
2019-11-13T00:21:30.9737381Z    |
2019-11-13T00:21:30.9738032Z 81 | E0136: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0136.md"),
2019-11-13T00:21:30.9738880Z 
2019-11-13T00:21:30.9738880Z 
2019-11-13T00:21:30.9739286Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0137.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9739640Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:82:8
2019-11-13T00:21:30.9739848Z    |
2019-11-13T00:21:30.9740336Z 82 | E0137: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0137.md"),
2019-11-13T00:21:30.9740676Z 
2019-11-13T00:21:30.9740676Z 
2019-11-13T00:21:30.9741106Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0138.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9741502Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:83:8
2019-11-13T00:21:30.9741882Z    |
2019-11-13T00:21:30.9742169Z 83 | E0138: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0138.md"),
2019-11-13T00:21:30.9742932Z 
2019-11-13T00:21:30.9742932Z 
2019-11-13T00:21:30.9743450Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0139.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9743898Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:84:8
2019-11-13T00:21:30.9744292Z    |
2019-11-13T00:21:30.9744766Z 84 | E0139: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0139.md"),
2019-11-13T00:21:30.9745218Z 
2019-11-13T00:21:30.9745218Z 
2019-11-13T00:21:30.9746172Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0152.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9747079Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:85:8
2019-11-13T00:21:30.9747327Z    |
2019-11-13T00:21:30.9747866Z 85 | E0152: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0152.md"),
2019-11-13T00:21:30.9748477Z 
2019-11-13T00:21:30.9748477Z 
2019-11-13T00:21:30.9748901Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0154.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9750863Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:86:8
2019-11-13T00:21:30.9751116Z    |
2019-11-13T00:21:30.9751621Z 86 | E0154: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0154.md"),
2019-11-13T00:21:30.9753310Z 
2019-11-13T00:21:30.9753310Z 
2019-11-13T00:21:30.9753859Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0158.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9754311Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:87:8
2019-11-13T00:21:30.9754574Z    |
2019-11-13T00:21:30.9754963Z 87 | E0158: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0158.md"),
2019-11-13T00:21:30.9755378Z 
2019-11-13T00:21:30.9755378Z 
2019-11-13T00:21:30.9756061Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0161.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9756742Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:88:8
2019-11-13T00:21:30.9757088Z    |
2019-11-13T00:21:30.9757447Z 88 | E0161: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0161.md"),
2019-11-13T00:21:30.9757815Z 
2019-11-13T00:21:30.9757815Z 
2019-11-13T00:21:30.9758212Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0162.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9758555Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:89:8
2019-11-13T00:21:30.9758758Z    |
2019-11-13T00:21:30.9759065Z 89 | E0162: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0162.md"),
2019-11-13T00:21:30.9759389Z 
2019-11-13T00:21:30.9759389Z 
2019-11-13T00:21:30.9759793Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/../../../../../../../../src/librustc_error_codes/error_codes/E0164.md: No such file or directory (os error 2)
2019-11-13T00:21:30.9760170Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:90:8
2019-11-13T00:21:30.9760393Z    |
2019-11-13T00:21:30.9760705Z 90 | E0164: include_str!("../../../../../../../../src/librustc_error_codes/error_codes/E0164.md"),
---
2019-11-13T00:21:31.0871088Z   local time: Wed Nov 13 00:21:30 UTC 2019
2019-11-13T00:21:31.0871138Z   network time: Wed, 13 Nov 2019 00:21:30 GMT
2019-11-13T00:21:31.0871189Z == end clock drift check ==
2019-11-13T00:21:34.1696884Z 
2019-11-13T00:21:34.1759018Z ##[error]Bash exited with code '1'.
2019-11-13T00:21:34.1805462Z ##[section]Starting: Checkout
2019-11-13T00:21:34.1807326Z ==============================================================================
2019-11-13T00:21:34.1807375Z Task         : Get sources
2019-11-13T00:21:34.1807435Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
