plain
2019-12-13T18:20:15.1694641Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-13T18:20:15.1850706Z ##[command]git config gc.auto 0
2019-12-13T18:20:15.1922999Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-13T18:20:15.1983254Z ##[command]git config --get-all http.proxy
2019-12-13T18:20:15.2109733Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67281/merge:refs/remotes/pull/67281/merge
---
2019-12-13T19:12:27.1902401Z .................................................................................................... 1600/9366
2019-12-13T19:12:31.2227670Z .................................................................................................... 1700/9366
2019-12-13T19:12:42.5261894Z ................................................................i................................... 1800/9366
2019-12-13T19:12:49.5562298Z .................................................................................................... 1900/9366
2019-12-13T19:13:02.8088211Z .................................................iiiii.............................................. 2000/9366
2019-12-13T19:13:12.1502410Z .................................................................................................... 2200/9366
2019-12-13T19:13:14.2405706Z .................................................................................................... 2300/9366
2019-12-13T19:13:17.0722946Z .................................................................................................... 2400/9366
2019-12-13T19:13:36.0849345Z .................................................................................................... 2500/9366
---
2019-12-13T19:15:45.7467810Z .................................................................................................... 4700/9366
2019-12-13T19:15:50.0879713Z .........................................................i...............i.......................... 4800/9366
2019-12-13T19:15:56.8448623Z .................................................................................................... 4900/9366
2019-12-13T19:16:03.9497823Z .................................................................................................... 5000/9366
2019-12-13T19:16:08.4153827Z .i.................................................................................................. 5100/9366
2019-12-13T19:16:17.1782022Z ...................................................................ii.ii...........i................ 5200/9366
2019-12-13T19:16:24.8092511Z ...i................................................................................................ 5400/9366
2019-12-13T19:16:33.5362741Z .................................................................................................... 5500/9366
2019-12-13T19:16:38.7032126Z .................................................i.................................................. 5600/9366
2019-12-13T19:16:44.8123824Z .................................................................................................... 5700/9366
2019-12-13T19:16:44.8123824Z .................................................................................................... 5700/9366
2019-12-13T19:16:53.5743755Z .................................................................................................... 5800/9366
2019-12-13T19:17:02.1920063Z .....................................ii...i..ii...........i......................................... 5900/9366
2019-12-13T19:17:19.5752923Z .................................................................................................... 6100/9366
2019-12-13T19:17:26.9925907Z .................................................................................................... 6200/9366
2019-12-13T19:17:26.9925907Z .................................................................................................... 6200/9366
2019-12-13T19:17:37.2824890Z .............................................................i..ii.................................. 6300/9366
2019-12-13T19:18:00.8245386Z .................................................................................................... 6500/9366
2019-12-13T19:18:02.5323821Z .................................i.................................................................. 6600/9366
2019-12-13T19:18:04.4405777Z .................................................................................................... 6700/9366
2019-12-13T19:18:06.4244668Z .........................i.......................................................................... 6800/9366
---
2019-12-13T19:19:29.6148696Z .................................................................................................... 7400/9366
2019-12-13T19:19:33.9253756Z .................................................................................................... 7500/9366
2019-12-13T19:19:40.1741936Z .................................................................................................... 7600/9366
2019-12-13T19:19:48.2539459Z .................................................................................................... 7700/9366
2019-12-13T19:19:54.3608809Z ...........................................iiii..................................................... 7800/9366
2019-12-13T19:20:06.8341237Z .................................................................................................... 8000/9366
2019-12-13T19:20:13.9878091Z .................................................................................................... 8100/9366
2019-12-13T19:20:25.8954491Z .................................................................................................... 8200/9366
2019-12-13T19:20:33.3212359Z .................................................................................................... 8300/9366
---
2019-12-13T19:22:34.0844371Z  finished in 5.675
2019-12-13T19:22:34.1017702Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-13T19:22:34.3221139Z 
2019-12-13T19:22:34.3222001Z running 166 tests
2019-12-13T19:22:36.9634219Z iiii......i........ii..iiii...i.............................i..i..................i....i............ 100/166
2019-12-13T19:22:38.7144891Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-13T19:22:38.7148114Z 
2019-12-13T19:22:38.7152065Z  finished in 4.613
2019-12-13T19:22:38.7304777Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-13T19:22:38.8819743Z 
---
2019-12-13T19:22:40.5287346Z  finished in 1.798
2019-12-13T19:22:40.5454088Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-13T19:22:40.6906294Z 
2019-12-13T19:22:40.6906527Z running 9 tests
2019-12-13T19:22:40.6907274Z iiiiiiiii
2019-12-13T19:22:40.6907880Z 
2019-12-13T19:22:40.6907932Z  finished in 0.145
2019-12-13T19:22:40.7073236Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-13T19:22:40.8874226Z 
---
2019-12-13T19:22:58.3927573Z  finished in 17.685
2019-12-13T19:22:58.7701154Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-13T19:22:58.9620669Z 
2019-12-13T19:22:58.9621086Z running 124 tests
2019-12-13T19:23:20.0217168Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2019-12-13T19:23:23.6231561Z .i.iii.....iiiiii.....ii
2019-12-13T19:23:23.6234283Z 
2019-12-13T19:23:23.6234575Z  finished in 24.853
2019-12-13T19:23:23.6237867Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-13T19:23:23.6238421Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-12-13T19:32:15.1678974Z .................................................................................................... 400/480
2019-12-13T19:32:23.6098686Z ................................................................................
2019-12-13T19:32:23.6101199Z failures:
2019-12-13T19:32:23.6101485Z 
2019-12-13T19:32:23.6102259Z ---- raw_vec.rs - raw_vec::RawVec<T, A>::double (line 286) stdout ----
2019-12-13T19:32:23.6102939Z error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-12-13T19:32:23.6103268Z  --> raw_vec.rs:288:1
2019-12-13T19:32:23.6103584Z 4 | extern crate alloc;
2019-12-13T19:32:23.6103707Z   | ^^^^^^^^^^^^^^^^^^^
2019-12-13T19:32:23.6103839Z 
2019-12-13T19:32:23.6104173Z error: aborting due to previous error
2019-12-13T19:32:23.6104173Z error: aborting due to previous error
2019-12-13T19:32:23.6104285Z 
2019-12-13T19:32:23.6104590Z Couldn't compile the test.
2019-12-13T19:32:23.6104929Z ---- prelude/mod.rs - prelude (line 6) stdout ----
2019-12-13T19:32:23.6105361Z error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-12-13T19:32:23.6105719Z  --> prelude/mod.rs:8:1
2019-12-13T19:32:23.6106019Z 5 | extern crate alloc;
2019-12-13T19:32:23.6106167Z   | ^^^^^^^^^^^^^^^^^^^
2019-12-13T19:32:23.6106279Z 
2019-12-13T19:32:23.6106406Z error: aborting due to previous error
2019-12-13T19:32:23.6106406Z error: aborting due to previous error
2019-12-13T19:32:23.6106537Z 
2019-12-13T19:32:23.6106822Z Couldn't compile the test.
2019-12-13T19:32:23.6107168Z ---- raw_vec.rs - raw_vec::RawVec<T, A>::reserve (line 491) stdout ----
2019-12-13T19:32:23.6107621Z error[E0523]: found two different crates with name `alloc` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
2019-12-13T19:32:23.6107936Z  --> raw_vec.rs:493:1
2019-12-13T19:32:23.6108238Z 4 | extern crate alloc;
2019-12-13T19:32:23.6108365Z   | ^^^^^^^^^^^^^^^^^^^
2019-12-13T19:32:23.6108486Z 
2019-12-13T19:32:23.6108637Z error: aborting due to previous error
---
2019-12-13T19:32:23.6272388Z   local time: Fri Dec 13 19:32:23 UTC 2019
2019-12-13T19:32:23.8997983Z   network time: Fri, 13 Dec 2019 19:32:23 GMT
2019-12-13T19:32:23.9000117Z == end clock drift check ==
2019-12-13T19:32:24.4545425Z 
2019-12-13T19:32:24.4646844Z ##[error]Bash exited with code '1'.
2019-12-13T19:32:24.4694591Z ##[section]Starting: Checkout
2019-12-13T19:32:24.4696655Z ==============================================================================
2019-12-13T19:32:24.4696834Z Task         : Get sources
2019-12-13T19:32:24.4696875Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
