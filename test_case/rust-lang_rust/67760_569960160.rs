plain
2019-12-31T15:17:57.9449794Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T15:17:57.9679010Z ##[command]git config gc.auto 0
2019-12-31T15:17:57.9746636Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T15:17:58.5138186Z ##[command]git config --get-all http.proxy
2019-12-31T15:17:58.5143865Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67760/merge:refs/remotes/pull/67760/merge
---
2019-12-31T16:17:28.4571581Z .................................................................................................... 1500/9464
2019-12-31T16:17:34.3322706Z .................................................................................................... 1600/9464
2019-12-31T16:17:39.2833066Z .................................................................................................... 1700/9464
2019-12-31T16:17:48.7137604Z .................................................................................................... 1800/9464
2019-12-31T16:17:56.8193008Z i................................................................................................... 1900/9464
2019-12-31T16:18:03.5092111Z ......................................................................................iiiii......... 2000/9464
2019-12-31T16:18:25.0793472Z .................................................................................................... 2200/9464
2019-12-31T16:18:27.5158610Z .................................................................................................... 2300/9464
2019-12-31T16:18:30.0106018Z .................................................................................................... 2400/9464
2019-12-31T16:18:36.2291320Z .................................................................................................... 2500/9464
---
2019-12-31T16:21:34.0006187Z .................i...............i.................................................................. 4900/9464
2019-12-31T16:21:43.9586379Z .................................................................................................... 5000/9464
2019-12-31T16:21:49.5398460Z ..............................................................i..................................... 5100/9464
2019-12-31T16:21:57.7177257Z .................................................................................................... 5200/9464
2019-12-31T16:22:05.2695444Z .............................ii.ii...........i...................................................... 5300/9464
2019-12-31T16:22:14.6252825Z .................................................................................................... 5500/9464
2019-12-31T16:22:24.7396745Z .................................................................................................... 5600/9464
2019-12-31T16:22:31.6389949Z ............i....................................................................................... 5700/9464
2019-12-31T16:22:37.6899694Z .................................................................................................... 5800/9464
2019-12-31T16:22:37.6899694Z .................................................................................................... 5800/9464
2019-12-31T16:22:48.3255012Z .................................................................................................... 5900/9464
2019-12-31T16:22:59.9659989Z ii...i..ii...........i.............................................................................. 6000/9464
2019-12-31T16:23:17.7058815Z .................................................................................................... 6200/9464
2019-12-31T16:23:25.0724654Z .................................................................................................... 6300/9464
2019-12-31T16:23:25.0724654Z .................................................................................................... 6300/9464
2019-12-31T16:23:47.7153359Z ...........................i..ii.................................................................... 6400/9464
2019-12-31T16:24:07.0845039Z .................................................................................................... 6600/9464
2019-12-31T16:24:09.2276784Z ..i................................................................................................. 6700/9464
2019-12-31T16:24:11.5937652Z .................................................................................................... 6800/9464
2019-12-31T16:24:14.1585428Z ..i................................................................................................. 6900/9464
---
2019-12-31T16:25:51.0641312Z .................................................................................................... 7500/9464
2019-12-31T16:25:56.0063612Z .................................................................................................... 7600/9464
2019-12-31T16:26:01.5693736Z .................................................................................................... 7700/9464
2019-12-31T16:26:11.5357169Z .................................................................................................... 7800/9464
2019-12-31T16:26:19.0995304Z .................................iiii............................................................... 7900/9464
2019-12-31T16:26:33.8778691Z .................................................................................................... 8100/9464
2019-12-31T16:26:42.6735414Z .................................................................................................... 8200/9464
2019-12-31T16:26:56.9206799Z .................................................................................................... 8300/9464
2019-12-31T16:27:04.4684940Z .................................................................................................... 8400/9464
---
2019-12-31T16:29:23.4587017Z  finished in 6.672
2019-12-31T16:29:23.4791489Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T16:29:23.6675057Z 
2019-12-31T16:29:23.6676708Z running 166 tests
2019-12-31T16:29:26.7362535Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-31T16:29:28.8624439Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-31T16:29:28.8627808Z 
2019-12-31T16:29:28.8631528Z  finished in 5.384
2019-12-31T16:29:28.8820454Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T16:29:29.0505690Z 
---
2019-12-31T16:29:31.0543501Z  finished in 2.171
2019-12-31T16:29:31.0749444Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T16:29:31.2398281Z 
2019-12-31T16:29:31.2398653Z running 9 tests
2019-12-31T16:29:31.2399648Z iiiiiiiii
2019-12-31T16:29:31.2400409Z 
2019-12-31T16:29:31.2400565Z  finished in 0.165
2019-12-31T16:29:31.2586892Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T16:29:31.4425634Z 
---
2019-12-31T16:29:51.2492875Z  finished in 19.593
2019-12-31T16:29:51.2493242Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T16:29:51.2493286Z 
2019-12-31T16:29:51.2493330Z running 124 tests
2019-12-31T16:30:15.3403741Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-31T16:30:19.4854905Z .i.iii.....iiiiii.....ii
2019-12-31T16:30:19.4856452Z 
2019-12-31T16:30:19.4858329Z  finished in 28.609
2019-12-31T16:30:19.4866230Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T16:30:19.4867904Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-31T16:43:38.1789411Z 
2019-12-31T16:43:38.1790907Z    Doc-tests core
2019-12-31T16:43:42.4585099Z 
2019-12-31T16:43:42.4586036Z running 2439 tests
2019-12-31T16:43:51.4363062Z ......iiiii......................................................................................... 100/2439
2019-12-31T16:44:00.4787387Z ..................................................................................ii................ 200/2439
2019-12-31T16:44:21.1381822Z ................i................................................................................... 400/2439
2019-12-31T16:44:21.1381822Z ................i................................................................................... 400/2439
2019-12-31T16:44:30.5771116Z ................................................................i..i..................iiii.......... 500/2439
2019-12-31T16:44:47.0905074Z .................................................................................................... 700/2439
2019-12-31T16:44:55.5878383Z .................................................................................................... 800/2439
2019-12-31T16:45:04.1324499Z .................................................................................................... 900/2439
2019-12-31T16:45:12.6842407Z .................................................................................................... 1000/2439
---
2019-12-31T16:48:56.2458067Z 
2019-12-31T16:48:56.2458355Z running 1002 tests
2019-12-31T16:49:14.1928332Z i................................................................................................... 100/1002
2019-12-31T16:49:24.1425204Z .................................................................................................... 200/1002
2019-12-31T16:49:31.1638657Z .................iii......i......i...i......i....................................................... 300/1002
2019-12-31T16:49:35.9785086Z .................................................................................................... 400/1002
2019-12-31T16:49:42.7886592Z .........................................i..i.....................................ii................ 500/1002
2019-12-31T16:49:55.2206034Z .................................................................................................... 700/1002
2019-12-31T16:49:55.2206034Z .................................................................................................... 700/1002
2019-12-31T16:50:01.9820177Z ............................iiii.................................................................... 800/1002
2019-12-31T16:50:15.8249357Z .................................................................................................... 900/1002
2019-12-31T16:50:22.7154693Z ..................................................iiii.............................................. 1000/1002
2019-12-31T16:50:22.7732557Z test result: ok. 982 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2019-12-31T16:50:22.7736041Z 
2019-12-31T16:50:22.7873607Z  finished in 178.863
2019-12-31T16:50:22.7890914Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2019-12-31T17:08:10.0861419Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2019-12-31T17:08:10.4459170Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2019-12-31T17:08:10.6066569Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2019-12-31T17:08:12.2275965Z     Finished release [optimized] target(s) in 1.77s
2019-12-31T17:08:12.2797972Z cargo/print.html:1517: broken link - std/primitive.char.html
2019-12-31T17:08:12.2823661Z cargo/print.html:2404: broken link - std/macro.debug_assert.html
2019-12-31T17:08:12.2851536Z cargo/print.html:3700: broken link - std/macro.env.html
2019-12-31T17:08:12.2857150Z cargo/print.html:3963: broken link - std/macro.include.html
2019-12-31T17:08:12.2857430Z cargo/print.html:3964: broken link - std/macro.concat.html
2019-12-31T17:08:12.2857706Z cargo/print.html:3964: broken link - std/macro.env.html
2019-12-31T17:08:12.2858259Z cargo/print.html:4254: broken link - std/macro.cfg.html
2019-12-31T17:08:12.2864356Z cargo/print.html:4942: broken link - std/primitive.char.html
2019-12-31T17:08:12.3077834Z cargo/reference/build-scripts.html:297: broken link - std/macro.env.html
2019-12-31T17:08:12.3110481Z cargo/reference/registries.html:292: broken link - std/primitive.char.html
2019-12-31T17:08:12.3126798Z cargo/reference/manifest.html:229: broken link - std/primitive.char.html
2019-12-31T17:08:12.3167791Z cargo/reference/profiles.html:210: broken link - std/macro.debug_assert.html
2019-12-31T17:08:12.3210040Z cargo/reference/build-script-examples.html:260: broken link - std/macro.include.html
2019-12-31T17:08:12.3210462Z cargo/reference/build-script-examples.html:261: broken link - std/macro.concat.html
2019-12-31T17:08:12.3210757Z cargo/reference/build-script-examples.html:261: broken link - std/macro.env.html
2019-12-31T17:08:12.3211039Z cargo/reference/build-script-examples.html:551: broken link - std/macro.cfg.html
2019-12-31T17:08:12.6292315Z nomicon/dropck.html:421: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T17:08:12.6316649Z nomicon/other-reprs.html:215: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:12.6345109Z nomicon/panic-handler.html:154: broken link - core/panic/struct.PanicInfo.html
2019-12-31T17:08:12.6364230Z nomicon/unchecked-uninit.html:159: broken link - core/mem/union.MaybeUninit.html
2019-12-31T17:08:12.6364569Z nomicon/unchecked-uninit.html:203: broken link - core/mem/union.MaybeUninit.html
2019-12-31T17:08:12.6364861Z nomicon/unchecked-uninit.html:239: broken link - core/ptr/index.html
2019-12-31T17:08:12.6365115Z nomicon/unchecked-uninit.html:241: broken link - core/ptr/fn.write.html
2019-12-31T17:08:12.6365361Z nomicon/unchecked-uninit.html:241: broken link - std/ptr/fn.copy.html
2019-12-31T17:08:12.6365644Z nomicon/unchecked-uninit.html:241: broken link - std/ptr/fn.copy_nonoverlapping.html
2019-12-31T17:08:12.6376749Z nomicon/safe-unsafe-meaning.html:183: broken link - std/primitive.pointer.html
2019-12-31T17:08:12.6377071Z nomicon/safe-unsafe-meaning.html:191: broken link - std/marker/trait.Send.html
2019-12-31T17:08:12.6377334Z nomicon/safe-unsafe-meaning.html:193: broken link - std/marker/trait.Sync.html
2019-12-31T17:08:12.6377594Z nomicon/safe-unsafe-meaning.html:195: broken link - std/alloc/trait.GlobalAlloc.html
2019-12-31T17:08:12.6383269Z nomicon/transmutes.html:157: broken link - std/mem/fn.transmute.html
2019-12-31T17:08:12.6383578Z nomicon/transmutes.html:178: broken link - std/mem/fn.transmute_copy.html
2019-12-31T17:08:12.6409822Z nomicon/print.html:283: broken link - std/primitive.pointer.html
2019-12-31T17:08:12.6410135Z nomicon/print.html:291: broken link - std/marker/trait.Send.html
2019-12-31T17:08:12.6410385Z nomicon/print.html:293: broken link - std/marker/trait.Sync.html
2019-12-31T17:08:12.6410635Z nomicon/print.html:295: broken link - std/alloc/trait.GlobalAlloc.html
2019-12-31T17:08:12.6413850Z nomicon/print.html:949: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:12.6422607Z nomicon/print.html:2331: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T17:08:12.6422889Z nomicon/print.html:2903: broken link - std/mem/fn.transmute.html
2019-12-31T17:08:12.6423139Z nomicon/print.html:2924: broken link - std/mem/fn.transmute_copy.html
2019-12-31T17:08:12.6423415Z nomicon/print.html:3127: broken link - core/mem/union.MaybeUninit.html
2019-12-31T17:08:12.6423678Z nomicon/print.html:3171: broken link - core/mem/union.MaybeUninit.html
2019-12-31T17:08:12.6423921Z nomicon/print.html:3207: broken link - core/ptr/index.html
2019-12-31T17:08:12.6424181Z nomicon/print.html:3209: broken link - core/ptr/fn.write.html
2019-12-31T17:08:12.6424419Z nomicon/print.html:3209: broken link - std/ptr/fn.copy.html
2019-12-31T17:08:12.6424667Z nomicon/print.html:3209: broken link - std/ptr/fn.copy_nonoverlapping.html
2019-12-31T17:08:12.6433669Z nomicon/print.html:5850: broken link - std/ops/trait.Drop.html
2019-12-31T17:08:12.6434239Z nomicon/print.html:6214: broken link - std/panic/fn.catch_unwind.html
2019-12-31T17:08:12.6434489Z nomicon/print.html:6230: broken link - std/panic/fn.catch_unwind.html
2019-12-31T17:08:12.6434772Z nomicon/print.html:6231: broken link - std/panic/fn.catch_unwind.html
2019-12-31T17:08:12.6435019Z nomicon/print.html:6294: broken link - core/panic/struct.PanicInfo.html
2019-12-31T17:08:12.6530093Z nomicon/ffi.html:355: broken link - std/ops/trait.Drop.html
2019-12-31T17:08:12.6530626Z nomicon/ffi.html:719: broken link - std/panic/fn.catch_unwind.html
2019-12-31T17:08:12.6530953Z nomicon/ffi.html:735: broken link - std/panic/fn.catch_unwind.html
2019-12-31T17:08:12.6531201Z nomicon/ffi.html:736: broken link - std/panic/fn.catch_unwind.html
2019-12-31T17:08:13.5640163Z book/ch09-02-recoverable-errors-with-result.html:191: broken link - std/index.html
2019-12-31T17:08:13.5773657Z book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html:468: broken link - std/prelude/index.html
2019-12-31T17:08:13.5981103Z book/procedural-macros.html:43: broken link - proc_macro/index.html
2019-12-31T17:08:13.6138643Z book/ch06-01-defining-an-enum.html:304: broken link - std/net/enum.IpAddr.html
2019-12-31T17:08:13.6140175Z book/ch06-01-defining-an-enum.html:441: broken link - std/option/enum.Option.html
2019-12-31T17:08:13.6140881Z book/ch06-01-defining-an-enum.html:519: broken link - std/option/enum.Option.html
2019-12-31T17:08:13.6500931Z book/appendix-03-derivable-traits.html:170: broken link - std/index.html
2019-12-31T17:08:13.6504720Z book/casting-between-types.html:51: broken link - std/mem/fn.transmute.html
2019-12-31T17:08:13.6685720Z book/print.html:884: broken link - std/prelude/index.html
2019-12-31T17:08:13.6686077Z book/print.html:929: broken link - std/string/struct.String.html
2019-12-31T17:08:13.6686402Z book/print.html:948: broken link - std/io/struct.Stdin.html
2019-12-31T17:08:13.6686684Z book/print.html:951: broken link - std/io/struct.Stdin.html
2019-12-31T17:08:13.6686949Z book/print.html:981: broken link - std/io/type.Result.html
2019-12-31T17:08:13.6687276Z book/print.html:982: broken link - std/result/enum.Result.html
2019-12-31T17:08:13.6687551Z book/print.html:994: broken link - std/result/enum.Result.html
2019-12-31T17:08:13.6687815Z book/print.html:1372: broken link - std/primitive.str.html
2019-12-31T17:08:13.6688102Z book/print.html:1907: broken link - std/num/struct.Wrapping.html
2019-12-31T17:08:13.6701001Z book/print.html:4846: broken link - std/net/enum.IpAddr.html
2019-12-31T17:08:13.6701335Z book/print.html:4983: broken link - std/option/enum.Option.html
2019-12-31T17:08:13.6701651Z book/print.html:5061: broken link - std/option/enum.Option.html
2019-12-31T17:08:13.6701969Z book/print.html:6246: broken link - std/prelude/index.html
2019-12-31T17:08:13.6702267Z book/print.html:6334: broken link - std/collections/index.html
2019-12-31T17:08:13.6707390Z book/print.html:7486: broken link - std/index.html
2019-12-31T17:08:13.6766391Z book/print.html:24040: broken link - std/index.html
2019-12-31T17:08:13.6796944Z book/ch08-00-common-collections.html:173: broken link - std/collections/index.html
2019-12-31T17:08:13.6833967Z book/borrow-and-asref.html:45: broken link - std/convert/trait.AsRef.html
2019-12-31T17:08:13.6834290Z book/borrow-and-asref.html:47: broken link - std/convert/trait.AsRef.html
2019-12-31T17:08:13.8002310Z book/ch02-00-guessing-game-tutorial.html:236: broken link - std/prelude/index.html
2019-12-31T17:08:13.8002789Z book/ch02-00-guessing-game-tutorial.html:281: broken link - std/string/struct.String.html
2019-12-31T17:08:13.8003079Z book/ch02-00-guessing-game-tutorial.html:300: broken link - std/io/struct.Stdin.html
2019-12-31T17:08:13.8003377Z book/ch02-00-guessing-game-tutorial.html:303: broken link - std/io/struct.Stdin.html
2019-12-31T17:08:13.8003649Z book/ch02-00-guessing-game-tutorial.html:333: broken link - std/io/type.Result.html
2019-12-31T17:08:13.8003919Z book/ch02-00-guessing-game-tutorial.html:334: broken link - std/result/enum.Result.html
2019-12-31T17:08:13.8004609Z book/ch02-00-guessing-game-tutorial.html:346: broken link - std/result/enum.Result.html
2019-12-31T17:08:13.8004879Z book/ch02-00-guessing-game-tutorial.html:724: broken link - std/primitive.str.html
2019-12-31T17:08:13.8025919Z book/ch03-02-data-types.html:256: broken link - std/num/struct.Wrapping.html
2019-12-31T17:08:13.9888868Z index.html:83: broken link - std/index.html
2019-12-31T17:08:13.9925246Z error-index.html:7: broken link - light1.42.0.css
2019-12-31T17:08:14.0033093Z reference/expressions/call-expr.html:169: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.0034047Z reference/expressions/call-expr.html:169: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.0034329Z reference/expressions/call-expr.html:170: broken link - std/ops/trait.FnOnce.html
2019-12-31T17:08:14.0094461Z reference/expressions/array-expr.html:233: broken link - std/ops/trait.Index.html
2019-12-31T17:08:14.0094778Z reference/expressions/array-expr.html:233: broken link - std/ops/trait.IndexMut.html
2019-12-31T17:08:14.0120546Z reference/expressions/await-expr.html:164: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0120862Z reference/expressions/await-expr.html:169: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0121126Z reference/expressions/await-expr.html:170: broken link - std/pin/struct.Pin.html
2019-12-31T17:08:14.0121408Z reference/expressions/await-expr.html:171: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0121688Z reference/expressions/await-expr.html:173: broken link - std/task/enum.Poll.html
2019-12-31T17:08:14.0121951Z reference/expressions/await-expr.html:177: broken link - std/task/enum.Poll.html
2019-12-31T17:08:14.0122228Z reference/expressions/await-expr.html:178: broken link - std/task/enum.Poll.html
2019-12-31T17:08:14.0122491Z reference/expressions/await-expr.html:186: broken link - std/task/struct.Context.html
2019-12-31T17:08:14.0247543Z reference/expressions/block-expr.html:251: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.0247915Z reference/expressions/block-expr.html:252: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0308727Z reference/special-types-and-traits.html:158: broken link - std/index.html
2019-12-31T17:08:14.0309158Z reference/special-types-and-traits.html:162: broken link - std/boxed/struct.Box.html
2019-12-31T17:08:14.0310184Z reference/special-types-and-traits.html:173: broken link - std/rc/struct.Rc.html
2019-12-31T17:08:14.0310481Z reference/special-types-and-traits.html:175: broken link - std/sync/struct.Arc.html
2019-12-31T17:08:14.0310778Z reference/special-types-and-traits.html:177: broken link - std/pin/struct.Pin.html
2019-12-31T17:08:14.0311052Z reference/special-types-and-traits.html:179: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.0311325Z reference/special-types-and-traits.html:184: broken link - std/marker/struct.PhantomData.html
2019-12-31T17:08:14.0311602Z reference/special-types-and-traits.html:188: broken link - std/ops/index.html
2019-12-31T17:08:14.0311871Z reference/special-types-and-traits.html:188: broken link - std/cmp/index.html
2019-12-31T17:08:14.0312131Z reference/special-types-and-traits.html:191: broken link - std/ops/trait.Deref.html
2019-12-31T17:08:14.0312414Z reference/special-types-and-traits.html:191: broken link - std/ops/trait.DerefMut.html
2019-12-31T17:08:14.0312676Z reference/special-types-and-traits.html:194: broken link - std/ops/trait.Drop.html
2019-12-31T17:08:14.0312946Z reference/special-types-and-traits.html:197: broken link - std/marker/trait.Copy.html
2019-12-31T17:08:14.0313228Z reference/special-types-and-traits.html:213: broken link - std/clone/trait.Clone.html
2019-12-31T17:08:14.0313493Z reference/special-types-and-traits.html:222: broken link - std/marker/trait.Send.html
2019-12-31T17:08:14.0313753Z reference/special-types-and-traits.html:225: broken link - std/marker/trait.Sync.html
2019-12-31T17:08:14.0314032Z reference/special-types-and-traits.html:229: broken link - std/marker/trait.Send.html
2019-12-31T17:08:14.0314575Z reference/special-types-and-traits.html:229: broken link - std/marker/trait.Sync.html
2019-12-31T17:08:14.0314846Z reference/special-types-and-traits.html:229: broken link - std/panic/trait.UnwindSafe.html
2019-12-31T17:08:14.0315137Z reference/special-types-and-traits.html:229: broken link - std/panic/trait.RefUnwindSafe.html
2019-12-31T17:08:14.0315403Z reference/special-types-and-traits.html:258: broken link - std/marker/trait.Sized.html
2019-12-31T17:08:14.0322867Z reference/attributes/codegen.html:246: broken link - std/macro.is_x86_feature_detected.html
2019-12-31T17:08:14.0334531Z reference/attributes/derive.html:162: broken link - std/cmp/trait.PartialEq.html
2019-12-31T17:08:14.0334822Z reference/attributes/derive.html:162: broken link - std/clone/trait.Clone.html
2019-12-31T17:08:14.0364178Z reference/attributes/testing.html:175: broken link - std/process/trait.Termination.html
2019-12-31T17:08:14.0414320Z reference/crates-and-source-files.html:239: broken link - std/index.html
2019-12-31T17:08:14.0414639Z reference/crates-and-source-files.html:239: broken link - std/prelude/index.html
2019-12-31T17:08:14.0414917Z reference/crates-and-source-files.html:241: broken link - core/index.html
2019-12-31T17:08:14.0415179Z reference/crates-and-source-files.html:242: broken link - core/prelude/index.html
2019-12-31T17:08:14.0415448Z reference/crates-and-source-files.html:266: broken link - std/process/trait.Termination.html
2019-12-31T17:08:14.0415733Z reference/crates-and-source-files.html:283: broken link - std/primitive.char.html
2019-12-31T17:08:14.0475576Z reference/type-layout.html:170: broken link - std/mem/fn.align_of_val.html
2019-12-31T17:08:14.0480342Z reference/type-layout.html:174: broken link - std/mem/fn.size_of_val.html
2019-12-31T17:08:14.0480744Z reference/type-layout.html:176: broken link - std/marker/trait.Sized.html
2019-12-31T17:08:14.0481037Z reference/type-layout.html:176: broken link - std/mem/fn.size_of.html
2019-12-31T17:08:14.0481320Z reference/type-layout.html:177: broken link - std/mem/fn.align_of.html
2019-12-31T17:08:14.0481698Z reference/type-layout.html:177: broken link - std/marker/trait.Sized.html
2019-12-31T17:08:14.0522427Z reference/items/unions.html:211: broken link - std/mem/fn.transmute.html
2019-12-31T17:08:14.0648121Z reference/items/enumerations.html:213: broken link - std/mem/fn.discriminant.html
2019-12-31T17:08:14.0661591Z reference/procedural-macros.html:187: broken link - std/macro.compile_error.html
2019-12-31T17:08:14.0661915Z reference/procedural-macros.html:190: broken link - proc_macro/index.html
2019-12-31T17:08:14.0662221Z reference/procedural-macros.html:192: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0662496Z reference/procedural-macros.html:217: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0662761Z reference/procedural-macros.html:218: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0663047Z reference/procedural-macros.html:254: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0663327Z reference/procedural-macros.html:255: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0663592Z reference/procedural-macros.html:257: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0663873Z reference/procedural-macros.html:314: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0664141Z reference/procedural-macros.html:317: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0664413Z reference/procedural-macros.html:317: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0664690Z reference/procedural-macros.html:318: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0664955Z reference/procedural-macros.html:332: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0793150Z reference/conditional-compilation.html:317: broken link - std/macro.debug_assert.html
2019-12-31T17:08:14.0844974Z reference/introduction.html:175: broken link - std/index.html
2019-12-31T17:08:14.0864526Z reference/print.html:177: broken link - std/index.html
2019-12-31T17:08:14.0893821Z reference/print.html:1985: broken link - std/macro.compile_error.html
2019-12-31T17:08:14.0894124Z reference/print.html:1988: broken link - proc_macro/index.html
2019-12-31T17:08:14.0894412Z reference/print.html:1990: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0894679Z reference/print.html:2015: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0895140Z reference/print.html:2016: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0895492Z reference/print.html:2052: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0895753Z reference/print.html:2053: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0896009Z reference/print.html:2055: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0896273Z reference/print.html:2112: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0896547Z reference/print.html:2115: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0896800Z reference/print.html:2115: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0897067Z reference/print.html:2116: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0897324Z reference/print.html:2130: broken link - proc_macro/struct.TokenStream.html
2019-12-31T17:08:14.0897563Z reference/print.html:2257: broken link - std/index.html
2019-12-31T17:08:14.0897830Z reference/print.html:2257: broken link - std/prelude/index.html
2019-12-31T17:08:14.0898070Z reference/print.html:2259: broken link - core/index.html
2019-12-31T17:08:14.0898315Z reference/print.html:2260: broken link - core/prelude/index.html
2019-12-31T17:08:14.0898569Z reference/print.html:2284: broken link - std/process/trait.Termination.html
2019-12-31T17:08:14.0898827Z reference/print.html:2301: broken link - std/primitive.char.html
2019-12-31T17:08:14.0900182Z reference/print.html:2473: broken link - std/macro.debug_assert.html
2019-12-31T17:08:14.0908758Z reference/print.html:3555: broken link - std/mem/fn.discriminant.html
2019-12-31T17:08:14.0909389Z reference/print.html:3721: broken link - std/mem/fn.transmute.html
2019-12-31T17:08:14.0934568Z reference/print.html:5462: broken link - std/process/trait.Termination.html
2019-12-31T17:08:14.0934881Z reference/print.html:5531: broken link - std/cmp/trait.PartialEq.html
2019-12-31T17:08:14.0935155Z reference/print.html:5531: broken link - std/clone/trait.Clone.html
2019-12-31T17:08:14.0939972Z reference/print.html:5901: broken link - std/macro.is_x86_feature_detected.html
2019-12-31T17:08:14.0948774Z reference/print.html:6323: broken link - std/boxed/struct.Box.html
2019-12-31T17:08:14.0955054Z reference/print.html:6615: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.0955381Z reference/print.html:6616: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0966932Z reference/print.html:7286: broken link - std/ops/trait.Index.html
2019-12-31T17:08:14.0967240Z reference/print.html:7286: broken link - std/ops/trait.IndexMut.html
2019-12-31T17:08:14.0972303Z reference/print.html:7519: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.0972650Z reference/print.html:7519: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.0972930Z reference/print.html:7520: broken link - std/ops/trait.FnOnce.html
2019-12-31T17:08:14.0984986Z reference/print.html:8443: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0985317Z reference/print.html:8448: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0985570Z reference/print.html:8449: broken link - std/pin/struct.Pin.html
2019-12-31T17:08:14.0985851Z reference/print.html:8450: broken link - std/future/trait.Future.html
2019-12-31T17:08:14.0986140Z reference/print.html:8452: broken link - std/task/enum.Poll.html
2019-12-31T17:08:14.0986406Z reference/print.html:8456: broken link - std/task/enum.Poll.html
2019-12-31T17:08:14.0986672Z reference/print.html:8457: broken link - std/task/enum.Poll.html
2019-12-31T17:08:14.0987221Z reference/print.html:8465: broken link - std/task/struct.Context.html
2019-12-31T17:08:14.1002073Z reference/print.html:9421: broken link - std/vec/struct.Vec.html
2019-12-31T17:08:14.1002376Z reference/print.html:9543: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.1002632Z reference/print.html:9543: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.1002880Z reference/print.html:9543: broken link - std/ops/trait.FnOnce.html
2019-12-31T17:08:14.1003309Z reference/print.html:9651: broken link - std/ops/trait.FnOnce.html
2019-12-31T17:08:14.1003673Z reference/print.html:9657: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.1003943Z reference/print.html:9661: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.1004207Z reference/print.html:9665: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.1004490Z reference/print.html:9665: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.1008635Z reference/print.html:9989: broken link - std/mem/fn.align_of_val.html
2019-12-31T17:08:14.1011906Z reference/print.html:9993: broken link - std/mem/fn.size_of_val.html
2019-12-31T17:08:14.1012268Z reference/print.html:9995: broken link - std/marker/trait.Sized.html
2019-12-31T17:08:14.1012517Z reference/print.html:9995: broken link - std/mem/fn.size_of.html
2019-12-31T17:08:14.1022834Z reference/print.html:9996: broken link - std/mem/fn.align_of.html
2019-12-31T17:08:14.1023160Z reference/print.html:9996: broken link - std/marker/trait.Sized.html
2019-12-31T17:08:14.1026329Z reference/print.html:10262: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1026643Z reference/print.html:10269: broken link - std/cell/struct.RefCell.html
2019-12-31T17:08:14.1026895Z reference/print.html:10271: broken link - std/sync/atomic/index.html
2019-12-31T17:08:14.1027142Z reference/print.html:10652: broken link - std/marker/trait.Unsize.html
2019-12-31T17:08:14.1027413Z reference/print.html:10652: broken link - std/ops/trait.CoerceUnsized.html
2019-12-31T17:08:14.1027679Z reference/print.html:10691: broken link - std/ops/trait.Drop.html
2019-12-31T17:08:14.1027931Z reference/print.html:10708: broken link - std/ptr/fn.drop_in_place.html
2019-12-31T17:08:14.1028201Z reference/print.html:10761: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T17:08:14.1028442Z reference/print.html:10973: broken link - std/index.html
2019-12-31T17:08:14.1028691Z reference/print.html:10977: broken link - std/boxed/struct.Box.html
2019-12-31T17:08:14.1031792Z reference/print.html:10988: broken link - std/rc/struct.Rc.html
2019-12-31T17:08:14.1032742Z reference/print.html:10990: broken link - std/sync/struct.Arc.html
2019-12-31T17:08:14.1032996Z reference/print.html:10992: broken link - std/pin/struct.Pin.html
2019-12-31T17:08:14.1033273Z reference/print.html:10994: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1033535Z reference/print.html:10999: broken link - std/marker/struct.PhantomData.html
2019-12-31T17:08:14.1033795Z reference/print.html:11003: broken link - std/ops/index.html
2019-12-31T17:08:14.1034088Z reference/print.html:11003: broken link - std/cmp/index.html
2019-12-31T17:08:14.1034426Z reference/print.html:11006: broken link - std/ops/trait.Deref.html
2019-12-31T17:08:14.1034709Z reference/print.html:11006: broken link - std/ops/trait.DerefMut.html
2019-12-31T17:08:14.1034992Z reference/print.html:11009: broken link - std/ops/trait.Drop.html
2019-12-31T17:08:14.1035276Z reference/print.html:11012: broken link - std/marker/trait.Copy.html
2019-12-31T17:08:14.1035548Z reference/print.html:11028: broken link - std/clone/trait.Clone.html
2019-12-31T17:08:14.1036227Z reference/print.html:11037: broken link - std/marker/trait.Send.html
2019-12-31T17:08:14.1036514Z reference/print.html:11040: broken link - std/marker/trait.Sync.html
2019-12-31T17:08:14.1036783Z reference/print.html:11044: broken link - std/marker/trait.Send.html
2019-12-31T17:08:14.1037058Z reference/print.html:11044: broken link - std/marker/trait.Sync.html
2019-12-31T17:08:14.1037604Z reference/print.html:11044: broken link - std/panic/trait.UnwindSafe.html
2019-12-31T17:08:14.1037888Z reference/print.html:11044: broken link - std/panic/trait.RefUnwindSafe.html
2019-12-31T17:08:14.1038174Z reference/print.html:11073: broken link - std/marker/trait.Sized.html
2019-12-31T17:08:14.1038452Z reference/print.html:11390: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1038726Z reference/print.html:11393: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1039464Z reference/print.html:11440: broken link - core/ptr/struct.NonNull.html
2019-12-31T17:08:14.1039816Z reference/print.html:11440: broken link - core/num/index.html
2019-12-31T17:08:14.1040793Z reference/print.html:11638: broken link - core/panic/struct.PanicInfo.html
2019-12-31T17:08:14.1041076Z reference/print.html:11675: broken link - std/panic/fn.set_hook.html
2019-12-31T17:08:14.1041334Z reference/print.html:11678: broken link - alloc/alloc/trait.GlobalAlloc.html
2019-12-31T17:08:14.1065580Z reference/runtime.html:162: broken link - core/panic/struct.PanicInfo.html
2019-12-31T17:08:14.1065890Z reference/runtime.html:199: broken link - std/panic/fn.set_hook.html
2019-12-31T17:08:14.1066147Z reference/runtime.html:202: broken link - alloc/alloc/trait.GlobalAlloc.html
2019-12-31T17:08:14.1085424Z reference/index.html:175: broken link - std/index.html
2019-12-31T17:08:14.1124171Z reference/expressions.html:277: broken link - std/boxed/struct.Box.html
2019-12-31T17:08:14.1146084Z reference/types/array.html:180: broken link - std/vec/struct.Vec.html
2019-12-31T17:08:14.1209356Z reference/types/function-item.html:201: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.1209734Z reference/types/function-item.html:201: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.1209998Z reference/types/function-item.html:201: broken link - std/ops/trait.FnOnce.html
2019-12-31T17:08:14.1272724Z reference/types/closure.html:263: broken link - std/ops/trait.FnOnce.html
2019-12-31T17:08:14.1273040Z reference/types/closure.html:269: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.1273292Z reference/types/closure.html:273: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.1273556Z reference/types/closure.html:277: broken link - std/ops/trait.Fn.html
2019-12-31T17:08:14.1273811Z reference/types/closure.html:277: broken link - std/ops/trait.FnMut.html
2019-12-31T17:08:14.1310812Z reference/interior-mutability.html:163: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1311148Z reference/interior-mutability.html:170: broken link - std/cell/struct.RefCell.html
2019-12-31T17:08:14.1311418Z reference/interior-mutability.html:172: broken link - std/sync/atomic/index.html
2019-12-31T17:08:14.1339576Z reference/type-coercions.html:322: broken link - std/marker/trait.Unsize.html
2019-12-31T17:08:14.1339909Z reference/type-coercions.html:322: broken link - std/ops/trait.CoerceUnsized.html
2019-12-31T17:08:14.1350631Z reference/destructors.html:165: broken link - std/ops/trait.Drop.html
2019-12-31T17:08:14.1350943Z reference/destructors.html:182: broken link - std/ptr/fn.drop_in_place.html
2019-12-31T17:08:14.1351235Z reference/destructors.html:235: broken link - std/mem/struct.ManuallyDrop.html
2019-12-31T17:08:14.1380807Z reference/behavior-considered-undefined.html:179: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1381138Z reference/behavior-considered-undefined.html:182: broken link - std/cell/struct.UnsafeCell.html
2019-12-31T17:08:14.1381436Z reference/behavior-considered-undefined.html:229: broken link - core/ptr/struct.NonNull.html
2019-12-31T17:08:14.1381707Z reference/behavior-considered-undefined.html:229: broken link - core/num/index.html
2019-12-31T17:08:14.1414692Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2019-12-31T17:08:14.1429553Z 
2019-12-31T17:08:14.1429642Z 
2019-12-31T17:08:14.1433014Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2019-12-31T17:08:14.1433166Z expected success, got: exit code: 101
---
2019-12-31T17:08:14.1507946Z   local time: Tue Dec 31 17:08:14 UTC 2019
2019-12-31T17:08:14.2233369Z   network time: Tue, 31 Dec 2019 17:08:14 GMT
2019-12-31T17:08:14.2233564Z == end clock drift check ==
2019-12-31T17:08:19.5209189Z 
2019-12-31T17:08:19.5307102Z ##[error]Bash exited with code '1'.
2019-12-31T17:08:19.5365671Z ##[section]Starting: Checkout
2019-12-31T17:08:19.5367950Z ==============================================================================
2019-12-31T17:08:19.5368011Z Task         : Get sources
2019-12-31T17:08:19.5368059Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
