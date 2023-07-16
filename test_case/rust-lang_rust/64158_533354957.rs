plain
2019-09-19T22:37:28.8578213Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T22:37:28.8782248Z ##[command]git config gc.auto 0
2019-09-19T22:37:28.8918333Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T22:37:28.8993461Z ##[command]git config --get-all http.proxy
2019-09-19T22:37:28.9148518Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-19T23:41:32.7270222Z .................................................................................................... 1500/9024
2019-09-19T23:41:38.8977260Z .................................................................................................... 1600/9024
2019-09-19T23:41:51.6695277Z .................................................................i...............i.................. 1700/9024
2019-09-19T23:41:59.1456319Z .................................................................................................... 1800/9024
2019-09-19T23:42:14.6796925Z ........................................................iiiii....................................... 1900/9024
2019-09-19T23:42:26.7192622Z .................................................................................................... 2100/9024
2019-09-19T23:42:29.3671358Z .................................................................................................... 2200/9024
2019-09-19T23:42:32.8949102Z .................................................................................................... 2300/9024
2019-09-19T23:42:41.4848383Z .................................................................................................... 2400/9024
---
2019-09-19T23:45:44.6507282Z ............................................i...............i....................................... 4700/9024
2019-09-19T23:45:55.6130831Z .................................................................................................... 4800/9024
2019-09-19T23:46:02.7818067Z .................................................................................................... 4900/9024
2019-09-19T23:46:12.7124473Z .................................................................................................... 5000/9024
2019-09-19T23:46:20.7128128Z ............................ii.ii................................................................... 5100/9024
2019-09-19T23:46:31.1755045Z .................................................................................................... 5300/9024
2019-09-19T23:46:41.8236338Z ............................................................................................i....... 5400/9024
2019-09-19T23:46:50.3826340Z .................................................................................................... 5500/9024
2019-09-19T23:46:55.4426484Z .................................................................................................... 5600/9024
2019-09-19T23:46:55.4426484Z .................................................................................................... 5600/9024
2019-09-19T23:47:06.2501199Z .......................................................................................ii...i..ii... 5700/9024
2019-09-19T23:47:31.9735717Z .................................................................................................... 5900/9024
2019-09-19T23:47:42.2911699Z .................................................................................................... 6000/9024
2019-09-19T23:47:42.2911699Z .................................................................................................... 6000/9024
2019-09-19T23:47:51.5507142Z .........................................................................................i..ii...... 6100/9024
2019-09-19T23:48:20.4985971Z .................................................................................................... 6300/9024
2019-09-19T23:48:24.9186458Z ................................................i................................................... 6400/9024
2019-09-19T23:48:27.2643815Z .................................................................................................... 6500/9024
2019-09-19T23:48:29.9008536Z ....................i............................................................................... 6600/9024
---
2019-09-19T23:53:08.6690339Z  finished in 5.220
2019-09-19T23:53:08.6879254Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T23:53:08.8614777Z 
2019-09-19T23:53:08.8615056Z running 150 tests
2019-09-19T23:53:12.2001393Z i....iii......iii..iiii....i.............................i..i..................i....i.........ii.i.i 100/150
2019-09-19T23:53:14.2337509Z ..iiii..............i.........iii.i.......ii......
2019-09-19T23:53:14.2338354Z 
2019-09-19T23:53:14.2345425Z  finished in 5.546
2019-09-19T23:53:14.2549177Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T23:53:14.4238336Z 
---
2019-09-19T23:53:16.5544805Z  finished in 2.300
2019-09-19T23:53:16.5754350Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T23:53:16.7341746Z 
2019-09-19T23:53:16.7342728Z running 9 tests
2019-09-19T23:53:16.7344236Z iiiiiiiii
2019-09-19T23:53:16.7344750Z 
2019-09-19T23:53:16.7344962Z  finished in 0.158
2019-09-19T23:53:16.7540018Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T23:53:16.9164680Z 
---
2019-09-19T23:53:35.6744730Z  finished in 18.920
2019-09-19T23:53:35.6951826Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T23:53:35.8622012Z 
2019-09-19T23:53:35.8622270Z running 123 tests
2019-09-19T23:54:00.2717323Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-19T23:54:05.0752451Z i.i.i......iii.i.....ii
2019-09-19T23:54:05.0754481Z 
2019-09-19T23:54:05.0758330Z  finished in 29.380
2019-09-19T23:54:05.0769326Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-19T23:54:05.0770127Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-20T00:08:22.1421401Z 
2019-09-20T00:08:22.1422439Z    Doc-tests core
2019-09-20T00:08:27.3156211Z 
2019-09-20T00:08:27.3157405Z running 2401 tests
2019-09-20T00:08:39.0382502Z ......iiiii......................................................................................... 100/2401
2019-09-20T00:08:50.4921380Z ...........................................................................ii....................... 200/2401
2019-09-20T00:09:03.9436445Z .................................................................................................i.. 300/2401
2019-09-20T00:09:17.7848315Z .................................................................................................... 400/2401
2019-09-20T00:09:28.8407248Z ............................................i..i.................iiii............................... 500/2401
2019-09-20T00:09:50.2817400Z .................................................................................................... 700/2401
2019-09-20T00:10:01.5140617Z .................................................................................................... 800/2401
2019-09-20T00:10:12.5195426Z .................................................................................................... 900/2401
2019-09-20T00:10:23.4049257Z .................................................................................................... 1000/2401
---
2019-09-20T00:15:30.7514694Z 
2019-09-20T00:15:30.7516141Z running 991 tests
2019-09-20T00:15:52.6630064Z i................................................................................................... 100/991
2019-09-20T00:16:04.8604006Z .................................................................................................... 200/991
2019-09-20T00:16:13.4847604Z .................iii......i......i...i......i....................................................... 300/991
2019-09-20T00:16:19.5206116Z .................................................................................................... 400/991
2019-09-20T00:16:27.8087803Z ..................................i..i.................................ii........................... 500/991
2019-09-20T00:16:43.4607679Z .................................................................................................... 700/991
2019-09-20T00:16:43.4607679Z .................................................................................................... 700/991
2019-09-20T00:16:51.9984990Z .................iiii............................................................................... 800/991
2019-09-20T00:17:07.5488206Z .................................................................................................... 900/991
2019-09-20T00:17:15.4732389Z .......................................iiii................................................
2019-09-20T00:17:15.4733416Z 
2019-09-20T00:17:15.4847994Z  finished in 246.351
2019-09-20T00:17:15.4867577Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-20T00:17:15.6971278Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-20T00:17:18.4663294Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-09-20T00:17:18.5531918Z error[E0432]: unresolved import `crate::test::RunStrategy`
2019-09-20T00:17:18.5533468Z  --> src/libtest/tests.rs:4:88
2019-09-20T00:17:18.5534419Z   |
2019-09-20T00:17:18.5535033Z 4 |     filter_tests, parse_opts, run_test, DynTestFn, DynTestName, MetricMap, RunIgnored, RunStrategy,
2019-09-20T00:17:18.5535662Z   |                                                                                        ^^^^^^^^^^^ no `RunStrategy` in `test`
2019-09-20T00:17:18.5535906Z 
2019-09-20T00:17:18.8890277Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-20T00:17:18.8890680Z     --> src/libtest/tests.rs:69:45
2019-09-20T00:17:18.8890949Z      |
2019-09-20T00:17:18.8891658Z 69   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:18.8892331Z      |                                             |
2019-09-20T00:17:18.8892651Z      |                                             call expression requires function
2019-09-20T00:17:18.8893052Z      | 
2019-09-20T00:17:18.8893368Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:18.8893368Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:18.8893580Z      |
2019-09-20T00:17:18.8894376Z 1068 |     InProcess,
2019-09-20T00:17:18.8894736Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-20T00:17:18.8895036Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:18.8895270Z      |
2019-09-20T00:17:18.8895591Z 69   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-20T00:17:18.8895974Z 
2019-09-20T00:17:18.9289928Z error[E0308]: mismatched types
2019-09-20T00:17:18.9290264Z   --> src/libtest/tests.rs:69:73
2019-09-20T00:17:18.9290554Z    |
2019-09-20T00:17:18.9290554Z    |
2019-09-20T00:17:18.9290884Z 69 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:18.9291301Z    |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:18.9291562Z    |
2019-09-20T00:17:18.9291933Z    = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:18.9292259Z 
2019-09-20T00:17:18.9455628Z error[E0308]: mismatched types
2019-09-20T00:17:18.9455989Z   --> src/libtest/tests.rs:69:89
2019-09-20T00:17:18.9456211Z    |
2019-09-20T00:17:18.9456211Z    |
2019-09-20T00:17:18.9456536Z 69 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:18.9456954Z    |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:18.9457491Z    = note: expected type `Concurrent`
2019-09-20T00:17:18.9457737Z               found type `bool`
2019-09-20T00:17:18.9457775Z 
2019-09-20T00:17:18.9457775Z 
2019-09-20T00:17:18.9482144Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-20T00:17:18.9482732Z     --> src/libtest/tests.rs:87:45
2019-09-20T00:17:18.9482958Z      |
2019-09-20T00:17:18.9483314Z 87   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:18.9484040Z      |                                             |
2019-09-20T00:17:18.9484513Z      |                                             call expression requires function
2019-09-20T00:17:18.9484794Z      | 
2019-09-20T00:17:18.9485062Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:18.9485062Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:18.9485286Z      |
2019-09-20T00:17:18.9485536Z 1068 |     InProcess,
2019-09-20T00:17:18.9485870Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-20T00:17:18.9486172Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:18.9486384Z      |
2019-09-20T00:17:18.9486726Z 87   |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-20T00:17:18.9487080Z 
2019-09-20T00:17:18.9639604Z error[E0308]: mismatched types
2019-09-20T00:17:18.9639922Z   --> src/libtest/tests.rs:87:73
2019-09-20T00:17:18.9640135Z    |
2019-09-20T00:17:18.9640135Z    |
2019-09-20T00:17:18.9640476Z 87 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:18.9640885Z    |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:18.9641125Z    |
2019-09-20T00:17:18.9641452Z    = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:18.9641760Z 
2019-09-20T00:17:18.9784166Z error[E0308]: mismatched types
2019-09-20T00:17:18.9784859Z   --> src/libtest/tests.rs:87:89
2019-09-20T00:17:18.9785321Z    |
2019-09-20T00:17:18.9785321Z    |
2019-09-20T00:17:18.9785887Z 87 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:18.9786512Z    |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:18.9787592Z    = note: expected type `Concurrent`
2019-09-20T00:17:18.9788038Z               found type `bool`
2019-09-20T00:17:18.9788470Z 
2019-09-20T00:17:18.9788470Z 
2019-09-20T00:17:18.9800731Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-20T00:17:18.9801045Z     --> src/libtest/tests.rs:105:45
2019-09-20T00:17:18.9801266Z      |
2019-09-20T00:17:18.9801621Z 105  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-20T00:17:18.9802578Z      |                                             |
2019-09-20T00:17:18.9802972Z      |                                             call expression requires function
2019-09-20T00:17:18.9803210Z      | 
2019-09-20T00:17:18.9803479Z     ::: src/libtest/lib.rs:1073:5
2019-09-20T00:17:18.9803479Z     ::: src/libtest/lib.rs:1073:5
2019-09-20T00:17:18.9804182Z      |
2019-09-20T00:17:18.9804460Z 1073 |     SpawnPrimary,
2019-09-20T00:17:18.9804811Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-20T00:17:18.9805114Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:18.9805343Z      |
2019-09-20T00:17:18.9805692Z 105  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, false);
2019-09-20T00:17:18.9806059Z 
2019-09-20T00:17:18.9951516Z error[E0308]: mismatched types
2019-09-20T00:17:18.9951815Z    --> src/libtest/tests.rs:105:76
2019-09-20T00:17:18.9952181Z     |
2019-09-20T00:17:18.9952181Z     |
2019-09-20T00:17:18.9952523Z 105 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-20T00:17:18.9952970Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:18.9953240Z     |
2019-09-20T00:17:18.9953572Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:18.9954435Z 
2019-09-20T00:17:19.0091363Z error[E0308]: mismatched types
2019-09-20T00:17:19.0091674Z    --> src/libtest/tests.rs:105:92
2019-09-20T00:17:19.0091991Z     |
2019-09-20T00:17:19.0091991Z     |
2019-09-20T00:17:19.0092422Z 105 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, false);
2019-09-20T00:17:19.0092849Z     |                                                                                            ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.0094132Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.0094423Z                found type `bool`
2019-09-20T00:17:19.0094462Z 
2019-09-20T00:17:19.0094462Z 
2019-09-20T00:17:19.0106486Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-20T00:17:19.0106804Z     --> src/libtest/tests.rs:125:45
2019-09-20T00:17:19.0107134Z      |
2019-09-20T00:17:19.0107571Z 125  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.0108404Z      |                                             |
2019-09-20T00:17:19.0108704Z      |                                             call expression requires function
2019-09-20T00:17:19.0108899Z      | 
2019-09-20T00:17:19.0109132Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.0109132Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.0109317Z      |
2019-09-20T00:17:19.0109538Z 1068 |     InProcess,
2019-09-20T00:17:19.0109959Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-20T00:17:19.0110251Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:19.0110454Z      |
2019-09-20T00:17:19.0110770Z 125  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-20T00:17:19.0111229Z 
2019-09-20T00:17:19.0251976Z error[E0308]: mismatched types
2019-09-20T00:17:19.0252385Z    --> src/libtest/tests.rs:125:73
2019-09-20T00:17:19.0252595Z     |
2019-09-20T00:17:19.0252595Z     |
2019-09-20T00:17:19.0252960Z 125 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.0253486Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:19.0254097Z     |
2019-09-20T00:17:19.0254457Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.0254764Z 
2019-09-20T00:17:19.0390377Z error[E0308]: mismatched types
2019-09-20T00:17:19.0390667Z    --> src/libtest/tests.rs:125:89
2019-09-20T00:17:19.0390904Z     |
2019-09-20T00:17:19.0390904Z     |
2019-09-20T00:17:19.0391229Z 125 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.0391608Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.0393889Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.0394352Z                found type `bool`
2019-09-20T00:17:19.0394397Z 
2019-09-20T00:17:19.0394397Z 
2019-09-20T00:17:19.0406382Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-20T00:17:19.0406710Z     --> src/libtest/tests.rs:145:45
2019-09-20T00:17:19.0406930Z      |
2019-09-20T00:17:19.0407453Z 145  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-20T00:17:19.0408222Z      |                                             |
2019-09-20T00:17:19.0408556Z      |                                             call expression requires function
2019-09-20T00:17:19.0408778Z      | 
2019-09-20T00:17:19.0409125Z     ::: src/libtest/lib.rs:1073:5
2019-09-20T00:17:19.0409125Z     ::: src/libtest/lib.rs:1073:5
2019-09-20T00:17:19.0409463Z      |
2019-09-20T00:17:19.0409729Z 1073 |     SpawnPrimary,
2019-09-20T00:17:19.0410155Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-20T00:17:19.0410590Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:19.0410817Z      |
2019-09-20T00:17:19.0411239Z 145  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, true);
2019-09-20T00:17:19.0411720Z 
2019-09-20T00:17:19.0556441Z error[E0308]: mismatched types
2019-09-20T00:17:19.0557250Z    --> src/libtest/tests.rs:145:76
2019-09-20T00:17:19.0557613Z     |
2019-09-20T00:17:19.0557613Z     |
2019-09-20T00:17:19.0558019Z 145 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-20T00:17:19.0558548Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:19.0558778Z     |
2019-09-20T00:17:19.0559103Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.0559379Z 
2019-09-20T00:17:19.0695259Z error[E0308]: mismatched types
2019-09-20T00:17:19.0695586Z    --> src/libtest/tests.rs:145:92
2019-09-20T00:17:19.0695812Z     |
2019-09-20T00:17:19.0695812Z     |
2019-09-20T00:17:19.0696168Z 145 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-20T00:17:19.0696849Z     |                                                                                            ^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.0697718Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.0697931Z                found type `bool`
2019-09-20T00:17:19.0697979Z 
2019-09-20T00:17:19.0697979Z 
2019-09-20T00:17:19.0710974Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-20T00:17:19.0711352Z     --> src/libtest/tests.rs:165:45
2019-09-20T00:17:19.0711607Z      |
2019-09-20T00:17:19.0712053Z 165  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.0712732Z      |                                             |
2019-09-20T00:17:19.0713047Z      |                                             call expression requires function
2019-09-20T00:17:19.0713282Z      | 
2019-09-20T00:17:19.0713999Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.0713999Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.0714227Z      |
2019-09-20T00:17:19.0714500Z 1068 |     InProcess,
2019-09-20T00:17:19.0714823Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-20T00:17:19.0715132Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:19.0715366Z      |
2019-09-20T00:17:19.0715686Z 165  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-20T00:17:19.0716048Z 
2019-09-20T00:17:19.0856627Z error[E0308]: mismatched types
2019-09-20T00:17:19.0856943Z    --> src/libtest/tests.rs:165:73
2019-09-20T00:17:19.0857310Z     |
2019-09-20T00:17:19.0857310Z     |
2019-09-20T00:17:19.0857618Z 165 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.0858018Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:19.0858258Z     |
2019-09-20T00:17:19.0858568Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.0858864Z 
2019-09-20T00:17:19.1008714Z error[E0308]: mismatched types
2019-09-20T00:17:19.1009299Z    --> src/libtest/tests.rs:165:89
2019-09-20T00:17:19.1009973Z     |
2019-09-20T00:17:19.1009973Z     |
2019-09-20T00:17:19.1010315Z 165 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1010741Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.1011278Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.1011677Z                found type `bool`
2019-09-20T00:17:19.1011732Z 
2019-09-20T00:17:19.1011732Z 
2019-09-20T00:17:19.1025352Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-20T00:17:19.1025667Z     --> src/libtest/tests.rs:187:45
2019-09-20T00:17:19.1025910Z      |
2019-09-20T00:17:19.1026260Z 187  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1026927Z      |                                             |
2019-09-20T00:17:19.1027259Z      |                                             call expression requires function
2019-09-20T00:17:19.1027591Z      | 
2019-09-20T00:17:19.1027844Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.1027844Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.1028051Z      |
2019-09-20T00:17:19.1028304Z 1068 |     InProcess,
2019-09-20T00:17:19.1028748Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-20T00:17:19.1029156Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:19.1029491Z      |
2019-09-20T00:17:19.1029829Z 187  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-20T00:17:19.1030192Z 
2019-09-20T00:17:19.1186701Z error[E0308]: mismatched types
2019-09-20T00:17:19.1187132Z    --> src/libtest/tests.rs:187:73
2019-09-20T00:17:19.1187343Z     |
2019-09-20T00:17:19.1187343Z     |
2019-09-20T00:17:19.1187652Z 187 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1188023Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:19.1188243Z     |
2019-09-20T00:17:19.1188546Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.1188820Z 
2019-09-20T00:17:19.1342892Z error[E0308]: mismatched types
2019-09-20T00:17:19.1343878Z    --> src/libtest/tests.rs:187:89
2019-09-20T00:17:19.1344164Z     |
2019-09-20T00:17:19.1344164Z     |
2019-09-20T00:17:19.1344500Z 187 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1344901Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.1345577Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.1345910Z                found type `bool`
2019-09-20T00:17:19.1345952Z 
2019-09-20T00:17:19.1345952Z 
2019-09-20T00:17:19.1367961Z error[E0618]: expected function, found enum variant `RunStrategy::InProcess`
2019-09-20T00:17:19.1368283Z     --> src/libtest/tests.rs:205:45
2019-09-20T00:17:19.1368484Z      |
2019-09-20T00:17:19.1368773Z 205  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1369629Z      |                                             |
2019-09-20T00:17:19.1369957Z      |                                             call expression requires function
2019-09-20T00:17:19.1370177Z      | 
2019-09-20T00:17:19.1370423Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.1370423Z     ::: src/libtest/lib.rs:1068:5
2019-09-20T00:17:19.1370642Z      |
2019-09-20T00:17:19.1370892Z 1068 |     InProcess,
2019-09-20T00:17:19.1371200Z      |     --------- `RunStrategy::InProcess` defined here
2019-09-20T00:17:19.1371502Z help: `RunStrategy::InProcess` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:19.1371716Z      |
2019-09-20T00:17:19.1372660Z 205  |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess, Concurrent::No, false);
2019-09-20T00:17:19.1373131Z 
2019-09-20T00:17:19.1523236Z error[E0308]: mismatched types
2019-09-20T00:17:19.1524151Z    --> src/libtest/tests.rs:205:73
2019-09-20T00:17:19.1524481Z     |
2019-09-20T00:17:19.1524481Z     |
2019-09-20T00:17:19.1524812Z 205 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1525267Z     |                                                                         ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:19.1525510Z     |
2019-09-20T00:17:19.1525859Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.1526446Z 
2019-09-20T00:17:19.1662329Z error[E0308]: mismatched types
2019-09-20T00:17:19.1662607Z    --> src/libtest/tests.rs:205:89
2019-09-20T00:17:19.1662801Z     |
2019-09-20T00:17:19.1662801Z     |
2019-09-20T00:17:19.1663108Z 205 |     run_test(&TestOpts::new(), false, desc, RunStrategy::InProcess(tx), Concurrent::No, false);
2019-09-20T00:17:19.1664206Z     |                                                                                         ^^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.1664944Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.1665206Z                found type `bool`
2019-09-20T00:17:19.1665245Z 
2019-09-20T00:17:19.1665245Z 
2019-09-20T00:17:19.1675890Z error[E0618]: expected function, found enum variant `RunStrategy::SpawnPrimary`
2019-09-20T00:17:19.1676225Z     --> src/libtest/tests.rs:223:45
2019-09-20T00:17:19.1676480Z      |
2019-09-20T00:17:19.1677300Z 223  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-20T00:17:19.1678122Z      |                                             |
2019-09-20T00:17:19.1678511Z      |                                             call expression requires function
2019-09-20T00:17:19.1678736Z      | 
2019-09-20T00:17:19.1678959Z     ::: src/libtest/lib.rs:1073:5
2019-09-20T00:17:19.1678959Z     ::: src/libtest/lib.rs:1073:5
2019-09-20T00:17:19.1679148Z      |
2019-09-20T00:17:19.1679389Z 1073 |     SpawnPrimary,
2019-09-20T00:17:19.1679680Z      |     ------------ `RunStrategy::SpawnPrimary` defined here
2019-09-20T00:17:19.1679954Z help: `RunStrategy::SpawnPrimary` is a unit variant, you need to write it without the parenthesis
2019-09-20T00:17:19.1680164Z      |
2019-09-20T00:17:19.1680451Z 223  |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary, Concurrent::No, true);
2019-09-20T00:17:19.1680786Z 
2019-09-20T00:17:19.1837351Z error[E0308]: mismatched types
2019-09-20T00:17:19.1837854Z    --> src/libtest/tests.rs:223:76
2019-09-20T00:17:19.1838148Z     |
2019-09-20T00:17:19.1838148Z     |
2019-09-20T00:17:19.1838493Z 223 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-20T00:17:19.1838934Z     |                                                                            ^^^^^^^^^^^^^^ expected struct `std::sync::mpsc::Sender`, found enum `Concurrent`
2019-09-20T00:17:19.1839209Z     |
2019-09-20T00:17:19.1839817Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.1840150Z 
2019-09-20T00:17:19.1984739Z error[E0308]: mismatched types
2019-09-20T00:17:19.1985090Z    --> src/libtest/tests.rs:223:92
2019-09-20T00:17:19.1985326Z     |
2019-09-20T00:17:19.1985326Z     |
2019-09-20T00:17:19.1985866Z 223 |     run_test(&TestOpts::new(), false, desc, RunStrategy::SpawnPrimary(tx), Concurrent::No, true);
2019-09-20T00:17:19.1986378Z     |                                                                                            ^^^^ expected enum `Concurrent`, found bool
2019-09-20T00:17:19.1986939Z     = note: expected type `Concurrent`
2019-09-20T00:17:19.1987193Z                found type `bool`
2019-09-20T00:17:19.1987232Z 
2019-09-20T00:17:19.2491106Z error[E0308]: mismatched types
2019-09-20T00:17:19.2491106Z error[E0308]: mismatched types
2019-09-20T00:17:19.2491433Z    --> src/libtest/tests.rs:519:29
2019-09-20T00:17:19.2491648Z     |
2019-09-20T00:17:19.2491999Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-20T00:17:19.2492403Z     |                             ^^^^ expected &TestOpts, found struct `TestDesc`
2019-09-20T00:17:19.2492989Z     = note: expected type `&TestOpts`
2019-09-20T00:17:19.2493242Z                found type `TestDesc`
2019-09-20T00:17:19.2493277Z 
2019-09-20T00:17:19.2657039Z error[E0308]: mismatched types
2019-09-20T00:17:19.2657039Z error[E0308]: mismatched types
2019-09-20T00:17:19.2657360Z    --> src/libtest/tests.rs:519:35
2019-09-20T00:17:19.2657590Z     |
2019-09-20T00:17:19.2657900Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-20T00:17:19.2658299Z     |                                   ^^ expected struct `TestDesc`, found struct `std::sync::mpsc::Sender`
2019-09-20T00:17:19.2658918Z     = note: expected type `TestDesc`
2019-09-20T00:17:19.2659303Z                found type `std::sync::mpsc::Sender<_>`
2019-09-20T00:17:19.2659337Z 
2019-09-20T00:17:19.2801911Z error[E0308]: mismatched types
2019-09-20T00:17:19.2801911Z error[E0308]: mismatched types
2019-09-20T00:17:19.2802189Z    --> src/libtest/tests.rs:519:39
2019-09-20T00:17:19.2802392Z     |
2019-09-20T00:17:19.2802671Z 519 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-20T00:17:19.2803048Z     |                                       ^^^^ expected struct `std::sync::mpsc::Sender`, found bool
2019-09-20T00:17:19.2803277Z     |
2019-09-20T00:17:19.2804233Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.2804877Z 
2019-09-20T00:17:19.2965558Z error[E0308]: mismatched types
2019-09-20T00:17:19.2965868Z    --> src/libtest/tests.rs:538:29
2019-09-20T00:17:19.2966100Z     |
2019-09-20T00:17:19.2966100Z     |
2019-09-20T00:17:19.2966425Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-20T00:17:19.2967138Z     |                             ^^^^ expected &TestOpts, found struct `TestDesc`
2019-09-20T00:17:19.2967736Z     = note: expected type `&TestOpts`
2019-09-20T00:17:19.2968093Z                found type `TestDesc`
2019-09-20T00:17:19.2968137Z 
2019-09-20T00:17:19.3122127Z error[E0308]: mismatched types
2019-09-20T00:17:19.3122127Z error[E0308]: mismatched types
2019-09-20T00:17:19.3122420Z    --> src/libtest/tests.rs:538:35
2019-09-20T00:17:19.3122628Z     |
2019-09-20T00:17:19.3122921Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-20T00:17:19.3123271Z     |                                   ^^ expected struct `TestDesc`, found struct `std::sync::mpsc::Sender`
2019-09-20T00:17:19.3124468Z     = note: expected type `TestDesc`
2019-09-20T00:17:19.3124726Z                found type `std::sync::mpsc::Sender<_>`
2019-09-20T00:17:19.3124776Z 
2019-09-20T00:17:19.3276307Z error[E0308]: mismatched types
2019-09-20T00:17:19.3276307Z error[E0308]: mismatched types
2019-09-20T00:17:19.3276846Z    --> src/libtest/tests.rs:538:39
2019-09-20T00:17:19.3277096Z     |
2019-09-20T00:17:19.3277410Z 538 |     crate::bench::benchmark(desc, tx, true, f);
2019-09-20T00:17:19.3277817Z     |                                       ^^^^ expected struct `std::sync::mpsc::Sender`, found bool
2019-09-20T00:17:19.3278301Z     |
2019-09-20T00:17:19.3278735Z     = note: expected type `std::sync::mpsc::Sender<(TestDesc, TestResult, std::vec::Vec<u8>)>`
2019-09-20T00:17:19.3278988Z 
2019-09-20T00:17:19.4684072Z error: aborting due to 34 previous errors
2019-09-20T00:17:19.4684182Z 
2019-09-20T00:17:19.4684475Z Some errors have detailed explanations: E0308, E0432, E0618.
---
2019-09-20T00:17:19.4983965Z == clock drift check ==
2019-09-20T00:17:19.5004067Z   local time: Fri Sep 20 00:17:19 UTC 2019
2019-09-20T00:17:19.6515210Z   network time: Fri, 20 Sep 2019 00:17:19 GMT
2019-09-20T00:17:19.6517188Z == end clock drift check ==
2019-09-20T00:17:20.2075181Z ##[error]Bash exited with code '1'.
2019-09-20T00:17:20.2111832Z ##[section]Starting: Checkout
2019-09-20T00:17:20.2114498Z ==============================================================================
2019-09-20T00:17:20.2114579Z Task         : Get sources
2019-09-20T00:17:20.2114628Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
