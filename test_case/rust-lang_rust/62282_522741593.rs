plain
2019-08-19T18:59:07.1111458Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T18:59:07.1322744Z ##[command]git config gc.auto 0
2019-08-19T18:59:07.1371915Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T18:59:07.1417768Z ##[command]git config --get-all http.proxy
2019-08-19T18:59:07.1551505Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62282/merge:refs/remotes/pull/62282/merge
---
2019-08-19T18:59:42.8919235Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T18:59:42.8919550Z 
2019-08-19T18:59:42.8919917Z   git checkout -b <new-branch-name>
2019-08-19T18:59:42.8920353Z 
2019-08-19T18:59:42.8920452Z HEAD is now at 24afcad88 Merge 62adcb76fec2e4f4998426290a12baf7a1e803c5 into f86521e0a33a2b54c4c23dbfc5250013f7a33b11
2019-08-19T18:59:42.9088684Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-19T18:59:42.9092234Z ==============================================================================
2019-08-19T18:59:42.9092296Z Task         : Bash
2019-08-19T18:59:42.9092343Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-19T19:59:24.7316614Z .................................................................................................... 1500/8929
2019-08-19T19:59:30.3240939Z .................................................................................................... 1600/8929
2019-08-19T19:59:43.1324707Z .................................i...............i.................................................. 1700/8929
2019-08-19T19:59:50.7443121Z .................................................................................................... 1800/8929
2019-08-19T20:00:05.0522981Z .........................iiiii...................................................................... 1900/8929
2019-08-19T20:00:15.6142857Z .................................................................................................... 2100/8929
2019-08-19T20:00:18.1389762Z .................................................................................................... 2200/8929
2019-08-19T20:00:22.9188445Z .................................................................................................... 2300/8929
2019-08-19T20:00:29.7095596Z .................................................................................................... 2400/8929
---
2019-08-19T20:03:22.6760726Z .................................................................................................... 4600/8929
2019-08-19T20:03:29.7280297Z .........i...............i.......................................................................... 4700/8929
2019-08-19T20:03:40.8790165Z .................................................................................................... 4800/8929
2019-08-19T20:03:46.8819977Z .................................................................................................... 4900/8929
2019-08-19T20:03:58.4137634Z ..........................................................................................ii.ii..... 5000/8929
2019-08-19T20:04:07.9860257Z .................................................................................................... 5200/8929
2019-08-19T20:04:17.7069893Z .................................................................................................... 5300/8929
2019-08-19T20:04:24.4144475Z ..............................................i..................................................... 5400/8929
2019-08-19T20:04:31.1352943Z .................................................................................................... 5500/8929
2019-08-19T20:04:31.1352943Z .................................................................................................... 5500/8929
2019-08-19T20:04:43.1990519Z .................................................................................................... 5600/8929
2019-08-19T20:04:55.3415431Z .......................................ii...i..ii...........i....................................... 5700/8929
2019-08-19T20:05:14.3754837Z .................................................................................................... 5900/8929
2019-08-19T20:05:19.2784637Z .................................................................................................... 6000/8929
2019-08-19T20:05:19.2784637Z .................................................................................................... 6000/8929
2019-08-19T20:05:32.2385605Z ........................................i..ii....................................................... 6100/8929
2019-08-19T20:05:53.0641454Z ..................................................................................i................. 6300/8929
2019-08-19T20:05:55.1983055Z .................................................................................................... 6400/8929
2019-08-19T20:05:57.4443954Z ......................................................i............................................. 6500/8929
2019-08-19T20:06:00.6268894Z .................................................................................................... 6600/8929
---
2019-08-19T20:07:00.8436292Z .................................................................................................... 7200/8929
2019-08-19T20:07:07.6889178Z .................................................................................................... 7300/8929
2019-08-19T20:07:17.8827267Z .................................................................................................... 7400/8929
2019-08-19T20:07:27.4010335Z .................................................................................................... 7500/8929
2019-08-19T20:07:34.1177141Z ..ii......i......................................................................................... 7600/8929
2019-08-19T20:07:54.8261268Z .................................................................................................... 7800/8929
2019-08-19T20:08:03.7419334Z .................................................................................................... 7900/8929
2019-08-19T20:08:13.3865393Z .................................................................................................... 8000/8929
2019-08-19T20:08:51.5336644Z .................................................................................................... 8100/8929
---
2019-08-19T20:10:38.5278064Z  finished in 21.458
2019-08-19T20:10:38.5448003Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T20:10:38.7031265Z 
2019-08-19T20:10:38.7031693Z running 148 tests
2019-08-19T20:10:41.8011331Z i....iii......iii..iiii....i............................i..i..................i....i.........ii.i.i. 100/148
2019-08-19T20:10:43.7201106Z .iiii..............i.........iii.i......ii......
2019-08-19T20:10:43.7202400Z 
2019-08-19T20:10:43.7214612Z  finished in 5.176
2019-08-19T20:10:43.7373971Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T20:10:43.8923938Z 
---
2019-08-19T20:10:45.9276880Z  finished in 2.190
2019-08-19T20:10:45.9453462Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T20:10:46.1098470Z 
2019-08-19T20:10:46.1098690Z running 9 tests
2019-08-19T20:10:46.1099485Z iiiiiiiii
2019-08-19T20:10:46.1099859Z 
2019-08-19T20:10:46.1099905Z  finished in 0.164
2019-08-19T20:10:46.1279231Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T20:10:46.2862777Z 
---
2019-08-19T20:11:04.5812382Z  finished in 17.936
2019-08-19T20:11:04.5812709Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T20:11:04.5812881Z 
2019-08-19T20:11:04.5812933Z running 122 tests
2019-08-19T20:11:28.0215569Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
2019-08-19T20:11:32.6661951Z .i.i......iii.i.....ii
2019-08-19T20:11:32.6662891Z 
2019-08-19T20:11:32.6666755Z  finished in 28.583
2019-08-19T20:11:32.6675214Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-19T20:11:32.6675725Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-19T20:25:19.8118797Z running 970 tests
2019-08-19T20:25:19.8231803Z .................................................................................................... 100/970
2019-08-19T20:25:19.8311345Z .................................................................................................... 200/970
2019-08-19T20:25:19.8377512Z .................................................................................................... 300/970
2019-08-19T20:25:19.9179362Z ................................................................................................ii.. 400/970
2019-08-19T20:25:20.0563687Z .................................................................................................... 600/970
2019-08-19T20:25:20.0682097Z .................................................................................................... 700/970
2019-08-19T20:25:20.0851696Z .................................................................................................... 800/970
2019-08-19T20:25:20.6606033Z .................................................................................................... 900/970
2019-08-19T20:25:20.6606033Z .................................................................................................... 900/970
2019-08-19T20:25:21.8238686Z ......................................................................
2019-08-19T20:25:21.8239872Z test result: ok. 968 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2019-08-19T20:25:21.8241480Z 
2019-08-19T20:25:21.8244549Z    Doc-tests core
2019-08-19T20:25:26.8274583Z 
2019-08-19T20:25:26.8274942Z running 2388 tests
2019-08-19T20:25:39.3277561Z ......iiiii......................................................................................... 100/2388
2019-08-19T20:25:51.5081702Z .........................................................................ii......................... 200/2388
2019-08-19T20:26:21.1241111Z .................................................................................................... 400/2388
2019-08-19T20:26:21.1241111Z .................................................................................................... 400/2388
2019-08-19T20:26:32.4454535Z ..............................i..i.................iiii............................................. 500/2388
2019-08-19T20:26:56.0603550Z .................................................................................................... 700/2388
2019-08-19T20:27:08.0495076Z .................................................................................................... 800/2388
2019-08-19T20:27:20.0214872Z .................................................................................................... 900/2388
2019-08-19T20:27:31.7280142Z .................................................................................................... 1000/2388
---
2019-08-19T20:29:19.2612854Z .................................................................................................... 1900/2388
2019-08-19T20:29:32.8167965Z .................................................................................................... 2000/2388
2019-08-19T20:29:45.2298628Z .................................................................................................... 2100/2388
2019-08-19T20:30:00.8003789Z .................................................................................................... 2200/2388
2019-08-19T20:30:17.0114294Z ..........F..F..FF...............................................................................i.. 2300/2388
2019-08-19T20:30:28.8522678Z failures:
2019-08-19T20:30:28.8522831Z 
2019-08-19T20:30:28.8524486Z ---- slice/mod.rs - slice::[T]::take_back (line 2571) stdout ----
2019-08-19T20:30:28.8524704Z Test executable failed (exit code 101).
2019-08-19T20:30:28.8524704Z Test executable failed (exit code 101).
2019-08-19T20:30:28.8524836Z 
2019-08-19T20:30:28.8525007Z stderr:
2019-08-19T20:30:28.8525409Z thread 'main' panicked at 'assertion failed: `(left == right)`
2019-08-19T20:30:28.8525786Z   left: `['b', 'c']`,
2019-08-19T20:30:28.8526194Z  right: `['a', 'b']`', slice/mod.rs:10:1
2019-08-19T20:30:28.8526573Z 
2019-08-19T20:30:28.8526695Z 
2019-08-19T20:30:28.8526695Z 
2019-08-19T20:30:28.8527235Z ---- slice/mod.rs - slice::[T]::take_first_mut (line 2663) stdout ----
2019-08-19T20:30:28.8527794Z  --> slice/mod.rs:2667:35
2019-08-19T20:30:28.8527965Z   |
2019-08-19T20:30:28.8527965Z   |
2019-08-19T20:30:28.8528139Z 7 | let first = <[_]>::take_first_mut(&mut slice).unwrap();
2019-08-19T20:30:28.8528500Z   |                                   ^^^^^^^^^^ types differ in mutability
2019-08-19T20:30:28.8528687Z   |
2019-08-19T20:30:28.8528849Z   = note: expected type `&mut &mut [_]`
2019-08-19T20:30:28.8529170Z              found type `&mut &[char]`
2019-08-19T20:30:28.8529462Z error: aborting due to previous error
2019-08-19T20:30:28.8529583Z 
2019-08-19T20:30:28.8529999Z For more information about this error, try `rustc --explain E0308`.
2019-08-19T20:30:28.8530387Z Couldn't compile the test.
2019-08-19T20:30:28.8530387Z Couldn't compile the test.
2019-08-19T20:30:28.8531038Z ---- slice/mod.rs - slice::[T]::take_last_mut (line 2714) stdout ----
2019-08-19T20:30:28.8531213Z error[E0308]: mismatched types
2019-08-19T20:30:28.8531556Z  --> slice/mod.rs:2718:33
2019-08-19T20:30:28.8531719Z   |
2019-08-19T20:30:28.8531860Z 7 | let last = <[_]>::take_last_mut(&mut slice).unwrap();
2019-08-19T20:30:28.8532043Z   |                                 ^^^^^^^^^^ types differ in mutability
2019-08-19T20:30:28.8532176Z   |
2019-08-19T20:30:28.8532362Z   = note: expected type `&mut &mut [_]`
2019-08-19T20:30:28.8532507Z              found type `&mut &[char]`
2019-08-19T20:30:28.8532754Z error: aborting due to previous error
2019-08-19T20:30:28.8533282Z 
2019-08-19T20:30:28.8533713Z For more information about this error, try `rustc --explain E0308`.
2019-08-19T20:30:28.8534082Z Couldn't compile the test.
2019-08-19T20:30:28.8534082Z Couldn't compile the test.
2019-08-19T20:30:28.8534514Z ---- slice/mod.rs - slice::[T]::take_last (line 2689) stdout ----
2019-08-19T20:30:28.8534702Z error[E0425]: cannot find value `first` in this scope
2019-08-19T20:30:28.8535269Z   --> slice/mod.rs:2696:12
2019-08-19T20:30:28.8535457Z    |
2019-08-19T20:30:28.8535807Z 10 | assert_eq!(first, &'c');
2019-08-19T20:30:28.8536144Z 
2019-08-19T20:30:28.8536288Z error: aborting due to previous error
2019-08-19T20:30:28.8536409Z 
2019-08-19T20:30:28.8536972Z For more information about this error, try `rustc --explain E0425`.
---
2019-08-19T20:30:28.8539114Z 
2019-08-19T20:30:28.8853699Z error: test failed, to rerun pass '--doc'
2019-08-19T20:30:28.8869846Z 
2019-08-19T20:30:28.8869988Z 
2019-08-19T20:30:28.8870698Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
2019-08-19T20:30:28.8871050Z 
2019-08-19T20:30:28.8871082Z 
2019-08-19T20:30:28.8884375Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-19T20:30:28.8884536Z Build completed unsuccessfully in 1:24:18
2019-08-19T20:30:28.8884536Z Build completed unsuccessfully in 1:24:18
2019-08-19T20:30:28.8947159Z == clock drift check ==
2019-08-19T20:30:28.8960872Z   local time: Mon Aug 19 20:30:28 UTC 2019
2019-08-19T20:30:29.1762296Z   network time: Mon, 19 Aug 2019 20:30:29 GMT
2019-08-19T20:30:29.1765310Z == end clock drift check ==
2019-08-19T20:30:29.7368099Z ##[error]Bash exited with code '1'.
2019-08-19T20:30:29.7408540Z ##[section]Starting: Checkout
2019-08-19T20:30:29.7410271Z ==============================================================================
2019-08-19T20:30:29.7410340Z Task         : Get sources
2019-08-19T20:30:29.7410384Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
