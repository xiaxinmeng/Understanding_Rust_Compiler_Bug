plain
2019-10-12T15:58:16.9677659Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T15:58:16.9778149Z ##[command]git config gc.auto 0
2019-10-12T15:58:16.9873690Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T15:58:16.9944031Z ##[command]git config --get-all http.proxy
2019-10-12T15:58:17.0111688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65339/merge:refs/remotes/pull/65339/merge
---
2019-10-12T17:03:49.0495781Z .................................................................................................... 1600/9171
2019-10-12T17:03:56.1237039Z .................................................................................................... 1700/9171
2019-10-12T17:04:09.9413747Z ......................i...............i............................................................. 1800/9171
2019-10-12T17:04:17.9949364Z .................................................................................................... 1900/9171
2019-10-12T17:04:34.1317387Z .............iiiii.................................................................................. 2000/9171
2019-10-12T17:04:45.8677153Z .................................................................................................... 2200/9171
2019-10-12T17:04:48.9054161Z .................................................................................................... 2300/9171
2019-10-12T17:04:55.2471119Z .................................................................................................... 2400/9171
2019-10-12T17:05:19.7659780Z .................................................................................................... 2500/9171
---
2019-10-12T17:08:38.5114319Z ....................i...............i............................................................... 4800/9171
2019-10-12T17:08:51.3068266Z .................................................................................................... 4900/9171
2019-10-12T17:08:58.3766342Z .................................................................................................... 5000/9171
2019-10-12T17:09:10.0144097Z .................................................................................................... 5100/9171
2019-10-12T17:09:16.9270639Z ....................ii.ii........................................................................... 5200/9171
2019-10-12T17:09:28.7267155Z .................................................................................................... 5400/9171
2019-10-12T17:09:40.2842489Z ......................................................................................i............. 5500/9171
2019-10-12T17:09:48.9345521Z .................................................................................................... 5600/9171
2019-10-12T17:09:56.2888459Z .................................................................................................... 5700/9171
2019-10-12T17:09:56.2888459Z .................................................................................................... 5700/9171
2019-10-12T17:10:07.3231511Z ...................................................................................ii...i..ii....... 5800/9171
2019-10-12T17:10:35.5712810Z .................................................................................................... 6000/9171
2019-10-12T17:10:46.6598369Z .................................................................................................... 6100/9171
2019-10-12T17:10:46.6598369Z .................................................................................................... 6100/9171
2019-10-12T17:10:55.4747592Z .........................................................................................i..ii...... 6200/9171
2019-10-12T17:11:27.0535609Z .................................................................................................... 6400/9171
2019-10-12T17:11:31.8651857Z .................................................i.................................................. 6500/9171
2019-10-12T17:11:34.3972219Z .................................................................................................... 6600/9171
2019-10-12T17:11:37.2243734Z ......................i............................................................................. 6700/9171
---
2019-10-12T17:16:45.1278634Z  finished in 6.128
2019-10-12T17:16:45.1501505Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:16:45.3688350Z 
2019-10-12T17:16:45.3688619Z running 153 tests
2019-10-12T17:16:49.0399179Z i....iii......iii..iiii....i.............................i..i..................i....i............ii. 100/153
2019-10-12T17:16:51.1890835Z i.i..iiii..............i.........iii.i.......ii......
2019-10-12T17:16:51.1896399Z 
2019-10-12T17:16:51.1900048Z  finished in 6.039
2019-10-12T17:16:51.2101798Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:16:51.3947596Z 
---
2019-10-12T17:16:53.7152997Z  finished in 2.505
2019-10-12T17:16:53.7360469Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:16:53.9094316Z 
2019-10-12T17:16:53.9096392Z running 9 tests
2019-10-12T17:16:53.9099278Z iiiiiiiii
2019-10-12T17:16:53.9101149Z 
2019-10-12T17:16:53.9101673Z  finished in 0.173
2019-10-12T17:16:53.9306214Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:16:54.1327986Z 
---
2019-10-12T17:17:14.4894256Z  finished in 20.559
2019-10-12T17:17:14.5144262Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:17:14.7266634Z 
2019-10-12T17:17:14.7267844Z running 123 tests
2019-10-12T17:17:41.6945117Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-12T17:17:46.9598706Z i.i.i......iii.i.....ii
2019-10-12T17:17:46.9599953Z 
2019-10-12T17:17:46.9604564Z  finished in 32.446
2019-10-12T17:17:46.9613769Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:17:46.9614441Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-12T17:32:02.9571408Z 
2019-10-12T17:32:02.9572193Z    Doc-tests core
2019-10-12T17:32:08.5327992Z 
2019-10-12T17:32:08.5328291Z running 2405 tests
2019-10-12T17:32:21.4004847Z ......iiiii......................................................................................... 100/2405
2019-10-12T17:32:33.5835406Z ...............................................................................ii................... 200/2405
2019-10-12T17:33:02.6738866Z .i.................................................................................................. 400/2405
2019-10-12T17:33:02.6738866Z .i.................................................................................................. 400/2405
2019-10-12T17:33:14.7494442Z ................................................i..i.................iiii........................... 500/2405
2019-10-12T17:33:37.9450519Z .................................................................................................... 700/2405
2019-10-12T17:33:49.8626367Z .................................................................................................... 800/2405
2019-10-12T17:34:01.6968330Z .................................................................................................... 900/2405
2019-10-12T17:34:13.5755964Z .................................................................................................... 1000/2405
---
2019-10-12T17:38:54.5618951Z 
2019-10-12T17:38:54.5622615Z running 994 tests
2019-10-12T17:39:18.8005675Z i................................................................................................... 100/994
2019-10-12T17:39:32.2853291Z .................................................................................................... 200/994
2019-10-12T17:39:42.0631983Z ...................iii......i......i...i......i..................................................... 300/994
2019-10-12T17:39:48.6190468Z .................................................................................................... 400/994
2019-10-12T17:39:58.1635400Z .....................................i..i.................................ii........................ 500/994
2019-10-12T17:40:15.1025007Z .................................................................................................... 700/994
2019-10-12T17:40:15.1025007Z .................................................................................................... 700/994
2019-10-12T17:40:24.6318920Z ....................iiii............................................................................ 800/994
2019-10-12T17:40:41.5519877Z .................................................................................................... 900/994
2019-10-12T17:40:50.3694706Z ..........................................iiii................................................
2019-10-12T17:40:50.3695054Z 
2019-10-12T17:40:50.3747770Z  finished in 217.260
2019-10-12T17:40:50.3765180Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-12T17:40:50.6228238Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-10-12T17:57:34.8838639Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2019-10-12T17:57:35.2933721Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2019-10-12T17:57:35.4913818Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2019-10-12T17:57:37.7588136Z     Finished release [optimized] target(s) in 2.46s
2019-10-12T17:57:40.3025886Z std/sync/atomic/enum.Ordering.html:22: broken link fragment `#Acquire` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3027965Z std/sync/atomic/enum.Ordering.html:24: broken link fragment `#Acquire` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3028347Z std/sync/atomic/enum.Ordering.html:26: broken link fragment `#Relaxed` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3030008Z std/sync/atomic/enum.Ordering.html:30: broken link fragment `#Release` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3030378Z std/sync/atomic/enum.Ordering.html:34: broken link fragment `#Relaxed` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3030588Z std/sync/atomic/enum.Ordering.html:37: broken link fragment `#Acquire` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3030822Z std/sync/atomic/enum.Ordering.html:37: broken link fragment `#Release` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3031020Z std/sync/atomic/enum.Ordering.html:38: broken link fragment `#Acquire` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3031209Z std/sync/atomic/enum.Ordering.html:38: broken link fragment `#Release` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3031424Z std/sync/atomic/enum.Ordering.html:40: broken link fragment `#Acquire` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3031612Z std/sync/atomic/enum.Ordering.html:41: broken link fragment `#Relaxed` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3031842Z std/sync/atomic/enum.Ordering.html:44: broken link fragment `#Acquire` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3032040Z std/sync/atomic/enum.Ordering.html:44: broken link fragment `#Release` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:40.3032262Z std/sync/atomic/enum.Ordering.html:44: broken link fragment `#AcqRel` pointing to `std/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4022944Z core/sync/atomic/enum.Ordering.html:22: broken link fragment `#Acquire` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4023152Z core/sync/atomic/enum.Ordering.html:24: broken link fragment `#Acquire` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4023230Z core/sync/atomic/enum.Ordering.html:26: broken link fragment `#Relaxed` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4024799Z core/sync/atomic/enum.Ordering.html:30: broken link fragment `#Release` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4025162Z core/sync/atomic/enum.Ordering.html:34: broken link fragment `#Relaxed` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4025439Z core/sync/atomic/enum.Ordering.html:37: broken link fragment `#Acquire` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4025962Z core/sync/atomic/enum.Ordering.html:37: broken link fragment `#Release` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4026479Z core/sync/atomic/enum.Ordering.html:38: broken link fragment `#Acquire` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4026766Z core/sync/atomic/enum.Ordering.html:38: broken link fragment `#Release` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4027034Z core/sync/atomic/enum.Ordering.html:40: broken link fragment `#Acquire` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4027292Z core/sync/atomic/enum.Ordering.html:41: broken link fragment `#Relaxed` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4027744Z core/sync/atomic/enum.Ordering.html:44: broken link fragment `#Acquire` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4028019Z core/sync/atomic/enum.Ordering.html:44: broken link fragment `#Release` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:43.4028314Z core/sync/atomic/enum.Ordering.html:44: broken link fragment `#AcqRel` pointing to `core/sync/atomic/enum.Ordering.html`
2019-10-12T17:57:46.5747152Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
2019-10-12T17:57:46.5768300Z 
2019-10-12T17:57:46.5768657Z 
2019-10-12T17:57:46.5769950Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2019-10-12T17:57:46.5770607Z expected success, got: exit code: 101
---
2019-10-12T17:57:46.5831599Z == clock drift check ==
2019-10-12T17:57:46.5855903Z   local time: Sat Oct 12 17:57:46 UTC 2019
2019-10-12T17:57:46.6847499Z   network time: Sat, 12 Oct 2019 17:57:46 GMT
2019-10-12T17:57:46.6848400Z == end clock drift check ==
2019-10-12T17:57:48.2463506Z ##[error]Bash exited with code '1'.
2019-10-12T17:57:48.2516255Z ##[section]Starting: Checkout
2019-10-12T17:57:48.2517996Z ==============================================================================
2019-10-12T17:57:48.2518064Z Task         : Get sources
2019-10-12T17:57:48.2518109Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
