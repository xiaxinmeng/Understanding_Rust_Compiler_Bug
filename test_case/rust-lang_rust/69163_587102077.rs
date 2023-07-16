plain
2020-02-17T16:06:40.8392298Z ========================== Starting Command Output ===========================
2020-02-17T16:06:40.8692177Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e1df4455-ab71-427a-b331-ceb20f1ac219.sh
2020-02-17T16:06:40.8692231Z 
2020-02-17T16:06:40.8695946Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-17T16:06:40.8701375Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-17T16:06:40.8703560Z Task         : Get sources
2020-02-17T16:06:40.8703608Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T16:06:40.8703637Z Version      : 1.0.0
2020-02-17T16:06:40.8703666Z Author       : Microsoft
---
2020-02-17T16:06:41.7508790Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-17T16:06:41.7607288Z ##[command]git config gc.auto 0
2020-02-17T16:06:41.7679665Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-17T16:06:41.7733897Z ##[command]git config --get-all http.proxy
2020-02-17T16:06:41.7865015Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69163/merge:refs/remotes/pull/69163/merge
---
2020-02-17T17:03:40.4195815Z .................................................................................................... 1700/9650
2020-02-17T17:03:44.8180715Z .................................................................................................... 1800/9650
2020-02-17T17:03:55.4915617Z ..................................i................................................................. 1900/9650
2020-02-17T17:04:02.4326634Z .................................................................................................... 2000/9650
2020-02-17T17:04:15.2273594Z ........................iiiii....................................................................... 2100/9650
2020-02-17T17:04:23.9322100Z .................................................................................................... 2300/9650
2020-02-17T17:04:26.0636155Z .................................................................................................... 2400/9650
2020-02-17T17:04:30.1719501Z .................................................................................................... 2500/9650
2020-02-17T17:04:48.7697439Z .................................................................................................... 2600/9650
---
2020-02-17T17:07:45.7642374Z .................................................................................................... 5600/9650
2020-02-17T17:07:54.9044185Z .......................................................................................i............ 5700/9650
2020-02-17T17:08:01.9201192Z .................................................................................................... 5800/9650
2020-02-17T17:08:06.7221167Z .....................................................................................i.............. 5900/9650
2020-02-17T17:08:15.4825433Z ..............................................................................ii...i..ii...........i 6000/9650
2020-02-17T17:08:35.5804683Z .................................................................................................... 6200/9650
2020-02-17T17:08:42.4910207Z .................................................................................................... 6300/9650
2020-02-17T17:08:49.7142609Z .................................................................................................... 6400/9650
2020-02-17T17:08:49.7142609Z .................................................................................................... 6400/9650
2020-02-17T17:09:03.6086062Z ......i..ii......................................................................................... 6500/9650
2020-02-17T17:09:21.9863009Z ..............................................................................................i..... 6700/9650
2020-02-17T17:09:23.9181149Z .................................................................................................... 6800/9650
2020-02-17T17:09:25.8783111Z .................................................................................................... 6900/9650
2020-02-17T17:09:28.0029999Z .....i.............................................................................................. 7000/9650
---
2020-02-17T17:10:54.1495293Z .................................................................................................... 7600/9650
2020-02-17T17:10:58.4643533Z .................................................................................................... 7700/9650
2020-02-17T17:11:04.0329887Z .................................................................................................... 7800/9650
2020-02-17T17:11:10.1256972Z .................................................................................................... 7900/9650
2020-02-17T17:11:18.8708746Z .......................................................................................iiiiiii.i.... 8000/9650
2020-02-17T17:11:33.5993798Z ...........................i......i................................................................. 8200/9650
2020-02-17T17:11:38.1156091Z .................................................................................................... 8300/9650
2020-02-17T17:11:48.4956305Z .................................................................................................... 8400/9650
2020-02-17T17:11:59.5133416Z .................................................................................................... 8500/9650
---
2020-02-17T17:14:11.3506237Z  finished in 6.970
2020-02-17T17:14:11.3693340Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-17T17:14:11.5562816Z 
2020-02-17T17:14:11.5562937Z running 178 tests
2020-02-17T17:14:14.3469852Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-17T17:14:17.3447350Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-17T17:14:17.3504093Z 
2020-02-17T17:14:17.3504160Z  finished in 5.081
2020-02-17T17:14:17.3504439Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-17T17:14:17.3504473Z 
---
2020-02-17T17:14:18.4277578Z  finished in 1.962
2020-02-17T17:14:18.4444353Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-17T17:14:18.5977146Z 
2020-02-17T17:14:18.5977372Z running 9 tests
2020-02-17T17:14:18.5978383Z iiiiiiiii
2020-02-17T17:14:18.5978724Z 
2020-02-17T17:14:18.5979985Z  finished in 0.153
2020-02-17T17:14:18.6144165Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-17T17:14:19.3495993Z 
---
2020-02-17T17:14:38.1243980Z  finished in 19.510
2020-02-17T17:14:38.1402358Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-17T17:14:38.2952131Z 
2020-02-17T17:14:38.2955825Z running 116 tests
2020-02-17T17:14:50.9344932Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-17T17:14:52.8453662Z ....iiii.....ii.
2020-02-17T17:14:52.8455387Z 
2020-02-17T17:14:52.8456552Z  finished in 14.705
2020-02-17T17:14:52.8461624Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-17T17:14:52.8462289Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-17T17:27:13.6709318Z 
2020-02-17T17:27:13.6710025Z    Doc-tests core
2020-02-17T17:27:18.2788994Z 
2020-02-17T17:27:18.2790427Z running 2471 tests
2020-02-17T17:27:27.0786063Z ......iiiii......................................................................................... 100/2471
2020-02-17T17:27:35.9830151Z ..................................................................................ii................ 200/2471
2020-02-17T17:27:56.7743826Z .................i.................................................................................. 400/2471
2020-02-17T17:27:56.7743826Z .................i.................................................................................. 400/2471
2020-02-17T17:28:06.3812324Z ......................................................................i..i..................iiii.... 500/2471
2020-02-17T17:28:23.2115099Z .................................................................................................... 700/2471
2020-02-17T17:28:31.9475016Z .................................................................................................... 800/2471
2020-02-17T17:28:40.7467770Z .................................................................................................... 900/2471
2020-02-17T17:28:49.5658830Z .................................................................................................... 1000/2471
---
2020-02-17T17:32:15.7238623Z .................................................................................................... 500/760
2020-02-17T17:32:15.7556970Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2741:22
2020-02-17T17:32:15.7573372Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2766:17
2020-02-17T17:32:15.7577591Z thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2778:21
2020-02-17T17:32:15.7612959Z ........thread '..<unnamed>' panicked at '.called `Result::unwrap()` on an `Err` value: RecvError.', .src/libstd/sync/mpsc/mod.rs.:.2645.:13.
2020-02-17T17:32:16.0112916Z ..................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1997:22
2020-02-17T17:32:16.0130175Z ...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libstd/sync/mpsc/mod.rs:2022:17
2020-02-17T17:32:16.0139028Z ..thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:2034:21
2020-02-17T17:32:16.0157074Z ....thread '.<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libstd/sync/mpsc/mod.rs:1916:13
2020-02-17T17:32:18.0529238Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2020-02-17T17:32:18.0538568Z .thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2020-02-17T17:32:18.0546970Z ...thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2020-02-17T17:32:18.0556040Z .thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2020-02-17T17:32:27.2334588Z 
2020-02-17T17:32:27.2335138Z running 1009 tests
2020-02-17T17:32:44.3451877Z i................................................................................................... 100/1009
2020-02-17T17:32:53.9707273Z .................................................................................................... 200/1009
2020-02-17T17:33:00.5879878Z ..................iii......i......i...i......i...................................................... 300/1009
2020-02-17T17:33:05.3603954Z .................................................................................................... 400/1009
2020-02-17T17:33:11.8751734Z ............................................i..i.....................................ii............. 500/1009
2020-02-17T17:33:24.1843195Z .................................................................................................... 700/1009
2020-02-17T17:33:24.1843195Z .................................................................................................... 700/1009
2020-02-17T17:33:30.7140999Z ...................................iiii............................................................. 800/1009
2020-02-17T17:33:43.9659688Z .................................................................................................... 900/1009
2020-02-17T17:33:50.4967963Z .........................................................iiii....................................... 1000/1009
2020-02-17T17:33:50.8613741Z test result: ok. 989 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-02-17T17:33:50.8613814Z 
2020-02-17T17:33:50.8714886Z  finished in 161.547
2020-02-17T17:33:50.8725273Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-02-17T17:50:46.1745716Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-02-17T17:50:46.5156651Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-02-17T17:50:46.6375690Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-02-17T17:50:47.9066774Z     Finished release [optimized] target(s) in 1.38s
2020-02-17T17:50:51.7093950Z std/io/struct.IntoInnerError.html:84: broken link - std/io/trait.Error.html
2020-02-17T17:50:51.7408533Z std/io/struct.Error.html:204: broken link - std/io/trait.Error.html
2020-02-17T17:50:51.7459787Z std/array/struct.TryFromSliceError.html:16: broken link - std/array/trait.Error.html
2020-02-17T17:50:51.7626315Z std/convert/enum.Infallible.html:66: broken link - std/convert/trait.Error.html
2020-02-17T17:50:52.0537387Z std/alloc/struct.CannotReallocInPlace.html:20: broken link - std/alloc/trait.Error.html
2020-02-17T17:50:52.0573493Z std/alloc/struct.LayoutErr.html:20: broken link - std/alloc/trait.Error.html
2020-02-17T17:50:52.0598634Z std/alloc/struct.AllocErr.html:21: broken link - std/alloc/trait.Error.html
2020-02-17T17:50:52.3855363Z std/string/struct.FromUtf16Error.html:23: broken link - std/string/trait.Error.html
2020-02-17T17:50:52.3868475Z std/string/struct.FromUtf8Error.html:77: broken link - std/string/trait.Error.html
2020-02-17T17:50:52.4652997Z std/time/struct.SystemTimeError.html:46: broken link - std/time/trait.Error.html
2020-02-17T17:50:52.7174922Z std/thread/struct.AccessError.html:17: broken link - std/thread/trait.Error.html
2020-02-17T17:50:52.8203147Z std/num/struct.ParseFloatError.html:19: broken link - std/num/trait.Error.html
2020-02-17T17:50:52.8263666Z std/num/struct.TryFromIntError.html:19: broken link - std/num/trait.Error.html
2020-02-17T17:50:52.8279700Z std/num/struct.ParseIntError.html:25: broken link - std/num/trait.Error.html
2020-02-17T17:50:52.9370738Z std/sync/enum.TryLockError.html:25: broken link - std/sync/trait.Error.html
2020-02-17T17:50:52.9399092Z std/sync/struct.PoisonError.html:67: broken link - std/sync/trait.Error.html
2020-02-17T17:50:52.9476395Z std/sync/mpsc/struct.SendError.html:22: broken link - std/sync/mpsc/trait.Error.html
2020-02-17T17:50:52.9570420Z std/sync/mpsc/enum.TrySendError.html:31: broken link - std/sync/mpsc/trait.Error.html
2020-02-17T17:50:52.9602812Z std/sync/mpsc/enum.TryRecvError.html:30: broken link - std/sync/mpsc/trait.Error.html
2020-02-17T17:50:52.9622222Z std/sync/mpsc/enum.RecvTimeoutError.html:30: broken link - std/sync/mpsc/trait.Error.html
2020-02-17T17:50:52.9639425Z std/sync/mpsc/struct.RecvError.html:23: broken link - std/sync/mpsc/trait.Error.html
2020-02-17T17:50:53.0446989Z std/env/enum.VarError.html:28: broken link - std/env/trait.Error.html
2020-02-17T17:50:53.0780273Z std/env/struct.JoinPathsError.html:14: broken link - std/env/trait.Error.html
2020-02-17T17:50:53.1499904Z std/ffi/struct.IntoStringError.html:26: broken link - std/ffi/trait.Error.html
2020-02-17T17:50:53.1528058Z std/ffi/struct.FromBytesWithNulError.html:27: broken link - std/ffi/trait.Error.html
2020-02-17T17:50:53.1563526Z std/ffi/struct.NulError.html:46: broken link - std/ffi/trait.Error.html
2020-02-17T17:50:53.2379627Z std/str/struct.Utf8Error.html:83: broken link - std/str/trait.Error.html
2020-02-17T17:50:53.2762407Z std/str/struct.ParseBoolError.html:17: broken link - std/str/trait.Error.html
2020-02-17T17:50:53.3700248Z std/char/struct.ParseCharError.html:17: broken link - std/char/trait.Error.html
2020-02-17T17:50:53.3836354Z std/char/struct.DecodeUtf16Error.html:18: broken link - std/char/trait.Error.html
2020-02-17T17:50:53.3900763Z std/char/struct.CharTryFromError.html:17: broken link - std/char/trait.Error.html
2020-02-17T17:50:53.4346269Z std/path/struct.StripPrefixError.html:20: broken link - std/path/trait.Error.html
2020-02-17T17:50:53.9424000Z std/boxed/struct.Box.html:637: broken link - std/boxed/trait.Error.html
2020-02-17T17:50:53.9459132Z std/cell/struct.BorrowError.html:13: broken link - std/cell/trait.Error.html
2020-02-17T17:50:53.9535699Z std/cell/struct.BorrowMutError.html:13: broken link - std/cell/trait.Error.html
2020-02-17T17:50:54.0347133Z std/fmt/struct.Error.html:51: broken link - std/fmt/trait.Error.html
2020-02-17T17:50:54.0445093Z std/net/struct.AddrParseError.html:34: broken link - std/net/trait.Error.html
2020-02-17T17:50:54.3687487Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-02-17T17:50:54.3717303Z 
2020-02-17T17:50:54.3717374Z 
2020-02-17T17:50:54.3717855Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-02-17T17:50:54.3717965Z expected success, got: exit code: 101
---
2020-02-17T17:50:54.5881568Z   local time: Mon Feb 17 17:50:54 UTC 2020
2020-02-17T17:50:54.8723404Z   network time: Mon, 17 Feb 2020 17:50:54 GMT
2020-02-17T17:50:54.8723868Z == end clock drift check ==
2020-02-17T17:50:56.5949386Z 
2020-02-17T17:50:56.6030600Z ##[error]Bash exited with code '1'.
2020-02-17T17:50:56.6042871Z ##[section]Finishing: Run build
2020-02-17T17:50:56.6072172Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-17T17:50:56.6074260Z Task         : Get sources
2020-02-17T17:50:56.6074316Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-17T17:50:56.6074692Z Version      : 1.0.0
2020-02-17T17:50:56.6074726Z Author       : Microsoft
2020-02-17T17:50:56.6074726Z Author       : Microsoft
2020-02-17T17:50:56.6074781Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-17T17:50:56.6074824Z ==============================================================================
2020-02-17T17:50:56.9844802Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-17T17:50:56.9880734Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69163/merge to s
2020-02-17T17:50:56.9982730Z Cleaning up task key
2020-02-17T17:50:56.9983383Z Start cleaning up orphan processes.
2020-02-17T17:50:57.0081175Z Terminate orphan process: pid (3317) (python)
2020-02-17T17:50:57.0765916Z ##[section]Finishing: Finalize Job
