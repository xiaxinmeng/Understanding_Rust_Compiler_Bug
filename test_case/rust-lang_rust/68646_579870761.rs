plain
2020-01-29T16:14:52.0178540Z ========================== Starting Command Output ===========================
2020-01-29T16:14:52.0180843Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6e030459-b286-47a7-9d97-bd15ada3df69.sh
2020-01-29T16:14:52.0180877Z 
2020-01-29T16:14:52.0184125Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-29T16:14:52.0190418Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68646/merge to s
2020-01-29T16:14:52.0192020Z Task         : Get sources
2020-01-29T16:14:52.0192056Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T16:14:52.0192131Z Version      : 1.0.0
2020-01-29T16:14:52.0192170Z Author       : Microsoft
---
2020-01-29T16:14:52.8756082Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-29T16:14:52.8846742Z ##[command]git config gc.auto 0
2020-01-29T16:14:52.8945080Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-29T16:14:52.8988599Z ##[command]git config --get-all http.proxy
2020-01-29T16:14:52.9133325Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68646/merge:refs/remotes/pull/68646/merge
---
2020-01-29T17:01:39.7001702Z .................................................................................................... 1700/9558
2020-01-29T17:01:44.5201779Z .................................................................................................... 1800/9558
2020-01-29T17:01:55.6941932Z .........................i.......................................................................... 1900/9558
2020-01-29T17:02:01.6843944Z .................................................................................................... 2000/9558
2020-01-29T17:02:14.5487471Z ...............iiiii................................................................................ 2100/9558
2020-01-29T17:02:23.2442574Z .................................................................................................... 2300/9558
2020-01-29T17:02:25.4758104Z .................................................................................................... 2400/9558
2020-01-29T17:02:30.4067118Z .................................................................................................... 2500/9558
2020-01-29T17:02:47.4337468Z .................................................................................................... 2600/9558
---
2020-01-29T17:05:00.2881290Z .................................................................................................... 4800/9558
2020-01-29T17:05:04.5952136Z ...........................................................i...............i........................ 4900/9558
2020-01-29T17:05:11.1307711Z .................................................................................................... 5000/9558
2020-01-29T17:05:18.0499748Z .................................................................................................... 5100/9558
2020-01-29T17:05:22.4993542Z ..i................................................................................................. 5200/9558
2020-01-29T17:05:31.4509524Z ...........................................................................ii.ii........i...i....... 5300/9558
2020-01-29T17:05:38.8693382Z .............i...................................................................................... 5500/9558
2020-01-29T17:05:47.2713360Z .................................................................................................... 5600/9558
2020-01-29T17:05:52.6174444Z ..............................................................i..................................... 5700/9558
2020-01-29T17:05:58.6331748Z .................................................................................................... 5800/9558
2020-01-29T17:05:58.6331748Z .................................................................................................... 5800/9558
2020-01-29T17:06:05.4533471Z .................................................................................................... 5900/9558
2020-01-29T17:06:12.7751725Z .....................................................ii...i..ii...........i......................... 6000/9558
2020-01-29T17:06:32.4522732Z .................................................................................................... 6200/9558
2020-01-29T17:06:38.2201966Z .................................................................................................... 6300/9558
2020-01-29T17:06:38.2201966Z .................................................................................................... 6300/9558
2020-01-29T17:06:42.1436514Z .................................................................................i..ii.............. 6400/9558
2020-01-29T17:07:04.7833373Z .................................................................................................... 6600/9558
2020-01-29T17:07:09.8329327Z .........................................................i.......................................... 6700/9558
2020-01-29T17:07:11.7548526Z .................................................................................................... 6800/9558
2020-01-29T17:07:13.8017080Z ........................................................i........................................... 6900/9558
---
2020-01-29T17:08:39.3267185Z .................................................................................................... 7600/9558
2020-01-29T17:08:43.7373871Z .................................................................................................... 7700/9558
2020-01-29T17:08:49.3090059Z .................................................................................................... 7800/9558
2020-01-29T17:08:58.7844041Z .................................................................................................... 7900/9558
2020-01-29T17:09:03.8497266Z ............iiiiiii.i............................................................................... 8000/9558
2020-01-29T17:09:16.9656497Z .................................................................................................... 8200/9558
2020-01-29T17:09:25.6763961Z .................................................................................................... 8300/9558
2020-01-29T17:09:37.0539691Z .................................................................................................... 8400/9558
2020-01-29T17:09:42.5156006Z .................................................................................................... 8500/9558
---
2020-01-29T17:11:42.7517747Z  finished in 6.538
2020-01-29T17:11:42.7737229Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T17:11:42.9279935Z 
2020-01-29T17:11:42.9280892Z running 169 tests
2020-01-29T17:11:45.5312485Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/169
2020-01-29T17:11:47.3601854Z i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-01-29T17:11:47.3604896Z 
2020-01-29T17:11:47.3609281Z  finished in 4.588
2020-01-29T17:11:47.3788690Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T17:11:47.5158978Z 
---
2020-01-29T17:11:49.1840206Z  finished in 1.805
2020-01-29T17:11:49.2006721Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T17:11:49.3243148Z 
2020-01-29T17:11:49.3244168Z running 9 tests
2020-01-29T17:11:49.3246058Z iiiiiiiii
2020-01-29T17:11:49.3246714Z 
2020-01-29T17:11:49.3248132Z  finished in 0.124
2020-01-29T17:11:49.3410481Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T17:11:49.4696554Z 
---
2020-01-29T17:12:06.8359905Z  finished in 17.494
2020-01-29T17:12:06.8546242Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T17:12:07.0010974Z 
2020-01-29T17:12:07.0011254Z running 116 tests
2020-01-29T17:12:18.2371542Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-01-29T17:12:19.8090187Z ....iiii.....ii.
2020-01-29T17:12:19.8090722Z 
2020-01-29T17:12:19.8095588Z  finished in 12.955
2020-01-29T17:12:19.8101713Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-29T17:12:19.8102370Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-29T17:23:24.1396224Z 
2020-01-29T17:23:24.1404843Z    Doc-tests core
2020-01-29T17:23:28.3710203Z 
2020-01-29T17:23:28.3711070Z running 2467 tests
2020-01-29T17:23:36.0081811Z ......iiiii......................................................................................... 100/2467
2020-01-29T17:23:43.4113863Z ..................................................................................ii................ 200/2467
2020-01-29T17:24:02.1781606Z .................i.................................................................................. 400/2467
2020-01-29T17:24:02.1781606Z .................i.................................................................................. 400/2467
2020-01-29T17:24:10.5720359Z ..................................................................i..i..................iiii........ 500/2467
2020-01-29T17:24:24.9596937Z .................................................................................................... 700/2467
2020-01-29T17:24:32.7273766Z .................................................................................................... 800/2467
2020-01-29T17:24:40.2482248Z .................................................................................................... 900/2467
2020-01-29T17:24:47.5686735Z .................................................................................................... 1000/2467
---
2020-01-29T17:27:40.3026087Z .................................................................................................... 500/760
2020-01-29T17:27:40.3306764Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-01-29T17:27:40.3322034Z ....thread '<unnamed>thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-01-29T17:27:40.3325332Z ' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-01-29T17:27:40.3343469Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2645:13
2020-01-29T17:27:40.6917450Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-01-29T17:27:40.6936029Z ......thread '<unnamed>.thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-01-29T17:27:40.6957340Z .....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-01-29T17:27:40.7048268Z .................. 600/760
2020-01-29T17:27:42.7346276Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-01-29T17:27:42.7347065Z .....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2020-01-29T17:27:51.5587293Z 
2020-01-29T17:27:51.5587557Z running 1003 tests
2020-01-29T17:28:06.4343355Z i................................................................................................... 100/1003
2020-01-29T17:28:15.2011236Z .................................................................................................... 200/1003
2020-01-29T17:28:21.0328415Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-29T17:28:25.1889061Z .................................................................................................... 400/1003
2020-01-29T17:28:31.2472598Z ..........................................i..i.....................................ii............... 500/1003
2020-01-29T17:28:41.6178574Z .................................................................................................... 700/1003
2020-01-29T17:28:41.6178574Z .................................................................................................... 700/1003
2020-01-29T17:28:47.2343574Z .............................iiii................................................................... 800/1003
2020-01-29T17:28:59.0425492Z .................................................................................................... 900/1003
2020-01-29T17:29:04.6255607Z ...................................................iiii............................................. 1000/1003
2020-01-29T17:29:04.6749841Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-29T17:29:04.6750037Z 
2020-01-29T17:29:04.6848131Z  finished in 141.638
2020-01-29T17:29:04.6858890Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-29T17:32:08.6439490Z ..................................................F....F............................................ 100/150
2020-01-29T17:32:08.6541179Z ..................................................
2020-01-29T17:32:08.6541298Z failures:
2020-01-29T17:32:08.6542715Z 
2020-01-29T17:32:08.6543325Z ---- spec::test_json_encode_decode::i586_pc_windows_msvc stdout ----
2020-01-29T17:32:08.6546053Z thread 'spec::test_json_encode_decode::i586_pc_windows_msvc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_target/spec/i686_pc_windows_msvc.rs:19:5
2020-01-29T17:32:08.6546238Z 
2020-01-29T17:32:08.6546505Z ---- spec::test_json_encode_decode::i686_pc_windows_msvc stdout ----
2020-01-29T17:32:08.6546505Z ---- spec::test_json_encode_decode::i686_pc_windows_msvc stdout ----
2020-01-29T17:32:08.6546837Z thread 'spec::test_json_encode_decode::i686_pc_windows_msvc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_target/spec/i686_pc_windows_msvc.rs:19:5
2020-01-29T17:32:08.6546919Z 
2020-01-29T17:32:08.6546962Z failures:
2020-01-29T17:32:08.6547009Z     spec::test_json_encode_decode::i586_pc_windows_msvc
2020-01-29T17:32:08.6547072Z     spec::test_json_encode_decode::i686_pc_windows_msvc
2020-01-29T17:32:08.6547072Z     spec::test_json_encode_decode::i686_pc_windows_msvc
2020-01-29T17:32:08.6547101Z 
2020-01-29T17:32:08.6547149Z test result: FAILED. 148 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2020-01-29T17:32:08.6547180Z 
2020-01-29T17:32:08.6563372Z error: test failed, to rerun pass '-p rustc_target --lib'
2020-01-29T17:32:08.6579689Z 
2020-01-29T17:32:08.6579689Z 
2020-01-29T17:32:08.6580546Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_target" "--" "--quiet"
2020-01-29T17:32:08.6580669Z 
2020-01-29T17:32:08.6580695Z 
2020-01-29T17:32:08.6630462Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-29T17:32:08.6631325Z Build completed unsuccessfully in 1:12:13
2020-01-29T17:32:08.6631325Z Build completed unsuccessfully in 1:12:13
2020-01-29T17:32:08.6647922Z == clock drift check ==
2020-01-29T17:32:08.6662803Z   local time: Wed Jan 29 17:32:08 UTC 2020
2020-01-29T17:32:08.8306239Z   network time: Wed, 29 Jan 2020 17:32:08 GMT
2020-01-29T17:32:08.8311422Z == end clock drift check ==
2020-01-29T17:32:09.2174196Z 
2020-01-29T17:32:09.2257374Z ##[error]Bash exited with code '1'.
2020-01-29T17:32:09.2266976Z ##[section]Finishing: Run build
2020-01-29T17:32:09.2292649Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68646/merge to s
2020-01-29T17:32:09.2294725Z Task         : Get sources
2020-01-29T17:32:09.2294780Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T17:32:09.2294845Z Version      : 1.0.0
2020-01-29T17:32:09.2294891Z Author       : Microsoft
2020-01-29T17:32:09.2294891Z Author       : Microsoft
2020-01-29T17:32:09.2294941Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-29T17:32:09.2295012Z ==============================================================================
2020-01-29T17:32:09.5708554Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-29T17:32:09.5742842Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68646/merge to s
2020-01-29T17:32:09.5833396Z Cleaning up task key
2020-01-29T17:32:09.5834165Z Start cleaning up orphan processes.
2020-01-29T17:32:09.5920596Z Terminate orphan process: pid (3666) (python)
2020-01-29T17:32:09.6133961Z ##[section]Finishing: Finalize Job
