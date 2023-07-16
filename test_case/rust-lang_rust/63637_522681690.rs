plain
2019-08-19T15:48:55.5832597Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T15:48:55.6063342Z ##[command]git config gc.auto 0
2019-08-19T15:48:55.6125887Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T15:48:55.6180004Z ##[command]git config --get-all http.proxy
2019-08-19T15:48:55.6323021Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63637/merge:refs/remotes/pull/63637/merge
---
2019-08-19T15:49:31.7737776Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T15:49:31.7739536Z 
2019-08-19T15:49:31.7741107Z   git checkout -b <new-branch-name>
2019-08-19T15:49:31.7742548Z 
2019-08-19T15:49:31.7743286Z HEAD is now at 64baa9a6f Merge 9534886ce8448c705680eb647d458dd706d8313e into cdff9189556bb7de2b9a8a72344c9d8ec6099fcd
2019-08-19T15:49:31.7891553Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-19T15:49:31.7895323Z ==============================================================================
2019-08-19T15:49:31.7895376Z Task         : Bash
2019-08-19T15:49:31.7895450Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-19T16:50:03.6302263Z .................................................................................................... 1500/8929
2019-08-19T16:50:09.2019625Z .................................................................................................... 1600/8929
2019-08-19T16:50:22.0247747Z .................................i...............i.................................................. 1700/8929
2019-08-19T16:50:29.5853544Z .................................................................................................... 1800/8929
2019-08-19T16:50:43.5848608Z .........................iiiii...................................................................... 1900/8929
2019-08-19T16:50:53.9656919Z .................................................................................................... 2100/8929
2019-08-19T16:50:56.4827388Z .................................................................................................... 2200/8929
2019-08-19T16:51:01.2476436Z .................................................................................................... 2300/8929
2019-08-19T16:51:07.9583408Z .................................................................................................... 2400/8929
---
2019-08-19T16:53:59.3325067Z .................................................................................................... 4600/8929
2019-08-19T16:54:06.3442661Z .........i...............i.......................................................................... 4700/8929
2019-08-19T16:54:17.4188635Z .................................................................................................... 4800/8929
2019-08-19T16:54:23.3978111Z .................................................................................................... 4900/8929
2019-08-19T16:54:34.9742669Z ..........................................................................................ii.ii..... 5000/8929
2019-08-19T16:54:44.4017688Z .................................................................................................... 5200/8929
2019-08-19T16:54:54.0628670Z .................................................................................................... 5300/8929
2019-08-19T16:55:00.8033021Z ..............................................i..................................................... 5400/8929
2019-08-19T16:55:07.4664749Z .................................................................................................... 5500/8929
2019-08-19T16:55:07.4664749Z .................................................................................................... 5500/8929
2019-08-19T16:55:18.0461658Z .................................................................................................... 5600/8929
2019-08-19T16:55:30.1968936Z .......................................ii...i..ii...........i....................................... 5700/8929
2019-08-19T16:55:49.8882812Z .................................................................................................... 5900/8929
2019-08-19T16:55:54.8140320Z .................................................................................................... 6000/8929
2019-08-19T16:55:54.8140320Z .................................................................................................... 6000/8929
2019-08-19T16:56:07.6804758Z ........................................i..ii....................................................... 6100/8929
2019-08-19T16:56:28.5590181Z ..................................................................................i................. 6300/8929
2019-08-19T16:56:30.6933610Z .................................................................................................... 6400/8929
2019-08-19T16:56:32.9083631Z ......................................................i............................................. 6500/8929
2019-08-19T16:56:36.0384743Z .................................................................................................... 6600/8929
---
2019-08-19T16:57:36.2357717Z .................................................................................................... 7200/8929
2019-08-19T16:57:42.9541309Z .................................................................................................... 7300/8929
2019-08-19T16:57:53.3613194Z .................................................................................................... 7400/8929
2019-08-19T16:58:03.0392840Z .................................................................................................... 7500/8929
2019-08-19T16:58:09.7624522Z ..ii......i......................................................................................... 7600/8929
2019-08-19T16:58:30.7031893Z .................................................................................................... 7800/8929
2019-08-19T16:58:39.6335869Z .................................................................................................... 7900/8929
2019-08-19T16:58:49.4029068Z .................................................................................................... 8000/8929
2019-08-19T16:59:27.7156718Z .................................................................................................... 8100/8929
---
2019-08-19T17:01:13.6928153Z  finished in 21.247
2019-08-19T17:01:13.7107889Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:01:13.8718811Z 
2019-08-19T17:01:13.8719300Z running 148 tests
2019-08-19T17:01:17.0153694Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/148
2019-08-19T17:01:18.8936009Z .iiii..............i.........iii.i......ii......
2019-08-19T17:01:18.8937591Z 
2019-08-19T17:01:18.8939257Z  finished in 5.183
2019-08-19T17:01:18.9118057Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:01:19.0721852Z 
---
2019-08-19T17:01:21.1863027Z  finished in 2.274
2019-08-19T17:01:21.2048436Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:01:21.3621445Z 
2019-08-19T17:01:21.3621626Z running 9 tests
2019-08-19T17:01:21.3628323Z iiiiiiiii
2019-08-19T17:01:21.3628893Z 
2019-08-19T17:01:21.3628935Z  finished in 0.157
2019-08-19T17:01:21.3791767Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:01:21.5369578Z 
---
2019-08-19T17:01:39.3785150Z  finished in 17.999
2019-08-19T17:01:39.3959472Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:01:39.5654898Z 
2019-08-19T17:01:39.5656161Z running 122 tests
2019-08-19T17:02:03.8123526Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-19T17:02:08.4812126Z .i.i......iii.i.....ii
2019-08-19T17:02:08.4812545Z 
2019-08-19T17:02:08.4814327Z  finished in 29.085
2019-08-19T17:02:08.4826545Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:02:08.4827060Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-19T17:15:38.5966336Z running 970 tests
2019-08-19T17:15:38.6099136Z .................................................................................................... 100/970
2019-08-19T17:15:38.6221252Z .................................................................................................... 200/970
2019-08-19T17:15:38.6345938Z .................................................................................................... 300/970
2019-08-19T17:15:38.7168096Z ................................................................................................ii.. 400/970
2019-08-19T17:15:38.8387594Z .................................................................................................... 600/970
2019-08-19T17:15:38.8533836Z .................................................................................................... 700/970
2019-08-19T17:15:38.8882045Z .................................................................................................... 800/970
2019-08-19T17:15:39.4179623Z .................................................................................................... 900/970
2019-08-19T17:15:39.4179623Z .................................................................................................... 900/970
2019-08-19T17:15:40.5978074Z ......................................................................
2019-08-19T17:15:40.5979525Z test result: ok. 968 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2019-08-19T17:15:40.5980447Z 
2019-08-19T17:15:40.5985692Z    Doc-tests core
2019-08-19T17:15:45.7404291Z 
2019-08-19T17:15:45.7411260Z running 2380 tests
2019-08-19T17:15:58.4511112Z ......iiiii......................................................................................... 100/2380
2019-08-19T17:16:10.8064985Z .........................................................................ii......................... 200/2380
2019-08-19T17:16:40.3570415Z .................................................................................................... 400/2380
2019-08-19T17:16:40.3570415Z .................................................................................................... 400/2380
2019-08-19T17:16:51.5887925Z ..............................i..i.................iiii............................................. 500/2380
2019-08-19T17:17:14.8760911Z .................................................................................................... 700/2380
2019-08-19T17:17:26.7058333Z .................................................................................................... 800/2380
2019-08-19T17:17:38.2851231Z .................................................................................................... 900/2380
2019-08-19T17:17:50.1384783Z .................................................................................................... 1000/2380
---
2019-08-19T17:23:02.2677238Z ........................................................ 300/762
2019-08-19T17:23:02.2689367Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
2019-08-19T17:23:02.3208021Z .................................................................................................... 400/762
2019-08-19T17:23:04.3915846Z .................................................................................................... 500/762
2019-08-19T17:23:04.4141733Z ...................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-19T17:23:04.4157423Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1084:5
2019-08-19T17:23:04.4170367Z .thread '<unnamed>.' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-19T17:23:04.4186527Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-19T17:23:04.6408171Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-19T17:23:04.6438036Z ...........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-19T17:23:04.6446650Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1084:5
2019-08-19T17:23:06.7057773Z ..........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:629:13
2019-08-19T17:23:06.7063089Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:584:13
2019-08-19T17:23:06.7068962Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:561:13
2019-08-19T17:23:06.7074339Z ..thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:689:13
---
2019-08-19T17:23:16.1917441Z 
2019-08-19T17:23:16.1917782Z running 991 tests
2019-08-19T17:23:40.1966911Z i................................................................................................... 100/991
2019-08-19T17:23:53.2590180Z .................................................................................................... 200/991
2019-08-19T17:24:02.1897474Z .................iii......i......i...i......i....................................................... 300/991
2019-08-19T17:24:07.2763268Z .................................................................................................... 400/991
2019-08-19T17:24:15.4795639Z ..................................i..i.................................ii........................... 500/991
2019-08-19T17:24:30.3305395Z .................................................................................................... 700/991
2019-08-19T17:24:30.3305395Z .................................................................................................... 700/991
2019-08-19T17:24:38.7596055Z .................iiii............................................................................... 800/991
2019-08-19T17:24:53.4842295Z .................................................................................................... 900/991
2019-08-19T17:25:01.1345231Z .......................................iiii................................................
2019-08-19T17:25:01.1345817Z 
2019-08-19T17:25:01.1533144Z  finished in 250.195
2019-08-19T17:25:01.1549381Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:25:01.3925636Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-08-19T17:40:33.6881409Z  finished in 33.124
2019-08-19T17:40:33.7218533Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T17:40:33.8903078Z 
2019-08-19T17:40:33.8904745Z running 200 tests
2019-08-19T17:41:07.3621342Z ....................i...ii................................................................i......... 100/200
2019-08-19T17:41:51.9027802Z ..............................iiii.......i...........iiii.iii....................................i.. 200/200
2019-08-19T17:41:51.9028807Z test result: ok. 183 passed; 0 failed; 17 ignored; 0 measured; 0 filtered out
2019-08-19T17:41:51.9028843Z 
2019-08-19T17:41:51.9036805Z  finished in 78.181
2019-08-19T17:41:51.9045943Z doc tests for: /checkout/src/doc/rustdoc/src/command-line-arguments.md
---
2019-08-19T17:42:22.7903806Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-08-19T17:42:25.7590732Z error[E0412]: cannot find type `Test` in module `compile`
2019-08-19T17:42:25.7596150Z    --> src/bootstrap/builder/tests.rs:369:44
2019-08-19T17:42:25.7596594Z     |
2019-08-19T17:42:25.7596813Z 369 |         first(builder.cache.all::<compile::Test>()),
2019-08-19T17:42:25.7597044Z     |                                            ^^^^ not found in `compile`
2019-08-19T17:42:25.7597643Z help: there is an enum variant `crate::builder::Kind::Test` and 3 others; try using the variant's enum
2019-08-19T17:42:25.7597921Z     |
2019-08-19T17:42:25.7598160Z 369 |         first(builder.cache.all::<crate::builder::Kind>()),
2019-08-19T17:42:25.7598612Z     |                                   ^^^^^^^^^^^^^^^^^^^^
2019-08-19T17:42:25.7598912Z 369 |         first(builder.cache.all::<crate::flags::Subcommand>()),
2019-08-19T17:42:25.7599158Z     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T17:42:25.7599366Z 369 |         first(builder.cache.all::<crate::test::TestKind>()),
2019-08-19T17:42:25.7599795Z 
2019-08-19T17:42:25.7599795Z 
2019-08-19T17:42:25.7604469Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7605354Z    --> src/bootstrap/builder/tests.rs:371:22
2019-08-19T17:42:25.7605869Z 371 |             compile::Test {
2019-08-19T17:42:25.7606089Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7606296Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7606493Z     |
2019-08-19T17:42:25.7606493Z     |
2019-08-19T17:42:25.7606723Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7607086Z     |
2019-08-19T17:42:25.7607272Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7607477Z     |
2019-08-19T17:42:25.7607663Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7607846Z     |
2019-08-19T17:42:25.7608034Z 
2019-08-19T17:42:25.7619784Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7620728Z    --> src/bootstrap/builder/tests.rs:375:22
2019-08-19T17:42:25.7621884Z 375 |             compile::Test {
2019-08-19T17:42:25.7622121Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7622378Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7622641Z     |
2019-08-19T17:42:25.7622641Z     |
2019-08-19T17:42:25.7622885Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7623101Z     |
2019-08-19T17:42:25.7623316Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7623543Z     |
2019-08-19T17:42:25.7623781Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7624006Z     |
2019-08-19T17:42:25.7624214Z 
2019-08-19T17:42:25.7636068Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7636801Z    --> src/bootstrap/builder/tests.rs:379:22
2019-08-19T17:42:25.7637357Z 379 |             compile::Test {
2019-08-19T17:42:25.7637592Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7637823Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7638028Z     |
2019-08-19T17:42:25.7638028Z     |
2019-08-19T17:42:25.7638227Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7638444Z     |
2019-08-19T17:42:25.7638642Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7638840Z     |
2019-08-19T17:42:25.7639054Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7639250Z     |
2019-08-19T17:42:25.7639430Z 
2019-08-19T17:42:25.7651201Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7652193Z    --> src/bootstrap/builder/tests.rs:383:22
2019-08-19T17:42:25.7652790Z 383 |             compile::Test {
2019-08-19T17:42:25.7653015Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7653244Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7653510Z     |
---
2019-08-19T17:42:25.7686506Z 
2019-08-19T17:42:25.7687478Z error[E0412]: cannot find type `Test` in module `compile`
2019-08-19T17:42:25.7688084Z    --> src/bootstrap/builder/tests.rs:455:44
2019-08-19T17:42:25.7688254Z     |
2019-08-19T17:42:25.7688546Z 455 |         first(builder.cache.all::<compile::Test>()),
2019-08-19T17:42:25.7688730Z     |                                            ^^^^ not found in `compile`
2019-08-19T17:42:25.7689138Z help: there is an enum variant `crate::builder::Kind::Test` and 3 others; try using the variant's enum
2019-08-19T17:42:25.7689518Z     |
2019-08-19T17:42:25.7689651Z 455 |         first(builder.cache.all::<crate::builder::Kind>()),
2019-08-19T17:42:25.7689811Z     |                                   ^^^^^^^^^^^^^^^^^^^^
2019-08-19T17:42:25.7689941Z 455 |         first(builder.cache.all::<crate::flags::Subcommand>()),
2019-08-19T17:42:25.7690069Z     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T17:42:25.7690214Z 455 |         first(builder.cache.all::<crate::test::TestKind>()),
2019-08-19T17:42:25.7690448Z 
2019-08-19T17:42:25.7690448Z 
2019-08-19T17:42:25.7690601Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7691340Z    --> src/bootstrap/builder/tests.rs:457:22
2019-08-19T17:42:25.7691722Z 457 |             compile::Test {
2019-08-19T17:42:25.7691869Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7692037Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7692359Z     |
2019-08-19T17:42:25.7692359Z     |
2019-08-19T17:42:25.7692502Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7692654Z     |
2019-08-19T17:42:25.7692792Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7692927Z     |
2019-08-19T17:42:25.7693081Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7693218Z     |
2019-08-19T17:42:25.7696418Z 
2019-08-19T17:42:25.7697525Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7698036Z    --> src/bootstrap/builder/tests.rs:461:22
2019-08-19T17:42:25.7698370Z 461 |             compile::Test {
2019-08-19T17:42:25.7698499Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7698630Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7698769Z     |
2019-08-19T17:42:25.7698769Z     |
2019-08-19T17:42:25.7698889Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7699007Z     |
2019-08-19T17:42:25.7699145Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7699273Z     |
2019-08-19T17:42:25.7699393Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7699527Z     |
2019-08-19T17:42:25.7699629Z 
2019-08-19T17:42:25.7715220Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7715662Z    --> src/bootstrap/builder/tests.rs:465:22
2019-08-19T17:42:25.7715751Z 465 |             compile::Test {
2019-08-19T17:42:25.7715817Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7715882Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7715926Z     |
2019-08-19T17:42:25.7715926Z     |
2019-08-19T17:42:25.7716144Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7716182Z     |
2019-08-19T17:42:25.7716220Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7716273Z     |
2019-08-19T17:42:25.7716311Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7716348Z     |
2019-08-19T17:42:25.7716381Z 
2019-08-19T17:42:25.7731642Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7732121Z    --> src/bootstrap/builder/tests.rs:469:22
2019-08-19T17:42:25.7732253Z 469 |             compile::Test {
2019-08-19T17:42:25.7732302Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7732356Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7732422Z     |
2019-08-19T17:42:25.7732422Z     |
2019-08-19T17:42:25.7732627Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7732683Z     |
2019-08-19T17:42:25.7732745Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7732788Z     |
2019-08-19T17:42:25.7732831Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7732873Z     |
2019-08-19T17:42:25.7732924Z 
2019-08-19T17:42:25.7747680Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7748270Z    --> src/bootstrap/builder/tests.rs:473:22
2019-08-19T17:42:25.7748401Z 473 |             compile::Test {
2019-08-19T17:42:25.7748445Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7748510Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7748551Z     |
2019-08-19T17:42:25.7748551Z     |
2019-08-19T17:42:25.7748756Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7748793Z     |
2019-08-19T17:42:25.7748847Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7748882Z     |
2019-08-19T17:42:25.7748926Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7748979Z     |
2019-08-19T17:42:25.7749003Z 
2019-08-19T17:42:25.7763509Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7764007Z    --> src/bootstrap/builder/tests.rs:477:22
2019-08-19T17:42:25.7764111Z 477 |             compile::Test {
2019-08-19T17:42:25.7764329Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7764405Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7764453Z     |
2019-08-19T17:42:25.7764453Z     |
2019-08-19T17:42:25.7764496Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7764721Z     |
2019-08-19T17:42:25.7764761Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7764797Z     |
2019-08-19T17:42:25.7764851Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7764887Z     |
2019-08-19T17:42:25.7764910Z 
2019-08-19T17:42:25.7779284Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7779752Z    --> src/bootstrap/builder/tests.rs:481:22
2019-08-19T17:42:25.7779846Z 481 |             compile::Test {
2019-08-19T17:42:25.7779911Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7779960Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7780015Z     |
2019-08-19T17:42:25.7780015Z     |
2019-08-19T17:42:25.7780073Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7780113Z     |
2019-08-19T17:42:25.7780152Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7780191Z     |
2019-08-19T17:42:25.7780250Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7780289Z     |
2019-08-19T17:42:25.7780315Z 
2019-08-19T17:42:25.7795989Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7796576Z    --> src/bootstrap/builder/tests.rs:485:22
2019-08-19T17:42:25.7796696Z 485 |             compile::Test {
2019-08-19T17:42:25.7796738Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7796784Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7796824Z     |
2019-08-19T17:42:25.7796824Z     |
2019-08-19T17:42:25.7796880Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7796917Z     |
2019-08-19T17:42:25.7796960Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7797014Z     |
2019-08-19T17:42:25.7797051Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7797086Z     |
2019-08-19T17:42:25.7797110Z 
2019-08-19T17:42:25.7811826Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7812306Z    --> src/bootstrap/builder/tests.rs:489:22
2019-08-19T17:42:25.7812434Z 489 |             compile::Test {
2019-08-19T17:42:25.7812484Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7812714Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7812798Z     |
---
2019-08-19T17:42:25.7813135Z 
2019-08-19T17:42:25.7827961Z error[E0412]: cannot find type `Test` in module `compile`
2019-08-19T17:42:25.7828568Z    --> src/bootstrap/builder/tests.rs:546:44
2019-08-19T17:42:25.7828649Z     |
2019-08-19T17:42:25.7828694Z 546 |         first(builder.cache.all::<compile::Test>()),
2019-08-19T17:42:25.7828743Z     |                                            ^^^^ not found in `compile`
2019-08-19T17:42:25.7829204Z help: there is an enum variant `crate::builder::Kind::Test` and 3 others; try using the variant's enum
2019-08-19T17:42:25.7829453Z     |
2019-08-19T17:42:25.7829671Z 546 |         first(builder.cache.all::<crate::builder::Kind>()),
2019-08-19T17:42:25.7829740Z     |                                   ^^^^^^^^^^^^^^^^^^^^
2019-08-19T17:42:25.7829788Z 546 |         first(builder.cache.all::<crate::flags::Subcommand>()),
2019-08-19T17:42:25.7829839Z     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^
2019-08-19T17:42:25.7829904Z 546 |         first(builder.cache.all::<crate::test::TestKind>()),
2019-08-19T17:42:25.7830146Z 
2019-08-19T17:42:25.7830146Z 
2019-08-19T17:42:25.7845335Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7845841Z    --> src/bootstrap/builder/tests.rs:548:22
2019-08-19T17:42:25.7845944Z 548 |             compile::Test {
2019-08-19T17:42:25.7846018Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7846089Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7846138Z     |
2019-08-19T17:42:25.7846138Z     |
2019-08-19T17:42:25.7846200Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7846243Z     |
2019-08-19T17:42:25.7846287Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7846349Z     |
2019-08-19T17:42:25.7846393Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7846437Z     |
2019-08-19T17:42:25.7846474Z 
2019-08-19T17:42:25.7862491Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7862967Z    --> src/bootstrap/builder/tests.rs:552:22
2019-08-19T17:42:25.7863089Z 552 |             compile::Test {
2019-08-19T17:42:25.7863138Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7863193Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7863261Z     |
2019-08-19T17:42:25.7863261Z     |
2019-08-19T17:42:25.7863305Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7863364Z     |
2019-08-19T17:42:25.7863409Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7863469Z     |
2019-08-19T17:42:25.7863513Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7863555Z     |
2019-08-19T17:42:25.7863583Z 
2019-08-19T17:42:25.7877894Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7878335Z    --> src/bootstrap/builder/tests.rs:556:22
2019-08-19T17:42:25.7878469Z 556 |             compile::Test {
2019-08-19T17:42:25.7878514Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7878617Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7878682Z     |
2019-08-19T17:42:25.7878682Z     |
2019-08-19T17:42:25.7878721Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7878759Z     |
2019-08-19T17:42:25.7878815Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7878853Z     |
2019-08-19T17:42:25.7879043Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7879111Z     |
2019-08-19T17:42:25.7879137Z 
2019-08-19T17:42:25.7893778Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7894261Z    --> src/bootstrap/builder/tests.rs:560:22
2019-08-19T17:42:25.7894390Z 560 |             compile::Test {
2019-08-19T17:42:25.7894439Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7894740Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7894783Z     |
2019-08-19T17:42:25.7894783Z     |
2019-08-19T17:42:25.7894985Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7895041Z     |
2019-08-19T17:42:25.7895079Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7895116Z     |
2019-08-19T17:42:25.7895153Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7895207Z     |
2019-08-19T17:42:25.7895232Z 
2019-08-19T17:42:25.7907919Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7908367Z    --> src/bootstrap/builder/tests.rs:564:22
2019-08-19T17:42:25.7908452Z 564 |             compile::Test {
2019-08-19T17:42:25.7908512Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7908558Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7908749Z     |
2019-08-19T17:42:25.7908749Z     |
2019-08-19T17:42:25.7908805Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7908841Z     |
2019-08-19T17:42:25.7908878Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7908914Z     |
2019-08-19T17:42:25.7908968Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7909004Z     |
2019-08-19T17:42:25.7909028Z 
2019-08-19T17:42:25.7923628Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7924114Z    --> src/bootstrap/builder/tests.rs:568:22
2019-08-19T17:42:25.7924232Z 568 |             compile::Test {
2019-08-19T17:42:25.7924302Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7924358Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7924405Z     |
2019-08-19T17:42:25.7924405Z     |
2019-08-19T17:42:25.7924469Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7924513Z     |
2019-08-19T17:42:25.7924716Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7924781Z     |
2019-08-19T17:42:25.7924820Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7924858Z     |
2019-08-19T17:42:25.7924884Z 
2019-08-19T17:42:25.7938807Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7939225Z    --> src/bootstrap/builder/tests.rs:572:22
2019-08-19T17:42:25.7939339Z 572 |             compile::Test {
2019-08-19T17:42:25.7939381Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7939440Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7939500Z     |
2019-08-19T17:42:25.7939500Z     |
2019-08-19T17:42:25.7939536Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7939572Z     |
2019-08-19T17:42:25.7939626Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7939662Z     |
2019-08-19T17:42:25.7939698Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7939742Z     |
2019-08-19T17:42:25.7939784Z 
2019-08-19T17:42:25.7955338Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7955774Z    --> src/bootstrap/builder/tests.rs:576:22
2019-08-19T17:42:25.7955894Z 576 |             compile::Test {
2019-08-19T17:42:25.7955938Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7956005Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7956049Z     |
2019-08-19T17:42:25.7956049Z     |
2019-08-19T17:42:25.7956239Z 1   | use crate::builder::Kind::Test;
2019-08-19T17:42:25.7956290Z     |
2019-08-19T17:42:25.7956347Z 1   | use crate::flags::Subcommand::Test;
2019-08-19T17:42:25.7956386Z     |
2019-08-19T17:42:25.7956424Z 1   | use crate::test::TestKind::Test;
2019-08-19T17:42:25.7956480Z     |
2019-08-19T17:42:25.7956506Z 
2019-08-19T17:42:25.7969493Z error[E0422]: cannot find struct, variant or union type `Test` in module `compile`
2019-08-19T17:42:25.7970431Z    --> src/bootstrap/builder/tests.rs:580:22
2019-08-19T17:42:25.7970538Z 580 |             compile::Test {
2019-08-19T17:42:25.7970579Z     |                      ^^^^ not found in `compile`
2019-08-19T17:42:25.7970646Z help: possible candidates are found in other modules, you can import them into scope
2019-08-19T17:42:25.7970685Z     |
---
2019-08-19T17:42:28.7639657Z == clock drift check ==
2019-08-19T17:42:28.7652863Z   local time: Mon Aug 19 17:42:28 UTC 2019
2019-08-19T17:42:28.8242867Z   network time: Mon, 19 Aug 2019 17:42:28 GMT
2019-08-19T17:42:28.8247226Z == end clock drift check ==
2019-08-19T17:42:32.3234467Z ##[error]Bash exited with code '1'.
2019-08-19T17:42:32.3276837Z ##[section]Starting: Checkout
2019-08-19T17:42:32.3278606Z ==============================================================================
2019-08-19T17:42:32.3278655Z Task         : Get sources
2019-08-19T17:42:32.3278696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
