plain
2019-12-31T18:03:09.4403917Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T18:03:09.4634800Z ##[command]git config gc.auto 0
2019-12-31T18:03:09.4708092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T18:03:09.4765214Z ##[command]git config --get-all http.proxy
2019-12-31T18:03:09.4899876Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67760/merge:refs/remotes/pull/67760/merge
---
2019-12-31T18:08:54.7187625Z fmt check
2019-12-31T18:09:00.2976451Z Build completed successfully in 0:01:26
2019-12-31T18:09:00.2982766Z + python2.7 ../x.py test
2019-12-31T18:09:00.4871606Z     Finished dev [unoptimized] target(s) in 0.14s
2019-12-31T18:09:06.8533740Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std"; dirty!
2019-12-31T18:09:06.8542339Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc"; dirty!
2019-12-31T18:09:06.9509131Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools"; dirty!
2019-12-31T18:09:07.1961964Z     Finished release [optimized] target(s) in 0.13s
2019-12-31T18:09:07.2011738Z tidy check
2019-12-31T18:09:08.3753494Z * 588 error codes
2019-12-31T18:09:08.3753614Z * highest error code: E0745
---
2019-12-31T18:28:06.2721749Z     Finished release [optimized] target(s) in 17m 56s
2019-12-31T18:28:06.3236285Z Installing libLLVM.so to stage 0 (x86_64-unknown-linux-gnu)
2019-12-31T18:28:06.3239639Z Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T18:28:06.3269465Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-12-31T18:28:06.3285123Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std"; dirty!
2019-12-31T18:28:06.7179339Z    Compiling cc v1.0.47
2019-12-31T18:28:06.7185574Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-12-31T18:28:11.7613294Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-31T18:28:12.6694174Z    Compiling libc v0.2.66
---
2019-12-31T18:28:52.2294885Z    Compiling getopts v0.2.21
2019-12-31T18:28:58.5690651Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-12-31T18:29:08.4492340Z     Finished release [optimized] target(s) in 1m 02s
2019-12-31T18:29:08.4618294Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T18:29:08.4632078Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc"; dirty!
2019-12-31T18:29:08.9268887Z    Compiling cfg-if v0.1.8
2019-12-31T18:29:08.9269244Z    Compiling libc v0.2.66
2019-12-31T18:29:08.9625392Z    Compiling semver-parser v0.7.0
2019-12-31T18:29:09.8435553Z    Compiling lazy_static v1.3.0
---
2019-12-31T18:53:26.8974948Z .................................................................................................... 1500/9464
2019-12-31T18:53:31.7045500Z .................................................................................................... 1600/9464
2019-12-31T18:53:35.8101631Z .................................................................................................... 1700/9464
2019-12-31T18:53:43.6708852Z .................................................................................................... 1800/9464
2019-12-31T18:53:50.4930502Z i................................................................................................... 1900/9464
2019-12-31T18:53:56.1883217Z ......................................................................................iiiii......... 2000/9464
2019-12-31T18:54:14.8804069Z .................................................................................................... 2200/9464
2019-12-31T18:54:16.9880995Z .................................................................................................... 2300/9464
2019-12-31T18:54:19.1408627Z .................................................................................................... 2400/9464
2019-12-31T18:54:24.6115609Z .................................................................................................... 2500/9464
---
2019-12-31T18:56:58.5261265Z .................i...............i.................................................................. 4900/9464
2019-12-31T18:57:06.7782511Z .................................................................................................... 5000/9464
2019-12-31T18:57:11.7542165Z ..............................................................i..................................... 5100/9464
2019-12-31T18:57:18.7944822Z .................................................................................................... 5200/9464
2019-12-31T18:57:25.5436468Z .............................ii.ii...........i...................................................... 5300/9464
2019-12-31T18:57:33.1720963Z .................................................................................................... 5500/9464
2019-12-31T18:57:41.7487706Z .................................................................................................... 5600/9464
2019-12-31T18:57:47.8755638Z ............i....................................................................................... 5700/9464
2019-12-31T18:57:53.0344091Z .................................................................................................... 5800/9464
2019-12-31T18:57:53.0344091Z .................................................................................................... 5800/9464
2019-12-31T18:58:01.9845826Z .................................................................................................... 5900/9464
2019-12-31T18:58:11.9489176Z ii...i..ii...........i.............................................................................. 6000/9464
2019-12-31T18:58:26.7783017Z .................................................................................................... 6200/9464
2019-12-31T18:58:32.7724394Z .................................................................................................... 6300/9464
2019-12-31T18:58:32.7724394Z .................................................................................................... 6300/9464
2019-12-31T18:58:44.8403219Z ...........................i..ii.................................................................... 6400/9464
2019-12-31T18:59:01.5069411Z .................................................................................................... 6600/9464
2019-12-31T18:59:03.2934229Z ..i................................................................................................. 6700/9464
2019-12-31T18:59:05.2456221Z .................................................................................................... 6800/9464
2019-12-31T18:59:07.4147776Z ..i................................................................................................. 6900/9464
---
2019-12-31T19:00:28.5309890Z .................................................................................................... 7500/9464
2019-12-31T19:00:32.7066616Z .................................................................................................... 7600/9464
2019-12-31T19:00:37.3941516Z .................................................................................................... 7700/9464
2019-12-31T19:00:45.8432783Z .................................................................................................... 7800/9464
2019-12-31T19:00:52.1978340Z .................................iiii............................................................... 7900/9464
2019-12-31T19:01:04.4801739Z .................................................................................................... 8100/9464
2019-12-31T19:01:11.7289200Z .................................................................................................... 8200/9464
2019-12-31T19:01:23.6532411Z .................................................................................................... 8300/9464
2019-12-31T19:01:30.0218842Z .................................................................................................... 8400/9464
---
2019-12-31T19:03:26.6821675Z  finished in 5.647
2019-12-31T19:03:26.6984824Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:03:26.8538978Z 
2019-12-31T19:03:26.8540329Z running 166 tests
2019-12-31T19:03:29.4237911Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-31T19:03:31.1664382Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-31T19:03:31.1664990Z 
2019-12-31T19:03:31.1668189Z  finished in 4.468
2019-12-31T19:03:31.1824674Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:03:31.3241501Z 
---
2019-12-31T19:03:32.9574432Z  finished in 1.775
2019-12-31T19:03:32.9744964Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:03:33.1168651Z 
2019-12-31T19:03:33.1169065Z running 9 tests
2019-12-31T19:03:33.1170058Z iiiiiiiii
2019-12-31T19:03:33.1172011Z 
2019-12-31T19:03:33.1172199Z  finished in 0.142
2019-12-31T19:03:33.1330939Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:03:33.2867016Z 
---
2019-12-31T19:03:49.6601490Z  finished in 16.527
2019-12-31T19:03:49.6789570Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:03:49.8346043Z 
2019-12-31T19:03:49.8346742Z running 124 tests
2019-12-31T19:04:10.2765152Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-31T19:04:13.7778829Z .i.iii.....iiiiii.....ii
2019-12-31T19:04:13.7780854Z 
2019-12-31T19:04:13.7781243Z  finished in 24.099
2019-12-31T19:04:13.7787532Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:04:13.7788241Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T19:04:13.7788241Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T19:04:13.7967451Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T19:04:13.9445631Z 
2019-12-31T19:04:13.9446376Z running 64 tests
2019-12-31T19:04:47.9568749Z ................................................................
2019-12-31T19:04:47.9570519Z test result: ok. 64 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-31T19:04:47.9571401Z 
2019-12-31T19:04:47.9571738Z  finished in 34.160
2019-12-31T19:04:47.9581649Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools"; dirty!
2019-12-31T19:04:48.1652031Z    Compiling cfg-if v0.1.8
2019-12-31T19:04:48.1656511Z    Compiling lazy_static v1.3.0
2019-12-31T19:04:48.2043184Z    Compiling libc v0.2.66
2019-12-31T19:04:48.2434619Z    Compiling semver-parser v0.7.0
---
2019-12-31T19:15:16.2727432Z 
2019-12-31T19:15:16.2728203Z    Doc-tests core
2019-12-31T19:15:19.8547400Z 
2019-12-31T19:15:19.8548931Z running 2439 tests
2019-12-31T19:15:27.5802188Z ......iiiii......................................................................................... 100/2439
2019-12-31T19:15:35.1598782Z ..................................................................................ii................ 200/2439
2019-12-31T19:15:52.6368422Z ................i................................................................................... 400/2439
2019-12-31T19:15:52.6368422Z ................i................................................................................... 400/2439
2019-12-31T19:16:00.8194247Z ................................................................i..i..................iiii.......... 500/2439
2019-12-31T19:16:15.3645029Z .................................................................................................... 700/2439
2019-12-31T19:16:23.0563112Z .................................................................................................... 800/2439
2019-12-31T19:16:30.5310953Z .................................................................................................... 900/2439
2019-12-31T19:16:38.0198858Z .................................................................................................... 1000/2439
---
2019-12-31T19:19:37.5506223Z .............................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2019-12-31T19:19:37.5559934Z .... 300/760
2019-12-31T19:19:37.5920690Z .................................................................................................... 400/760
2019-12-31T19:19:39.6835888Z .................................................................................................... 500/760
2019-12-31T19:19:39.7042583Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T19:19:39.7077051Z ....thread '<unnamed>thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1189:5
2019-12-31T19:19:39.7091117Z ' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError..', ..src/libcore/result.rs:...thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T19:19:39.7100589Z 1189:.......5..
2019-12-31T19:19:39.9056951Z .................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T19:19:39.9084271Z ....thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T19:19:39.9117860Z .......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T19:19:39.9494222Z ................... 600/760
2019-12-31T19:19:41.9779482Z ....................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2019-12-31T19:19:41.9787717Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
---
2019-12-31T19:19:50.9824549Z 
2019-12-31T19:19:50.9824796Z running 1002 tests
2019-12-31T19:20:05.7990919Z i................................................................................................... 100/1002
2019-12-31T19:20:14.0356021Z .................................................................................................... 200/1002
2019-12-31T19:20:19.9350634Z .................iii.......i.....i...i......i....................................................... 300/1002
2019-12-31T19:20:23.9569368Z .................................................................................................... 400/1002
2019-12-31T19:20:29.6656779Z .........................................i..i.....................................ii................ 500/1002
2019-12-31T19:20:40.1761607Z .................................................................................................... 700/1002
2019-12-31T19:20:40.1761607Z .................................................................................................... 700/1002
2019-12-31T19:20:45.7053037Z ............................iiii.................................................................... 800/1002
2019-12-31T19:20:57.6373622Z .................................................................................................... 900/1002
2019-12-31T19:21:03.4242036Z ..................................................iiii.............................................. 1000/1002
2019-12-31T19:21:03.4773039Z test result: ok. 982 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-12-31T19:21:03.4773238Z 
2019-12-31T19:21:03.4857457Z  finished in 150.841
2019-12-31T19:21:03.4870072Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-12-31T19:24:29.6849901Z 
2019-12-31T19:24:29.6849945Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-31T19:24:29.6850030Z 
2019-12-31T19:24:29.6881025Z  finished in 0.521
2019-12-31T19:24:29.6890649Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools"; dirty!
2019-12-31T19:24:29.8811789Z    Compiling cfg-if v0.1.8
2019-12-31T19:24:29.8819925Z    Compiling libc v0.2.66
2019-12-31T19:24:29.9264629Z    Compiling lazy_static v1.3.0
2019-12-31T19:24:30.0018457Z    Compiling semver-parser v0.7.0
---
2019-12-31T19:34:49.3573587Z Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
2019-12-31T19:34:49.5973894Z Documenting standalone (x86_64-unknown-linux-gnu)
2019-12-31T19:34:49.9458380Z Documenting book redirect pages (x86_64-unknown-linux-gnu)
2019-12-31T19:34:50.7482323Z Documenting stage2 std (x86_64-unknown-linux-gnu)
2019-12-31T19:34:50.7489374Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/crate-docs"; dirty!
2019-12-31T19:34:50.7497155Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std"; dirty!
2019-12-31T19:34:51.0711059Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-31T19:34:56.3342489Z    Compiling compiler_builtins v0.1.22
2019-12-31T19:35:06.2983964Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2019-12-31T19:35:07.1305142Z  Documenting alloc v0.0.0 (/checkout/src/liballoc)
---
2019-12-31T19:35:57.2708775Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2019-12-31T19:35:57.5632967Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2019-12-31T19:35:57.6984104Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2019-12-31T19:35:59.2644919Z     Finished release [optimized] target(s) in 1.49s
2019-12-31T19:35:59.2692121Z cargo/print.html:1517: broken link - std/primitive.char.html
2019-12-31T19:35:59.2692386Z cargo/print.html:2404: broken link - std/macro.debug_assert.html
2019-12-31T19:35:59.2692605Z cargo/print.html:3700: broken link - std/macro.env.html
2019-12-31T19:35:59.2692841Z cargo/print.html:3963: broken link - std/macro.include.html
2019-12-31T19:35:59.2693049Z cargo/print.html:3964: broken link - std/macro.concat.html
2019-12-31T19:35:59.2693251Z cargo/print.html:3964: broken link - std/macro.env.html
2019-12-31T19:35:59.2693498Z cargo/print.html:4254: broken link - std/macro.cfg.html
2019-12-31T19:35:59.2693897Z cargo/print.html:4942: broken link - std/primitive.char.html
2019-12-31T19:35:59.2694184Z cargo/reference/build-scripts.html:297: broken link - std/macro.env.html
2019-12-31T19:35:59.2694427Z cargo/reference/registries.html:292: broken link - std/primitive.char.html
2019-12-31T19:35:59.2694647Z cargo/reference/manifest.html:229: broken link - std/primitive.char.html
2019-12-31T19:35:59.2694868Z cargo/reference/profiles.html:210: broken link - std/macro.debug_assert.html
2019-12-31T19:35:59.2695112Z cargo/reference/build-script-examples.html:260: broken link - std/macro.include.html
2019-12-31T19:35:59.2695345Z cargo/reference/build-script-examples.html:261: broken link - std/macro.concat.html
2019-12-31T19:35:59.2695571Z cargo/reference/build-script-examples.html:261: broken link - std/macro.env.html
2019-12-31T19:35:59.2695817Z cargo/reference/build-script-examples.html:551: broken link - std/macro.cfg.html
2019-12-31T19:35:59.4008776Z nomicon/dropck.html:421: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T19:35:59.4028273Z nomicon/other-reprs.html:215: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:35:59.4051534Z nomicon/panic-handler.html:154: broken link - core/panic/struct.PanicInfo.html
2019-12-31T19:35:59.4066282Z nomicon/unchecked-uninit.html:159: broken link - core/mem/union.MaybeUninit.html
2019-12-31T19:35:59.4066595Z nomicon/unchecked-uninit.html:203: broken link - core/mem/union.MaybeUninit.html
2019-12-31T19:35:59.4066819Z nomicon/unchecked-uninit.html:239: broken link - core/ptr/index.html
2019-12-31T19:35:59.4067038Z nomicon/unchecked-uninit.html:241: broken link - core/ptr/fn.write.html
2019-12-31T19:35:59.4067279Z nomicon/unchecked-uninit.html:241: broken link - std/ptr/fn.copy.html
2019-12-31T19:35:59.4067508Z nomicon/unchecked-uninit.html:241: broken link - std/ptr/fn.copy_nonoverlapping.html
2019-12-31T19:35:59.4075926Z nomicon/safe-unsafe-meaning.html:183: broken link - std/primitive.pointer.html
2019-12-31T19:35:59.4076234Z nomicon/safe-unsafe-meaning.html:191: broken link - std/marker/trait.Send.html
2019-12-31T19:35:59.4076466Z nomicon/safe-unsafe-meaning.html:193: broken link - std/marker/trait.Sync.html
2019-12-31T19:35:59.4076715Z nomicon/safe-unsafe-meaning.html:195: broken link - std/alloc/trait.GlobalAlloc.html
2019-12-31T19:35:59.4079959Z nomicon/transmutes.html:157: broken link - std/mem/fn.transmute.html
2019-12-31T19:35:59.4080207Z nomicon/transmutes.html:178: broken link - std/mem/fn.transmute_copy.html
2019-12-31T19:35:59.4103081Z nomicon/print.html:283: broken link - std/primitive.pointer.html
2019-12-31T19:35:59.4103649Z nomicon/print.html:291: broken link - std/marker/trait.Send.html
2019-12-31T19:35:59.4109105Z nomicon/print.html:293: broken link - std/marker/trait.Sync.html
2019-12-31T19:35:59.4109539Z nomicon/print.html:295: broken link - std/alloc/trait.GlobalAlloc.html
2019-12-31T19:35:59.4109803Z nomicon/print.html:949: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:35:59.4111566Z nomicon/print.html:2331: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T19:35:59.4119526Z nomicon/print.html:2903: broken link - std/mem/fn.transmute.html
2019-12-31T19:35:59.4120103Z nomicon/print.html:2924: broken link - std/mem/fn.transmute_copy.html
2019-12-31T19:35:59.4120523Z nomicon/print.html:3127: broken link - core/mem/union.MaybeUninit.html
2019-12-31T19:35:59.4120944Z nomicon/print.html:3171: broken link - core/mem/union.MaybeUninit.html
2019-12-31T19:35:59.4121318Z nomicon/print.html:3207: broken link - core/ptr/index.html
2019-12-31T19:35:59.4121753Z nomicon/print.html:3209: broken link - core/ptr/fn.write.html
2019-12-31T19:35:59.4122156Z nomicon/print.html:3209: broken link - std/ptr/fn.copy.html
2019-12-31T19:35:59.4122564Z nomicon/print.html:3209: broken link - std/ptr/fn.copy_nonoverlapping.html
2019-12-31T19:35:59.4123046Z nomicon/print.html:5850: broken link - std/ops/trait.Drop.html
2019-12-31T19:35:59.4123452Z nomicon/print.html:6214: broken link - std/panic/fn.catch_unwind.html
2019-12-31T19:35:59.4124034Z nomicon/print.html:6230: broken link - std/panic/fn.catch_unwind.html
2019-12-31T19:35:59.4124507Z nomicon/print.html:6231: broken link - std/panic/fn.catch_unwind.html
2019-12-31T19:35:59.4124910Z nomicon/print.html:6294: broken link - core/panic/struct.PanicInfo.html
2019-12-31T19:35:59.4198060Z nomicon/ffi.html:355: broken link - std/ops/trait.Drop.html
2019-12-31T19:35:59.4198345Z nomicon/ffi.html:719: broken link - std/panic/fn.catch_unwind.html
2019-12-31T19:35:59.4198601Z nomicon/ffi.html:735: broken link - std/panic/fn.catch_unwind.html
2019-12-31T19:35:59.4198829Z nomicon/ffi.html:736: broken link - std/panic/fn.catch_unwind.html
2019-12-31T19:36:00.1829491Z book/ch09-02-recoverable-errors-with-result.html:191: broken link - std/index.html
2019-12-31T19:36:00.1943726Z book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html:468: broken link - std/prelude/index.html
2019-12-31T19:36:00.2121244Z book/procedural-macros.html:43: broken link - proc_macro/index.html
2019-12-31T19:36:00.2286456Z book/ch06-01-defining-an-enum.html:304: broken link - std/net/enum.IpAddr.html
2019-12-31T19:36:00.2286779Z book/ch06-01-defining-an-enum.html:441: broken link - std/option/enum.Option.html
2019-12-31T19:36:00.2287025Z book/ch06-01-defining-an-enum.html:519: broken link - std/option/enum.Option.html
2019-12-31T19:36:00.2598265Z book/appendix-03-derivable-traits.html:170: broken link - std/index.html
2019-12-31T19:36:00.2598558Z book/casting-between-types.html:51: broken link - std/mem/fn.transmute.html
2019-12-31T19:36:00.2752112Z book/print.html:884: broken link - std/prelude/index.html
2019-12-31T19:36:00.2752409Z book/print.html:929: broken link - std/string/struct.String.html
2019-12-31T19:36:00.2752696Z book/print.html:948: broken link - std/io/struct.Stdin.html
2019-12-31T19:36:00.2752938Z book/print.html:951: broken link - std/io/struct.Stdin.html
2019-12-31T19:36:00.2753170Z book/print.html:981: broken link - std/io/type.Result.html
2019-12-31T19:36:00.2753450Z book/print.html:982: broken link - std/result/enum.Result.html
2019-12-31T19:36:00.2753693Z book/print.html:994: broken link - std/result/enum.Result.html
2019-12-31T19:36:00.2753925Z book/print.html:1372: broken link - std/primitive.str.html
2019-12-31T19:36:00.2754181Z book/print.html:1907: broken link - std/num/struct.Wrapping.html
2019-12-31T19:36:00.2763640Z book/print.html:4846: broken link - std/net/enum.IpAddr.html
2019-12-31T19:36:00.2763887Z book/print.html:4983: broken link - std/option/enum.Option.html
2019-12-31T19:36:00.2766811Z book/print.html:5061: broken link - std/option/enum.Option.html
2019-12-31T19:36:00.2767043Z book/print.html:6246: broken link - std/prelude/index.html
2019-12-31T19:36:00.2767260Z book/print.html:6334: broken link - std/collections/index.html
2019-12-31T19:36:00.2767536Z book/print.html:7486: broken link - std/index.html
2019-12-31T19:36:00.2816075Z book/print.html:24040: broken link - std/index.html
2019-12-31T19:36:00.2840649Z book/ch08-00-common-collections.html:173: broken link - std/collections/index.html
2019-12-31T19:36:00.2873953Z book/borrow-and-asref.html:45: broken link - std/convert/trait.AsRef.html
2019-12-31T19:36:00.2874231Z book/borrow-and-asref.html:47: broken link - std/convert/trait.AsRef.html
2019-12-31T19:36:00.3929173Z book/ch02-00-guessing-game-tutorial.html:236: broken link - std/prelude/index.html
2019-12-31T19:36:00.3929576Z book/ch02-00-guessing-game-tutorial.html:281: broken link - std/string/struct.String.html
2019-12-31T19:36:00.3929854Z book/ch02-00-guessing-game-tutorial.html:300: broken link - std/io/struct.Stdin.html
2019-12-31T19:36:00.3930139Z book/ch02-00-guessing-game-tutorial.html:303: broken link - std/io/struct.Stdin.html
2019-12-31T19:36:00.3930406Z book/ch02-00-guessing-game-tutorial.html:333: broken link - std/io/type.Result.html
2019-12-31T19:36:00.3930671Z book/ch02-00-guessing-game-tutorial.html:334: broken link - std/result/enum.Result.html
2019-12-31T19:36:00.3930951Z book/ch02-00-guessing-game-tutorial.html:346: broken link - std/result/enum.Result.html
2019-12-31T19:36:00.3931636Z book/ch02-00-guessing-game-tutorial.html:724: broken link - std/primitive.str.html
2019-12-31T19:36:00.3950422Z book/ch03-02-data-types.html:256: broken link - std/num/struct.Wrapping.html
2019-12-31T19:36:00.5523104Z index.html:83: broken link - std/index.html
2019-12-31T19:36:00.5553574Z error-index.html:7: broken link - light1.42.0.css
2019-12-31T19:36:00.5647853Z reference/expressions/call-expr.html:169: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.5648169Z reference/expressions/call-expr.html:169: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.5648405Z reference/expressions/call-expr.html:170: broken link - std/ops/trait.FnOnce.html
2019-12-31T19:36:00.5687278Z reference/expressions/array-expr.html:233: broken link - std/ops/trait.Index.html
2019-12-31T19:36:00.5687594Z reference/expressions/array-expr.html:233: broken link - std/ops/trait.IndexMut.html
2019-12-31T19:36:00.5706851Z reference/expressions/await-expr.html:164: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.5707380Z reference/expressions/await-expr.html:169: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.5707629Z reference/expressions/await-expr.html:170: broken link - std/pin/struct.Pin.html
2019-12-31T19:36:00.5707862Z reference/expressions/await-expr.html:171: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.5708109Z reference/expressions/await-expr.html:173: broken link - std/task/enum.Poll.html
2019-12-31T19:36:00.5708339Z reference/expressions/await-expr.html:177: broken link - std/task/enum.Poll.html
2019-12-31T19:36:00.5708564Z reference/expressions/await-expr.html:178: broken link - std/task/enum.Poll.html
2019-12-31T19:36:00.5708809Z reference/expressions/await-expr.html:186: broken link - std/task/struct.Context.html
2019-12-31T19:36:00.5811191Z reference/expressions/block-expr.html:251: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.5811466Z reference/expressions/block-expr.html:252: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.5861539Z reference/special-types-and-traits.html:158: broken link - std/index.html
2019-12-31T19:36:00.5861826Z reference/special-types-and-traits.html:162: broken link - std/boxed/struct.Box.html
2019-12-31T19:36:00.5862059Z reference/special-types-and-traits.html:173: broken link - std/rc/struct.Rc.html
2019-12-31T19:36:00.5862320Z reference/special-types-and-traits.html:175: broken link - std/sync/struct.Arc.html
2019-12-31T19:36:00.5862550Z reference/special-types-and-traits.html:177: broken link - std/pin/struct.Pin.html
2019-12-31T19:36:00.5862786Z reference/special-types-and-traits.html:179: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.5863046Z reference/special-types-and-traits.html:184: broken link - std/marker/struct.PhantomData.html
2019-12-31T19:36:00.5863578Z reference/special-types-and-traits.html:188: broken link - std/ops/index.html
2019-12-31T19:36:00.5863848Z reference/special-types-and-traits.html:188: broken link - std/cmp/index.html
2019-12-31T19:36:00.5864110Z reference/special-types-and-traits.html:191: broken link - std/ops/trait.Deref.html
2019-12-31T19:36:00.5864346Z reference/special-types-and-traits.html:191: broken link - std/ops/trait.DerefMut.html
2019-12-31T19:36:00.5864592Z reference/special-types-and-traits.html:194: broken link - std/ops/trait.Drop.html
2019-12-31T19:36:00.5864825Z reference/special-types-and-traits.html:197: broken link - std/marker/trait.Copy.html
2019-12-31T19:36:00.5865056Z reference/special-types-and-traits.html:213: broken link - std/clone/trait.Clone.html
2019-12-31T19:36:00.5865303Z reference/special-types-and-traits.html:222: broken link - std/marker/trait.Send.html
2019-12-31T19:36:00.5865538Z reference/special-types-and-traits.html:225: broken link - std/marker/trait.Sync.html
2019-12-31T19:36:00.5865769Z reference/special-types-and-traits.html:229: broken link - std/marker/trait.Send.html
2019-12-31T19:36:00.5866015Z reference/special-types-and-traits.html:229: broken link - std/marker/trait.Sync.html
2019-12-31T19:36:00.5866401Z reference/special-types-and-traits.html:229: broken link - std/panic/trait.UnwindSafe.html
2019-12-31T19:36:00.5866694Z reference/special-types-and-traits.html:229: broken link - std/panic/trait.RefUnwindSafe.html
2019-12-31T19:36:00.5866951Z reference/special-types-and-traits.html:258: broken link - std/marker/trait.Sized.html
2019-12-31T19:36:00.5870350Z reference/attributes/codegen.html:246: broken link - std/macro.is_x86_feature_detected.html
2019-12-31T19:36:00.5880349Z reference/attributes/derive.html:162: broken link - std/cmp/trait.PartialEq.html
2019-12-31T19:36:00.5880660Z reference/attributes/derive.html:162: broken link - std/clone/trait.Clone.html
2019-12-31T19:36:00.5904370Z reference/attributes/testing.html:175: broken link - std/process/trait.Termination.html
2019-12-31T19:36:00.5946114Z reference/crates-and-source-files.html:239: broken link - std/index.html
2019-12-31T19:36:00.5946666Z reference/crates-and-source-files.html:239: broken link - std/prelude/index.html
2019-12-31T19:36:00.5947352Z reference/crates-and-source-files.html:241: broken link - core/index.html
2019-12-31T19:36:00.5947839Z reference/crates-and-source-files.html:242: broken link - core/prelude/index.html
2019-12-31T19:36:00.5948652Z reference/crates-and-source-files.html:266: broken link - std/process/trait.Termination.html
2019-12-31T19:36:00.5949131Z reference/crates-and-source-files.html:283: broken link - std/primitive.char.html
2019-12-31T19:36:00.6000464Z reference/type-layout.html:170: broken link - std/mem/fn.align_of_val.html
2019-12-31T19:36:00.6001174Z reference/type-layout.html:174: broken link - std/mem/fn.size_of_val.html
2019-12-31T19:36:00.6001630Z reference/type-layout.html:176: broken link - std/marker/trait.Sized.html
2019-12-31T19:36:00.6002011Z reference/type-layout.html:176: broken link - std/mem/fn.size_of.html
2019-12-31T19:36:00.6002412Z reference/type-layout.html:177: broken link - std/mem/fn.align_of.html
2019-12-31T19:36:00.6002816Z reference/type-layout.html:177: broken link - std/marker/trait.Sized.html
2019-12-31T19:36:00.6037260Z reference/items/unions.html:211: broken link - std/mem/fn.transmute.html
2019-12-31T19:36:00.6138547Z reference/items/enumerations.html:213: broken link - std/mem/fn.discriminant.html
2019-12-31T19:36:00.6154821Z reference/procedural-macros.html:187: broken link - std/macro.compile_error.html
2019-12-31T19:36:00.6155362Z reference/procedural-macros.html:190: broken link - proc_macro/index.html
2019-12-31T19:36:00.6155822Z reference/procedural-macros.html:192: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6156222Z reference/procedural-macros.html:217: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6156632Z reference/procedural-macros.html:218: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6157030Z reference/procedural-macros.html:254: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6157414Z reference/procedural-macros.html:255: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6157852Z reference/procedural-macros.html:257: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6158248Z reference/procedural-macros.html:314: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6158650Z reference/procedural-macros.html:317: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6159038Z reference/procedural-macros.html:317: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6159422Z reference/procedural-macros.html:318: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6159827Z reference/procedural-macros.html:332: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6244371Z reference/conditional-compilation.html:317: broken link - std/macro.debug_assert.html
2019-12-31T19:36:00.6259466Z reference/introduction.html:175: broken link - std/index.html
2019-12-31T19:36:00.6336427Z reference/print.html:177: broken link - std/index.html
2019-12-31T19:36:00.6361670Z reference/print.html:1985: broken link - std/macro.compile_error.html
2019-12-31T19:36:00.6362270Z reference/print.html:1988: broken link - proc_macro/index.html
2019-12-31T19:36:00.6362738Z reference/print.html:1990: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6363166Z reference/print.html:2015: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6363613Z reference/print.html:2016: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6364018Z reference/print.html:2052: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6364435Z reference/print.html:2053: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6364842Z reference/print.html:2055: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6365237Z reference/print.html:2112: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6365655Z reference/print.html:2115: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6366265Z reference/print.html:2115: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6366678Z reference/print.html:2116: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6367105Z reference/print.html:2130: broken link - proc_macro/struct.TokenStream.html
2019-12-31T19:36:00.6368064Z reference/print.html:2257: broken link - std/index.html
2019-12-31T19:36:00.6368328Z reference/print.html:2257: broken link - std/prelude/index.html
2019-12-31T19:36:00.6368556Z reference/print.html:2259: broken link - core/index.html
2019-12-31T19:36:00.6368786Z reference/print.html:2260: broken link - core/prelude/index.html
2019-12-31T19:36:00.6369043Z reference/print.html:2284: broken link - std/process/trait.Termination.html
2019-12-31T19:36:00.6369277Z reference/print.html:2301: broken link - std/primitive.char.html
2019-12-31T19:36:00.6372867Z reference/print.html:2473: broken link - std/macro.debug_assert.html
2019-12-31T19:36:00.6373635Z reference/print.html:3555: broken link - std/mem/fn.discriminant.html
2019-12-31T19:36:00.6373897Z reference/print.html:3721: broken link - std/mem/fn.transmute.html
2019-12-31T19:36:00.6393427Z reference/print.html:5462: broken link - std/process/trait.Termination.html
2019-12-31T19:36:00.6393728Z reference/print.html:5531: broken link - std/cmp/trait.PartialEq.html
2019-12-31T19:36:00.6396080Z reference/print.html:5531: broken link - std/clone/trait.Clone.html
2019-12-31T19:36:00.6396794Z reference/print.html:5901: broken link - std/macro.is_x86_feature_detected.html
2019-12-31T19:36:00.6403546Z reference/print.html:6323: broken link - std/boxed/struct.Box.html
2019-12-31T19:36:00.6407583Z reference/print.html:6615: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6407860Z reference/print.html:6616: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.6416998Z reference/print.html:7286: broken link - std/ops/trait.Index.html
2019-12-31T19:36:00.6419576Z reference/print.html:7286: broken link - std/ops/trait.IndexMut.html
2019-12-31T19:36:00.6420348Z reference/print.html:7519: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6420588Z reference/print.html:7519: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6420806Z reference/print.html:7520: broken link - std/ops/trait.FnOnce.html
2019-12-31T19:36:00.6429452Z reference/print.html:8443: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.6429704Z reference/print.html:8448: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.6429919Z reference/print.html:8449: broken link - std/pin/struct.Pin.html
2019-12-31T19:36:00.6430154Z reference/print.html:8450: broken link - std/future/trait.Future.html
2019-12-31T19:36:00.6430372Z reference/print.html:8452: broken link - std/task/enum.Poll.html
2019-12-31T19:36:00.6430584Z reference/print.html:8456: broken link - std/task/enum.Poll.html
2019-12-31T19:36:00.6430811Z reference/print.html:8457: broken link - std/task/enum.Poll.html
2019-12-31T19:36:00.6431079Z reference/print.html:8465: broken link - std/task/struct.Context.html
2019-12-31T19:36:00.6442874Z reference/print.html:9421: broken link - std/vec/struct.Vec.html
2019-12-31T19:36:00.6443209Z reference/print.html:9543: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6443471Z reference/print.html:9543: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6443714Z reference/print.html:9543: broken link - std/ops/trait.FnOnce.html
2019-12-31T19:36:00.6448277Z reference/print.html:9651: broken link - std/ops/trait.FnOnce.html
2019-12-31T19:36:00.6459546Z reference/print.html:9657: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6459790Z reference/print.html:9661: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6460031Z reference/print.html:9665: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6460252Z reference/print.html:9665: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6463703Z reference/print.html:9989: broken link - std/mem/fn.align_of_val.html
2019-12-31T19:36:00.6464245Z reference/print.html:9993: broken link - std/mem/fn.size_of_val.html
2019-12-31T19:36:00.6464481Z reference/print.html:9995: broken link - std/marker/trait.Sized.html
2019-12-31T19:36:00.6464697Z reference/print.html:9995: broken link - std/mem/fn.size_of.html
2019-12-31T19:36:00.6464933Z reference/print.html:9996: broken link - std/mem/fn.align_of.html
2019-12-31T19:36:00.6465149Z reference/print.html:9996: broken link - std/marker/trait.Sized.html
2019-12-31T19:36:00.6465370Z reference/print.html:10262: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6465611Z reference/print.html:10269: broken link - std/cell/struct.RefCell.html
2019-12-31T19:36:00.6465826Z reference/print.html:10271: broken link - std/sync/atomic/index.html
2019-12-31T19:36:00.6466041Z reference/print.html:10652: broken link - std/marker/trait.Unsize.html
2019-12-31T19:36:00.6466281Z reference/print.html:10652: broken link - std/ops/trait.CoerceUnsized.html
2019-12-31T19:36:00.6466494Z reference/print.html:10691: broken link - std/ops/trait.Drop.html
2019-12-31T19:36:00.6466730Z reference/print.html:10708: broken link - std/ptr/fn.drop_in_place.html
2019-12-31T19:36:00.6466970Z reference/print.html:10761: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T19:36:00.6467176Z reference/print.html:10973: broken link - std/index.html
2019-12-31T19:36:00.6467390Z reference/print.html:10977: broken link - std/boxed/struct.Box.html
2019-12-31T19:36:00.6467619Z reference/print.html:10988: broken link - std/rc/struct.Rc.html
2019-12-31T19:36:00.6467834Z reference/print.html:10990: broken link - std/sync/struct.Arc.html
2019-12-31T19:36:00.6468046Z reference/print.html:10992: broken link - std/pin/struct.Pin.html
2019-12-31T19:36:00.6468285Z reference/print.html:10994: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6468508Z reference/print.html:10999: broken link - std/marker/struct.PhantomData.html
2019-12-31T19:36:00.6468718Z reference/print.html:11003: broken link - std/ops/index.html
2019-12-31T19:36:00.6468956Z reference/print.html:11003: broken link - std/cmp/index.html
2019-12-31T19:36:00.6469175Z reference/print.html:11006: broken link - std/ops/trait.Deref.html
2019-12-31T19:36:00.6469394Z reference/print.html:11006: broken link - std/ops/trait.DerefMut.html
2019-12-31T19:36:00.6469624Z reference/print.html:11009: broken link - std/ops/trait.Drop.html
2019-12-31T19:36:00.6469838Z reference/print.html:11012: broken link - std/marker/trait.Copy.html
2019-12-31T19:36:00.6470051Z reference/print.html:11028: broken link - std/clone/trait.Clone.html
2019-12-31T19:36:00.6470282Z reference/print.html:11037: broken link - std/marker/trait.Send.html
2019-12-31T19:36:00.6470500Z reference/print.html:11040: broken link - std/marker/trait.Sync.html
2019-12-31T19:36:00.6470714Z reference/print.html:11044: broken link - std/marker/trait.Send.html
2019-12-31T19:36:00.6470943Z reference/print.html:11044: broken link - std/marker/trait.Sync.html
2019-12-31T19:36:00.6471165Z reference/print.html:11044: broken link - std/panic/trait.UnwindSafe.html
2019-12-31T19:36:00.6471490Z reference/print.html:11044: broken link - std/panic/trait.RefUnwindSafe.html
2019-12-31T19:36:00.6471764Z reference/print.html:11073: broken link - std/marker/trait.Sized.html
2019-12-31T19:36:00.6471990Z reference/print.html:11390: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6472208Z reference/print.html:11393: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6472438Z reference/print.html:11440: broken link - core/ptr/struct.NonNull.html
2019-12-31T19:36:00.6472650Z reference/print.html:11440: broken link - core/num/index.html
2019-12-31T19:36:00.6473434Z reference/print.html:11638: broken link - core/panic/struct.PanicInfo.html
2019-12-31T19:36:00.6473684Z reference/print.html:11675: broken link - std/panic/fn.set_hook.html
2019-12-31T19:36:00.6473907Z reference/print.html:11678: broken link - alloc/alloc/trait.GlobalAlloc.html
2019-12-31T19:36:00.6490814Z reference/runtime.html:162: broken link - core/panic/struct.PanicInfo.html
2019-12-31T19:36:00.6491291Z reference/runtime.html:199: broken link - std/panic/fn.set_hook.html
2019-12-31T19:36:00.6491525Z reference/runtime.html:202: broken link - alloc/alloc/trait.GlobalAlloc.html
2019-12-31T19:36:00.6506315Z reference/index.html:175: broken link - std/index.html
2019-12-31T19:36:00.6537214Z reference/expressions.html:277: broken link - std/boxed/struct.Box.html
2019-12-31T19:36:00.6554589Z reference/types/array.html:180: broken link - std/vec/struct.Vec.html
2019-12-31T19:36:00.6605023Z reference/types/function-item.html:201: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6605306Z reference/types/function-item.html:201: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6605538Z reference/types/function-item.html:201: broken link - std/ops/trait.FnOnce.html
2019-12-31T19:36:00.6659243Z reference/types/closure.html:263: broken link - std/ops/trait.FnOnce.html
2019-12-31T19:36:00.6659516Z reference/types/closure.html:269: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6659761Z reference/types/closure.html:273: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6659999Z reference/types/closure.html:277: broken link - std/ops/trait.Fn.html
2019-12-31T19:36:00.6660224Z reference/types/closure.html:277: broken link - std/ops/trait.FnMut.html
2019-12-31T19:36:00.6690473Z reference/interior-mutability.html:163: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6690787Z reference/interior-mutability.html:170: broken link - std/cell/struct.RefCell.html
2019-12-31T19:36:00.6691021Z reference/interior-mutability.html:172: broken link - std/sync/atomic/index.html
2019-12-31T19:36:00.6713227Z reference/type-coercions.html:322: broken link - std/marker/trait.Unsize.html
2019-12-31T19:36:00.6713545Z reference/type-coercions.html:322: broken link - std/ops/trait.CoerceUnsized.html
2019-12-31T19:36:00.6720753Z reference/destructors.html:165: broken link - std/ops/trait.Drop.html
2019-12-31T19:36:00.6721027Z reference/destructors.html:182: broken link - std/ptr/fn.drop_in_place.html
2019-12-31T19:36:00.6721325Z reference/destructors.html:235: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T19:36:00.6744860Z reference/behavior-considered-undefined.html:179: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6745176Z reference/behavior-considered-undefined.html:182: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T19:36:00.6745430Z reference/behavior-considered-undefined.html:229: broken link - core/ptr/struct.NonNull.html
2019-12-31T19:36:00.6745664Z reference/behavior-considered-undefined.html:229: broken link - core/num/index.html
2019-12-31T19:36:00.6770152Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2019-12-31T19:36:00.6781163Z 
2019-12-31T19:36:00.6781229Z 
2019-12-31T19:36:00.6781921Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2019-12-31T19:36:00.6782009Z expected success, got: exit code: 101
---
2019-12-31T19:36:00.6865296Z   local time: Tue Dec 31 19:36:00 UTC 2019
2019-12-31T19:36:00.9549928Z   network time: Tue, 31 Dec 2019 19:36:00 GMT
2019-12-31T19:36:00.9555751Z == end clock drift check ==
2019-12-31T19:36:07.8309611Z 
2019-12-31T19:36:07.8430704Z ##[error]Bash exited with code '1'.
2019-12-31T19:36:07.8469183Z ##[section]Starting: Checkout
2019-12-31T19:36:07.8471098Z ==============================================================================
2019-12-31T19:36:07.8471321Z Task         : Get sources
2019-12-31T19:36:07.8471361Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
