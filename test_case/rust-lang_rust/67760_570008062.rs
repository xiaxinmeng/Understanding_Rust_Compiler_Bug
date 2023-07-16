plain
2019-12-31T22:28:32.7477349Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T22:28:32.7674031Z ##[command]git config gc.auto 0
2019-12-31T22:28:32.7750787Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T22:28:32.7815493Z ##[command]git config --get-all http.proxy
2019-12-31T22:28:32.7975704Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67760/merge:refs/remotes/pull/67760/merge
---
2019-12-31T22:35:04.6247010Z Found 486 error codes
2019-12-31T22:35:04.6247110Z Found 0 error codes with no tests
2019-12-31T22:35:04.6247957Z Done!
2019-12-31T22:35:04.6247995Z fmt check
2019-12-31T22:35:10.7941856Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std"; dirty!
2019-12-31T22:35:11.1643059Z    Compiling cc v1.0.47
2019-12-31T22:35:11.1643999Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-12-31T22:35:16.8779754Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-31T22:35:17.8703413Z    Compiling libc v0.2.66
---
2019-12-31T22:35:56.0896458Z    Compiling getopts v0.2.21
2019-12-31T22:36:02.1349659Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-12-31T22:36:10.3253074Z     Finished release [optimized] target(s) in 59.52s
2019-12-31T22:36:10.3401780Z Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T22:36:10.3416459Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc"; dirty!
2019-12-31T22:36:10.8367674Z    Compiling cfg-if v0.1.8
2019-12-31T22:36:10.8367897Z    Compiling libc v0.2.66
2019-12-31T22:36:10.8867484Z    Compiling semver-parser v0.7.0
2019-12-31T22:36:11.6984516Z    Compiling lazy_static v1.3.0
---
2019-12-31T22:56:31.0487731Z     Finished release [optimized] target(s) in 20m 20s
2019-12-31T22:56:31.1033247Z Installing libLLVM.so to stage 0 (x86_64-unknown-linux-gnu)
2019-12-31T22:56:31.1035415Z Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T22:56:31.1070177Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-12-31T22:56:31.1087240Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std"; dirty!
2019-12-31T22:56:31.4780108Z    Compiling cc v1.0.47
2019-12-31T22:56:31.4822718Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-12-31T22:56:37.0382546Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-31T22:56:38.0441528Z    Compiling libc v0.2.66
---
2019-12-31T22:57:20.1422169Z    Compiling getopts v0.2.21
2019-12-31T22:57:27.4982485Z    Compiling test v0.0.0 (/checkout/src/libtest)
2019-12-31T22:57:37.9692719Z     Finished release [optimized] target(s) in 1m 06s
2019-12-31T22:57:37.9841620Z Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T22:57:37.9853962Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc"; dirty!
2019-12-31T22:57:38.5190849Z    Compiling cfg-if v0.1.8
2019-12-31T22:57:38.5191383Z    Compiling libc v0.2.66
2019-12-31T22:57:38.5584501Z    Compiling semver-parser v0.7.0
2019-12-31T22:57:39.4981180Z    Compiling lazy_static v1.3.0
---
2019-12-31T23:24:04.7434495Z .................................................................................................... 1500/9464
2019-12-31T23:24:10.0140880Z .................................................................................................... 1600/9464
2019-12-31T23:24:14.5867796Z .................................................................................................... 1700/9464
2019-12-31T23:24:23.1960513Z .................................................................................................... 1800/9464
2019-12-31T23:24:30.5526137Z i................................................................................................... 1900/9464
2019-12-31T23:24:36.6082093Z ......................................................................................iiiii......... 2000/9464
2019-12-31T23:24:56.1462630Z .................................................................................................... 2200/9464
2019-12-31T23:24:58.2602873Z .................................................................................................... 2300/9464
2019-12-31T23:25:00.4296812Z .................................................................................................... 2400/9464
2019-12-31T23:25:05.9425562Z .................................................................................................... 2500/9464
---
2019-12-31T23:27:49.8603941Z .................i...............i.................................................................. 4900/9464
2019-12-31T23:27:58.6841619Z .................................................................................................... 5000/9464
2019-12-31T23:28:03.7306412Z ..............................................................i..................................... 5100/9464
2019-12-31T23:28:10.9917867Z .................................................................................................... 5200/9464
2019-12-31T23:28:18.2868990Z .............................ii.ii...........i...................................................... 5300/9464
2019-12-31T23:28:26.1147979Z .................................................................................................... 5500/9464
2019-12-31T23:28:35.1211092Z .................................................................................................... 5600/9464
2019-12-31T23:28:41.4151178Z ............i....................................................................................... 5700/9464
2019-12-31T23:28:46.8525679Z .................................................................................................... 5800/9464
2019-12-31T23:28:46.8525679Z .................................................................................................... 5800/9464
2019-12-31T23:28:56.4759446Z .................................................................................................... 5900/9464
2019-12-31T23:29:07.1308261Z ii...i..ii...........i.............................................................................. 6000/9464
2019-12-31T23:29:22.9013845Z .................................................................................................... 6200/9464
2019-12-31T23:29:29.2752773Z .................................................................................................... 6300/9464
2019-12-31T23:29:29.2752773Z .................................................................................................... 6300/9464
2019-12-31T23:29:45.7236251Z ...........................i..ii.................................................................... 6400/9464
2019-12-31T23:30:03.5946114Z .................................................................................................... 6600/9464
2019-12-31T23:30:05.6411706Z ..i................................................................................................. 6700/9464
2019-12-31T23:30:07.6940522Z .................................................................................................... 6800/9464
2019-12-31T23:30:09.9914799Z ..i................................................................................................. 6900/9464
---
2019-12-31T23:31:37.3713436Z .................................................................................................... 7500/9464
2019-12-31T23:31:41.7477942Z .................................................................................................... 7600/9464
2019-12-31T23:31:46.6526710Z .................................................................................................... 7700/9464
2019-12-31T23:31:55.6384185Z .................................................................................................... 7800/9464
2019-12-31T23:32:02.3457706Z .................................iiii............................................................... 7900/9464
2019-12-31T23:32:15.6415653Z .................................................................................................... 8100/9464
2019-12-31T23:32:23.6607600Z .................................................................................................... 8200/9464
2019-12-31T23:32:36.6891168Z .................................................................................................... 8300/9464
2019-12-31T23:32:43.5399278Z .................................................................................................... 8400/9464
---
2019-12-31T23:34:49.3810341Z  finished in 5.942
2019-12-31T23:34:49.3960964Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:34:49.5801567Z 
2019-12-31T23:34:49.5801794Z running 166 tests
2019-12-31T23:34:52.2896861Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-31T23:34:54.1905591Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-31T23:34:54.1910243Z 
2019-12-31T23:34:54.1915032Z  finished in 4.795
2019-12-31T23:34:54.2083467Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:34:54.3599707Z 
---
2019-12-31T23:34:56.8827034Z  finished in 1.894
2019-12-31T23:34:56.8827394Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:34:56.8827438Z 
2019-12-31T23:34:56.8827672Z running 9 tests
2019-12-31T23:34:56.8828269Z iiiiiiiii
2019-12-31T23:34:56.8828845Z 
2019-12-31T23:34:56.8875891Z  finished in 0.154
2019-12-31T23:34:56.8885590Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:34:56.8885662Z 
---
2019-12-31T23:35:14.3310428Z  finished in 18.033
2019-12-31T23:35:14.3506458Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:35:14.5137630Z 
2019-12-31T23:35:14.5138539Z running 124 tests
2019-12-31T23:35:36.3731543Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-31T23:35:40.0350530Z .i.iii.....iiiiii.....ii
2019-12-31T23:35:40.0351990Z 
2019-12-31T23:35:40.0352242Z  finished in 25.684
2019-12-31T23:35:40.0358411Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:35:40.0359017Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T23:35:40.0359017Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2019-12-31T23:35:40.0584205Z Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T23:35:40.2193621Z 
2019-12-31T23:35:40.2194170Z running 64 tests
2019-12-31T23:36:15.7390120Z ................................................................
2019-12-31T23:36:15.7390822Z test result: ok. 64 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-31T23:36:15.7390858Z 
2019-12-31T23:36:15.7394426Z  finished in 35.681
2019-12-31T23:36:15.7403990Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools"; dirty!
2019-12-31T23:36:15.9464018Z    Compiling cfg-if v0.1.8
2019-12-31T23:36:15.9476831Z    Compiling libc v0.2.66
2019-12-31T23:36:15.9809301Z    Compiling lazy_static v1.3.0
2019-12-31T23:36:16.0576914Z    Compiling semver-parser v0.7.0
---
2019-12-31T23:42:03.7273045Z ..........................................................
2019-12-31T23:42:03.7275489Z test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-31T23:42:03.7275546Z 
2019-12-31T23:42:03.7277793Z  finished in 2.369
2019-12-31T23:42:03.7297444Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools"; dirty!
2019-12-31T23:42:03.9377595Z    Compiling cfg-if v0.1.8
2019-12-31T23:42:03.9378647Z    Compiling libc v0.2.66
2019-12-31T23:42:03.9821408Z    Compiling lazy_static v1.3.0
2019-12-31T23:42:04.0603038Z    Compiling semver-parser v0.7.0
---
2019-12-31T23:47:24.9297148Z 
2019-12-31T23:47:24.9302569Z    Doc-tests core
2019-12-31T23:47:28.7862595Z 
2019-12-31T23:47:28.7864034Z running 2439 tests
2019-12-31T23:47:37.2332598Z ......iiiii......................................................................................... 100/2439
2019-12-31T23:47:45.4073352Z ..................................................................................ii................ 200/2439
2019-12-31T23:48:04.2720011Z ................i................................................................................... 400/2439
2019-12-31T23:48:04.2720011Z ................i................................................................................... 400/2439
2019-12-31T23:48:12.6897492Z ................................................................i..i..................iiii.......... 500/2439
2019-12-31T23:48:27.5760521Z .................................................................................................... 700/2439
2019-12-31T23:48:35.4031371Z .................................................................................................... 800/2439
2019-12-31T23:48:43.1890161Z .................................................................................................... 900/2439
2019-12-31T23:48:51.0662956Z .................................................................................................... 1000/2439
---
2019-12-31T23:52:00.8118057Z .................................................thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:871:13
2019-12-31T23:52:00.8130154Z ... 300/760
2019-12-31T23:52:00.9061019Z .................................................................................................... 400/760
2019-12-31T23:52:02.9707869Z .................................................................................................... 500/760
2019-12-31T23:52:03.0057108Z .................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T23:52:03.0076742Z ....thread 'thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:1189:5
2019-12-31T23:52:03.0082941Z .<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T23:52:03.0103647Z ......thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T23:52:04.1458273Z ..........................................thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T23:52:04.1459087Z ...........thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:1189:5
2019-12-31T23:52:04.1459579Z .thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs.:1189:5
2019-12-31T23:52:05.3424978Z ...................thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:633:13
2019-12-31T23:52:05.3425304Z thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:587:13
2019-12-31T23:52:05.3431082Z .....thread '<unnamed>' panicked at 'test panic in inner thread to poison mutex', src/libstd/sync/mutex.rs:563:13
2019-12-31T23:52:05.3436175Z thread '<unnamed>' panicked at 'explicit panic', src/libstd/sync/mutex.rs:694:13
---
2019-12-31T23:52:15.1486265Z 
2019-12-31T23:52:15.1487161Z running 1002 tests
2019-12-31T23:52:30.6704071Z i................................................................................................... 100/1002
2019-12-31T23:52:39.6164712Z .................................................................................................... 200/1002
2019-12-31T23:52:46.0179387Z .................iii......i......i...i......i....................................................... 300/1002
2019-12-31T23:52:50.2807368Z .................................................................................................... 400/1002
2019-12-31T23:52:56.5133576Z .........................................i..i.....................................ii................ 500/1002
2019-12-31T23:53:07.7974624Z .................................................................................................... 700/1002
2019-12-31T23:53:07.7974624Z .................................................................................................... 700/1002
2019-12-31T23:53:13.7690601Z ............................iiii.................................................................... 800/1002
2019-12-31T23:53:26.3233801Z .................................................................................................... 900/1002
2019-12-31T23:53:32.4588052Z ..................................................iiii.............................................. 1000/1002
2019-12-31T23:53:32.5082565Z test result: ok. 982 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-12-31T23:53:32.5083351Z 
2019-12-31T23:53:32.5185928Z  finished in 160.865
2019-12-31T23:53:32.5200422Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-12-31T23:57:18.0263749Z 
2019-12-31T23:57:18.0263785Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-31T23:57:18.0263808Z 
2019-12-31T23:57:18.0292873Z  finished in 0.581
2019-12-31T23:57:18.0298819Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools"; dirty!
2019-12-31T23:57:18.2408978Z    Compiling cfg-if v0.1.8
2019-12-31T23:57:18.2412896Z    Compiling libc v0.2.66
2019-12-31T23:57:18.2766908Z    Compiling lazy_static v1.3.0
2019-12-31T23:57:18.3496068Z    Compiling semver-parser v0.7.0
---
2020-01-01T00:08:47.8203624Z Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
2020-01-01T00:08:48.1085100Z Documenting standalone (x86_64-unknown-linux-gnu)
2020-01-01T00:08:48.5160528Z Documenting book redirect pages (x86_64-unknown-linux-gnu)
2020-01-01T00:08:49.6580038Z Documenting stage2 std (x86_64-unknown-linux-gnu)
2020-01-01T00:08:49.6591009Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/crate-docs"; dirty!
2020-01-01T00:08:49.6597781Z Clearing out "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std"; dirty!
2020-01-01T00:08:50.0495856Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-01T00:08:56.1632010Z    Compiling compiler_builtins v0.1.22
2020-01-01T00:09:07.7119918Z     Checking rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-01-01T00:09:08.7229187Z  Documenting alloc v0.0.0 (/checkout/src/liballoc)
---
2020-01-01T00:10:08.8209395Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-01-01T00:10:09.2079142Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-01-01T00:10:09.3765387Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-01-01T00:10:11.0176318Z     Finished release [optimized] target(s) in 1.80s
2020-01-01T00:10:11.0712600Z cargo/print.html:1517: broken link - std/primitive.char.html
2020-01-01T00:10:11.0741041Z cargo/print.html:2404: broken link - std/macro.debug_assert.html
2020-01-01T00:10:11.0772319Z cargo/print.html:3700: broken link - std/macro.env.html
2020-01-01T00:10:11.0779502Z cargo/print.html:3963: broken link - std/macro.include.html
2020-01-01T00:10:11.0779884Z cargo/print.html:3964: broken link - std/macro.concat.html
2020-01-01T00:10:11.0780611Z cargo/print.html:3964: broken link - std/macro.env.html
2020-01-01T00:10:11.0780868Z cargo/print.html:4254: broken link - std/macro.cfg.html
2020-01-01T00:10:11.0789515Z cargo/print.html:4942: broken link - std/primitive.char.html
2020-01-01T00:10:11.1011014Z cargo/reference/build-scripts.html:297: broken link - std/macro.env.html
2020-01-01T00:10:11.1044948Z cargo/reference/registries.html:292: broken link - std/primitive.char.html
2020-01-01T00:10:11.1056357Z cargo/reference/manifest.html:229: broken link - std/primitive.char.html
2020-01-01T00:10:11.1088704Z cargo/reference/profiles.html:210: broken link - std/macro.debug_assert.html
2020-01-01T00:10:11.1136153Z cargo/reference/build-script-examples.html:260: broken link - std/macro.include.html
2020-01-01T00:10:11.1136500Z cargo/reference/build-script-examples.html:261: broken link - std/macro.concat.html
2020-01-01T00:10:11.1136806Z cargo/reference/build-script-examples.html:261: broken link - std/macro.env.html
2020-01-01T00:10:11.1137090Z cargo/reference/build-script-examples.html:551: broken link - std/macro.cfg.html
2020-01-01T00:10:11.4283439Z nomicon/dropck.html:421: broken link - std/mem/struct.ManuallyDrop.html
2020-01-01T00:10:11.4309332Z nomicon/other-reprs.html:215: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:11.4339195Z nomicon/panic-handler.html:154: broken link - core/panic/struct.PanicInfo.html
2020-01-01T00:10:11.4360547Z nomicon/unchecked-uninit.html:159: broken link - core/mem/union.MaybeUninit.html
2020-01-01T00:10:11.4360906Z nomicon/unchecked-uninit.html:203: broken link - core/mem/union.MaybeUninit.html
2020-01-01T00:10:11.4361190Z nomicon/unchecked-uninit.html:239: broken link - core/ptr/index.html
2020-01-01T00:10:11.4361494Z nomicon/unchecked-uninit.html:241: broken link - core/ptr/fn.write.html
2020-01-01T00:10:11.4361791Z nomicon/unchecked-uninit.html:241: broken link - std/ptr/fn.copy.html
2020-01-01T00:10:11.4362075Z nomicon/unchecked-uninit.html:241: broken link - std/ptr/fn.copy_nonoverlapping.html
2020-01-01T00:10:11.4374503Z nomicon/safe-unsafe-meaning.html:183: broken link - std/primitive.pointer.html
2020-01-01T00:10:11.4375023Z nomicon/safe-unsafe-meaning.html:191: broken link - std/marker/trait.Send.html
2020-01-01T00:10:11.4375302Z nomicon/safe-unsafe-meaning.html:193: broken link - std/marker/trait.Sync.html
2020-01-01T00:10:11.4379527Z nomicon/safe-unsafe-meaning.html:195: broken link - std/alloc/trait.GlobalAlloc.html
2020-01-01T00:10:11.4380480Z nomicon/transmutes.html:157: broken link - std/mem/fn.transmute.html
2020-01-01T00:10:11.4380767Z nomicon/transmutes.html:178: broken link - std/mem/fn.transmute_copy.html
2020-01-01T00:10:11.4409233Z nomicon/print.html:283: broken link - std/primitive.pointer.html
2020-01-01T00:10:11.4410085Z nomicon/print.html:291: broken link - std/marker/trait.Send.html
2020-01-01T00:10:11.4410421Z nomicon/print.html:293: broken link - std/marker/trait.Sync.html
2020-01-01T00:10:11.4410679Z nomicon/print.html:295: broken link - std/alloc/trait.GlobalAlloc.html
2020-01-01T00:10:11.4410959Z nomicon/print.html:949: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:11.4418569Z nomicon/print.html:2331: broken link - std/mem/struct.ManuallyDrop.html
2020-01-01T00:10:11.4418838Z nomicon/print.html:2903: broken link - std/mem/fn.transmute.html
2020-01-01T00:10:11.4419205Z nomicon/print.html:2924: broken link - std/mem/fn.transmute_copy.html
2020-01-01T00:10:11.4419455Z nomicon/print.html:3127: broken link - core/mem/union.MaybeUninit.html
2020-01-01T00:10:11.4420497Z nomicon/print.html:3171: broken link - core/mem/union.MaybeUninit.html
2020-01-01T00:10:11.4420819Z nomicon/print.html:3207: broken link - core/ptr/index.html
2020-01-01T00:10:11.4421270Z nomicon/print.html:3209: broken link - core/ptr/fn.write.html
2020-01-01T00:10:11.4421656Z nomicon/print.html:3209: broken link - std/ptr/fn.copy.html
2020-01-01T00:10:11.4421940Z nomicon/print.html:3209: broken link - std/ptr/fn.copy_nonoverlapping.html
2020-01-01T00:10:11.4429811Z nomicon/print.html:5850: broken link - std/ops/trait.Drop.html
2020-01-01T00:10:11.4430219Z nomicon/print.html:6214: broken link - std/panic/fn.catch_unwind.html
2020-01-01T00:10:11.4430508Z nomicon/print.html:6230: broken link - std/panic/fn.catch_unwind.html
2020-01-01T00:10:11.4430765Z nomicon/print.html:6231: broken link - std/panic/fn.catch_unwind.html
2020-01-01T00:10:11.4431021Z nomicon/print.html:6294: broken link - core/panic/struct.PanicInfo.html
2020-01-01T00:10:11.4530656Z nomicon/ffi.html:355: broken link - std/ops/trait.Drop.html
2020-01-01T00:10:11.4530966Z nomicon/ffi.html:719: broken link - std/panic/fn.catch_unwind.html
2020-01-01T00:10:11.4531222Z nomicon/ffi.html:735: broken link - std/panic/fn.catch_unwind.html
2020-01-01T00:10:11.4531530Z nomicon/ffi.html:736: broken link - std/panic/fn.catch_unwind.html
2020-01-01T00:10:12.3957848Z book/ch09-02-recoverable-errors-with-result.html:191: broken link - std/index.html
2020-01-01T00:10:12.4101438Z book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html:468: broken link - std/prelude/index.html
2020-01-01T00:10:12.4341875Z book/procedural-macros.html:43: broken link - proc_macro/index.html
2020-01-01T00:10:12.4551635Z book/ch06-01-defining-an-enum.html:304: broken link - std/net/enum.IpAddr.html
2020-01-01T00:10:12.4552142Z book/ch06-01-defining-an-enum.html:441: broken link - std/option/enum.Option.html
2020-01-01T00:10:12.4552444Z book/ch06-01-defining-an-enum.html:519: broken link - std/option/enum.Option.html
2020-01-01T00:10:12.4935925Z book/appendix-03-derivable-traits.html:170: broken link - std/index.html
2020-01-01T00:10:12.4936242Z book/casting-between-types.html:51: broken link - std/mem/fn.transmute.html
2020-01-01T00:10:12.5149311Z book/print.html:884: broken link - std/prelude/index.html
2020-01-01T00:10:12.5149628Z book/print.html:929: broken link - std/string/struct.String.html
2020-01-01T00:10:12.5150331Z book/print.html:948: broken link - std/io/struct.Stdin.html
2020-01-01T00:10:12.5150619Z book/print.html:951: broken link - std/io/struct.Stdin.html
2020-01-01T00:10:12.5150877Z book/print.html:981: broken link - std/io/type.Result.html
2020-01-01T00:10:12.5151133Z book/print.html:982: broken link - std/result/enum.Result.html
2020-01-01T00:10:12.5151412Z book/print.html:994: broken link - std/result/enum.Result.html
2020-01-01T00:10:12.5151667Z book/print.html:1372: broken link - std/primitive.str.html
2020-01-01T00:10:12.5151926Z book/print.html:1907: broken link - std/num/struct.Wrapping.html
2020-01-01T00:10:12.5158959Z book/print.html:4846: broken link - std/net/enum.IpAddr.html
2020-01-01T00:10:12.5159219Z book/print.html:4983: broken link - std/option/enum.Option.html
2020-01-01T00:10:12.5162909Z book/print.html:5061: broken link - std/option/enum.Option.html
2020-01-01T00:10:12.5163837Z book/print.html:6246: broken link - std/prelude/index.html
2020-01-01T00:10:12.5164442Z book/print.html:6334: broken link - std/collections/index.html
2020-01-01T00:10:12.5168910Z book/print.html:7486: broken link - std/index.html
2020-01-01T00:10:12.5229561Z book/print.html:24040: broken link - std/index.html
2020-01-01T00:10:12.5262989Z book/ch08-00-common-collections.html:173: broken link - std/collections/index.html
2020-01-01T00:10:12.5304130Z book/borrow-and-asref.html:45: broken link - std/convert/trait.AsRef.html
2020-01-01T00:10:12.5304484Z book/borrow-and-asref.html:47: broken link - std/convert/trait.AsRef.html
2020-01-01T00:10:12.6701976Z book/ch02-00-guessing-game-tutorial.html:236: broken link - std/prelude/index.html
2020-01-01T00:10:12.6702440Z book/ch02-00-guessing-game-tutorial.html:281: broken link - std/string/struct.String.html
2020-01-01T00:10:12.6702751Z book/ch02-00-guessing-game-tutorial.html:300: broken link - std/io/struct.Stdin.html
2020-01-01T00:10:12.6703243Z book/ch02-00-guessing-game-tutorial.html:303: broken link - std/io/struct.Stdin.html
2020-01-01T00:10:12.6704012Z book/ch02-00-guessing-game-tutorial.html:333: broken link - std/io/type.Result.html
2020-01-01T00:10:12.6704292Z book/ch02-00-guessing-game-tutorial.html:334: broken link - std/result/enum.Result.html
2020-01-01T00:10:12.6704561Z book/ch02-00-guessing-game-tutorial.html:346: broken link - std/result/enum.Result.html
2020-01-01T00:10:12.6704845Z book/ch02-00-guessing-game-tutorial.html:724: broken link - std/primitive.str.html
2020-01-01T00:10:12.6726802Z book/ch03-02-data-types.html:256: broken link - std/num/struct.Wrapping.html
2020-01-01T00:10:12.8703756Z index.html:83: broken link - std/index.html
2020-01-01T00:10:12.8740182Z error-index.html:7: broken link - light1.42.0.css
2020-01-01T00:10:12.8848864Z reference/expressions/call-expr.html:169: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.8849225Z reference/expressions/call-expr.html:169: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:12.8850091Z reference/expressions/call-expr.html:170: broken link - std/ops/trait.FnOnce.html
2020-01-01T00:10:12.8908526Z reference/expressions/array-expr.html:233: broken link - std/ops/trait.Index.html
2020-01-01T00:10:12.8908891Z reference/expressions/array-expr.html:233: broken link - std/ops/trait.IndexMut.html
2020-01-01T00:10:12.8938236Z reference/expressions/await-expr.html:164: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.8938565Z reference/expressions/await-expr.html:169: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.8938870Z reference/expressions/await-expr.html:170: broken link - std/pin/struct.Pin.html
2020-01-01T00:10:12.8939159Z reference/expressions/await-expr.html:171: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.8939425Z reference/expressions/await-expr.html:173: broken link - std/task/enum.Poll.html
2020-01-01T00:10:12.8939792Z reference/expressions/await-expr.html:177: broken link - std/task/enum.Poll.html
2020-01-01T00:10:12.8940081Z reference/expressions/await-expr.html:178: broken link - std/task/enum.Poll.html
2020-01-01T00:10:12.8940393Z reference/expressions/await-expr.html:186: broken link - std/task/struct.Context.html
2020-01-01T00:10:12.9068777Z reference/expressions/block-expr.html:251: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.9069107Z reference/expressions/block-expr.html:252: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.9132208Z reference/special-types-and-traits.html:158: broken link - std/index.html
2020-01-01T00:10:12.9132568Z reference/special-types-and-traits.html:162: broken link - std/boxed/struct.Box.html
2020-01-01T00:10:12.9132845Z reference/special-types-and-traits.html:173: broken link - std/rc/struct.Rc.html
2020-01-01T00:10:12.9133278Z reference/special-types-and-traits.html:175: broken link - std/sync/struct.Arc.html
2020-01-01T00:10:12.9133571Z reference/special-types-and-traits.html:177: broken link - std/pin/struct.Pin.html
2020-01-01T00:10:12.9133864Z reference/special-types-and-traits.html:179: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:12.9134171Z reference/special-types-and-traits.html:184: broken link - std/marker/struct.PhantomData.html
2020-01-01T00:10:12.9134433Z reference/special-types-and-traits.html:188: broken link - std/ops/index.html
2020-01-01T00:10:12.9134686Z reference/special-types-and-traits.html:188: broken link - std/cmp/index.html
2020-01-01T00:10:12.9134963Z reference/special-types-and-traits.html:191: broken link - std/ops/trait.Deref.html
2020-01-01T00:10:12.9135234Z reference/special-types-and-traits.html:191: broken link - std/ops/trait.DerefMut.html
2020-01-01T00:10:12.9135492Z reference/special-types-and-traits.html:194: broken link - std/ops/trait.Drop.html
2020-01-01T00:10:12.9135770Z reference/special-types-and-traits.html:197: broken link - std/marker/trait.Copy.html
2020-01-01T00:10:12.9136033Z reference/special-types-and-traits.html:213: broken link - std/clone/trait.Clone.html
2020-01-01T00:10:12.9136478Z reference/special-types-and-traits.html:222: broken link - std/marker/trait.Send.html
2020-01-01T00:10:12.9136908Z reference/special-types-and-traits.html:225: broken link - std/marker/trait.Sync.html
2020-01-01T00:10:12.9137176Z reference/special-types-and-traits.html:229: broken link - std/marker/trait.Send.html
2020-01-01T00:10:12.9137435Z reference/special-types-and-traits.html:229: broken link - std/marker/trait.Sync.html
2020-01-01T00:10:12.9137720Z reference/special-types-and-traits.html:229: broken link - std/panic/trait.UnwindSafe.html
2020-01-01T00:10:12.9137995Z reference/special-types-and-traits.html:229: broken link - std/panic/trait.RefUnwindSafe.html
2020-01-01T00:10:12.9138258Z reference/special-types-and-traits.html:258: broken link - std/marker/trait.Sized.html
2020-01-01T00:10:12.9157879Z reference/attributes/codegen.html:246: broken link - std/macro.is_x86_feature_detected.html
2020-01-01T00:10:12.9158216Z reference/attributes/derive.html:162: broken link - std/cmp/trait.PartialEq.html
2020-01-01T00:10:12.9158492Z reference/attributes/derive.html:162: broken link - std/clone/trait.Clone.html
2020-01-01T00:10:12.9186615Z reference/attributes/testing.html:175: broken link - std/process/trait.Termination.html
2020-01-01T00:10:12.9245102Z reference/crates-and-source-files.html:239: broken link - std/index.html
2020-01-01T00:10:12.9245426Z reference/crates-and-source-files.html:239: broken link - std/prelude/index.html
2020-01-01T00:10:12.9245681Z reference/crates-and-source-files.html:241: broken link - core/index.html
2020-01-01T00:10:12.9245928Z reference/crates-and-source-files.html:242: broken link - core/prelude/index.html
2020-01-01T00:10:12.9246209Z reference/crates-and-source-files.html:266: broken link - std/process/trait.Termination.html
2020-01-01T00:10:12.9246650Z reference/crates-and-source-files.html:283: broken link - std/primitive.char.html
2020-01-01T00:10:12.9308175Z reference/type-layout.html:170: broken link - std/mem/fn.align_of_val.html
2020-01-01T00:10:12.9308542Z reference/type-layout.html:174: broken link - std/mem/fn.size_of_val.html
2020-01-01T00:10:12.9310439Z reference/type-layout.html:176: broken link - std/marker/trait.Sized.html
2020-01-01T00:10:12.9310725Z reference/type-layout.html:176: broken link - std/mem/fn.size_of.html
2020-01-01T00:10:12.9312032Z reference/type-layout.html:177: broken link - std/mem/fn.align_of.html
2020-01-01T00:10:12.9312382Z reference/type-layout.html:177: broken link - std/marker/trait.Sized.html
2020-01-01T00:10:12.9357267Z reference/items/unions.html:211: broken link - std/mem/fn.transmute.html
2020-01-01T00:10:12.9503837Z reference/items/enumerations.html:213: broken link - std/mem/fn.discriminant.html
2020-01-01T00:10:12.9518332Z reference/procedural-macros.html:187: broken link - std/macro.compile_error.html
2020-01-01T00:10:12.9518627Z reference/procedural-macros.html:190: broken link - proc_macro/index.html
2020-01-01T00:10:12.9518928Z reference/procedural-macros.html:192: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9519210Z reference/procedural-macros.html:217: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9519508Z reference/procedural-macros.html:218: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9520220Z reference/procedural-macros.html:254: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9520500Z reference/procedural-macros.html:255: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9520790Z reference/procedural-macros.html:257: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9521066Z reference/procedural-macros.html:314: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9521337Z reference/procedural-macros.html:317: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9521626Z reference/procedural-macros.html:317: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9522058Z reference/procedural-macros.html:318: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9522387Z reference/procedural-macros.html:332: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9641105Z reference/conditional-compilation.html:317: broken link - std/macro.debug_assert.html
2020-01-01T00:10:12.9661761Z reference/introduction.html:175: broken link - std/index.html
2020-01-01T00:10:12.9752918Z reference/print.html:177: broken link - std/index.html
2020-01-01T00:10:12.9781954Z reference/print.html:1985: broken link - std/macro.compile_error.html
2020-01-01T00:10:12.9782259Z reference/print.html:1988: broken link - proc_macro/index.html
2020-01-01T00:10:12.9782529Z reference/print.html:1990: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9782830Z reference/print.html:2015: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9783096Z reference/print.html:2016: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9783528Z reference/print.html:2052: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9783806Z reference/print.html:2053: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9784070Z reference/print.html:2055: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9784320Z reference/print.html:2112: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9784593Z reference/print.html:2115: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9784843Z reference/print.html:2115: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9785090Z reference/print.html:2116: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9785363Z reference/print.html:2130: broken link - proc_macro/struct.TokenStream.html
2020-01-01T00:10:12.9785597Z reference/print.html:2257: broken link - std/index.html
2020-01-01T00:10:12.9785834Z reference/print.html:2257: broken link - std/prelude/index.html
2020-01-01T00:10:12.9786088Z reference/print.html:2259: broken link - core/index.html
2020-01-01T00:10:12.9786336Z reference/print.html:2260: broken link - core/prelude/index.html
2020-01-01T00:10:12.9786597Z reference/print.html:2284: broken link - std/process/trait.Termination.html
2020-01-01T00:10:12.9786860Z reference/print.html:2301: broken link - std/primitive.char.html
2020-01-01T00:10:12.9787105Z reference/print.html:2473: broken link - std/macro.debug_assert.html
2020-01-01T00:10:12.9796522Z reference/print.html:3555: broken link - std/mem/fn.discriminant.html
2020-01-01T00:10:12.9796788Z reference/print.html:3721: broken link - std/mem/fn.transmute.html
2020-01-01T00:10:12.9821208Z reference/print.html:5462: broken link - std/process/trait.Termination.html
2020-01-01T00:10:12.9821543Z reference/print.html:5531: broken link - std/cmp/trait.PartialEq.html
2020-01-01T00:10:12.9821808Z reference/print.html:5531: broken link - std/clone/trait.Clone.html
2020-01-01T00:10:12.9825297Z reference/print.html:5901: broken link - std/macro.is_x86_feature_detected.html
2020-01-01T00:10:12.9834075Z reference/print.html:6323: broken link - std/boxed/struct.Box.html
2020-01-01T00:10:12.9838438Z reference/print.html:6615: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.9838709Z reference/print.html:6616: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.9849528Z reference/print.html:7286: broken link - std/ops/trait.Index.html
2020-01-01T00:10:12.9850358Z reference/print.html:7286: broken link - std/ops/trait.IndexMut.html
2020-01-01T00:10:12.9854154Z reference/print.html:7519: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.9854440Z reference/print.html:7519: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:12.9854671Z reference/print.html:7520: broken link - std/ops/trait.FnOnce.html
2020-01-01T00:10:12.9867509Z reference/print.html:8443: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.9867829Z reference/print.html:8448: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.9868063Z reference/print.html:8449: broken link - std/pin/struct.Pin.html
2020-01-01T00:10:12.9868453Z reference/print.html:8450: broken link - std/future/trait.Future.html
2020-01-01T00:10:12.9868840Z reference/print.html:8452: broken link - std/task/enum.Poll.html
2020-01-01T00:10:12.9869074Z reference/print.html:8456: broken link - std/task/enum.Poll.html
2020-01-01T00:10:12.9869304Z reference/print.html:8457: broken link - std/task/enum.Poll.html
2020-01-01T00:10:12.9869570Z reference/print.html:8465: broken link - std/task/struct.Context.html
2020-01-01T00:10:12.9885894Z reference/print.html:9421: broken link - std/vec/struct.Vec.html
2020-01-01T00:10:12.9886191Z reference/print.html:9543: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.9886436Z reference/print.html:9543: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:12.9886672Z reference/print.html:9543: broken link - std/ops/trait.FnOnce.html
2020-01-01T00:10:12.9893632Z reference/print.html:9651: broken link - std/ops/trait.FnOnce.html
2020-01-01T00:10:12.9901249Z reference/print.html:9657: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:12.9901555Z reference/print.html:9661: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.9901821Z reference/print.html:9665: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:12.9902117Z reference/print.html:9665: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:12.9902882Z reference/print.html:9989: broken link - std/mem/fn.align_of_val.html
2020-01-01T00:10:12.9903248Z reference/print.html:9993: broken link - std/mem/fn.size_of_val.html
2020-01-01T00:10:12.9903671Z reference/print.html:9995: broken link - std/marker/trait.Sized.html
2020-01-01T00:10:12.9904286Z reference/print.html:9995: broken link - std/mem/fn.size_of.html
2020-01-01T00:10:12.9904743Z reference/print.html:9996: broken link - std/mem/fn.align_of.html
2020-01-01T00:10:12.9905404Z reference/print.html:9996: broken link - std/marker/trait.Sized.html
2020-01-01T00:10:12.9905848Z reference/print.html:10262: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:12.9906330Z reference/print.html:10269: broken link - std/cell/struct.RefCell.html
2020-01-01T00:10:12.9906782Z reference/print.html:10271: broken link - std/sync/atomic/index.html
2020-01-01T00:10:12.9907243Z reference/print.html:10652: broken link - std/marker/trait.Unsize.html
2020-01-01T00:10:12.9907844Z reference/print.html:10652: broken link - std/ops/trait.CoerceUnsized.html
2020-01-01T00:10:12.9908265Z reference/print.html:10691: broken link - std/ops/trait.Drop.html
2020-01-01T00:10:12.9908715Z reference/print.html:10708: broken link - std/ptr/fn.drop_in_place.html
2020-01-01T00:10:12.9909148Z reference/print.html:10761: broken link - std/mem/struct.ManuallyDrop.html
2020-01-01T00:10:12.9910324Z reference/print.html:10973: broken link - std/index.html
2020-01-01T00:10:12.9910851Z reference/print.html:10977: broken link - std/boxed/struct.Box.html
2020-01-01T00:10:12.9911547Z reference/print.html:10988: broken link - std/rc/struct.Rc.html
2020-01-01T00:10:12.9912977Z reference/print.html:10990: broken link - std/sync/struct.Arc.html
2020-01-01T00:10:12.9916344Z reference/print.html:10992: broken link - std/pin/struct.Pin.html
2020-01-01T00:10:12.9916839Z reference/print.html:10994: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:12.9917278Z reference/print.html:10999: broken link - std/marker/struct.PhantomData.html
2020-01-01T00:10:12.9917591Z reference/print.html:11003: broken link - std/ops/index.html
2020-01-01T00:10:12.9917850Z reference/print.html:11003: broken link - std/cmp/index.html
2020-01-01T00:10:12.9918084Z reference/print.html:11006: broken link - std/ops/trait.Deref.html
2020-01-01T00:10:12.9918322Z reference/print.html:11006: broken link - std/ops/trait.DerefMut.html
2020-01-01T00:10:12.9918573Z reference/print.html:11009: broken link - std/ops/trait.Drop.html
2020-01-01T00:10:12.9918812Z reference/print.html:11012: broken link - std/marker/trait.Copy.html
2020-01-01T00:10:12.9919046Z reference/print.html:11028: broken link - std/clone/trait.Clone.html
2020-01-01T00:10:12.9919485Z reference/print.html:11037: broken link - std/marker/trait.Send.html
2020-01-01T00:10:12.9920669Z reference/print.html:11040: broken link - std/marker/trait.Sync.html
2020-01-01T00:10:12.9921127Z reference/print.html:11044: broken link - std/marker/trait.Send.html
2020-01-01T00:10:12.9921607Z reference/print.html:11044: broken link - std/marker/trait.Sync.html
2020-01-01T00:10:12.9929777Z reference/print.html:11044: broken link - std/panic/trait.UnwindSafe.html
2020-01-01T00:10:12.9930374Z reference/print.html:11044: broken link - std/panic/trait.RefUnwindSafe.html
2020-01-01T00:10:12.9930652Z reference/print.html:11073: broken link - std/marker/trait.Sized.html
2020-01-01T00:10:12.9930913Z reference/print.html:11390: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:12.9931191Z reference/print.html:11393: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:12.9931451Z reference/print.html:11440: broken link - core/ptr/struct.NonNull.html
2020-01-01T00:10:12.9931715Z reference/print.html:11440: broken link - core/num/index.html
2020-01-01T00:10:12.9936463Z reference/print.html:11638: broken link - core/panic/struct.PanicInfo.html
2020-01-01T00:10:12.9936918Z reference/print.html:11675: broken link - std/panic/fn.set_hook.html
2020-01-01T00:10:12.9937165Z reference/print.html:11678: broken link - alloc/alloc/trait.GlobalAlloc.html
2020-01-01T00:10:12.9970462Z reference/runtime.html:162: broken link - core/panic/struct.PanicInfo.html
2020-01-01T00:10:12.9970775Z reference/runtime.html:199: broken link - std/panic/fn.set_hook.html
2020-01-01T00:10:12.9971041Z reference/runtime.html:202: broken link - alloc/alloc/trait.GlobalAlloc.html
2020-01-01T00:10:13.0041184Z reference/index.html:175: broken link - std/index.html
2020-01-01T00:10:13.0041539Z reference/expressions.html:277: broken link - std/boxed/struct.Box.html
2020-01-01T00:10:13.0051715Z reference/types/array.html:180: broken link - std/vec/struct.Vec.html
2020-01-01T00:10:13.0115495Z reference/types/function-item.html:201: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:13.0115815Z reference/types/function-item.html:201: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:13.0116081Z reference/types/function-item.html:201: broken link - std/ops/trait.FnOnce.html
2020-01-01T00:10:13.0182964Z reference/types/closure.html:263: broken link - std/ops/trait.FnOnce.html
2020-01-01T00:10:13.0183304Z reference/types/closure.html:269: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:13.0183895Z reference/types/closure.html:273: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:13.0184169Z reference/types/closure.html:277: broken link - std/ops/trait.Fn.html
2020-01-01T00:10:13.0184411Z reference/types/closure.html:277: broken link - std/ops/trait.FnMut.html
2020-01-01T00:10:13.0223965Z reference/interior-mutability.html:163: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:13.0224313Z reference/interior-mutability.html:170: broken link - std/cell/struct.RefCell.html
2020-01-01T00:10:13.0224602Z reference/interior-mutability.html:172: broken link - std/sync/atomic/index.html
2020-01-01T00:10:13.0252940Z reference/type-coercions.html:322: broken link - std/marker/trait.Unsize.html
2020-01-01T00:10:13.0253265Z reference/type-coercions.html:322: broken link - std/ops/trait.CoerceUnsized.html
2020-01-01T00:10:13.0262985Z reference/destructors.html:165: broken link - std/ops/trait.Drop.html
2020-01-01T00:10:13.0263321Z reference/destructors.html:182: broken link - std/ptr/fn.drop_in_place.html
2020-01-01T00:10:13.0263600Z reference/destructors.html:235: broken link - std/mem/struct.ManuallyDrop.html
2020-01-01T00:10:13.0299050Z reference/behavior-considered-undefined.html:179: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:13.0299425Z reference/behavior-considered-undefined.html:182: broken link - std/cell/struct.UnsafeCell.html
2020-01-01T00:10:13.0299973Z reference/behavior-considered-undefined.html:229: broken link - core/ptr/struct.NonNull.html
2020-01-01T00:10:13.0300447Z reference/behavior-considered-undefined.html:229: broken link - core/num/index.html
2020-01-01T00:10:13.0323075Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-01-01T00:10:13.0331570Z 
2020-01-01T00:10:13.0331683Z 
2020-01-01T00:10:13.0332355Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-01-01T00:10:13.0332435Z expected success, got: exit code: 101
2020-01-01T00:10:13.0332435Z expected success, got: exit code: 101
2020-01-01T00:10:13.0332495Z 
2020-01-01T00:10:13.0332522Z 
2020-01-01T00:10:13.0340634Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-01T00:10:13.0340703Z Build completed unsuccessfully in 1:35:18
2020-01-01T00:10:13.0395922Z == clock drift check ==
2020-01-01T00:10:13.2427875Z   local time: Wed Jan  1 00:10:13 UTC 2020
2020-01-01T00:10:13.5346419Z   network time: Wed, 01 Jan 2020 00:10:13 GMT
2020-01-01T00:10:17.7165985Z 
2020-01-01T00:10:17.7165985Z 
2020-01-01T00:10:17.7273760Z ##[error]Bash exited with code '1'.
2020-01-01T00:10:17.7317024Z ##[section]Starting: Checkout
2020-01-01T00:10:17.7319090Z ==============================================================================
2020-01-01T00:10:17.7319148Z Task         : Get sources
2020-01-01T00:10:17.7319215Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
