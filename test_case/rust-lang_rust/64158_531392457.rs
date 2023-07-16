plain
2019-09-13T19:31:53.7285538Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T19:31:54.5200068Z ##[command]git config gc.auto 0
2019-09-13T19:31:54.5202880Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T19:31:54.5204990Z ##[command]git config --get-all http.proxy
2019-09-13T19:31:54.5208539Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-13T20:35:36.5494982Z .................................................................................................... 1500/9012
2019-09-13T20:35:43.0563928Z .................................................................................................... 1600/9012
2019-09-13T20:35:56.7051080Z ..........................................................i...............i......................... 1700/9012
2019-09-13T20:36:04.9942204Z .................................................................................................... 1800/9012
2019-09-13T20:36:21.1875510Z .................................................iiiii.............................................. 1900/9012
2019-09-13T20:36:33.3876654Z .................................................................................................... 2100/9012
2019-09-13T20:36:36.1657574Z .................................................................................................... 2200/9012
2019-09-13T20:36:40.0513578Z .................................................................................................... 2300/9012
2019-09-13T20:36:48.6159609Z .................................................................................................... 2400/9012
---
2019-09-13T20:39:58.7151265Z ....................................i...............i............................................... 4700/9012
2019-09-13T20:40:10.3495821Z .................................................................................................... 4800/9012
2019-09-13T20:40:17.3044301Z .................................................................................................... 4900/9012
2019-09-13T20:40:28.3720416Z .................................................................................................... 5000/9012
2019-09-13T20:40:34.8250578Z ...................ii.ii............................................................................ 5100/9012
2019-09-13T20:40:45.7391101Z .................................................................................................... 5300/9012
2019-09-13T20:40:56.2551056Z ...................................................................................i................ 5400/9012
2019-09-13T20:41:04.6057911Z .................................................................................................... 5500/9012
2019-09-13T20:41:10.4343288Z .................................................................................................... 5600/9012
2019-09-13T20:41:10.4343288Z .................................................................................................... 5600/9012
2019-09-13T20:41:21.3189810Z ..............................................................................ii...i..ii...........i 5700/9012
2019-09-13T20:41:47.8484876Z .................................................................................................... 5900/9012
2019-09-13T20:41:58.3505316Z .................................................................................................... 6000/9012
2019-09-13T20:41:58.3505316Z .................................................................................................... 6000/9012
2019-09-13T20:42:03.4901757Z ................................................................................i..ii............... 6100/9012
2019-09-13T20:42:34.9220505Z .................................................................................................... 6300/9012
2019-09-13T20:42:37.3539955Z .......................................i............................................................ 6400/9012
2019-09-13T20:42:39.6824830Z .................................................................................................... 6500/9012
2019-09-13T20:42:42.4119947Z ...........i........................................................................................ 6600/9012
---
2019-09-13T20:47:22.1702599Z  finished in 5.370
2019-09-13T20:47:22.1914492Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T20:47:22.3788913Z 
2019-09-13T20:47:22.3790267Z running 150 tests
2019-09-13T20:47:25.7925047Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-13T20:47:27.8720020Z ..iiii..............i.........iii.i.......ii......
2019-09-13T20:47:27.8726865Z 
2019-09-13T20:47:27.8733370Z  finished in 5.682
2019-09-13T20:47:27.8928149Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T20:47:28.0700490Z 
---
2019-09-13T20:47:30.1611713Z  finished in 2.267
2019-09-13T20:47:30.1808981Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T20:47:30.3393470Z 
2019-09-13T20:47:30.3394157Z running 9 tests
2019-09-13T20:47:30.3395138Z iiiiiiiii
2019-09-13T20:47:30.3398082Z 
2019-09-13T20:47:30.3398186Z  finished in 0.158
2019-09-13T20:47:30.3579618Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T20:47:30.5412165Z 
---
2019-09-13T20:47:49.1568325Z  finished in 18.798
2019-09-13T20:47:49.1829264Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T20:47:49.3721265Z 
2019-09-13T20:47:49.3721747Z running 123 tests
2019-09-13T20:48:14.3703460Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-13T20:48:19.2740655Z i.i.i......iii.i.....ii
2019-09-13T20:48:19.2742847Z 
2019-09-13T20:48:19.2748384Z  finished in 30.092
2019-09-13T20:48:19.2757714Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T20:48:19.2758779Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-13T21:02:40.3348593Z 
2019-09-13T21:02:40.3349760Z    Doc-tests core
2019-09-13T21:02:45.7060738Z 
2019-09-13T21:02:45.7061907Z running 2400 tests
2019-09-13T21:02:57.3292322Z ......iiiii......................................................................................... 100/2400
2019-09-13T21:03:08.5630079Z ...........................................................................ii....................... 200/2400
2019-09-13T21:03:21.3728187Z .................................................................................................i.. 300/2400
2019-09-13T21:03:35.1330280Z .................................................................................................... 400/2400
2019-09-13T21:03:45.9541694Z ............................................i..i.................iiii............................... 500/2400
2019-09-13T21:04:07.4149924Z .................................................................................................... 700/2400
2019-09-13T21:04:18.2322656Z .................................................................................................... 800/2400
2019-09-13T21:04:29.0114026Z .................................................................................................... 900/2400
2019-09-13T21:04:39.8945920Z .................................................................................................... 1000/2400
---
2019-09-13T21:09:40.0950962Z ............................................... 300/763
2019-09-13T21:09:40.0969124Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-09-13T21:09:40.1429738Z .................................................................................................... 400/763
2019-09-13T21:09:42.2255395Z .................................................................................................... 500/763
2019-09-13T21:09:42.2587992Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T21:09:42.2628682Z ....thread '<unnamed>thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-09-13T21:09:42.2646487Z .' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs.:1165:5
2019-09-13T21:09:42.2669921Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T21:09:42.4872091Z ............................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T21:09:42.4898169Z ........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T21:09:42.4905642Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T21:09:44.5426794Z .....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-09-13T21:09:44.5443011Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-09-13T21:09:44.5446801Z ......thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-09-13T21:09:44.5463725Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-09-13T21:09:53.9399417Z 
2019-09-13T21:09:53.9402704Z running 991 tests
2019-09-13T21:10:16.3043973Z i................................................................................................... 100/991
2019-09-13T21:10:28.5421921Z .................................................................................................... 200/991
2019-09-13T21:10:37.3218849Z .................iii......i......i...i......i....................................................... 300/991
2019-09-13T21:10:43.3835630Z .................................................................................................... 400/991
2019-09-13T21:10:51.8206865Z ..................................i..i.................................ii........................... 500/991
2019-09-13T21:11:07.7362332Z .................................................................................................... 700/991
2019-09-13T21:11:07.7362332Z .................................................................................................... 700/991
2019-09-13T21:11:16.2159697Z .................iiii............................................................................... 800/991
2019-09-13T21:11:31.8473022Z .................................................................................................... 900/991
2019-09-13T21:11:39.9017721Z .......................................iiii................................................
2019-09-13T21:11:39.9019927Z 
2019-09-13T21:11:39.9125921Z  finished in 251.094
2019-09-13T21:11:39.9144248Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T21:11:40.2481496Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-13T21:11:43.2312704Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-09-13T21:11:43.3229516Z error[E0432]: unresolved import `crate::test::RunStrategy`
2019-09-13T21:11:43.3230017Z  --> src/libtest/tests.rs:4:88
2019-09-13T21:11:43.3230281Z   |
2019-09-13T21:11:43.3232202Z 4 |     filter_tests, parse_opts, run_test, DynTestFn, DynTestName, MetricMap, RunIgnored, RunStrategy,
2019-09-13T21:11:43.3233096Z   |                                                                                        ^^^^^^^^^^^ no `RunStrategy` in `test`
2019-09-13T21:11:43.3233427Z 
2019-09-13T21:11:43.6583570Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T21:11:43.6584099Z     --> src/libtest/tests.rs:69:45
2019-09-13T21:11:43.6584411Z      |
2019-09-13T21:11:43.6584750Z 69   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.6585741Z      |                                             |
2019-09-13T21:11:43.6586068Z      |                                             call expression requires function
2019-09-13T21:11:43.6586312Z      | 
2019-09-13T21:11:43.6586563Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.6586563Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.6586780Z      |
2019-09-13T21:11:43.6587238Z 1098 |     InProcess,
2019-09-13T21:11:43.6587569Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T21:11:43.6587887Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.6588130Z      |
2019-09-13T21:11:43.6588457Z 69   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T21:11:43.6588830Z 
2019-09-13T21:11:43.6973307Z error[E0308]: mismatched types
2019-09-13T21:11:43.6973663Z   --> src/libtest/tests.rs:69:73
2019-09-13T21:11:43.6973980Z    |
2019-09-13T21:11:43.6973980Z    |
2019-09-13T21:11:43.6974358Z 69 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.6974851Z    |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.6975147Z    |
2019-09-13T21:11:43.6975523Z    = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.6975830Z               found type `Concurrent`
2019-09-13T21:11:43.7121264Z error[E0308]: mismatched types
2019-09-13T21:11:43.7121651Z   --> src/libtest/tests.rs:69:89
2019-09-13T21:11:43.7121906Z    |
2019-09-13T21:11:43.7121906Z    |
2019-09-13T21:11:43.7122288Z 69 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.7122769Z    |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.7123368Z    = note: expected type `Concurrent`
2019-09-13T21:11:43.7123647Z               found type `bool`
2019-09-13T21:11:43.7123688Z 
2019-09-13T21:11:43.7123688Z 
2019-09-13T21:11:43.7134562Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T21:11:43.7134922Z     --> src/libtest/tests.rs:87:45
2019-09-13T21:11:43.7135171Z      |
2019-09-13T21:11:43.7135744Z 87   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.7136534Z      |                                             |
2019-09-13T21:11:43.7136916Z      |                                             call expression requires function
2019-09-13T21:11:43.7137166Z      | 
2019-09-13T21:11:43.7137611Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.7137611Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.7137856Z      |
2019-09-13T21:11:43.7138142Z 1098 |     InProcess,
2019-09-13T21:11:43.7138530Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T21:11:43.7139471Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.7139978Z      |
2019-09-13T21:11:43.7140408Z 87   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T21:11:43.7140843Z 
2019-09-13T21:11:43.7287893Z error[E0308]: mismatched types
2019-09-13T21:11:43.7288238Z   --> src/libtest/tests.rs:87:73
2019-09-13T21:11:43.7288492Z    |
2019-09-13T21:11:43.7288492Z    |
2019-09-13T21:11:43.7288904Z 87 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.7289732Z    |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.7290057Z    |
2019-09-13T21:11:43.7290478Z    = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.7290770Z               found type `Concurrent`
2019-09-13T21:11:43.7429924Z error[E0308]: mismatched types
2019-09-13T21:11:43.7430296Z   --> src/libtest/tests.rs:87:89
2019-09-13T21:11:43.7430608Z    |
2019-09-13T21:11:43.7430608Z    |
2019-09-13T21:11:43.7430998Z 87 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.7431455Z    |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.7432060Z    = note: expected type `Concurrent`
2019-09-13T21:11:43.7432355Z               found type `bool`
2019-09-13T21:11:43.7432406Z 
2019-09-13T21:11:43.7432406Z 
2019-09-13T21:11:43.7443533Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-13T21:11:43.7443903Z     --> src/libtest/tests.rs:105:45
2019-09-13T21:11:43.7444351Z      |
2019-09-13T21:11:43.7444800Z 105  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-13T21:11:43.7445562Z      |                                             |
2019-09-13T21:11:43.7445943Z      |                                             call expression requires function
2019-09-13T21:11:43.7446348Z      | 
2019-09-13T21:11:43.7446628Z     ::: src/libtest/lib.rs:1103:5
2019-09-13T21:11:43.7446628Z     ::: src/libtest/lib.rs:1103:5
2019-09-13T21:11:43.7446890Z      |
2019-09-13T21:11:43.7447195Z 1103 |     SpawnPrimary,
2019-09-13T21:11:43.7447569Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-13T21:11:43.7447933Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.7448179Z      |
2019-09-13T21:11:43.7448549Z 105  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, false);
2019-09-13T21:11:43.7448983Z 
2019-09-13T21:11:43.7587942Z error[E0308]: mismatched types
2019-09-13T21:11:43.7588337Z    --> src/libtest/tests.rs:105:76
2019-09-13T21:11:43.7588593Z     |
2019-09-13T21:11:43.7588593Z     |
2019-09-13T21:11:43.7588969Z 105 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-13T21:11:43.7589854Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.7590163Z     |
2019-09-13T21:11:43.7590568Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.7590881Z                found type `Concurrent`
2019-09-13T21:11:43.7733691Z error[E0308]: mismatched types
2019-09-13T21:11:43.7734037Z    --> src/libtest/tests.rs:105:92
2019-09-13T21:11:43.7734287Z     |
2019-09-13T21:11:43.7734287Z     |
2019-09-13T21:11:43.7734696Z 105 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-13T21:11:43.7735152Z     |                                                                                            ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.7735772Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.7736051Z                found type `bool`
2019-09-13T21:11:43.7736110Z 
2019-09-13T21:11:43.7736110Z 
2019-09-13T21:11:43.7746016Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T21:11:43.7746423Z     --> src/libtest/tests.rs:125:45
2019-09-13T21:11:43.7746706Z      |
2019-09-13T21:11:43.7747078Z 125  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.7747825Z      |                                             |
2019-09-13T21:11:43.7748346Z      |                                             call expression requires function
2019-09-13T21:11:43.7748630Z      | 
2019-09-13T21:11:43.7748914Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.7748914Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.7749154Z      |
2019-09-13T21:11:43.7749459Z 1098 |     InProcess,
2019-09-13T21:11:43.7750167Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T21:11:43.7750581Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.7750854Z      |
2019-09-13T21:11:43.7751236Z 125  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T21:11:43.7751662Z 
2019-09-13T21:11:43.7890686Z error[E0308]: mismatched types
2019-09-13T21:11:43.7891029Z    --> src/libtest/tests.rs:125:73
2019-09-13T21:11:43.7891312Z     |
2019-09-13T21:11:43.7891312Z     |
2019-09-13T21:11:43.7891688Z 125 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.7892166Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.7892479Z     |
2019-09-13T21:11:43.7892860Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.7893182Z                found type `Concurrent`
2019-09-13T21:11:43.8028483Z error[E0308]: mismatched types
2019-09-13T21:11:43.8028851Z    --> src/libtest/tests.rs:125:89
2019-09-13T21:11:43.8029107Z     |
2019-09-13T21:11:43.8029107Z     |
2019-09-13T21:11:43.8029866Z 125 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.8030368Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.8031207Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.8031561Z                found type `bool`
2019-09-13T21:11:43.8031603Z 
2019-09-13T21:11:43.8031603Z 
2019-09-13T21:11:43.8042548Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-13T21:11:43.8042892Z     --> src/libtest/tests.rs:145:45
2019-09-13T21:11:43.8043142Z      |
2019-09-13T21:11:43.8043541Z 145  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T21:11:43.8044553Z      |                                             |
2019-09-13T21:11:43.8044934Z      |                                             call expression requires function
2019-09-13T21:11:43.8045187Z      | 
2019-09-13T21:11:43.8045488Z     ::: src/libtest/lib.rs:1103:5
2019-09-13T21:11:43.8045488Z     ::: src/libtest/lib.rs:1103:5
2019-09-13T21:11:43.8045732Z      |
2019-09-13T21:11:43.8046024Z 1103 |     SpawnPrimary,
2019-09-13T21:11:43.8046414Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-13T21:11:43.8046763Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.8047020Z      |
2019-09-13T21:11:43.8047420Z 145  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, true);
2019-09-13T21:11:43.8047825Z 
2019-09-13T21:11:43.8200069Z error[E0308]: mismatched types
2019-09-13T21:11:43.8200433Z    --> src/libtest/tests.rs:145:76
2019-09-13T21:11:43.8200741Z     |
2019-09-13T21:11:43.8200741Z     |
2019-09-13T21:11:43.8201130Z 145 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T21:11:43.8201660Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.8201938Z     |
2019-09-13T21:11:43.8202335Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.8202625Z                found type `Concurrent`
2019-09-13T21:11:43.8337447Z error[E0308]: mismatched types
2019-09-13T21:11:43.8337832Z    --> src/libtest/tests.rs:145:92
2019-09-13T21:11:43.8338101Z     |
2019-09-13T21:11:43.8338101Z     |
2019-09-13T21:11:43.8338487Z 145 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T21:11:43.8339225Z     |                                                                                            ^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.8340902Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.8341181Z                found type `bool`
2019-09-13T21:11:43.8341224Z 
2019-09-13T21:11:43.8341224Z 
2019-09-13T21:11:43.8351714Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T21:11:43.8352059Z     --> src/libtest/tests.rs:165:45
2019-09-13T21:11:43.8352312Z      |
2019-09-13T21:11:43.8352728Z 165  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.8353767Z      |                                             |
2019-09-13T21:11:43.8354132Z      |                                             call expression requires function
2019-09-13T21:11:43.8354383Z      | 
2019-09-13T21:11:43.8354687Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.8354687Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.8354932Z      |
2019-09-13T21:11:43.8355223Z 1098 |     InProcess,
2019-09-13T21:11:43.8355623Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T21:11:43.8355968Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.8356224Z      |
2019-09-13T21:11:43.8356617Z 165  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T21:11:43.8357019Z 
2019-09-13T21:11:43.8517415Z error[E0308]: mismatched types
2019-09-13T21:11:43.8517757Z    --> src/libtest/tests.rs:165:73
2019-09-13T21:11:43.8518039Z     |
2019-09-13T21:11:43.8518039Z     |
2019-09-13T21:11:43.8518660Z 165 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.8519206Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.8519911Z     |
2019-09-13T21:11:43.8520309Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.8520630Z                found type `Concurrent`
2019-09-13T21:11:43.8684692Z error[E0308]: mismatched types
2019-09-13T21:11:43.8685070Z    --> src/libtest/tests.rs:165:89
2019-09-13T21:11:43.8685345Z     |
2019-09-13T21:11:43.8685345Z     |
2019-09-13T21:11:43.8685725Z 165 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.8686402Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.8687081Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.8687366Z                found type `bool`
2019-09-13T21:11:43.8687408Z 
2019-09-13T21:11:43.8687408Z 
2019-09-13T21:11:43.8700665Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T21:11:43.8701267Z     --> src/libtest/tests.rs:187:45
2019-09-13T21:11:43.8701531Z      |
2019-09-13T21:11:43.8701942Z 187  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.8702794Z      |                                             |
2019-09-13T21:11:43.8703162Z      |                                             call expression requires function
2019-09-13T21:11:43.8703415Z      | 
2019-09-13T21:11:43.8703713Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.8703713Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.8703972Z      |
2019-09-13T21:11:43.8704264Z 1098 |     InProcess,
2019-09-13T21:11:43.8704646Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T21:11:43.8705002Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.8705253Z      |
2019-09-13T21:11:43.8705644Z 187  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T21:11:43.8706044Z 
2019-09-13T21:11:43.8850208Z error[E0308]: mismatched types
2019-09-13T21:11:43.8850568Z    --> src/libtest/tests.rs:187:73
2019-09-13T21:11:43.8850821Z     |
2019-09-13T21:11:43.8850821Z     |
2019-09-13T21:11:43.8851230Z 187 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.8851718Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.8852005Z     |
2019-09-13T21:11:43.8852385Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.8852673Z                found type `Concurrent`
2019-09-13T21:11:43.9002303Z error[E0308]: mismatched types
2019-09-13T21:11:43.9002650Z    --> src/libtest/tests.rs:187:89
2019-09-13T21:11:43.9002938Z     |
2019-09-13T21:11:43.9002938Z     |
2019-09-13T21:11:43.9003510Z 187 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.9004040Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.9004656Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.9004958Z                found type `bool`
2019-09-13T21:11:43.9005118Z 
2019-09-13T21:11:43.9005118Z 
2019-09-13T21:11:43.9024701Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T21:11:43.9025073Z     --> src/libtest/tests.rs:205:45
2019-09-13T21:11:43.9025328Z      |
2019-09-13T21:11:43.9025727Z 205  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.9026486Z      |                                             |
2019-09-13T21:11:43.9026866Z      |                                             call expression requires function
2019-09-13T21:11:43.9027136Z      | 
2019-09-13T21:11:43.9027419Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.9027419Z     ::: src/libtest/lib.rs:1098:5
2019-09-13T21:11:43.9027680Z      |
2019-09-13T21:11:43.9027975Z 1098 |     InProcess,
2019-09-13T21:11:43.9028351Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T21:11:43.9028715Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.9028968Z      |
2019-09-13T21:11:43.9029705Z 205  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T21:11:43.9030259Z 
2019-09-13T21:11:43.9182941Z error[E0308]: mismatched types
2019-09-13T21:11:43.9183339Z    --> src/libtest/tests.rs:205:73
2019-09-13T21:11:43.9183616Z     |
2019-09-13T21:11:43.9183616Z     |
2019-09-13T21:11:43.9184001Z 205 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.9184502Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.9184769Z     |
2019-09-13T21:11:43.9185164Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.9185471Z                found type `Concurrent`
2019-09-13T21:11:43.9324425Z error[E0308]: mismatched types
2019-09-13T21:11:43.9324961Z    --> src/libtest/tests.rs:205:89
2019-09-13T21:11:43.9325281Z     |
2019-09-13T21:11:43.9325281Z     |
2019-09-13T21:11:43.9325688Z 205 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T21:11:43.9326151Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.9326758Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.9327217Z                found type `bool`
2019-09-13T21:11:43.9327279Z 
2019-09-13T21:11:43.9327279Z 
2019-09-13T21:11:43.9337159Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-13T21:11:43.9337510Z     --> src/libtest/tests.rs:223:45
2019-09-13T21:11:43.9337796Z      |
2019-09-13T21:11:43.9338179Z 223  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T21:11:43.9338934Z      |                                             |
2019-09-13T21:11:43.9339307Z      |                                             call expression requires function
2019-09-13T21:11:43.9339951Z      | 
2019-09-13T21:11:43.9340247Z     ::: src/libtest/lib.rs:1103:5
2019-09-13T21:11:43.9340247Z     ::: src/libtest/lib.rs:1103:5
2019-09-13T21:11:43.9340505Z      |
2019-09-13T21:11:43.9340820Z 1103 |     SpawnPrimary,
2019-09-13T21:11:43.9341191Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-13T21:11:43.9341535Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-13T21:11:43.9341801Z      |
2019-09-13T21:11:43.9342169Z 223  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, true);
2019-09-13T21:11:43.9342600Z 
2019-09-13T21:11:43.9486070Z error[E0308]: mismatched types
2019-09-13T21:11:43.9486414Z    --> src/libtest/tests.rs:223:76
2019-09-13T21:11:43.9486697Z     |
2019-09-13T21:11:43.9486697Z     |
2019-09-13T21:11:43.9487079Z 223 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T21:11:43.9487565Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T21:11:43.9487864Z     |
2019-09-13T21:11:43.9488242Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:43.9488739Z                found type `Concurrent`
2019-09-13T21:11:43.9631337Z error[E0308]: mismatched types
2019-09-13T21:11:43.9631708Z    --> src/libtest/tests.rs:223:92
2019-09-13T21:11:43.9631963Z     |
2019-09-13T21:11:43.9631963Z     |
2019-09-13T21:11:43.9632345Z 223 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T21:11:43.9632826Z     |                                                                                            ^^^^ expected enum `Concurrent`, found bool
2019-09-13T21:11:43.9633686Z     = note: expected type `Concurrent`
2019-09-13T21:11:43.9633981Z                found type `bool`
2019-09-13T21:11:43.9634024Z 
2019-09-13T21:11:44.0143441Z error[E0308]: mismatched types
2019-09-13T21:11:44.0143441Z error[E0308]: mismatched types
2019-09-13T21:11:44.0143792Z    --> src/libtest/tests.rs:519:29
2019-09-13T21:11:44.0144049Z     |
2019-09-13T21:11:44.0144395Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T21:11:44.0144792Z     |                             ^^^^ expected &TestOpts, found struct `TestDesc`
2019-09-13T21:11:44.0145411Z     = note: expected type `&TestOpts`
2019-09-13T21:11:44.0145687Z                found type `TestDesc`
2019-09-13T21:11:44.0145725Z 
2019-09-13T21:11:44.0325914Z error[E0308]: mismatched types
2019-09-13T21:11:44.0325914Z error[E0308]: mismatched types
2019-09-13T21:11:44.0326258Z    --> src/libtest/tests.rs:519:35
2019-09-13T21:11:44.0326514Z     |
2019-09-13T21:11:44.0326863Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T21:11:44.0327295Z     |                                   ^^ expected struct `TestDesc`, found struct `std::sync::mpsc::Sender`
2019-09-13T21:11:44.0327890Z     = note: expected type `TestDesc`
2019-09-13T21:11:44.0328196Z                found type `std::sync::mpsc::Sender<_>`
2019-09-13T21:11:44.0328237Z 
2019-09-13T21:11:44.0491069Z error[E0308]: mismatched types
2019-09-13T21:11:44.0491069Z error[E0308]: mismatched types
2019-09-13T21:11:44.0491434Z    --> src/libtest/tests.rs:519:39
2019-09-13T21:11:44.0491694Z     |
2019-09-13T21:11:44.0492044Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T21:11:44.0492470Z     |                                       ^^^^ expected struct `std::sync::mpsc::Sender`, found bool
2019-09-13T21:11:44.0492728Z     |
2019-09-13T21:11:44.0493123Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:44.0493452Z 
2019-09-13T21:11:44.0670808Z error[E0308]: mismatched types
2019-09-13T21:11:44.0671362Z    --> src/libtest/tests.rs:538:29
2019-09-13T21:11:44.0671682Z     |
2019-09-13T21:11:44.0671682Z     |
2019-09-13T21:11:44.0672035Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T21:11:44.0672429Z     |                             ^^^^ expected &TestOpts, found struct `TestDesc`
2019-09-13T21:11:44.0673019Z     = note: expected type `&TestOpts`
2019-09-13T21:11:44.0673451Z                found type `TestDesc`
2019-09-13T21:11:44.0673491Z 
2019-09-13T21:11:44.0828917Z error[E0308]: mismatched types
2019-09-13T21:11:44.0828917Z error[E0308]: mismatched types
2019-09-13T21:11:44.0829255Z    --> src/libtest/tests.rs:538:35
2019-09-13T21:11:44.0829887Z     |
2019-09-13T21:11:44.0830268Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T21:11:44.0830705Z     |                                   ^^ expected struct `TestDesc`, found struct `std::sync::mpsc::Sender`
2019-09-13T21:11:44.0831298Z     = note: expected type `TestDesc`
2019-09-13T21:11:44.0831590Z                found type `std::sync::mpsc::Sender<_>`
2019-09-13T21:11:44.0831642Z 
2019-09-13T21:11:44.0976529Z error[E0308]: mismatched types
2019-09-13T21:11:44.0976529Z error[E0308]: mismatched types
2019-09-13T21:11:44.0976875Z    --> src/libtest/tests.rs:538:39
2019-09-13T21:11:44.0977129Z     |
2019-09-13T21:11:44.0977497Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T21:11:44.0977921Z     |                                       ^^^^ expected struct `std::sync::mpsc::Sender`, found bool
2019-09-13T21:11:44.0978178Z     |
2019-09-13T21:11:44.0978576Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T21:11:44.0978901Z 
2019-09-13T21:11:44.2400556Z error: aborting due to 34 previous errors
2019-09-13T21:11:44.2400695Z 
2019-09-13T21:11:44.2401029Z Some errors have detailed explanations: E0308, E0432, E0618.
---
2019-09-13T21:11:44.2680757Z == clock drift check ==
2019-09-13T21:11:44.2693962Z   local time: Fri Sep 13 21:11:44 UTC 2019
2019-09-13T21:11:44.3583657Z   network time: Fri, 13 Sep 2019 21:11:44 GMT
2019-09-13T21:11:44.3583740Z == end clock drift check ==
2019-09-13T21:11:44.8978583Z ##[error]Bash exited with code '1'.
2019-09-13T21:11:44.9022581Z ##[section]Starting: Checkout
2019-09-13T21:11:44.9024582Z ==============================================================================
2019-09-13T21:11:44.9024644Z Task         : Get sources
2019-09-13T21:11:44.9024698Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
