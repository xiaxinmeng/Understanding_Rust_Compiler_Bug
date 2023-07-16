plain
2019-09-13T02:07:11.6624965Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-13T02:07:11.6826074Z ##[command]git config gc.auto 0
2019-09-13T02:07:11.6904780Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-13T02:07:11.6968519Z ##[command]git config --get-all http.proxy
2019-09-13T02:07:11.7127070Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-13T03:10:54.3787042Z .................................................................................................... 1500/9012
2019-09-13T03:11:00.5225599Z .................................................................................................... 1600/9012
2019-09-13T03:11:13.7949396Z ..........................................................i...............i......................... 1700/9012
2019-09-13T03:11:21.9005711Z .................................................................................................... 1800/9012
2019-09-13T03:11:37.1725739Z .................................................iiiii.............................................. 1900/9012
2019-09-13T03:11:48.8350496Z .................................................................................................... 2100/9012
2019-09-13T03:11:51.6011440Z .................................................................................................... 2200/9012
2019-09-13T03:11:55.3425673Z .................................................................................................... 2300/9012
2019-09-13T03:12:03.7753654Z .................................................................................................... 2400/9012
---
2019-09-13T03:15:10.0247694Z ....................................i...............i............................................... 4700/9012
2019-09-13T03:15:21.9810798Z .................................................................................................... 4800/9012
2019-09-13T03:15:28.9652514Z .................................................................................................... 4900/9012
2019-09-13T03:15:40.0231337Z .................................................................................................... 5000/9012
2019-09-13T03:15:46.4348546Z ...................ii.ii............................................................................ 5100/9012
2019-09-13T03:15:57.4005670Z .................................................................................................... 5300/9012
2019-09-13T03:16:07.9041393Z ...................................................................................i................ 5400/9012
2019-09-13T03:16:16.4901211Z .................................................................................................... 5500/9012
2019-09-13T03:16:22.3414359Z .................................................................................................... 5600/9012
2019-09-13T03:16:22.3414359Z .................................................................................................... 5600/9012
2019-09-13T03:16:33.0971823Z ..............................................................................ii...i..ii...........i 5700/9012
2019-09-13T03:16:59.3570903Z .................................................................................................... 5900/9012
2019-09-13T03:17:10.0146868Z .................................................................................................... 6000/9012
2019-09-13T03:17:10.0146868Z .................................................................................................... 6000/9012
2019-09-13T03:17:19.4023368Z ................................................................................i..ii............... 6100/9012
2019-09-13T03:17:50.5691866Z .................................................................................................... 6300/9012
2019-09-13T03:17:53.1066347Z .......................................i............................................................ 6400/9012
2019-09-13T03:17:55.2802882Z .................................................................................................... 6500/9012
2019-09-13T03:17:57.9807469Z ...........i........................................................................................ 6600/9012
---
2019-09-13T03:22:41.0512909Z  finished in 5.235
2019-09-13T03:22:41.0707334Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:22:41.6972257Z 
2019-09-13T03:22:41.6978444Z running 150 tests
2019-09-13T03:22:44.6088146Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-13T03:22:46.7009550Z ..iiii..............i.........iii.i.......ii......
2019-09-13T03:22:46.7010414Z 
2019-09-13T03:22:46.7017023Z  finished in 5.631
2019-09-13T03:22:46.7218555Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:22:46.8809686Z 
---
2019-09-13T03:22:49.0203107Z  finished in 2.298
2019-09-13T03:22:49.0400341Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:22:49.7007937Z 
2019-09-13T03:22:49.7008140Z running 9 tests
2019-09-13T03:22:49.7009397Z iiiiiiiii
2019-09-13T03:22:49.7009811Z 
2019-09-13T03:22:49.7009855Z  finished in 0.161
2019-09-13T03:22:49.7010445Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:22:49.7010501Z 
---
2019-09-13T03:23:08.3285176Z  finished in 19.108
2019-09-13T03:23:08.3511802Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:23:08.5143790Z 
2019-09-13T03:23:08.5144098Z running 123 tests
2019-09-13T03:23:33.9648147Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-13T03:23:38.8991482Z i.i.i......iii.i.....ii
2019-09-13T03:23:38.8993105Z 
2019-09-13T03:23:38.8999183Z  finished in 30.549
2019-09-13T03:23:38.9008309Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:23:38.9009266Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-13T03:38:14.6556387Z 
2019-09-13T03:38:14.6557231Z    Doc-tests core
2019-09-13T03:38:20.0714759Z 
2019-09-13T03:38:20.0715563Z running 2400 tests
2019-09-13T03:38:31.9961505Z ......iiiii......................................................................................... 100/2400
2019-09-13T03:38:43.4631197Z ...........................................................................ii....................... 200/2400
2019-09-13T03:38:56.2955140Z .................................................................................................i.. 300/2400
2019-09-13T03:39:11.4375802Z .................................................................................................... 400/2400
2019-09-13T03:39:22.5669074Z ............................................i..i.................iiii............................... 500/2400
2019-09-13T03:39:44.2271062Z .................................................................................................... 700/2400
2019-09-13T03:39:55.4196371Z .................................................................................................... 800/2400
2019-09-13T03:40:06.4828398Z .................................................................................................... 900/2400
2019-09-13T03:40:17.9390757Z .................................................................................................... 1000/2400
---
2019-09-13T03:45:17.0560394Z .................................................................................................... 100/763
2019-09-13T03:45:17.5142333Z .................................................................................................... 200/763
2019-09-13T03:45:17.6010700Z .....................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1403:17
2019-09-13T03:45:17.6144359Z ............................................... 300/763
2019-09-13T03:45:17.6158508Z .thread '..<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-09-13T03:45:19.7214716Z .................................................................................................... 500/763
2019-09-13T03:45:19.7567393Z ....................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T03:45:19.7602526Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T03:45:19.7617029Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-09-13T03:45:19.7617029Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1165:5
2019-09-13T03:45:19.7642966Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T03:45:20.0463734Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T03:45:20.0502504Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError.', src/libcore/result.rs:1165.:5
2019-09-13T03:45:20.0519082Z ....thread '<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1165:5
2019-09-13T03:45:22.1010460Z ......................thread '<unnamed>' panicked at 'explicit panic', .....src/libstd/sync/mutex.rs:629:13
2019-09-13T03:45:22.1014901Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-09-13T03:45:22.1015346Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-09-13T03:45:22.1017859Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-09-13T03:45:31.5641741Z 
2019-09-13T03:45:31.5649674Z running 991 tests
2019-09-13T03:45:54.2344153Z i................................................................................................... 100/991
2019-09-13T03:46:06.5900360Z .................................................................................................... 200/991
2019-09-13T03:46:15.4259772Z .................iii......i......i...i......i....................................................... 300/991
2019-09-13T03:46:21.8948804Z .................................................................................................... 400/991
2019-09-13T03:46:30.3715374Z ..................................i..i.................................ii........................... 500/991
2019-09-13T03:46:46.3996123Z .................................................................................................... 700/991
2019-09-13T03:46:46.3996123Z .................................................................................................... 700/991
2019-09-13T03:46:55.0386653Z .................iiii............................................................................... 800/991
2019-09-13T03:47:11.0877454Z .................................................................................................... 900/991
2019-09-13T03:47:19.2120375Z .......................................iiii................................................
2019-09-13T03:47:19.2121508Z 
2019-09-13T03:47:19.2176398Z  finished in 252.882
2019-09-13T03:47:19.2193975Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-13T03:47:19.5431144Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-13T03:47:22.9074224Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-09-13T03:47:22.9984290Z error[E0432]: unresolved import `crate::test::RunStrategy`
2019-09-13T03:47:22.9997591Z  --> src/libtest/tests.rs:4:88
2019-09-13T03:47:23.0006008Z   |
2019-09-13T03:47:23.0016312Z 4 |     filter_tests, parse_opts, run_test, DynTestFn, DynTestName, MetricMap, RunIgnored, RunStrategy,
2019-09-13T03:47:23.0034369Z   |                                                                                        ^^^^^^^^^^^ no `RunStrategy` in `test`
2019-09-13T03:47:23.0034496Z 
2019-09-13T03:47:23.3502837Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T03:47:23.3503265Z     --> src/libtest/tests.rs:69:45
2019-09-13T03:47:23.3504138Z      |
2019-09-13T03:47:23.3506659Z 69   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.3508300Z      |                                             |
2019-09-13T03:47:23.3509379Z      |                                             call expression requires function
2019-09-13T03:47:23.3510135Z      | 
2019-09-13T03:47:23.3510785Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.3510785Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.3511417Z      |
2019-09-13T03:47:23.3512090Z 1099 |     InProcess,
2019-09-13T03:47:23.3512805Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T03:47:23.3513525Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.3514120Z      |
2019-09-13T03:47:23.3514883Z 69   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T03:47:23.3516202Z 
2019-09-13T03:47:23.3909453Z error[E0308]: mismatched types
2019-09-13T03:47:23.3909882Z   --> src/libtest/tests.rs:69:73
2019-09-13T03:47:23.3910200Z    |
2019-09-13T03:47:23.3910200Z    |
2019-09-13T03:47:23.3910606Z 69 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.3911105Z    |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.3911412Z    |
2019-09-13T03:47:23.3911812Z    = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.3912139Z               found type `Concurrent`
2019-09-13T03:47:23.4058405Z error[E0308]: mismatched types
2019-09-13T03:47:23.4059168Z   --> src/libtest/tests.rs:69:89
2019-09-13T03:47:23.4059458Z    |
2019-09-13T03:47:23.4059458Z    |
2019-09-13T03:47:23.4059847Z 69 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.4064704Z    |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.4065402Z    = note: expected type `Concurrent`
2019-09-13T03:47:23.4065707Z               found type `bool`
2019-09-13T03:47:23.4065752Z 
2019-09-13T03:47:23.4065752Z 
2019-09-13T03:47:23.4081763Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T03:47:23.4082395Z     --> src/libtest/tests.rs:87:45
2019-09-13T03:47:23.4082668Z      |
2019-09-13T03:47:23.4083081Z 87   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.4083872Z      |                                             |
2019-09-13T03:47:23.4084255Z      |                                             call expression requires function
2019-09-13T03:47:23.4084522Z      | 
2019-09-13T03:47:23.4084846Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.4084846Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.4085107Z      |
2019-09-13T03:47:23.4085411Z 1099 |     InProcess,
2019-09-13T03:47:23.4085805Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T03:47:23.4086169Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.4086431Z      |
2019-09-13T03:47:23.4086950Z 87   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T03:47:23.4087430Z 
2019-09-13T03:47:23.4263888Z error[E0308]: mismatched types
2019-09-13T03:47:23.4264253Z   --> src/libtest/tests.rs:87:73
2019-09-13T03:47:23.4264522Z    |
2019-09-13T03:47:23.4264522Z    |
2019-09-13T03:47:23.4264927Z 87 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.4265422Z    |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.4265737Z    |
2019-09-13T03:47:23.4266136Z    = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.4266447Z               found type `Concurrent`
2019-09-13T03:47:23.4440715Z error[E0308]: mismatched types
2019-09-13T03:47:23.4441094Z   --> src/libtest/tests.rs:87:89
2019-09-13T03:47:23.4441391Z    |
2019-09-13T03:47:23.4441391Z    |
2019-09-13T03:47:23.4441784Z 87 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.4442273Z    |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.4442907Z    = note: expected type `Concurrent`
2019-09-13T03:47:23.4443501Z               found type `bool`
2019-09-13T03:47:23.4443545Z 
2019-09-13T03:47:23.4443545Z 
2019-09-13T03:47:23.4452179Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-13T03:47:23.4452555Z     --> src/libtest/tests.rs:105:45
2019-09-13T03:47:23.4452816Z      |
2019-09-13T03:47:23.4453208Z 105  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-13T03:47:23.4454007Z      |                                             |
2019-09-13T03:47:23.4454400Z      |                                             call expression requires function
2019-09-13T03:47:23.4454679Z      | 
2019-09-13T03:47:23.4454973Z     ::: src/libtest/lib.rs:1104:5
2019-09-13T03:47:23.4454973Z     ::: src/libtest/lib.rs:1104:5
2019-09-13T03:47:23.4455246Z      |
2019-09-13T03:47:23.4455557Z 1104 |     SpawnPrimary,
2019-09-13T03:47:23.4455944Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-13T03:47:23.4456511Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.4456831Z      |
2019-09-13T03:47:23.4457213Z 105  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, false);
2019-09-13T03:47:23.4457663Z 
2019-09-13T03:47:23.4600430Z error[E0308]: mismatched types
2019-09-13T03:47:23.4600809Z    --> src/libtest/tests.rs:105:76
2019-09-13T03:47:23.4601069Z     |
2019-09-13T03:47:23.4601069Z     |
2019-09-13T03:47:23.4601458Z 105 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-13T03:47:23.4601987Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.4602267Z     |
2019-09-13T03:47:23.4602686Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.4602988Z                found type `Concurrent`
2019-09-13T03:47:23.4753051Z error[E0308]: mismatched types
2019-09-13T03:47:23.4753405Z    --> src/libtest/tests.rs:105:92
2019-09-13T03:47:23.4753672Z     |
2019-09-13T03:47:23.4753672Z     |
2019-09-13T03:47:23.4754108Z 105 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-13T03:47:23.4754586Z     |                                                                                            ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.4755489Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.4755778Z                found type `bool`
2019-09-13T03:47:23.4755839Z 
2019-09-13T03:47:23.4755839Z 
2019-09-13T03:47:23.4767254Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T03:47:23.4767597Z     --> src/libtest/tests.rs:125:45
2019-09-13T03:47:23.4767895Z      |
2019-09-13T03:47:23.4768302Z 125  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.4769729Z      |                                             |
2019-09-13T03:47:23.4770111Z      |                                             call expression requires function
2019-09-13T03:47:23.4770394Z      | 
2019-09-13T03:47:23.4770691Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.4770691Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.4770947Z      |
2019-09-13T03:47:23.4771266Z 1099 |     InProcess,
2019-09-13T03:47:23.4771835Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T03:47:23.4772253Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.4772549Z      |
2019-09-13T03:47:23.4772929Z 125  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T03:47:23.4773356Z 
2019-09-13T03:47:23.4930387Z error[E0308]: mismatched types
2019-09-13T03:47:23.4930752Z    --> src/libtest/tests.rs:125:73
2019-09-13T03:47:23.4931053Z     |
2019-09-13T03:47:23.4931053Z     |
2019-09-13T03:47:23.4931459Z 125 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.4931953Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.4932263Z     |
2019-09-13T03:47:23.4932656Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.4932977Z                found type `Concurrent`
2019-09-13T03:47:23.5073154Z error[E0308]: mismatched types
2019-09-13T03:47:23.5073529Z    --> src/libtest/tests.rs:125:89
2019-09-13T03:47:23.5073792Z     |
2019-09-13T03:47:23.5073792Z     |
2019-09-13T03:47:23.5074195Z 125 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.5074689Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.5075590Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.5075884Z                found type `bool`
2019-09-13T03:47:23.5075926Z 
2019-09-13T03:47:23.5075926Z 
2019-09-13T03:47:23.5090450Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-13T03:47:23.5090798Z     --> src/libtest/tests.rs:145:45
2019-09-13T03:47:23.5091073Z      |
2019-09-13T03:47:23.5091483Z 145  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T03:47:23.5092266Z      |                                             |
2019-09-13T03:47:23.5092641Z      |                                             call expression requires function
2019-09-13T03:47:23.5092903Z      | 
2019-09-13T03:47:23.5093212Z     ::: src/libtest/lib.rs:1104:5
2019-09-13T03:47:23.5093212Z     ::: src/libtest/lib.rs:1104:5
2019-09-13T03:47:23.5093468Z      |
2019-09-13T03:47:23.5093968Z 1104 |     SpawnPrimary,
2019-09-13T03:47:23.5094436Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-13T03:47:23.5094811Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.5095071Z      |
2019-09-13T03:47:23.5095467Z 145  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, true);
2019-09-13T03:47:23.5095882Z 
2019-09-13T03:47:23.5239527Z error[E0308]: mismatched types
2019-09-13T03:47:23.5239890Z    --> src/libtest/tests.rs:145:76
2019-09-13T03:47:23.5240156Z     |
2019-09-13T03:47:23.5240156Z     |
2019-09-13T03:47:23.5240566Z 145 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T03:47:23.5241072Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.5241367Z     |
2019-09-13T03:47:23.5241759Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.5242062Z                found type `Concurrent`
2019-09-13T03:47:23.5384351Z error[E0308]: mismatched types
2019-09-13T03:47:23.5384703Z    --> src/libtest/tests.rs:145:92
2019-09-13T03:47:23.5384993Z     |
2019-09-13T03:47:23.5384993Z     |
2019-09-13T03:47:23.5385386Z 145 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T03:47:23.5386109Z     |                                                                                            ^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.5386751Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.5387060Z                found type `bool`
2019-09-13T03:47:23.5387103Z 
2019-09-13T03:47:23.5387103Z 
2019-09-13T03:47:23.5407025Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T03:47:23.5407398Z     --> src/libtest/tests.rs:165:45
2019-09-13T03:47:23.5407664Z      |
2019-09-13T03:47:23.5408067Z 165  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.5409406Z      |                                             |
2019-09-13T03:47:23.5409832Z      |                                             call expression requires function
2019-09-13T03:47:23.5410102Z      | 
2019-09-13T03:47:23.5410604Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.5410604Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.5410952Z      |
2019-09-13T03:47:23.5411260Z 1099 |     InProcess,
2019-09-13T03:47:23.5411651Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T03:47:23.5412019Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.5412282Z      |
2019-09-13T03:47:23.5412661Z 165  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T03:47:23.5413100Z 
2019-09-13T03:47:23.5555403Z error[E0308]: mismatched types
2019-09-13T03:47:23.5555784Z    --> src/libtest/tests.rs:165:73
2019-09-13T03:47:23.5556070Z     |
2019-09-13T03:47:23.5556070Z     |
2019-09-13T03:47:23.5556454Z 165 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.5556956Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.5557231Z     |
2019-09-13T03:47:23.5557635Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.5557946Z                found type `Concurrent`
2019-09-13T03:47:23.5702087Z error[E0308]: mismatched types
2019-09-13T03:47:23.5702446Z    --> src/libtest/tests.rs:165:89
2019-09-13T03:47:23.5702997Z     |
2019-09-13T03:47:23.5702997Z     |
2019-09-13T03:47:23.5703411Z 165 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.5703882Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.5704530Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.5704824Z                found type `bool`
2019-09-13T03:47:23.5704868Z 
2019-09-13T03:47:23.5704868Z 
2019-09-13T03:47:23.5714010Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T03:47:23.5714374Z     --> src/libtest/tests.rs:187:45
2019-09-13T03:47:23.5714663Z      |
2019-09-13T03:47:23.5715057Z 187  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.5715832Z      |                                             |
2019-09-13T03:47:23.5716384Z      |                                             call expression requires function
2019-09-13T03:47:23.5716729Z      | 
2019-09-13T03:47:23.5717027Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.5717027Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.5717295Z      |
2019-09-13T03:47:23.5717596Z 1099 |     InProcess,
2019-09-13T03:47:23.5717991Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T03:47:23.5718342Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.5718616Z      |
2019-09-13T03:47:23.5719348Z 187  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T03:47:23.5719801Z 
2019-09-13T03:47:23.5863219Z error[E0308]: mismatched types
2019-09-13T03:47:23.5863587Z    --> src/libtest/tests.rs:187:73
2019-09-13T03:47:23.5863875Z     |
2019-09-13T03:47:23.5863875Z     |
2019-09-13T03:47:23.5864266Z 187 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.5864754Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.5865050Z     |
2019-09-13T03:47:23.5865449Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.5865770Z                found type `Concurrent`
2019-09-13T03:47:23.6042614Z error[E0308]: mismatched types
2019-09-13T03:47:23.6042998Z    --> src/libtest/tests.rs:187:89
2019-09-13T03:47:23.6043265Z     |
2019-09-13T03:47:23.6043265Z     |
2019-09-13T03:47:23.6043654Z 187 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.6044140Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.6044771Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.6046432Z                found type `bool`
2019-09-13T03:47:23.6046517Z 
2019-09-13T03:47:23.6046517Z 
2019-09-13T03:47:23.6069453Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-13T03:47:23.6069820Z     --> src/libtest/tests.rs:205:45
2019-09-13T03:47:23.6070085Z      |
2019-09-13T03:47:23.6070493Z 205  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.6071462Z      |                                             |
2019-09-13T03:47:23.6071991Z      |                                             call expression requires function
2019-09-13T03:47:23.6072274Z      | 
2019-09-13T03:47:23.6072568Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.6072568Z     ::: src/libtest/lib.rs:1099:5
2019-09-13T03:47:23.6072849Z      |
2019-09-13T03:47:23.6073154Z 1099 |     InProcess,
2019-09-13T03:47:23.6073534Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-13T03:47:23.6073990Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.6074252Z      |
2019-09-13T03:47:23.6074662Z 205  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-13T03:47:23.6075092Z 
2019-09-13T03:47:23.6222302Z error[E0308]: mismatched types
2019-09-13T03:47:23.6222765Z    --> src/libtest/tests.rs:205:73
2019-09-13T03:47:23.6223028Z     |
2019-09-13T03:47:23.6223028Z     |
2019-09-13T03:47:23.6223439Z 205 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.6223948Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.6224228Z     |
2019-09-13T03:47:23.6224640Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.6225190Z                found type `Concurrent`
2019-09-13T03:47:23.6364966Z error[E0308]: mismatched types
2019-09-13T03:47:23.6365313Z    --> src/libtest/tests.rs:205:89
2019-09-13T03:47:23.6365604Z     |
2019-09-13T03:47:23.6365604Z     |
2019-09-13T03:47:23.6365997Z 205 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-13T03:47:23.6366481Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.6367119Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.6367439Z                found type `bool`
2019-09-13T03:47:23.6367483Z 
2019-09-13T03:47:23.6367483Z 
2019-09-13T03:47:23.6379013Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-13T03:47:23.6379388Z     --> src/libtest/tests.rs:223:45
2019-09-13T03:47:23.6379650Z      |
2019-09-13T03:47:23.6380041Z 223  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T03:47:23.6381061Z      |                                             |
2019-09-13T03:47:23.6381448Z      |                                             call expression requires function
2019-09-13T03:47:23.6381737Z      | 
2019-09-13T03:47:23.6382029Z     ::: src/libtest/lib.rs:1104:5
2019-09-13T03:47:23.6382029Z     ::: src/libtest/lib.rs:1104:5
2019-09-13T03:47:23.6382284Z      |
2019-09-13T03:47:23.6382612Z 1104 |     SpawnPrimary,
2019-09-13T03:47:23.6382995Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-13T03:47:23.6383376Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-13T03:47:23.6383642Z      |
2019-09-13T03:47:23.6384023Z 223  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, true);
2019-09-13T03:47:23.6384462Z 
2019-09-13T03:47:23.6535121Z error[E0308]: mismatched types
2019-09-13T03:47:23.6535493Z    --> src/libtest/tests.rs:223:76
2019-09-13T03:47:23.6535759Z     |
2019-09-13T03:47:23.6535759Z     |
2019-09-13T03:47:23.6536166Z 223 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T03:47:23.6536681Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-13T03:47:23.6537196Z     |
2019-09-13T03:47:23.6537610Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.6537913Z                found type `Concurrent`
2019-09-13T03:47:23.6680899Z error[E0308]: mismatched types
2019-09-13T03:47:23.6681242Z    --> src/libtest/tests.rs:223:92
2019-09-13T03:47:23.6681502Z     |
2019-09-13T03:47:23.6681502Z     |
2019-09-13T03:47:23.6681925Z 223 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-13T03:47:23.6682402Z     |                                                                                            ^^^^ expected enum `Concurrent`, found bool
2019-09-13T03:47:23.6683051Z     = note: expected type `Concurrent`
2019-09-13T03:47:23.6683340Z                found type `bool`
2019-09-13T03:47:23.6683382Z 
2019-09-13T03:47:23.7252951Z error[E0308]: mismatched types
2019-09-13T03:47:23.7252951Z error[E0308]: mismatched types
2019-09-13T03:47:23.7253354Z    --> src/libtest/tests.rs:519:29
2019-09-13T03:47:23.7253626Z     |
2019-09-13T03:47:23.7254190Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T03:47:23.7254695Z     |                             ^^^^ expected &TestOpts, found struct `TestDesc`
2019-09-13T03:47:23.7255334Z     = note: expected type `&TestOpts`
2019-09-13T03:47:23.7255621Z                found type `TestDesc`
2019-09-13T03:47:23.7255661Z 
2019-09-13T03:47:23.7438147Z error[E0308]: mismatched types
2019-09-13T03:47:23.7438147Z error[E0308]: mismatched types
2019-09-13T03:47:23.7438497Z    --> src/libtest/tests.rs:519:35
2019-09-13T03:47:23.7438739Z     |
2019-09-13T03:47:23.7439399Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T03:47:23.7439830Z     |                                   ^^ expected struct `TestDesc`, found struct `std::sync::mpsc::Sender`
2019-09-13T03:47:23.7440411Z     = note: expected type `TestDesc`
2019-09-13T03:47:23.7440683Z                found type `std::sync::mpsc::Sender<_>`
2019-09-13T03:47:23.7440722Z 
2019-09-13T03:47:23.7585081Z error[E0308]: mismatched types
2019-09-13T03:47:23.7585081Z error[E0308]: mismatched types
2019-09-13T03:47:23.7585396Z    --> src/libtest/tests.rs:519:39
2019-09-13T03:47:23.7585658Z     |
2019-09-13T03:47:23.7585963Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T03:47:23.7586367Z     |                                       ^^^^ expected struct `std::sync::mpsc::Sender`, found bool
2019-09-13T03:47:23.7586643Z     |
2019-09-13T03:47:23.7586992Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.7587543Z 
2019-09-13T03:47:23.7750618Z error[E0308]: mismatched types
2019-09-13T03:47:23.7750925Z    --> src/libtest/tests.rs:538:29
2019-09-13T03:47:23.7751191Z     |
2019-09-13T03:47:23.7751191Z     |
2019-09-13T03:47:23.7751494Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T03:47:23.7751886Z     |                             ^^^^ expected &TestOpts, found struct `TestDesc`
2019-09-13T03:47:23.7752422Z     = note: expected type `&TestOpts`
2019-09-13T03:47:23.7752707Z                found type `TestDesc`
2019-09-13T03:47:23.7752743Z 
2019-09-13T03:47:23.7917737Z error[E0308]: mismatched types
2019-09-13T03:47:23.7917737Z error[E0308]: mismatched types
2019-09-13T03:47:23.7918044Z    --> src/libtest/tests.rs:538:35
2019-09-13T03:47:23.7918307Z     |
2019-09-13T03:47:23.7918610Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T03:47:23.7919512Z     |                                   ^^ expected struct `TestDesc`, found struct `std::sync::mpsc::Sender`
2019-09-13T03:47:23.7920122Z     = note: expected type `TestDesc`
2019-09-13T03:47:23.7920430Z                found type `std::sync::mpsc::Sender<_>`
2019-09-13T03:47:23.7920469Z 
2019-09-13T03:47:23.8067661Z error[E0308]: mismatched types
2019-09-13T03:47:23.8067661Z error[E0308]: mismatched types
2019-09-13T03:47:23.8067974Z    --> src/libtest/tests.rs:538:39
2019-09-13T03:47:23.8068241Z     |
2019-09-13T03:47:23.8068547Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-13T03:47:23.8069219Z     |                                       ^^^^ expected struct `std::sync::mpsc::Sender`, found bool
2019-09-13T03:47:23.8069537Z     |
2019-09-13T03:47:23.8069889Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-13T03:47:23.8070214Z 
2019-09-13T03:47:23.9521613Z error: aborting due to 34 previous errors
2019-09-13T03:47:23.9521730Z 
2019-09-13T03:47:23.9522118Z Some errors have detailed explanations: E0308, E0432, E0618.
---
2019-09-13T03:47:23.9787856Z == clock drift check ==
2019-09-13T03:47:23.9806965Z   local time: Fri Sep 13 03:47:23 UTC 2019
2019-09-13T03:47:24.0684064Z   network time: Fri, 13 Sep 2019 03:47:24 GMT
2019-09-13T03:47:24.0684445Z == end clock drift check ==
2019-09-13T03:47:24.6504976Z ##[error]Bash exited with code '1'.
2019-09-13T03:47:24.6543054Z ##[section]Starting: Checkout
2019-09-13T03:47:24.6545077Z ==============================================================================
2019-09-13T03:47:24.6545153Z Task         : Get sources
2019-09-13T03:47:24.6545199Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
