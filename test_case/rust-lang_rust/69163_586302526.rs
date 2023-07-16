plain
2020-02-14T12:05:32.7401133Z ========================== Starting Command Output ===========================
2020-02-14T12:05:32.7402568Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e8757c02-f7c0-49fe-8eeb-81c2e86844c2.sh
2020-02-14T12:05:32.7402606Z 
2020-02-14T12:05:32.7405878Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-14T12:05:32.7469334Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-14T12:05:32.7471456Z Task         : Get sources
2020-02-14T12:05:32.7471492Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T12:05:32.7471525Z Version      : 1.0.0
2020-02-14T12:05:32.7471575Z Author       : Microsoft
---
2020-02-14T12:05:33.8174048Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-14T12:05:33.8185594Z ##[command]git config gc.auto 0
2020-02-14T12:05:33.8187930Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-14T12:05:33.8189701Z ##[command]git config --get-all http.proxy
2020-02-14T12:05:33.8197884Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69163/merge:refs/remotes/pull/69163/merge
---
2020-02-14T13:10:37.2692752Z .................................................................................................... 1700/9637
2020-02-14T13:10:42.3747799Z .................................................................................................... 1800/9637
2020-02-14T13:10:55.1369557Z ..............................i..................................................................... 1900/9637
2020-02-14T13:11:03.1434044Z .................................................................................................... 2000/9637
2020-02-14T13:11:18.7376284Z ....................iiiii........................................................................... 2100/9637
2020-02-14T13:11:29.1744743Z .................................................................................................... 2300/9637
2020-02-14T13:11:31.7311053Z .................................................................................................... 2400/9637
2020-02-14T13:11:36.7549409Z .................................................................................................... 2500/9637
2020-02-14T13:11:59.6193660Z .................................................................................................... 2600/9637
---
2020-02-14T13:14:46.0491033Z ........................................................................i...............i........... 4900/9637
2020-02-14T13:14:54.1720635Z .................................................................................................... 5000/9637
2020-02-14T13:15:03.0505240Z .................................................................................................... 5100/9637
2020-02-14T13:15:08.0179805Z ..............i..................................................................................... 5200/9637
2020-02-14T13:15:19.9243495Z ........................................................................................ii.ii....... 5300/9637
2020-02-14T13:15:24.2827490Z .i...i.............................................................................................. 5400/9637
2020-02-14T13:15:35.2150732Z .................................................................................................... 5600/9637
2020-02-14T13:15:45.8729690Z .............................................................................i...................... 5700/9637
2020-02-14T13:15:53.8816956Z .................................................................................................... 5800/9637
2020-02-14T13:16:00.4836366Z ...........................................................................i........................ 5900/9637
2020-02-14T13:16:00.4836366Z ...........................................................................i........................ 5900/9637
2020-02-14T13:16:10.8879810Z .....................................................................ii...i..ii...........i......... 6000/9637
2020-02-14T13:16:33.6507783Z .................................................................................................... 6200/9637
2020-02-14T13:16:41.5190473Z .................................................................................................... 6300/9637
2020-02-14T13:16:49.2467773Z .................................................................................................i.. 6400/9637
2020-02-14T13:17:03.4125158Z ii.................................................................................................. 6500/9637
---
2020-02-14T13:19:16.3931401Z .................................................................................................... 7600/9637
2020-02-14T13:19:20.9807693Z .................................................................................................... 7700/9637
2020-02-14T13:19:27.5525616Z .................................................................................................... 7800/9637
2020-02-14T13:19:36.0894944Z .................................................................................................... 7900/9637
2020-02-14T13:19:45.3429915Z ............................................................................iiiiiii.i............... 8000/9637
2020-02-14T13:20:02.7693241Z ................i......i............................................................................ 8200/9637
2020-02-14T13:20:08.5714745Z .................................................................................................... 8300/9637
2020-02-14T13:20:22.5536138Z .................................................................................................... 8400/9637
2020-02-14T13:20:33.0313106Z .................................................................................................... 8500/9637
---
2020-02-14T13:23:08.9913512Z  finished in 7.649
2020-02-14T13:23:09.0121756Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T13:23:09.1997578Z 
2020-02-14T13:23:09.2000050Z running 178 tests
2020-02-14T13:23:12.3013707Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-14T13:23:14.7843377Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-14T13:23:14.7845557Z 
2020-02-14T13:23:14.7850011Z  finished in 5.773
2020-02-14T13:23:14.8049330Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T13:23:14.9879688Z 
---
2020-02-14T13:23:17.0142655Z  finished in 2.208
2020-02-14T13:23:17.0341637Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T13:23:17.2050558Z 
2020-02-14T13:23:17.2050830Z running 9 tests
2020-02-14T13:23:17.2051966Z iiiiiiiii
2020-02-14T13:23:17.2052727Z 
2020-02-14T13:23:17.2054646Z  finished in 0.170
2020-02-14T13:23:17.2255182Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T13:23:17.3974914Z 
---
2020-02-14T13:23:38.7565195Z  finished in 21.531
2020-02-14T13:23:38.7810798Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T13:23:38.9547270Z 
2020-02-14T13:23:38.9550322Z running 116 tests
2020-02-14T13:23:53.3789466Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-14T13:23:55.3680716Z ....iiii.....ii.
2020-02-14T13:23:55.3681154Z 
2020-02-14T13:23:55.3683701Z  finished in 16.587
2020-02-14T13:23:55.3690348Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T13:23:55.3691008Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-14T13:38:27.6097838Z 
2020-02-14T13:38:27.6098874Z    Doc-tests core
2020-02-14T13:38:32.7627325Z 
2020-02-14T13:38:32.7628023Z running 2471 tests
2020-02-14T13:38:42.2928241Z ......iiiii......................................................................................... 100/2471
2020-02-14T13:38:51.6010802Z ..................................................................................ii................ 200/2471
2020-02-14T13:39:14.3200861Z .................i.................................................................................. 400/2471
2020-02-14T13:39:14.3200861Z .................i.................................................................................. 400/2471
2020-02-14T13:39:24.4347415Z ......................................................................i..i..................iiii.... 500/2471
2020-02-14T13:39:41.6217647Z .................................................................................................... 700/2471
2020-02-14T13:39:50.4510925Z .................................................................................................... 800/2471
2020-02-14T13:39:59.2923137Z .................................................................................................... 900/2471
2020-02-14T13:40:08.3018846Z .................................................................................................... 1000/2471
---
2020-02-14T13:43:46.5280110Z .................................................................................................... 500/760
2020-02-14T13:43:46.5564486Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-02-14T13:43:46.5586610Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-02-14T13:43:46.5590934Z <unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-02-14T13:43:46.5641503Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError..', src/libstd/sync/mpsc/mod.rs:2645:13
2020-02-14T13:43:46.7576964Z ........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-02-14T13:43:46.7601196Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-02-14T13:43:46.7638130Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-14T13:43:48.8117257Z ........................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-14T13:43:48.8117680Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-02-14T13:43:48.8118005Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-02-14T13:43:48.8118271Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-02-14T13:43:57.8538788Z 
2020-02-14T13:43:57.8539499Z running 1009 tests
2020-02-14T13:44:17.4932554Z i................................................................................................... 100/1009
2020-02-14T13:44:28.2241608Z .................................................................................................... 200/1009
2020-02-14T13:44:35.7231091Z ..................iii......i......i...i......i...................................................... 300/1009
2020-02-14T13:44:41.0336919Z .................................................................................................... 400/1009
2020-02-14T13:44:48.1529596Z ............................................i..i.....................................ii............. 500/1009
2020-02-14T13:45:01.5515716Z .................................................................................................... 700/1009
2020-02-14T13:45:01.5515716Z .................................................................................................... 700/1009
2020-02-14T13:45:08.6731817Z ...................................iiii............................................................. 800/1009
2020-02-14T13:45:23.7174450Z .................................................................................................... 900/1009
2020-02-14T13:45:30.9707093Z .........................................................iiii....................................... 1000/1009
2020-02-14T13:45:31.3779128Z test result: ok. 989 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-14T13:45:31.3779244Z 
2020-02-14T13:45:31.3911441Z  finished in 181.199
2020-02-14T13:45:31.3924973Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-14T14:06:19.2290612Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-02-14T14:06:19.6766684Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-02-14T14:06:19.8487488Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-02-14T14:06:21.5612715Z     Finished release [optimized] target(s) in 1.88s
2020-02-14T14:06:25.0445130Z std/sync/mpsc/struct.RecvError.html:23: broken link - std/sync/mpsc/trait.Error.html
2020-02-14T14:06:25.0471973Z std/sync/mpsc/enum.TryRecvError.html:30: broken link - std/sync/mpsc/trait.Error.html
2020-02-14T14:06:25.0493447Z std/sync/mpsc/enum.RecvTimeoutError.html:30: broken link - std/sync/mpsc/trait.Error.html
2020-02-14T14:06:25.0629125Z std/sync/mpsc/enum.TrySendError.html:31: broken link - std/sync/mpsc/trait.Error.html
2020-02-14T14:06:25.0650771Z std/sync/mpsc/struct.SendError.html:22: broken link - std/sync/mpsc/trait.Error.html
2020-02-14T14:06:25.1213670Z std/sync/struct.PoisonError.html:67: broken link - std/sync/trait.Error.html
2020-02-14T14:06:25.1245025Z std/sync/enum.TryLockError.html:25: broken link - std/sync/trait.Error.html
2020-02-14T14:06:25.1798675Z std/boxed/struct.Box.html:637: broken link - std/boxed/trait.Error.html
2020-02-14T14:06:25.1845710Z std/string/struct.FromUtf8Error.html:77: broken link - std/string/trait.Error.html
2020-02-14T14:06:25.1935960Z std/string/struct.FromUtf16Error.html:23: broken link - std/string/trait.Error.html
2020-02-14T14:06:25.2739644Z std/char/struct.ParseCharError.html:17: broken link - std/char/trait.Error.html
2020-02-14T14:06:25.2826634Z std/char/struct.CharTryFromError.html:17: broken link - std/char/trait.Error.html
2020-02-14T14:06:25.2912433Z std/char/struct.DecodeUtf16Error.html:18: broken link - std/char/trait.Error.html
2020-02-14T14:06:25.4201324Z std/ffi/struct.NulError.html:46: broken link - std/ffi/trait.Error.html
2020-02-14T14:06:25.4345993Z std/ffi/struct.IntoStringError.html:26: broken link - std/ffi/trait.Error.html
2020-02-14T14:06:25.4408805Z std/ffi/struct.FromBytesWithNulError.html:27: broken link - std/ffi/trait.Error.html
2020-02-14T14:06:25.5133855Z std/convert/enum.Infallible.html:66: broken link - std/convert/trait.Error.html
2020-02-14T14:06:26.2796469Z std/str/struct.Utf8Error.html:83: broken link - std/str/trait.Error.html
2020-02-14T14:06:26.2813624Z std/str/struct.ParseBoolError.html:17: broken link - std/str/trait.Error.html
2020-02-14T14:06:26.4701741Z std/fmt/struct.Error.html:51: broken link - std/fmt/trait.Error.html
2020-02-14T14:06:26.5491780Z std/cell/struct.BorrowError.html:13: broken link - std/cell/trait.Error.html
2020-02-14T14:06:26.5504033Z std/cell/struct.BorrowMutError.html:13: broken link - std/cell/trait.Error.html
2020-02-14T14:06:26.6382648Z std/thread/struct.AccessError.html:17: broken link - std/thread/trait.Error.html
2020-02-14T14:06:26.9541250Z std/net/struct.AddrParseError.html:34: broken link - std/net/trait.Error.html
2020-02-14T14:06:27.3460253Z std/time/struct.SystemTimeError.html:46: broken link - std/time/trait.Error.html
2020-02-14T14:06:27.3558225Z std/array/struct.TryFromSliceError.html:16: broken link - std/array/trait.Error.html
2020-02-14T14:06:27.4673982Z std/io/struct.Error.html:204: broken link - std/io/trait.Error.html
2020-02-14T14:06:27.5195757Z std/io/struct.IntoInnerError.html:84: broken link - std/io/trait.Error.html
2020-02-14T14:06:27.5982273Z std/path/struct.StripPrefixError.html:20: broken link - std/path/trait.Error.html
2020-02-14T14:06:27.7529889Z std/num/struct.TryFromIntError.html:19: broken link - std/num/trait.Error.html
2020-02-14T14:06:27.7584272Z std/num/struct.ParseIntError.html:25: broken link - std/num/trait.Error.html
2020-02-14T14:06:27.7606109Z std/num/struct.ParseFloatError.html:19: broken link - std/num/trait.Error.html
2020-02-14T14:06:27.9180881Z std/alloc/struct.AllocErr.html:21: broken link - std/alloc/trait.Error.html
2020-02-14T14:06:27.9227861Z std/alloc/struct.LayoutErr.html:20: broken link - std/alloc/trait.Error.html
2020-02-14T14:06:27.9249446Z std/alloc/struct.CannotReallocInPlace.html:20: broken link - std/alloc/trait.Error.html
2020-02-14T14:06:27.9551391Z std/env/struct.JoinPathsError.html:14: broken link - std/env/trait.Error.html
2020-02-14T14:06:27.9640207Z std/env/enum.VarError.html:28: broken link - std/env/trait.Error.html
2020-02-14T14:06:29.6910346Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-02-14T14:06:29.6948243Z 
2020-02-14T14:06:29.6948543Z 
2020-02-14T14:06:29.6949291Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-02-14T14:06:29.6949563Z expected success, got: exit code: 101
---
2020-02-14T14:06:29.7054280Z   local time: Fri Feb 14 14:06:29 UTC 2020
2020-02-14T14:06:29.8720379Z   network time: Fri, 14 Feb 2020 14:06:29 GMT
2020-02-14T14:06:29.8721498Z == end clock drift check ==
2020-02-14T14:06:31.7178944Z 
2020-02-14T14:06:31.7288219Z ##[error]Bash exited with code '1'.
2020-02-14T14:06:31.7304061Z ##[section]Finishing: Run build
2020-02-14T14:06:31.7329583Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-14T14:06:31.7331628Z Task         : Get sources
2020-02-14T14:06:31.7331676Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T14:06:31.7331722Z Version      : 1.0.0
2020-02-14T14:06:31.7331782Z Author       : Microsoft
2020-02-14T14:06:31.7331782Z Author       : Microsoft
2020-02-14T14:06:31.7331830Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-14T14:06:31.7331880Z ==============================================================================
2020-02-14T14:06:32.2193251Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-14T14:06:32.2246554Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-14T14:06:32.2373497Z Cleaning up task key
2020-02-14T14:06:32.2374256Z Start cleaning up orphan processes.
2020-02-14T14:06:32.2481621Z Terminate orphan process: pid (4087) (python)
2020-02-14T14:06:32.3239497Z ##[section]Finishing: Finalize Job
