plain
2019-11-12T16:00:16.8527063Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T16:00:16.8718718Z ##[command]git config gc.auto 0
2019-11-12T16:00:16.8799613Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T16:00:16.8867108Z ##[command]git config --get-all http.proxy
2019-11-12T16:00:16.9017676Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66314/merge:refs/remotes/pull/66314/merge
---
2019-11-12T16:59:55.5714170Z .................................................................................................... 1400/9232
2019-11-12T17:00:01.9387346Z .................................................................................................... 1500/9232
2019-11-12T17:00:08.0578546Z .................................................................................................... 1600/9232
2019-11-12T17:00:17.5185264Z .................................................................................................... 1700/9232
2019-11-12T17:00:26.0158308Z ..i................................................................................................. 1800/9232
2019-11-12T17:00:33.7013685Z ......................................................................................iiiii......... 1900/9232
2019-11-12T17:00:54.2417734Z .................................................................................................... 2100/9232
2019-11-12T17:00:56.6741654Z .................................................................................................... 2200/9232
2019-11-12T17:00:59.2402605Z .................................................................................................... 2300/9232
2019-11-12T17:01:09.0103055Z .................................................................................................... 2400/9232
---
2019-11-12T17:04:03.6579622Z ...................................................................................i...............i 4700/9232
2019-11-12T17:04:10.7273750Z .................................................................................................... 4800/9232
2019-11-12T17:04:20.2558678Z .................................................................................................... 4900/9232
2019-11-12T17:04:25.5972729Z .................................................................................................... 5000/9232
2019-11-12T17:04:37.1186853Z ......................................................................................ii.ii......... 5100/9232
2019-11-12T17:04:40.8987434Z ..i................................................................................................. 5200/9232
2019-11-12T17:04:54.7273106Z .................................................................................................... 5400/9232
2019-11-12T17:05:02.8012778Z ....................................................................i............................... 5500/9232
2019-11-12T17:05:10.2969027Z .................................................................................................... 5600/9232
2019-11-12T17:05:18.0582502Z .................................................................................................... 5700/9232
2019-11-12T17:05:18.0582502Z .................................................................................................... 5700/9232
2019-11-12T17:05:27.9985180Z .....................................................ii...i..ii...........i......................... 5800/9232
2019-11-12T17:05:50.8139659Z .................................................................................................... 6000/9232
2019-11-12T17:05:58.3285694Z .................................................................................................... 6100/9232
2019-11-12T17:05:58.3285694Z .................................................................................................... 6100/9232
2019-11-12T17:06:03.3151366Z ........................................................................i..ii....................... 6200/9232
2019-11-12T17:06:32.6097611Z .................................................................................................... 6400/9232
2019-11-12T17:06:34.8759911Z ........................................i........................................................... 6500/9232
2019-11-12T17:06:37.1528226Z .................................................................................................... 6600/9232
2019-11-12T17:06:39.5193576Z ........................i........................................................................... 6700/9232
---
2019-11-12T17:11:56.4146676Z  finished in 5.605
2019-11-12T17:11:56.4335059Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T17:11:56.6071181Z 
2019-11-12T17:11:56.6071819Z running 156 tests
2019-11-12T17:11:59.5106314Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-12T17:12:01.4456455Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-12T17:12:01.4459319Z 
2019-11-12T17:12:01.4459580Z  finished in 5.012
2019-11-12T17:12:01.4625348Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T17:12:01.6185571Z 
---
2019-11-12T17:12:03.5355811Z  finished in 2.073
2019-11-12T17:12:03.5526724Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T17:12:03.7177412Z 
2019-11-12T17:12:03.7178081Z running 9 tests
2019-11-12T17:12:03.7178976Z iiiiiiiii
2019-11-12T17:12:03.7179753Z 
2019-11-12T17:12:03.7185496Z  finished in 0.165
2019-11-12T17:12:03.7405844Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T17:12:03.9141423Z 
---
2019-11-12T17:12:23.3334940Z  finished in 19.593
2019-11-12T17:12:23.3552205Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T17:12:23.5391566Z 
2019-11-12T17:12:23.5392238Z running 123 tests
2019-11-12T17:12:47.5210102Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-12T17:12:52.1068624Z i.i.i......iii.i.....ii
2019-11-12T17:12:52.1070224Z 
2019-11-12T17:12:52.1071923Z  finished in 28.752
2019-11-12T17:12:52.1083197Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T17:12:52.1086047Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-12T17:24:57.1778375Z 
2019-11-12T17:24:57.1782529Z    Doc-tests core
2019-11-12T17:25:02.0229905Z 
2019-11-12T17:25:02.0230778Z running 2418 tests
2019-11-12T17:25:12.7948713Z ......iiiii......................................................................................... 100/2418
2019-11-12T17:25:23.3836175Z ................................................................................ii.................. 200/2418
2019-11-12T17:25:47.8723884Z ..i................................................................................................. 400/2418
2019-11-12T17:25:47.8723884Z ..i................................................................................................. 400/2418
2019-11-12T17:25:58.1540983Z ..................................................i..i.................iiii......................... 500/2418
2019-11-12T17:26:17.8442915Z .................................................................................................... 700/2418
2019-11-12T17:26:28.0585162Z .................................................................................................... 800/2418
2019-11-12T17:26:38.1428420Z .................................................................................................... 900/2418
2019-11-12T17:26:48.2707994Z .................................................................................................... 1000/2418
---
2019-11-12T17:30:43.8652054Z ............................................... 300/763
2019-11-12T17:30:43.8675416Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-11-12T17:30:43.9749357Z .................................................................................................... 400/763
2019-11-12T17:30:46.0674409Z .................................................................................................... 500/763
2019-11-12T17:30:46.0950849Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-12T17:30:46.0980197Z ....thread '<unnamed>' panicked at 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-11-12T17:30:46.1003192Z called `Result::unwrap()` on an `Err` value: RecvError.', .src/libcore/result.rs..:..1165.thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-12T17:30:46.3336221Z .........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-12T17:30:46.3369722Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-11-12T17:30:46.3374902Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-11-12T17:30:46.3396465Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
---
2019-11-12T17:30:57.7274922Z 
2019-11-12T17:30:57.7275436Z running 1000 tests
2019-11-12T17:31:18.5927663Z i................................................................................................... 100/1000
2019-11-12T17:31:30.0205014Z .................................................................................................... 200/1000
2019-11-12T17:31:38.0472527Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-12T17:31:43.3718607Z .................................................................................................... 400/1000
2019-11-12T17:31:50.8914562Z ...........................................i..i.................................ii.................. 500/1000
2019-11-12T17:32:04.8050432Z .................................................................................................... 700/1000
2019-11-12T17:32:04.8050432Z .................................................................................................... 700/1000
2019-11-12T17:32:12.1544215Z ..........................iiii...................................................................... 800/1000
2019-11-12T17:32:27.7877912Z .................................................................................................... 900/1000
2019-11-12T17:32:35.5194536Z ................................................iiii................................................ 1000/1000
2019-11-12T17:32:35.5197640Z test result: ok. 980 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-11-12T17:32:35.5197676Z 
2019-11-12T17:32:35.5283706Z  finished in 193.387
2019-11-12T17:32:35.5299998Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-11-12T17:49:30.2659816Z Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
2019-11-12T17:49:30.4916315Z    Compiling same-file v1.0.4
2019-11-12T17:49:30.6036539Z    Compiling walkdir v2.2.7
2019-11-12T17:49:32.0142767Z    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
2019-11-12T17:49:33.0506266Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0001.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0507709Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:8:8
2019-11-12T17:49:33.0508203Z   |
2019-11-12T17:49:33.0508736Z 8 | E0001: include_str!("./error_codes/E0001.md"),
2019-11-12T17:49:33.0509414Z 
2019-11-12T17:49:33.0509414Z 
2019-11-12T17:49:33.0510205Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0002.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0510880Z  --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:9:8
2019-11-12T17:49:33.0511948Z   |
2019-11-12T17:49:33.0512244Z 9 | E0002: include_str!("./error_codes/E0002.md"),
2019-11-12T17:49:33.0512606Z 
2019-11-12T17:49:33.0512606Z 
2019-11-12T17:49:33.0546110Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0004.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0546881Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:10:8
2019-11-12T17:49:33.0547121Z    |
2019-11-12T17:49:33.0547434Z 10 | E0004: include_str!("./error_codes/E0004.md"),
2019-11-12T17:49:33.0547936Z 
2019-11-12T17:49:33.0547936Z 
2019-11-12T17:49:33.0548429Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0005.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0548833Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:11:8
2019-11-12T17:49:33.0549068Z    |
2019-11-12T17:49:33.0549354Z 11 | E0005: include_str!("./error_codes/E0005.md"),
2019-11-12T17:49:33.0549674Z 
2019-11-12T17:49:33.0549674Z 
2019-11-12T17:49:33.0550120Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0007.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0550523Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:12:8
2019-11-12T17:49:33.0550766Z    |
2019-11-12T17:49:33.0551050Z 12 | E0007: include_str!("./error_codes/E0007.md"),
2019-11-12T17:49:33.0551383Z 
2019-11-12T17:49:33.0551383Z 
2019-11-12T17:49:33.0551812Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0009.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0552216Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:13:8
2019-11-12T17:49:33.0552454Z    |
2019-11-12T17:49:33.0552830Z 13 | E0009: include_str!("./error_codes/E0009.md"),
2019-11-12T17:49:33.0553204Z 
2019-11-12T17:49:33.0553204Z 
2019-11-12T17:49:33.0553633Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0010.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0554023Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:14:8
2019-11-12T17:49:33.0554264Z    |
2019-11-12T17:49:33.0554545Z 14 | E0010: include_str!("./error_codes/E0010.md"),
2019-11-12T17:49:33.0554885Z 
2019-11-12T17:49:33.0554885Z 
2019-11-12T17:49:33.0555316Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0013.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0555722Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:15:8
2019-11-12T17:49:33.0556049Z    |
2019-11-12T17:49:33.0556696Z 15 | E0013: include_str!("./error_codes/E0013.md"),
2019-11-12T17:49:33.0557098Z 
2019-11-12T17:49:33.0557098Z 
2019-11-12T17:49:33.0557541Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0014.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0557954Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:16:8
2019-11-12T17:49:33.0558174Z    |
2019-11-12T17:49:33.0558457Z 16 | E0014: include_str!("./error_codes/E0014.md"),
2019-11-12T17:49:33.0558804Z 
2019-11-12T17:49:33.0558804Z 
2019-11-12T17:49:33.0559240Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0015.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0559653Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:17:8
2019-11-12T17:49:33.0559868Z    |
2019-11-12T17:49:33.0560151Z 17 | E0015: include_str!("./error_codes/E0015.md"),
2019-11-12T17:49:33.0560501Z 
2019-11-12T17:49:33.0560501Z 
2019-11-12T17:49:33.0560926Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0017.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0561448Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:18:8
2019-11-12T17:49:33.0561705Z    |
2019-11-12T17:49:33.0562008Z 18 | E0017: include_str!("./error_codes/E0017.md"),
2019-11-12T17:49:33.0562329Z 
2019-11-12T17:49:33.0562329Z 
2019-11-12T17:49:33.0562764Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0019.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0563182Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:19:8
2019-11-12T17:49:33.0563399Z    |
2019-11-12T17:49:33.0563697Z 19 | E0019: include_str!("./error_codes/E0019.md"),
2019-11-12T17:49:33.0564018Z 
2019-11-12T17:49:33.0564018Z 
2019-11-12T17:49:33.0564458Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0023.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0564985Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:20:8
2019-11-12T17:49:33.0565494Z    |
2019-11-12T17:49:33.0565827Z 20 | E0023: include_str!("./error_codes/E0023.md"),
2019-11-12T17:49:33.0566151Z 
2019-11-12T17:49:33.0566151Z 
2019-11-12T17:49:33.0595656Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0025.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0596113Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:21:8
2019-11-12T17:49:33.0596362Z    |
2019-11-12T17:49:33.0596650Z 21 | E0025: include_str!("./error_codes/E0025.md"),
2019-11-12T17:49:33.0596998Z 
2019-11-12T17:49:33.0596998Z 
2019-11-12T17:49:33.0597434Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0026.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0597844Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:22:8
2019-11-12T17:49:33.0598077Z    |
2019-11-12T17:49:33.0598360Z 22 | E0026: include_str!("./error_codes/E0026.md"),
2019-11-12T17:49:33.0598837Z 
2019-11-12T17:49:33.0598837Z 
2019-11-12T17:49:33.0599317Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0027.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0599728Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:23:8
2019-11-12T17:49:33.0599956Z    |
2019-11-12T17:49:33.0600239Z 23 | E0027: include_str!("./error_codes/E0027.md"),
2019-11-12T17:49:33.0600576Z 
2019-11-12T17:49:33.0600576Z 
2019-11-12T17:49:33.0601011Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0029.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0601425Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:24:8
2019-11-12T17:49:33.0601640Z    |
2019-11-12T17:49:33.0601923Z 24 | E0029: include_str!("./error_codes/E0029.md"),
2019-11-12T17:49:33.0602391Z 
2019-11-12T17:49:33.0602391Z 
2019-11-12T17:49:33.0602827Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0030.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0603240Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:25:8
2019-11-12T17:49:33.0603455Z    |
2019-11-12T17:49:33.0603754Z 25 | E0030: include_str!("./error_codes/E0030.md"),
2019-11-12T17:49:33.0604084Z 
2019-11-12T17:49:33.0604084Z 
2019-11-12T17:49:33.0604508Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0033.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0604927Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:26:8
2019-11-12T17:49:33.0605395Z    |
2019-11-12T17:49:33.0605731Z 26 | E0033: include_str!("./error_codes/E0033.md"),
2019-11-12T17:49:33.0606057Z 
2019-11-12T17:49:33.0606057Z 
2019-11-12T17:49:33.0606498Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0034.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0607030Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:27:8
2019-11-12T17:49:33.0607288Z    |
2019-11-12T17:49:33.0607590Z 27 | E0034: include_str!("./error_codes/E0034.md"),
2019-11-12T17:49:33.0607909Z 
2019-11-12T17:49:33.0607909Z 
2019-11-12T17:49:33.0608348Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0038.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0608755Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:28:8
2019-11-12T17:49:33.0608969Z    |
2019-11-12T17:49:33.0609276Z 28 | E0038: include_str!("./error_codes/E0038.md"),
2019-11-12T17:49:33.0609595Z 
2019-11-12T17:49:33.0609595Z 
2019-11-12T17:49:33.0610039Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0040.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0610427Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:29:8
2019-11-12T17:49:33.0610775Z    |
2019-11-12T17:49:33.0611062Z 29 | E0040: include_str!("./error_codes/E0040.md"),
2019-11-12T17:49:33.0611388Z 
2019-11-12T17:49:33.0611388Z 
2019-11-12T17:49:33.0611833Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0044.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0612225Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:30:8
2019-11-12T17:49:33.0612468Z    |
2019-11-12T17:49:33.0612750Z 30 | E0044: include_str!("./error_codes/E0044.md"),
2019-11-12T17:49:33.0613081Z 
2019-11-12T17:49:33.0613081Z 
2019-11-12T17:49:33.0613515Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0045.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0613908Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:31:8
2019-11-12T17:49:33.0614141Z    |
2019-11-12T17:49:33.0614433Z 31 | E0045: include_str!("./error_codes/E0045.md"),
2019-11-12T17:49:33.0614768Z 
2019-11-12T17:49:33.0614768Z 
2019-11-12T17:49:33.0615276Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0046.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0615702Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:32:8
2019-11-12T17:49:33.0615937Z    |
2019-11-12T17:49:33.0616220Z 32 | E0046: include_str!("./error_codes/E0046.md"),
2019-11-12T17:49:33.0616565Z 
2019-11-12T17:49:33.0616565Z 
2019-11-12T17:49:33.0616992Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0049.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0617404Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:33:8
2019-11-12T17:49:33.0617624Z    |
2019-11-12T17:49:33.0617904Z 33 | E0049: include_str!("./error_codes/E0049.md"),
2019-11-12T17:49:33.0618318Z 
2019-11-12T17:49:33.0618318Z 
2019-11-12T17:49:33.0618772Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0050.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0619188Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:34:8
2019-11-12T17:49:33.0619405Z    |
2019-11-12T17:49:33.0619684Z 34 | E0050: include_str!("./error_codes/E0050.md"),
2019-11-12T17:49:33.0620020Z 
2019-11-12T17:49:33.0620020Z 
2019-11-12T17:49:33.0620444Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0053.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0620862Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:35:8
2019-11-12T17:49:33.0621083Z    |
2019-11-12T17:49:33.0621365Z 35 | E0053: include_str!("./error_codes/E0053.md"),
2019-11-12T17:49:33.0621701Z 
2019-11-12T17:49:33.0621701Z 
2019-11-12T17:49:33.0641647Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0054.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0648712Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:36:8
2019-11-12T17:49:33.0649083Z    |
2019-11-12T17:49:33.0649703Z 36 | E0054: include_str!("./error_codes/E0054.md"),
2019-11-12T17:49:33.0650112Z 
2019-11-12T17:49:33.0650112Z 
2019-11-12T17:49:33.0650575Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0055.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0650975Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:37:8
2019-11-12T17:49:33.0651230Z    |
2019-11-12T17:49:33.0651520Z 37 | E0055: include_str!("./error_codes/E0055.md"),
2019-11-12T17:49:33.0651869Z 
2019-11-12T17:49:33.0651869Z 
2019-11-12T17:49:33.0652306Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0057.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0652718Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:38:8
2019-11-12T17:49:33.0653094Z    |
2019-11-12T17:49:33.0653384Z 38 | E0057: include_str!("./error_codes/E0057.md"),
2019-11-12T17:49:33.0653726Z 
2019-11-12T17:49:33.0653726Z 
2019-11-12T17:49:33.0654162Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0059.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0654576Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:39:8
2019-11-12T17:49:33.0654792Z    |
2019-11-12T17:49:33.0655098Z 39 | E0059: include_str!("./error_codes/E0059.md"),
2019-11-12T17:49:33.0655427Z 
2019-11-12T17:49:33.0655427Z 
2019-11-12T17:49:33.0655880Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0060.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0656596Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:40:8
2019-11-12T17:49:33.0656900Z    |
2019-11-12T17:49:33.0657189Z 40 | E0060: include_str!("./error_codes/E0060.md"),
2019-11-12T17:49:33.0657538Z 
2019-11-12T17:49:33.0657538Z 
2019-11-12T17:49:33.0657971Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0061.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0658473Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:41:8
2019-11-12T17:49:33.0658758Z    |
2019-11-12T17:49:33.0659046Z 41 | E0061: include_str!("./error_codes/E0061.md"),
2019-11-12T17:49:33.0659387Z 
2019-11-12T17:49:33.0659387Z 
2019-11-12T17:49:33.0659813Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0062.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0660242Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:42:8
2019-11-12T17:49:33.0660463Z    |
2019-11-12T17:49:33.0660764Z 42 | E0062: include_str!("./error_codes/E0062.md"),
2019-11-12T17:49:33.0661084Z 
2019-11-12T17:49:33.0661084Z 
2019-11-12T17:49:33.0661598Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0063.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0662118Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:43:8
2019-11-12T17:49:33.0662358Z    |
2019-11-12T17:49:33.0662652Z 43 | E0063: include_str!("./error_codes/E0063.md"),
2019-11-12T17:49:33.0662989Z 
2019-11-12T17:49:33.0662989Z 
2019-11-12T17:49:33.0663419Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0067.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0663836Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:44:8
2019-11-12T17:49:33.0664056Z    |
2019-11-12T17:49:33.0664340Z 44 | E0067: include_str!("./error_codes/E0067.md"),
2019-11-12T17:49:33.0664690Z 
2019-11-12T17:49:33.0664690Z 
2019-11-12T17:49:33.0665117Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0069.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0665531Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:45:8
2019-11-12T17:49:33.0665761Z    |
2019-11-12T17:49:33.0666064Z 45 | E0069: include_str!("./error_codes/E0069.md"),
2019-11-12T17:49:33.0666791Z 
2019-11-12T17:49:33.0666791Z 
2019-11-12T17:49:33.0667368Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0070.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0667810Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:46:8
2019-11-12T17:49:33.0668048Z    |
2019-11-12T17:49:33.0668350Z 46 | E0070: include_str!("./error_codes/E0070.md"),
2019-11-12T17:49:33.0668684Z 
2019-11-12T17:49:33.0668684Z 
2019-11-12T17:49:33.0669122Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0071.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0669519Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:47:8
2019-11-12T17:49:33.0669758Z    |
2019-11-12T17:49:33.0670042Z 47 | E0071: include_str!("./error_codes/E0071.md"),
2019-11-12T17:49:33.0670488Z 
2019-11-12T17:49:33.0670488Z 
2019-11-12T17:49:33.0670929Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0072.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0671345Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:48:8
2019-11-12T17:49:33.0671563Z    |
2019-11-12T17:49:33.0671865Z 48 | E0072: include_str!("./error_codes/E0072.md"),
2019-11-12T17:49:33.0672195Z 
2019-11-12T17:49:33.0672195Z 
2019-11-12T17:49:33.0672622Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0073.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0673046Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:49:8
2019-11-12T17:49:33.0673264Z    |
2019-11-12T17:49:33.0673568Z 49 | E0073: include_str!("./error_codes/E0073.md"),
2019-11-12T17:49:33.0673885Z 
2019-11-12T17:49:33.0673885Z 
2019-11-12T17:49:33.0674328Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0074.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0674731Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:50:8
2019-11-12T17:49:33.0675044Z    |
2019-11-12T17:49:33.0675364Z 50 | E0074: include_str!("./error_codes/E0074.md"),
2019-11-12T17:49:33.0675750Z 
2019-11-12T17:49:33.0675750Z 
2019-11-12T17:49:33.0676184Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0075.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0676881Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:51:8
2019-11-12T17:49:33.0677112Z    |
2019-11-12T17:49:33.0677411Z 51 | E0075: include_str!("./error_codes/E0075.md"),
2019-11-12T17:49:33.0677757Z 
2019-11-12T17:49:33.0677757Z 
2019-11-12T17:49:33.0678182Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0076.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0678592Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:52:8
2019-11-12T17:49:33.0678952Z    |
2019-11-12T17:49:33.0679262Z 52 | E0076: include_str!("./error_codes/E0076.md"),
2019-11-12T17:49:33.0679591Z 
2019-11-12T17:49:33.0679591Z 
2019-11-12T17:49:33.0704634Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0077.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0705076Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:53:8
2019-11-12T17:49:33.0705347Z    |
2019-11-12T17:49:33.0705641Z 53 | E0077: include_str!("./error_codes/E0077.md"),
2019-11-12T17:49:33.0705985Z 
2019-11-12T17:49:33.0705985Z 
2019-11-12T17:49:33.0706754Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0080.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0707223Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:54:8
2019-11-12T17:49:33.0707449Z    |
2019-11-12T17:49:33.0707739Z 54 | E0080: include_str!("./error_codes/E0080.md"),
2019-11-12T17:49:33.0708095Z 
2019-11-12T17:49:33.0708095Z 
2019-11-12T17:49:33.0708669Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0081.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0709187Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:55:8
2019-11-12T17:49:33.0709485Z    |
2019-11-12T17:49:33.0709841Z 55 | E0081: include_str!("./error_codes/E0081.md"),
2019-11-12T17:49:33.0710205Z 
2019-11-12T17:49:33.0710205Z 
2019-11-12T17:49:33.0710680Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0084.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0711114Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:56:8
2019-11-12T17:49:33.0711378Z    |
2019-11-12T17:49:33.0711694Z 56 | E0084: include_str!("./error_codes/E0084.md"),
2019-11-12T17:49:33.0712060Z 
2019-11-12T17:49:33.0712060Z 
2019-11-12T17:49:33.0712649Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0087.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0713080Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:57:8
2019-11-12T17:49:33.0713346Z    |
2019-11-12T17:49:33.0713662Z 57 | E0087: include_str!("./error_codes/E0087.md"),
2019-11-12T17:49:33.0714033Z 
2019-11-12T17:49:33.0714033Z 
2019-11-12T17:49:33.0727884Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0088.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0728368Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:58:8
2019-11-12T17:49:33.0728597Z    |
2019-11-12T17:49:33.0728926Z 58 | E0088: include_str!("./error_codes/E0088.md"),
2019-11-12T17:49:33.0729253Z 
2019-11-12T17:49:33.0729253Z 
2019-11-12T17:49:33.0729703Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0089.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0730106Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:59:8
2019-11-12T17:49:33.0730346Z    |
2019-11-12T17:49:33.0730634Z 59 | E0089: include_str!("./error_codes/E0089.md"),
2019-11-12T17:49:33.0731120Z 
2019-11-12T17:49:33.0731120Z 
2019-11-12T17:49:33.0731591Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0090.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0732005Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:60:8
2019-11-12T17:49:33.0732239Z    |
2019-11-12T17:49:33.0732527Z 60 | E0090: include_str!("./error_codes/E0090.md"),
2019-11-12T17:49:33.0732878Z 
2019-11-12T17:49:33.0732878Z 
2019-11-12T17:49:33.0733306Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0091.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0733725Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:61:8
2019-11-12T17:49:33.0733945Z    |
2019-11-12T17:49:33.0734376Z 61 | E0091: include_str!("./error_codes/E0091.md"),
2019-11-12T17:49:33.0734701Z 
2019-11-12T17:49:33.0734701Z 
2019-11-12T17:49:33.0735157Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0092.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0735556Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:62:8
2019-11-12T17:49:33.0735789Z    |
2019-11-12T17:49:33.0736078Z 62 | E0092: include_str!("./error_codes/E0092.md"),
2019-11-12T17:49:33.0736724Z 
2019-11-12T17:49:33.0736724Z 
2019-11-12T17:49:33.0737173Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0093.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0737574Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:63:8
2019-11-12T17:49:33.0737813Z    |
2019-11-12T17:49:33.0738101Z 63 | E0093: include_str!("./error_codes/E0093.md"),
2019-11-12T17:49:33.0738448Z 
2019-11-12T17:49:33.0738448Z 
2019-11-12T17:49:33.0738876Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0094.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0739409Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:64:8
2019-11-12T17:49:33.0739673Z    |
2019-11-12T17:49:33.0739978Z 64 | E0094: include_str!("./error_codes/E0094.md"),
2019-11-12T17:49:33.0740301Z 
2019-11-12T17:49:33.0740301Z 
2019-11-12T17:49:33.0755567Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0106.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0756019Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:65:8
2019-11-12T17:49:33.0756571Z    |
2019-11-12T17:49:33.0756933Z 65 | E0106: include_str!("./error_codes/E0106.md"),
2019-11-12T17:49:33.0757276Z 
2019-11-12T17:49:33.0757276Z 
2019-11-12T17:49:33.0757709Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0107.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0758299Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:66:8
2019-11-12T17:49:33.0758524Z    |
2019-11-12T17:49:33.0758820Z 66 | E0107: include_str!("./error_codes/E0107.md"),
2019-11-12T17:49:33.0759161Z 
2019-11-12T17:49:33.0759161Z 
2019-11-12T17:49:33.0759591Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0109.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0760007Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:67:8
2019-11-12T17:49:33.0760236Z    |
2019-11-12T17:49:33.0760539Z 67 | E0109: include_str!("./error_codes/E0109.md"),
2019-11-12T17:49:33.0760869Z 
2019-11-12T17:49:33.0760869Z 
2019-11-12T17:49:33.0761313Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0110.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0761709Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:68:8
2019-11-12T17:49:33.0761955Z    |
2019-11-12T17:49:33.0762241Z 68 | E0110: include_str!("./error_codes/E0110.md"),
2019-11-12T17:49:33.0762576Z 
2019-11-12T17:49:33.0762576Z 
2019-11-12T17:49:33.0763089Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0116.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0763518Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:69:8
2019-11-12T17:49:33.0763754Z    |
2019-11-12T17:49:33.0764039Z 69 | E0116: include_str!("./error_codes/E0116.md"),
2019-11-12T17:49:33.0764387Z 
2019-11-12T17:49:33.0764387Z 
2019-11-12T17:49:33.0764822Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0117.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0765578Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:70:8
2019-11-12T17:49:33.0765858Z    |
2019-11-12T17:49:33.0766168Z 70 | E0117: include_str!("./error_codes/E0117.md"),
2019-11-12T17:49:33.0767057Z 
2019-11-12T17:49:33.0767057Z 
2019-11-12T17:49:33.0767506Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0118.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0767922Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:71:8
2019-11-12T17:49:33.0768145Z    |
2019-11-12T17:49:33.0768452Z 71 | E0118: include_str!("./error_codes/E0118.md"),
2019-11-12T17:49:33.0768779Z 
2019-11-12T17:49:33.0768779Z 
2019-11-12T17:49:33.0769229Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0119.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0769633Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:72:8
2019-11-12T17:49:33.0769871Z    |
2019-11-12T17:49:33.0770157Z 72 | E0119: include_str!("./error_codes/E0119.md"),
2019-11-12T17:49:33.0770489Z 
2019-11-12T17:49:33.0770489Z 
2019-11-12T17:49:33.0770917Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0120.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0771335Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:73:8
2019-11-12T17:49:33.0771556Z    |
2019-11-12T17:49:33.0771937Z 73 | E0120: include_str!("./error_codes/E0120.md"),
2019-11-12T17:49:33.0772310Z 
2019-11-12T17:49:33.0772310Z 
2019-11-12T17:49:33.0779858Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0121.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0780308Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:74:8
2019-11-12T17:49:33.0780553Z    |
2019-11-12T17:49:33.0780856Z 74 | E0121: include_str!("./error_codes/E0121.md"),
2019-11-12T17:49:33.0781196Z 
2019-11-12T17:49:33.0781196Z 
2019-11-12T17:49:33.0781628Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0124.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0782023Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:75:8
2019-11-12T17:49:33.0782457Z    |
2019-11-12T17:49:33.0782743Z 75 | E0124: include_str!("./error_codes/E0124.md"),
2019-11-12T17:49:33.0783089Z 
2019-11-12T17:49:33.0783089Z 
2019-11-12T17:49:33.0783520Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0128.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0783933Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:76:8
2019-11-12T17:49:33.0784164Z    |
2019-11-12T17:49:33.0784463Z 76 | E0128: include_str!("./error_codes/E0128.md"),
2019-11-12T17:49:33.0784786Z 
2019-11-12T17:49:33.0784786Z 
2019-11-12T17:49:33.0785220Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0130.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0785636Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:77:8
2019-11-12T17:49:33.0785854Z    |
2019-11-12T17:49:33.0786156Z 77 | E0130: include_str!("./error_codes/E0130.md"),
2019-11-12T17:49:33.0786904Z 
2019-11-12T17:49:33.0786904Z 
2019-11-12T17:49:33.0787477Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0131.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0787914Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:78:8
2019-11-12T17:49:33.0788152Z    |
2019-11-12T17:49:33.0788439Z 78 | E0131: include_str!("./error_codes/E0131.md"),
2019-11-12T17:49:33.0788784Z 
2019-11-12T17:49:33.0788784Z 
2019-11-12T17:49:33.0803466Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0132.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0803943Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:79:8
2019-11-12T17:49:33.0804577Z    |
2019-11-12T17:49:33.0805002Z 79 | E0132: include_str!("./error_codes/E0132.md"),
2019-11-12T17:49:33.0805602Z 
2019-11-12T17:49:33.0805602Z 
2019-11-12T17:49:33.0806056Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0133.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0807040Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:80:8
2019-11-12T17:49:33.0807276Z    |
2019-11-12T17:49:33.0807593Z 80 | E0133: include_str!("./error_codes/E0133.md"),
2019-11-12T17:49:33.0807917Z 
2019-11-12T17:49:33.0807917Z 
2019-11-12T17:49:33.0808366Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0136.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0808771Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:81:8
2019-11-12T17:49:33.0809010Z    |
2019-11-12T17:49:33.0809305Z 81 | E0136: include_str!("./error_codes/E0136.md"),
2019-11-12T17:49:33.0809644Z 
2019-11-12T17:49:33.0809644Z 
2019-11-12T17:49:33.0810073Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0137.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0810483Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:82:8
2019-11-12T17:49:33.0810714Z    |
2019-11-12T17:49:33.0811002Z 82 | E0137: include_str!("./error_codes/E0137.md"),
2019-11-12T17:49:33.0811477Z 
2019-11-12T17:49:33.0811477Z 
2019-11-12T17:49:33.0811938Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0138.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0812357Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:83:8
2019-11-12T17:49:33.0812587Z    |
2019-11-12T17:49:33.0812893Z 83 | E0138: include_str!("./error_codes/E0138.md"),
2019-11-12T17:49:33.0813214Z 
2019-11-12T17:49:33.0813214Z 
2019-11-12T17:49:33.0813667Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0139.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0814065Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:84:8
2019-11-12T17:49:33.0814303Z    |
2019-11-12T17:49:33.0814703Z 84 | E0139: include_str!("./error_codes/E0139.md"),
2019-11-12T17:49:33.0815039Z 
2019-11-12T17:49:33.0815039Z 
2019-11-12T17:49:33.0815477Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0152.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0815872Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:85:8
2019-11-12T17:49:33.0816110Z    |
2019-11-12T17:49:33.0816804Z 85 | E0152: include_str!("./error_codes/E0152.md"),
2019-11-12T17:49:33.0817172Z 
2019-11-12T17:49:33.0817172Z 
2019-11-12T17:49:33.0817600Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0154.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0818024Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:86:8
2019-11-12T17:49:33.0818244Z    |
2019-11-12T17:49:33.0818544Z 86 | E0154: include_str!("./error_codes/E0154.md"),
2019-11-12T17:49:33.0818874Z 
2019-11-12T17:49:33.0818874Z 
2019-11-12T17:49:33.0819318Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0158.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0819820Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:87:8
2019-11-12T17:49:33.0820097Z    |
2019-11-12T17:49:33.0820388Z 87 | E0158: include_str!("./error_codes/E0158.md"),
2019-11-12T17:49:33.0820708Z 
2019-11-12T17:49:33.0820708Z 
2019-11-12T17:49:33.0821153Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0161.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0821557Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:88:8
2019-11-12T17:49:33.0821807Z    |
2019-11-12T17:49:33.0822094Z 88 | E0161: include_str!("./error_codes/E0161.md"),
2019-11-12T17:49:33.0822429Z 
2019-11-12T17:49:33.0822429Z 
2019-11-12T17:49:33.0837951Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0162.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0838586Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:89:8
2019-11-12T17:49:33.0838816Z    |
2019-11-12T17:49:33.0839145Z 89 | E0162: include_str!("./error_codes/E0162.md"),
2019-11-12T17:49:33.0839474Z 
2019-11-12T17:49:33.0839474Z 
2019-11-12T17:49:33.0839923Z error: couldn't read /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/./error_codes/E0164.md: No such file or directory (os error 2)
2019-11-12T17:49:33.0840319Z   --> /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/error_index_generator-309a7073e45bc2ae/out/error_0.rs:90:8
2019-11-12T17:49:33.0840570Z    |
2019-11-12T17:49:33.0840857Z 90 | E0164: include_str!("./error_codes/E0164.md"),
---
2019-11-12T17:49:33.6635918Z   local time: Tue Nov 12 17:49:33 UTC 2019
2019-11-12T17:49:33.8349657Z   network time: Tue, 12 Nov 2019 17:49:33 GMT
2019-11-12T17:49:33.8355198Z == end clock drift check ==
2019-11-12T17:49:38.4761822Z 
2019-11-12T17:49:38.4876993Z ##[error]Bash exited with code '1'.
2019-11-12T17:49:38.4940839Z ##[section]Starting: Checkout
2019-11-12T17:49:38.4942766Z ==============================================================================
2019-11-12T17:49:38.4942828Z Task         : Get sources
2019-11-12T17:49:38.4942881Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
