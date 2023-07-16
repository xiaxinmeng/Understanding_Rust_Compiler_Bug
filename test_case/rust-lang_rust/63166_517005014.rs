plain
2019-07-31T18:47:42.2571873Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T18:47:42.2774953Z ##[command]git config gc.auto 0
2019-07-31T18:47:42.2843403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T18:47:42.2899618Z ##[command]git config --get-all http.proxy
2019-07-31T18:47:42.3029504Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63166/merge:refs/remotes/pull/63166/merge
---
2019-07-31T18:48:16.5760686Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T18:48:16.5760727Z 
2019-07-31T18:48:16.5761219Z   git checkout -b <new-branch-name>
2019-07-31T18:48:16.5761284Z 
2019-07-31T18:48:16.5761334Z HEAD is now at 714cdff09 Merge 6c130817623426697d8ebdf5d505487bd11ee2f6 into 9152fe4ea053a29469691349f4b63aa94c9aac56
2019-07-31T18:48:16.5913924Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T18:48:16.5916298Z ==============================================================================
2019-07-31T18:48:16.5916344Z Task         : Bash
2019-07-31T18:48:16.5916380Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T19:49:14.9977886Z .................................................................................................... 1400/8819
2019-07-31T19:49:20.6443067Z .................................................................................................... 1500/8819
2019-07-31T19:49:33.1012667Z .................................................................i...............i.................. 1600/8819
2019-07-31T19:49:40.3542524Z .................................................................................................... 1700/8819
2019-07-31T19:49:54.8937336Z ...................................................iiiii............................................ 1800/8819
2019-07-31T19:50:05.7187563Z .................................................................................................... 2000/8819
2019-07-31T19:50:08.1683773Z .................................................................................................... 2100/8819
2019-07-31T19:50:11.6958566Z .................................................................................................... 2200/8819
2019-07-31T19:50:18.0626033Z .................................................................................................... 2300/8819
---
2019-07-31T19:54:05.0642985Z .................................................................................................... 5300/8819
2019-07-31T19:54:12.2110609Z ..............i..................................................................................... 5400/8819
2019-07-31T19:54:17.6322891Z .................................................................................................... 5500/8819
2019-07-31T19:54:29.5304629Z .................................................................................................... 5600/8819
2019-07-31T19:54:42.9944272Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-07-31T19:54:59.2612759Z .................................................................................................... 5900/8819
2019-07-31T19:55:03.8581518Z .................................................................................................... 6000/8819
2019-07-31T19:55:03.8581518Z .................................................................................................... 6000/8819
2019-07-31T19:55:17.4089758Z ........i..ii....................................................................................... 6100/8819
2019-07-31T19:55:36.0588040Z ...................................................i................................................ 6300/8819
2019-07-31T19:55:38.1927036Z .................................................................................................... 6400/8819
2019-07-31T19:55:40.5284580Z .....................i.............................................................................. 6500/8819
2019-07-31T19:55:44.9502924Z .................................................................................................... 6600/8819
---
2019-07-31T20:00:15.5728670Z  finished in 20.292
2019-07-31T20:00:15.5729049Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-31T20:00:15.5729190Z 
2019-07-31T20:00:15.5729336Z running 146 tests
2019-07-31T20:00:18.3204295Z i....iii......iii..iiii....i............................i..i................i....i.........ii.i.i..i 100/146
2019-07-31T20:00:20.1294419Z iii..............i.........iii.i......ii......
2019-07-31T20:00:20.1295123Z 
2019-07-31T20:00:20.1295202Z  finished in 5.147
2019-07-31T20:00:20.1468612Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-31T20:00:20.3015564Z 
---
2019-07-31T20:00:22.3562345Z  finished in 2.209
2019-07-31T20:00:22.3753371Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-31T20:00:22.5283376Z 
2019-07-31T20:00:22.5283647Z running 9 tests
2019-07-31T20:00:22.5284349Z iiiiiiiii
2019-07-31T20:00:22.5284806Z 
2019-07-31T20:00:22.5284847Z  finished in 0.153
2019-07-31T20:00:22.5474278Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-31T20:00:22.7048033Z 
---
2019-07-31T20:00:40.6491792Z  finished in 18.102
2019-07-31T20:00:40.6676374Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-31T20:00:40.8250581Z 
2019-07-31T20:00:40.8251511Z running 122 tests
2019-07-31T20:01:03.7227220Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-07-31T20:01:08.1868711Z .i.i......iii.i.....ii
2019-07-31T20:01:08.1873531Z 
2019-07-31T20:01:08.1873611Z  finished in 27.519
2019-07-31T20:01:08.1882110Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-07-31T20:01:08.1882620Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-07-31T20:14:44.0471558Z 
2019-07-31T20:14:44.0478808Z    Doc-tests core
2019-07-31T20:14:48.2162767Z 
2019-07-31T20:14:48.2169595Z running 2395 tests
2019-07-31T20:15:01.0417862Z ......iiiii......................................................................................... 100/2395
2019-07-31T20:15:13.1796406Z .........................................................................ii......................... 200/2395
2019-07-31T20:15:41.3361906Z .................................................................................................... 400/2395
2019-07-31T20:15:41.3361906Z .................................................................................................... 400/2395
2019-07-31T20:15:51.9971601Z ..............................i..i.................iiii............................................. 500/2395
2019-07-31T20:16:14.3227201Z .................................................................................................... 700/2395
2019-07-31T20:16:26.9847824Z .................................................................................................... 800/2395
2019-07-31T20:16:39.6416757Z .................................................................................................... 900/2395
2019-07-31T20:16:52.3179164Z .................................................................................................... 1000/2395
---
2019-07-31T20:18:19.0685630Z .................................................................................................... 1700/2395
2019-07-31T20:18:31.5505683Z .................................................................................................... 1800/2395
2019-07-31T20:18:44.4213388Z .................................................................................................... 1900/2395
2019-07-31T20:18:58.8450700Z .................................................................................................... 2000/2395
2019-07-31T20:19:10.6447549Z ..............................................................................................F.FFFF 2100/2395
2019-07-31T20:19:24.8296763Z FFFFFFFF.FFF........................................................................................ 2200/2395
2019-07-31T20:19:55.7428035Z ....i..........................................................................................
2019-07-31T20:19:55.7428875Z failures:
2019-07-31T20:19:55.7429545Z 
2019-07-31T20:19:55.7429545Z 
2019-07-31T20:19:55.7430863Z ---- result.rs - result::Result<&'_ T, &'_ E>::cloned (line 1057) stdout ----
2019-07-31T20:19:55.7431207Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7431940Z   |
2019-07-31T20:19:55.7431940Z   |
2019-07-31T20:19:55.7432182Z 5 | assert_eq!(Err(&1).cloned(), Err(1));
2019-07-31T20:19:55.7432390Z   |                    ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7432595Z   |
2019-07-31T20:19:55.7432832Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7433050Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7433289Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7433666Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7434310Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7434780Z  --> result.rs:1060:20
2019-07-31T20:19:55.7435044Z   |
2019-07-31T20:19:55.7435044Z   |
2019-07-31T20:19:55.7435963Z 6 | assert_eq!(Ok(&42).cloned(), Ok(42));
2019-07-31T20:19:55.7436252Z   |                    ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7436467Z   |
2019-07-31T20:19:55.7436714Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7436936Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7437179Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7437411Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7437838Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7438031Z 
2019-07-31T20:19:55.7438554Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7439223Z Couldn't compile the test.
2019-07-31T20:19:55.7439223Z Couldn't compile the test.
2019-07-31T20:19:55.7440114Z ---- result.rs - result::Result<&'_ T, &'_ mut E>::cloned (line 1091) stdout ----
2019-07-31T20:19:55.7440431Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7441126Z   |
2019-07-31T20:19:55.7441126Z   |
2019-07-31T20:19:55.7441329Z 5 | assert_eq!(Err(&mut 1).cloned(), Err(1));
2019-07-31T20:19:55.7441554Z   |                        ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7441752Z   |
2019-07-31T20:19:55.7441984Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7442217Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7442429Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7442670Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7443072Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7443498Z  --> result.rs:1094:20
2019-07-31T20:19:55.7443756Z   |
2019-07-31T20:19:55.7443756Z   |
2019-07-31T20:19:55.7443961Z 6 | assert_eq!(Ok(&42).cloned(), Ok(42));
2019-07-31T20:19:55.7444181Z   |                    ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7444380Z   |
2019-07-31T20:19:55.7444583Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7444810Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7445018Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7445228Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7446118Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7446328Z 
2019-07-31T20:19:55.7446858Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7447562Z Couldn't compile the test.
2019-07-31T20:19:55.7447562Z Couldn't compile the test.
2019-07-31T20:19:55.7448104Z ---- result.rs - result::Result<&'_ T, &'_ E>::copied (line 909) stdout ----
2019-07-31T20:19:55.7448837Z  --> result.rs:911:1
2019-07-31T20:19:55.7449262Z   |
2019-07-31T20:19:55.7449262Z   |
2019-07-31T20:19:55.7449486Z 5 | assert_eq!(Err(&1), Err(1));
2019-07-31T20:19:55.7450065Z   | |
2019-07-31T20:19:55.7450065Z   | |
2019-07-31T20:19:55.7450291Z   | expected &{integer}, found integer
2019-07-31T20:19:55.7450509Z   | help: try using a variant of the expected type: `Ok(*right_val)`
2019-07-31T20:19:55.7450732Z   |
2019-07-31T20:19:55.7451098Z   = note: expected type `std::result::Result<_, &{integer}>`
2019-07-31T20:19:55.7451304Z              found type `std::result::Result<_, {integer}>`
2019-07-31T20:19:55.7452252Z 
2019-07-31T20:19:55.7452477Z error[E0308]: mismatched types
2019-07-31T20:19:55.7452931Z  --> result.rs:912:1
2019-07-31T20:19:55.7453190Z   |
2019-07-31T20:19:55.7453190Z   |
2019-07-31T20:19:55.7453392Z 6 | assert_eq!(Ok(&42), Ok(42));
2019-07-31T20:19:55.7453811Z   | |
2019-07-31T20:19:55.7453811Z   | |
2019-07-31T20:19:55.7454007Z   | expected &{integer}, found integer
2019-07-31T20:19:55.7454234Z   | help: try using a variant of the expected type: `Err(*right_val)`
2019-07-31T20:19:55.7454432Z   |
2019-07-31T20:19:55.7455180Z   = note: expected type `std::result::Result<&{integer}, _>`
2019-07-31T20:19:55.7455423Z              found type `std::result::Result<{integer}, _>`
2019-07-31T20:19:55.7456618Z 
2019-07-31T20:19:55.7456810Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7456946Z 
2019-07-31T20:19:55.7457341Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7457341Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7457732Z Couldn't compile the test.
2019-07-31T20:19:55.7458134Z ---- result.rs - result::Result<&'_ T, E>::cloned_ok (line 977) stdout ----
2019-07-31T20:19:55.7458313Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7458841Z   |
2019-07-31T20:19:55.7459162Z 8 | let cloned = x.cloned();
2019-07-31T20:19:55.7459302Z   |                ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7459430Z   |
2019-07-31T20:19:55.7459430Z   |
2019-07-31T20:19:55.7459693Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7459846Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7459997Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7460166Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7460425Z error: aborting due to previous error
2019-07-31T20:19:55.7460558Z 
2019-07-31T20:19:55.7460931Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7461449Z Couldn't compile the test.
2019-07-31T20:19:55.7461449Z Couldn't compile the test.
2019-07-31T20:19:55.7461868Z ---- result.rs - result::Result<&'_ T, &'_ mut E>::copied (line 943) stdout ----
2019-07-31T20:19:55.7462651Z  --> result.rs:945:1
2019-07-31T20:19:55.7462857Z   |
2019-07-31T20:19:55.7462857Z   |
2019-07-31T20:19:55.7462997Z 5 | assert_eq!(Err(&mut 1), Err(1));
2019-07-31T20:19:55.7463297Z   | |
2019-07-31T20:19:55.7463439Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7463439Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7463605Z   | help: try using a variant of the expected type: `Ok(*right_val)`
2019-07-31T20:19:55.7463761Z   |
2019-07-31T20:19:55.7463909Z   = note: expected type `std::result::Result<_, &mut {integer}>`
2019-07-31T20:19:55.7464246Z              found type `std::result::Result<_, {integer}>`
2019-07-31T20:19:55.7464904Z 
2019-07-31T20:19:55.7465064Z error[E0308]: mismatched types
2019-07-31T20:19:55.7465396Z  --> result.rs:946:1
2019-07-31T20:19:55.7466361Z   |
2019-07-31T20:19:55.7466361Z   |
2019-07-31T20:19:55.7466530Z 6 | assert_eq!(Ok(&42), Ok(42));
2019-07-31T20:19:55.7466814Z   | |
2019-07-31T20:19:55.7466814Z   | |
2019-07-31T20:19:55.7466975Z   | expected &{integer}, found integer
2019-07-31T20:19:55.7467147Z   | help: try using a variant of the expected type: `Err(*right_val)`
2019-07-31T20:19:55.7467290Z   |
2019-07-31T20:19:55.7467457Z   = note: expected type `std::result::Result<&{integer}, _>`
2019-07-31T20:19:55.7467717Z              found type `std::result::Result<{integer}, _>`
2019-07-31T20:19:55.7468468Z 
2019-07-31T20:19:55.7468611Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7468733Z 
2019-07-31T20:19:55.7469130Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7469130Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7469644Z Couldn't compile the test.
2019-07-31T20:19:55.7470042Z ---- result.rs - result::Result<&'_ mut T, &'_ E>::cloned (line 1074) stdout ----
2019-07-31T20:19:55.7470238Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7470743Z   |
2019-07-31T20:19:55.7470743Z   |
2019-07-31T20:19:55.7470879Z 5 | assert_eq!(Err(&1).cloned(), Err(1));
2019-07-31T20:19:55.7471019Z   |                    ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7471172Z   |
2019-07-31T20:19:55.7471328Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7471492Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7471665Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7471812Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7472093Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7472420Z  --> result.rs:1077:24
2019-07-31T20:19:55.7472584Z   |
2019-07-31T20:19:55.7472584Z   |
2019-07-31T20:19:55.7472744Z 6 | assert_eq!(Ok(&mut 42).cloned(), Ok(42));
2019-07-31T20:19:55.7472886Z   |                        ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7473038Z   |
2019-07-31T20:19:55.7473182Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7473339Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7473514Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7473666Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7473939Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7474060Z 
2019-07-31T20:19:55.7474425Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7474791Z Couldn't compile the test.
2019-07-31T20:19:55.7474791Z Couldn't compile the test.
2019-07-31T20:19:55.7475184Z ---- result.rs - result::Result<&'_ T, E>::copied_ok (line 829) stdout ----
2019-07-31T20:19:55.7475355Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7476417Z   |
2019-07-31T20:19:55.7476417Z   |
2019-07-31T20:19:55.7476558Z 8 | let copied = x.copied();
2019-07-31T20:19:55.7476725Z   |                ^^^^^^ multiple `copied` found
2019-07-31T20:19:55.7476885Z   |
2019-07-31T20:19:55.7477042Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7477363Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7477518Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7477686Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7477954Z error: aborting due to previous error
2019-07-31T20:19:55.7478073Z 
2019-07-31T20:19:55.7478501Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7478871Z Couldn't compile the test.
2019-07-31T20:19:55.7478871Z Couldn't compile the test.
2019-07-31T20:19:55.7479481Z ---- result.rs - result::Result<&'_ mut T, &'_ mut E>::cloned (line 1108) stdout ----
2019-07-31T20:19:55.7479681Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7480593Z   |
2019-07-31T20:19:55.7480593Z   |
2019-07-31T20:19:55.7480776Z 5 | assert_eq!(Err(&mut 1).cloned(), Err(1));
2019-07-31T20:19:55.7480958Z   |                        ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7481109Z   |
2019-07-31T20:19:55.7481337Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7481494Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7481640Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7481806Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7482089Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7482455Z  --> result.rs:1111:24
2019-07-31T20:19:55.7482628Z   |
2019-07-31T20:19:55.7482628Z   |
2019-07-31T20:19:55.7482762Z 6 | assert_eq!(Ok(&mut 42).cloned(), Ok(42));
2019-07-31T20:19:55.7482932Z   |                        ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7483067Z   |
2019-07-31T20:19:55.7483216Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7483390Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7483555Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7483701Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7483955Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7484091Z 
2019-07-31T20:19:55.7484451Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7484804Z Couldn't compile the test.
2019-07-31T20:19:55.7484804Z Couldn't compile the test.
2019-07-31T20:19:55.7485216Z ---- result.rs - result::Result<&'_ mut T, &'_ E>::copied (line 926) stdout ----
2019-07-31T20:19:55.7486341Z  --> result.rs:928:1
2019-07-31T20:19:55.7487315Z   |
2019-07-31T20:19:55.7487315Z   |
2019-07-31T20:19:55.7487535Z 5 | assert_eq!(Err(&1), Err(1));
2019-07-31T20:19:55.7487822Z   | |
2019-07-31T20:19:55.7487822Z   | |
2019-07-31T20:19:55.7487981Z   | expected &{integer}, found integer
2019-07-31T20:19:55.7488134Z   | help: try using a variant of the expected type: `Ok(*right_val)`
2019-07-31T20:19:55.7488279Z   |
2019-07-31T20:19:55.7488421Z   = note: expected type `std::result::Result<_, &{integer}>`
2019-07-31T20:19:55.7492881Z              found type `std::result::Result<_, {integer}>`
2019-07-31T20:19:55.7493984Z 
2019-07-31T20:19:55.7494135Z error[E0308]: mismatched types
2019-07-31T20:19:55.7494484Z  --> result.rs:929:1
2019-07-31T20:19:55.7494656Z   |
2019-07-31T20:19:55.7494656Z   |
2019-07-31T20:19:55.7494819Z 6 | assert_eq!(Ok(&mut 42), Ok(42));
2019-07-31T20:19:55.7495926Z   | |
2019-07-31T20:19:55.7496099Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7496099Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7496253Z   | help: try using a variant of the expected type: `Err(*right_val)`
2019-07-31T20:19:55.7496419Z   |
2019-07-31T20:19:55.7496565Z   = note: expected type `std::result::Result<&mut {integer}, _>`
2019-07-31T20:19:55.7496715Z              found type `std::result::Result<{integer}, _>`
2019-07-31T20:19:55.7497436Z 
2019-07-31T20:19:55.7497602Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7497724Z 
2019-07-31T20:19:55.7498115Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7498115Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7498489Z Couldn't compile the test.
2019-07-31T20:19:55.7499053Z ---- result.rs - result::Result<&'_ mut T, &'_ mut E>::copied (line 960) stdout ----
2019-07-31T20:19:55.7500034Z  --> result.rs:962:1
2019-07-31T20:19:55.7500389Z   |
2019-07-31T20:19:55.7500389Z   |
2019-07-31T20:19:55.7500530Z 5 | assert_eq!(Err(&mut 1), Err(1));
2019-07-31T20:19:55.7500823Z   | |
2019-07-31T20:19:55.7500962Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7500962Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7501130Z   | help: try using a variant of the expected type: `Ok(*right_val)`
2019-07-31T20:19:55.7501268Z   |
2019-07-31T20:19:55.7501406Z   = note: expected type `std::result::Result<_, &mut {integer}>`
2019-07-31T20:19:55.7501547Z              found type `std::result::Result<_, {integer}>`
2019-07-31T20:19:55.7502352Z 
2019-07-31T20:19:55.7502486Z error[E0308]: mismatched types
2019-07-31T20:19:55.7502804Z  --> result.rs:963:1
2019-07-31T20:19:55.7502999Z   |
2019-07-31T20:19:55.7502999Z   |
2019-07-31T20:19:55.7503140Z 6 | assert_eq!(Ok(&mut 42), Ok(42));
2019-07-31T20:19:55.7503616Z   | |
2019-07-31T20:19:55.7503755Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7503755Z   | expected mutable reference, found integer
2019-07-31T20:19:55.7503999Z   | help: try using a variant of the expected type: `Err(*right_val)`
2019-07-31T20:19:55.7504141Z   |
2019-07-31T20:19:55.7504278Z   = note: expected type `std::result::Result<&mut {integer}, _>`
2019-07-31T20:19:55.7504422Z              found type `std::result::Result<{integer}, _>`
2019-07-31T20:19:55.7505262Z 
2019-07-31T20:19:55.7505417Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7505918Z 
2019-07-31T20:19:55.7506345Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7506345Z For more information about this error, try `rustc --explain E0308`.
2019-07-31T20:19:55.7506737Z Couldn't compile the test.
2019-07-31T20:19:55.7507170Z ---- result.rs - result::Result<&'_ mut T, E>::cloned_ok (line 997) stdout ----
2019-07-31T20:19:55.7507394Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7507930Z   |
2019-07-31T20:19:55.7508075Z 8 | let cloned = x.cloned();
2019-07-31T20:19:55.7508220Z   |                ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7508383Z   |
2019-07-31T20:19:55.7508383Z   |
2019-07-31T20:19:55.7508531Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7508685Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7508856Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7509308Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7509608Z error: aborting due to previous error
2019-07-31T20:19:55.7509728Z 
2019-07-31T20:19:55.7510302Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7510679Z Couldn't compile the test.
2019-07-31T20:19:55.7510679Z Couldn't compile the test.
2019-07-31T20:19:55.7511450Z ---- result.rs - result::Result<&'_ mut T, E>::copied_ok (line 849) stdout ----
2019-07-31T20:19:55.7511697Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7513046Z   |
2019-07-31T20:19:55.7513046Z   |
2019-07-31T20:19:55.7513117Z 8 | let copied = x.copied();
2019-07-31T20:19:55.7513164Z   |                ^^^^^^ multiple `copied` found
2019-07-31T20:19:55.7513206Z   |
2019-07-31T20:19:55.7513276Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7513331Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7513386Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7513566Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7513661Z error: aborting due to previous error
2019-07-31T20:19:55.7513690Z 
2019-07-31T20:19:55.7514039Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7514237Z Couldn't compile the test.
2019-07-31T20:19:55.7514237Z Couldn't compile the test.
2019-07-31T20:19:55.7514476Z ---- result.rs - result::Result<T, &'_ E>::cloned_err (line 1017) stdout ----
2019-07-31T20:19:55.7514548Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7514775Z   |
2019-07-31T20:19:55.7514817Z 8 | let cloned = x.cloned();
2019-07-31T20:19:55.7514882Z   |                ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7514923Z   |
2019-07-31T20:19:55.7514923Z   |
2019-07-31T20:19:55.7514971Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7515052Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7515112Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7515167Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7515260Z error: aborting due to previous error
2019-07-31T20:19:55.7515653Z 
2019-07-31T20:19:55.7515946Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7516167Z Couldn't compile the test.
2019-07-31T20:19:55.7516167Z Couldn't compile the test.
2019-07-31T20:19:55.7516412Z ---- result.rs - result::Result<T, &'_ E>::copied_err (line 869) stdout ----
2019-07-31T20:19:55.7516469Z error[E0282]: type annotations needed for `std::result::Result<T, &i32>`
2019-07-31T20:19:55.7516725Z   |
2019-07-31T20:19:55.7516725Z   |
2019-07-31T20:19:55.7516766Z 6 | let x = Err(&val);
2019-07-31T20:19:55.7516997Z   |     -   ^^^ cannot infer type for `T`
2019-07-31T20:19:55.7517045Z   |     |
2019-07-31T20:19:55.7517110Z   |     consider giving `x` the explicit type `std::result::Result<T, &i32>`, where the type parameter `T` is specified
2019-07-31T20:19:55.7517219Z error: aborting due to previous error
2019-07-31T20:19:55.7517249Z 
2019-07-31T20:19:55.7517488Z For more information about this error, try `rustc --explain E0282`.
2019-07-31T20:19:55.7517684Z Couldn't compile the test.
2019-07-31T20:19:55.7517684Z Couldn't compile the test.
2019-07-31T20:19:55.7517951Z ---- result.rs - result::Result<T, &'_ mut E>::cloned_err (line 1037) stdout ----
2019-07-31T20:19:55.7518006Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7518259Z   |
2019-07-31T20:19:55.7518302Z 8 | let cloned = x.cloned();
2019-07-31T20:19:55.7518349Z   |                ^^^^^^ multiple `cloned` found
2019-07-31T20:19:55.7518411Z   |
2019-07-31T20:19:55.7518411Z   |
2019-07-31T20:19:55.7518461Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7518525Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7518711Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7518768Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7518846Z error: aborting due to previous error
2019-07-31T20:19:55.7518894Z 
2019-07-31T20:19:55.7519161Z For more information about this error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7519360Z Couldn't compile the test.
2019-07-31T20:19:55.7519360Z Couldn't compile the test.
2019-07-31T20:19:55.7519785Z ---- result.rs - result::Result<T, &'_ mut E>::copied_err (line 889) stdout ----
2019-07-31T20:19:55.7520171Z error[E0425]: cannot find value `cloned` in this scope
2019-07-31T20:19:55.7520445Z   |
2019-07-31T20:19:55.7520445Z   |
2019-07-31T20:19:55.7520486Z 9 | assert_eq!(cloned, Err(12));
2019-07-31T20:19:55.7520648Z 
2019-07-31T20:19:55.7520715Z error[E0034]: multiple applicable items in scope
2019-07-31T20:19:55.7520934Z  --> result.rs:894:16
2019-07-31T20:19:55.7520976Z   |
2019-07-31T20:19:55.7520976Z   |
2019-07-31T20:19:55.7521016Z 8 | let copied = x.copied();
2019-07-31T20:19:55.7521080Z   |                ^^^^^^ multiple `copied` found
2019-07-31T20:19:55.7521119Z   |
2019-07-31T20:19:55.7521165Z   = note: candidate #1 is defined in an impl for the type `std::result::Result<&_, &_>`
2019-07-31T20:19:55.7521236Z   = note: candidate #2 is defined in an impl for the type `std::result::Result<&mut _, &_>`
2019-07-31T20:19:55.7521288Z   = note: candidate #3 is defined in an impl for the type `std::result::Result<&_, &mut _>`
2019-07-31T20:19:55.7521340Z   = note: candidate #4 is defined in an impl for the type `std::result::Result<&mut _, &mut _>`
2019-07-31T20:19:55.7521431Z error: aborting due to 2 previous errors
2019-07-31T20:19:55.7521459Z 
2019-07-31T20:19:55.7521509Z Some errors have detailed explanations: E0034, E0425.
2019-07-31T20:19:55.7521754Z For more information about an error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7521754Z For more information about an error, try `rustc --explain E0034`.
2019-07-31T20:19:55.7521947Z Couldn't compile the test.
2019-07-31T20:19:55.7521976Z 
2019-07-31T20:19:55.7522032Z failures:
2019-07-31T20:19:55.7522247Z     result.rs - result::Result<&'_ T, &'_ E>::cloned (line 1057)
2019-07-31T20:19:55.7522465Z     result.rs - result::Result<&'_ T, &'_ E>::copied (line 909)
2019-07-31T20:19:55.7522688Z     result.rs - result::Result<&'_ T, &'_ mut E>::cloned (line 1091)
2019-07-31T20:19:55.7522928Z     result.rs - result::Result<&'_ T, &'_ mut E>::copied (line 943)
2019-07-31T20:19:55.7523144Z     result.rs - result::Result<&'_ T, E>::cloned_ok (line 977)
2019-07-31T20:19:55.7523361Z     result.rs - result::Result<&'_ T, E>::copied_ok (line 829)
2019-07-31T20:19:55.7523604Z     result.rs - result::Result<&'_ mut T, &'_ E>::cloned (line 1074)
2019-07-31T20:19:55.7523824Z     result.rs - result::Result<&'_ mut T, &'_ E>::copied (line 926)
2019-07-31T20:19:55.7524057Z     result.rs - result::Result<&'_ mut T, &'_ mut E>::cloned (line 1108)
2019-07-31T20:19:55.7524310Z     result.rs - result::Result<&'_ mut T, &'_ mut E>::copied (line 960)
2019-07-31T20:19:55.7524530Z     result.rs - result::Result<&'_ mut T, E>::cloned_ok (line 997)
2019-07-31T20:19:55.7524749Z     result.rs - result::Result<&'_ mut T, E>::copied_ok (line 849)
2019-07-31T20:19:55.7524986Z     result.rs - result::Result<T, &'_ E>::cloned_err (line 1017)
2019-07-31T20:19:55.7525204Z     result.rs - result::Result<T, &'_ E>::copied_err (line 869)
2019-07-31T20:19:55.7525425Z     result.rs - result::Result<T, &'_ mut E>::cloned_err (line 1037)
2019-07-31T20:19:55.7526101Z     result.rs - result::Result<T, &'_ mut E>::copied_err (line 889)
2019-07-31T20:19:55.7526192Z test result: FAILED. 2364 passed; 16 failed; 15 ignored; 0 measured; 0 filtered out
2019-07-31T20:19:55.7526225Z 
2019-07-31T20:19:55.7778491Z error: test failed, to rerun pass '--doc'
2019-07-31T20:19:55.7792774Z 
2019-07-31T20:19:55.7792774Z 
2019-07-31T20:19:55.7792868Z 
2019-07-31T20:19:55.7793885Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
2019-07-31T20:19:55.7794005Z 
2019-07-31T20:19:55.7794053Z 
2019-07-31T20:19:55.7840586Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-31T20:19:55.7840938Z Build completed unsuccessfully in 1:25:03
2019-07-31T20:19:55.7840938Z Build completed unsuccessfully in 1:25:03
2019-07-31T20:19:56.3300402Z ##[error]Bash exited with code '1'.
2019-07-31T20:19:56.3340830Z ##[section]Starting: Checkout
2019-07-31T20:19:56.3342561Z ==============================================================================
2019-07-31T20:19:56.3342609Z Task         : Get sources
2019-07-31T20:19:56.3342845Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
