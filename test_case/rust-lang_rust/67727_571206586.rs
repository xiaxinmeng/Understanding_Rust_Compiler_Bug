plain
2020-01-06T15:09:16.3887441Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T15:09:16.4089777Z ##[command]git config gc.auto 0
2020-01-06T15:09:16.4165842Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T15:09:16.4224476Z ##[command]git config --get-all http.proxy
2020-01-06T15:09:16.4386734Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67727/merge:refs/remotes/pull/67727/merge
---
2020-01-06T16:03:47.9903483Z .................................................................................................... 1500/9477
2020-01-06T16:03:53.8423549Z .................................................................................................... 1600/9477
2020-01-06T16:03:58.7239245Z .................................................................................................... 1700/9477
2020-01-06T16:04:08.0724491Z .................................................................................................... 1800/9477
2020-01-06T16:04:16.2894378Z .i.................................................................................................. 1900/9477
2020-01-06T16:04:22.9996130Z .........................................................................................iiiii...... 2000/9477
2020-01-06T16:04:45.1315284Z .................................................................................................... 2200/9477
2020-01-06T16:04:47.5066117Z .................................................................................................... 2300/9477
2020-01-06T16:04:49.9167588Z .................................................................................................... 2400/9477
2020-01-06T16:04:55.8792120Z .................................................................................................... 2500/9477
---
2020-01-06T16:07:54.6626609Z .....................i...............i.............................................................. 4900/9477
2020-01-06T16:08:04.3762681Z .................................................................................................... 5000/9477
2020-01-06T16:08:10.3836770Z ..................................................................i................................. 5100/9477
2020-01-06T16:08:18.3516155Z .................................................................................................... 5200/9477
2020-01-06T16:08:26.0114108Z .................................ii.ii...........i.................................................. 5300/9477
2020-01-06T16:08:35.3921262Z .................................................................................................... 5500/9477
2020-01-06T16:08:45.2430103Z .................................................................................................... 5600/9477
2020-01-06T16:08:52.7261967Z .................i.................................................................................. 5700/9477
2020-01-06T16:08:59.1248214Z .................................................................................................... 5800/9477
2020-01-06T16:08:59.1248214Z .................................................................................................... 5800/9477
2020-01-06T16:09:11.1488143Z .................................................................................................... 5900/9477
2020-01-06T16:09:23.0418440Z .......ii...i..ii...........i....................................................................... 6000/9477
2020-01-06T16:09:40.7638902Z .................................................................................................... 6200/9477
2020-01-06T16:09:48.5356576Z .................................................................................................... 6300/9477
2020-01-06T16:09:48.5356576Z .................................................................................................... 6300/9477
2020-01-06T16:10:02.6514263Z ..................................i..ii............................................................. 6400/9477
2020-01-06T16:10:23.8052838Z .................................................................................................... 6600/9477
2020-01-06T16:10:25.9740637Z .........i.......................................................................................... 6700/9477
2020-01-06T16:10:28.4240401Z .................................................................................................... 6800/9477
2020-01-06T16:10:31.0829847Z .........i.......................................................................................... 6900/9477
---
2020-01-06T16:12:08.9567430Z .................................................................................................... 7500/9477
2020-01-06T16:12:13.0601849Z .................................................................................................... 7600/9477
2020-01-06T16:12:18.2396419Z .................................................................................................... 7700/9477
2020-01-06T16:12:28.8536409Z .................................................................................................... 7800/9477
2020-01-06T16:12:37.2361113Z .............................................iiii................................................... 7900/9477
2020-01-06T16:12:52.0672397Z .................................................................................................... 8100/9477
2020-01-06T16:12:58.6182976Z .................................................................................................... 8200/9477
2020-01-06T16:13:14.0010464Z .................................................................................................... 8300/9477
2020-01-06T16:13:21.6208889Z .................................................................................................... 8400/9477
---
2020-01-06T16:15:44.5857093Z  finished in 6.667
2020-01-06T16:15:44.6035963Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T16:15:44.7589020Z 
2020-01-06T16:15:44.7589272Z running 166 tests
2020-01-06T16:15:47.9903679Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-06T16:15:50.2034806Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-06T16:15:50.2035583Z 
2020-01-06T16:15:50.2042033Z  finished in 5.600
2020-01-06T16:15:50.2222774Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T16:15:50.3808490Z 
---
2020-01-06T16:15:52.3161564Z  finished in 2.094
2020-01-06T16:15:52.3334349Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T16:15:52.4862711Z 
2020-01-06T16:15:52.4862963Z running 9 tests
2020-01-06T16:15:52.4863735Z iiiiiiiii
2020-01-06T16:15:52.4864126Z 
2020-01-06T16:15:52.4868798Z  finished in 0.153
2020-01-06T16:15:52.5039040Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T16:15:52.6680798Z 
---
2020-01-06T16:16:12.4817298Z  finished in 19.977
2020-01-06T16:16:12.5011114Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T16:16:12.6697124Z 
2020-01-06T16:16:12.6698015Z running 124 tests
2020-01-06T16:16:38.1776843Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-06T16:16:42.5052979Z .i.iii.....iiiiii.....ii
2020-01-06T16:16:42.5054414Z 
2020-01-06T16:16:42.5058511Z  finished in 30.004
2020-01-06T16:16:42.5065218Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T16:16:42.5066111Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-06T16:27:23.9416502Z   local time: Mon Jan  6 16:27:23 UTC 2020
2020-01-06T16:27:24.2387723Z   network time: Mon, 06 Jan 2020 16:27:24 GMT
2020-01-06T16:27:24.2392159Z == end clock drift check ==
2020-01-06T16:27:25.7163648Z 
2020-01-06T16:27:25.7285013Z ##[error]Bash exited with code '1'.
2020-01-06T16:27:25.7326927Z ##[section]Starting: Checkout
2020-01-06T16:27:25.7328911Z ==============================================================================
2020-01-06T16:27:25.7328993Z Task         : Get sources
2020-01-06T16:27:25.7329046Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
