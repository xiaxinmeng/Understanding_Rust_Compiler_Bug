plain
2020-01-11T17:07:41.3277746Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T17:07:41.3367572Z ##[command]git config gc.auto 0
2020-01-11T17:07:41.3467798Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T17:07:41.3545204Z ##[command]git config --get-all http.proxy
2020-01-11T17:07:41.3682369Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66584/merge:refs/remotes/pull/66584/merge
---
2020-01-11T18:03:55.1640843Z .........................................i...............i.......................................... 4900/9516
2020-01-11T18:04:04.1243654Z .................................................................................................... 5000/9516
2020-01-11T18:04:10.4046039Z ....................................................................................i............... 5100/9516
2020-01-11T18:04:15.6301389Z .................................................................................................... 5200/9516
2020-01-11T18:04:25.7345337Z .......................................................ii.ii...........i............................ 5300/9516
2020-01-11T18:04:34.4841427Z .................................................................................................... 5500/9516
2020-01-11T18:04:44.3599801Z .................................................................................................... 5600/9516
2020-01-11T18:04:50.7021014Z ........................................i........................................................... 5700/9516
2020-01-11T18:04:57.0677045Z .................................................................................................... 5800/9516
2020-01-11T18:04:57.0677045Z .................................................................................................... 5800/9516
2020-01-11T18:05:07.6612347Z .................................................................................................... 5900/9516
2020-01-11T18:05:17.6321747Z ...............................ii...i..ii...........i............................................... 6000/9516
2020-01-11T18:05:35.6243088Z .................................................................................................... 6200/9516
2020-01-11T18:05:43.5974285Z .................................................................................................... 6300/9516
2020-01-11T18:05:43.5974285Z .................................................................................................... 6300/9516
2020-01-11T18:05:55.8164940Z ..........................................................i..ii..................................... 6400/9516
2020-01-11T18:06:22.6238540Z .................................................................................................... 6600/9516
2020-01-11T18:06:24.5427705Z .................................i.................................................................. 6700/9516
2020-01-11T18:06:26.6068859Z .................................................................................................... 6800/9516
2020-01-11T18:06:28.9777598Z .................................i.................................................................. 6900/9516
---
2020-01-11T18:08:01.7311899Z .................................................................................................... 7500/9516
2020-01-11T18:08:05.9110123Z .................................................................................................... 7600/9516
2020-01-11T18:08:11.5621227Z .................................................................................................... 7700/9516
2020-01-11T18:08:18.6888871Z .................................................................................................... 7800/9516
2020-01-11T18:08:28.1009490Z ..................................................................................iiii.............. 7900/9516
2020-01-11T18:08:43.8501346Z ................i......i............................................................................ 8100/9516
2020-01-11T18:08:48.8298843Z .................................................................................................... 8200/9516
2020-01-11T18:09:01.9275715Z .................................................................................................... 8300/9516
2020-01-11T18:09:11.6600860Z .................................................................................................... 8400/9516
---
2020-01-11T18:11:31.0119871Z  finished in 6.663
2020-01-11T18:11:31.0301998Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T18:11:31.1878313Z 
2020-01-11T18:11:31.1878426Z running 166 tests
2020-01-11T18:11:34.1280163Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-11T18:11:36.4417049Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-11T18:11:36.4420229Z 
2020-01-11T18:11:36.4422487Z  finished in 5.412
2020-01-11T18:11:36.4642774Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T18:11:36.6210276Z 
---
2020-01-11T18:11:39.1172439Z  finished in 2.069
2020-01-11T18:11:39.1173189Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T18:11:39.1173461Z 
2020-01-11T18:11:39.1173621Z running 9 tests
2020-01-11T18:11:39.1174395Z iiiiiiiii
2020-01-11T18:11:39.1175615Z 
2020-01-11T18:11:39.1175756Z  finished in 0.153
2020-01-11T18:11:39.1176227Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T18:11:39.1176392Z 
---
2020-01-11T18:11:58.2920542Z  finished in 19.568
2020-01-11T18:11:58.3113061Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T18:11:58.4702970Z 
2020-01-11T18:11:59.1253417Z running 124 tests
2020-01-11T18:12:22.1230585Z .iiiii..ii.....i..i...i..i.i.i..i..i..iii....ii.ii....ii..........iiii..........i.....i..ii.......ii 100/124
2020-01-11T18:12:26.0608504Z .i.iii.....iiiiii.....ii
2020-01-11T18:12:26.0609662Z 
2020-01-11T18:12:26.0609888Z  finished in 27.749
2020-01-11T18:12:26.0616449Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-11T18:12:26.0616950Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-01-11T18:24:52.9599246Z 
2020-01-11T18:24:52.9601800Z    Doc-tests core
2020-01-11T18:24:57.3985067Z 
2020-01-11T18:24:57.3986182Z running 2442 tests
2020-01-11T18:25:06.4007083Z ......iiiii......................................................................................... 100/2442
2020-01-11T18:25:15.1091795Z ..................................................................................ii................ 200/2442
2020-01-11T18:25:36.5624826Z ................i................................................................................... 400/2442
2020-01-11T18:25:36.5624826Z ................i................................................................................... 400/2442
2020-01-11T18:25:46.4336733Z .................................................................i..i..................iiii......... 500/2442
2020-01-11T18:26:03.0261973Z .................................................................................................... 700/2442
2020-01-11T18:26:11.5627032Z .................................................................................................... 800/2442
2020-01-11T18:26:20.2680167Z .................................................................................................... 900/2442
2020-01-11T18:26:28.8489678Z .................................................................................................... 1000/2442
---
2020-01-11T18:30:00.6539405Z 
2020-01-11T18:30:00.6540923Z running 1003 tests
2020-01-11T18:30:18.6087733Z i................................................................................................... 100/1003
2020-01-11T18:30:28.4604119Z .................................................................................................... 200/1003
2020-01-11T18:30:35.3509231Z ..................iii......i......i...i......i...................................................... 300/1003
2020-01-11T18:30:40.3186859Z .................................................................................................... 400/1003
2020-01-11T18:30:47.0151382Z ..........................................i..i.....................................ii............... 500/1003
2020-01-11T18:30:59.4372172Z .................................................................................................... 700/1003
2020-01-11T18:30:59.4372172Z .................................................................................................... 700/1003
2020-01-11T18:31:05.8473903Z .............................iiii................................................................... 800/1003
2020-01-11T18:31:19.6151899Z .................................................................................................... 900/1003
2020-01-11T18:31:26.4625820Z ...................................................iiii............................................. 1000/1003
2020-01-11T18:31:26.5578204Z test result: ok. 983 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-01-11T18:31:26.5578471Z 
2020-01-11T18:31:26.5687545Z  finished in 167.619
2020-01-11T18:31:26.5701526Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-01-11T18:48:29.1381698Z Rustbook (x86_64-unknown-linux-gnu) - edition-guide
2020-01-11T18:48:29.5401682Z Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
2020-01-11T18:48:29.6975223Z    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
2020-01-11T18:48:31.1317550Z     Finished release [optimized] target(s) in 1.58s
2020-01-11T18:48:36.8856359Z std/net/enum.IpAddr.html:35: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.8857767Z std/net/enum.IpAddr.html:47: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.8858711Z std/net/enum.IpAddr.html:59: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.8859438Z std/net/enum.IpAddr.html:71: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.8860023Z std/net/enum.IpAddr.html:86: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.8860545Z std/net/enum.IpAddr.html:96: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.8861300Z std/net/enum.IpAddr.html:106: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9118851Z std/net/struct.Ipv4Addr.html:53: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9122802Z std/net/struct.Ipv4Addr.html:73: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9124847Z std/net/struct.Ipv4Addr.html:94: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9125753Z std/net/struct.Ipv4Addr.html:106: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9126411Z std/net/struct.Ipv4Addr.html:171: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9126953Z std/net/struct.Ipv4Addr.html:183: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9127532Z std/net/struct.Ipv4Addr.html:204: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9128060Z std/net/struct.Ipv4Addr.html:218: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9129136Z std/net/struct.Ipv4Addr.html:240: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9129736Z std/net/struct.Ipv4Addr.html:253: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9130244Z std/net/struct.Ipv4Addr.html:264: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9130712Z std/net/struct.Ipv4Addr.html:282: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9131203Z std/net/struct.Ipv4Addr.html:295: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9131667Z std/net/struct.Ipv4Addr.html:306: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9161690Z std/net/struct.Ipv6Addr.html:48: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9167121Z std/net/struct.Ipv6Addr.html:59: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9168251Z std/net/struct.Ipv6Addr.html:70: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9168958Z std/net/struct.Ipv6Addr.html:87: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9170901Z std/net/struct.Ipv6Addr.html:98: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9172410Z std/net/struct.Ipv6Addr.html:136: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9173085Z std/net/struct.Ipv6Addr.html:174: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9173641Z std/net/struct.Ipv6Addr.html:198: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9174162Z std/net/struct.Ipv6Addr.html:210: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9174676Z std/net/struct.Ipv6Addr.html:233: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9175322Z std/net/struct.Ipv6Addr.html:246: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9176091Z std/net/struct.Ipv6Addr.html:257: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:36.9176558Z std/net/struct.Ipv6Addr.html:272: broken link fragment `#ip-addresses-helpers-stability-guarantees` pointing to `std/net/index.html`
2020-01-11T18:48:39.4459819Z thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:43:9
2020-01-11T18:48:39.4505418Z 
2020-01-11T18:48:39.4505922Z 
2020-01-11T18:48:39.4506724Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
2020-01-11T18:48:39.4507028Z expected success, got: exit code: 101
---
2020-01-11T18:48:39.4586051Z   local time: Sat Jan 11 18:48:39 UTC 2020
2020-01-11T18:48:39.7500817Z   network time: Sat, 11 Jan 2020 18:48:39 GMT
2020-01-11T18:48:39.7504636Z == end clock drift check ==
2020-01-11T18:48:41.4090344Z 
2020-01-11T18:48:41.4200104Z ##[error]Bash exited with code '1'.
2020-01-11T18:48:41.4268386Z ##[section]Starting: Checkout
2020-01-11T18:48:41.4269750Z ==============================================================================
2020-01-11T18:48:41.4269795Z Task         : Get sources
2020-01-11T18:48:41.4269845Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
