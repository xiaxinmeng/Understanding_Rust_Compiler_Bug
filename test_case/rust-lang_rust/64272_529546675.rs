plain
2019-09-09T14:24:26.4387585Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T14:24:26.4598834Z ##[command]git config gc.auto 0
2019-09-09T14:24:26.4661175Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T14:24:26.4730660Z ##[command]git config --get-all http.proxy
2019-09-09T14:24:26.4861092Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64272/merge:refs/remotes/pull/64272/merge
---
2019-09-09T15:22:01.6115346Z .................................................................................................... 1500/9009
2019-09-09T15:22:07.1184140Z .................................................................................................... 1600/9009
2019-09-09T15:22:18.9530391Z ......................................................i...............i............................. 1700/9009
2019-09-09T15:22:26.3595594Z .................................................................................................... 1800/9009
2019-09-09T15:22:39.7894966Z .............................................iiiii.................................................. 1900/9009
2019-09-09T15:22:49.9842879Z .................................................................................................... 2100/9009
2019-09-09T15:22:52.3385948Z .................................................................................................... 2200/9009
2019-09-09T15:22:55.7118244Z .................................................................................................... 2300/9009
2019-09-09T15:23:03.0444979Z .................................................................................................... 2400/9009
---
2019-09-09T15:25:48.5666908Z ...................................i...............i................................................ 4700/9009
2019-09-09T15:25:59.2394946Z .................................................................................................... 4800/9009
2019-09-09T15:26:05.1863035Z .................................................................................................... 4900/9009
2019-09-09T15:26:15.0079986Z .................................................................................................... 5000/9009
2019-09-09T15:26:20.6727257Z .................ii.ii.............................................................................. 5100/9009
2019-09-09T15:26:30.3886623Z .................................................................................................... 5300/9009
2019-09-09T15:26:39.8372162Z ................................................................................i................... 5400/9009
2019-09-09T15:26:47.1618705Z .................................................................................................... 5500/9009
2019-09-09T15:26:52.6726075Z .................................................................................................... 5600/9009
2019-09-09T15:26:52.6726075Z .................................................................................................... 5600/9009
2019-09-09T15:27:02.6258012Z ..........................................................................ii...i..ii...........i.... 5700/9009
2019-09-09T15:27:26.0378840Z .................................................................................................... 5900/9009
2019-09-09T15:27:32.9298862Z .................................................................................................... 6000/9009
2019-09-09T15:27:32.9298862Z .................................................................................................... 6000/9009
2019-09-09T15:27:37.7852710Z ............................................................................i..ii................... 6100/9009
2019-09-09T15:28:05.1068504Z .................................................................................................... 6300/9009
2019-09-09T15:28:07.0502387Z ...................................i................................................................ 6400/9009
2019-09-09T15:28:09.0279071Z .................................................................................................... 6500/9009
2019-09-09T15:28:11.2831406Z .......i............................................................................................ 6600/9009
---
2019-09-09T15:32:34.8032824Z  finished in 18.378
2019-09-09T15:32:34.8203570Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:32:34.9751856Z 
2019-09-09T15:32:34.9752744Z running 150 tests
2019-09-09T15:32:37.9604209Z i....iii......iii..iiii....i..............................i.i..................i....i.........ii.i.i 100/150
2019-09-09T15:32:39.8057058Z ..iiii..............i.........iii.i.......ii......
2019-09-09T15:32:39.8058732Z 
2019-09-09T15:32:39.8075581Z  finished in 4.986
2019-09-09T15:32:39.8244012Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:32:39.9712724Z 
---
2019-09-09T15:32:41.9282228Z  finished in 2.103
2019-09-09T15:32:41.9470340Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:32:42.0908325Z 
2019-09-09T15:32:42.0909530Z running 9 tests
2019-09-09T15:32:42.0911207Z iiiiiiiii
2019-09-09T15:32:42.0911729Z 
2019-09-09T15:32:42.0913330Z  finished in 0.144
2019-09-09T15:32:42.1078742Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:32:42.2504295Z 
---
2019-09-09T15:32:59.2386651Z  finished in 17.130
2019-09-09T15:32:59.2557941Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:32:59.3984102Z 
2019-09-09T15:32:59.3984249Z running 123 tests
2019-09-09T15:33:20.9431916Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-09-09T15:33:25.1507974Z i.i.i......iii.i.....ii
2019-09-09T15:33:25.1509694Z 
2019-09-09T15:33:25.1513504Z  finished in 25.895
2019-09-09T15:33:25.1520271Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:33:25.1521515Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-09-09T15:46:08.4937742Z .................................................................................................... 100/977
2019-09-09T15:46:08.5074881Z .................................................................................................... 200/977
2019-09-09T15:46:08.5165102Z .................................................................................................... 300/977
2019-09-09T15:46:08.5297921Z .................................................................................................... 400/977
2019-09-09T15:46:08.6922143Z ..ii................................................................................................ 500/977
2019-09-09T15:46:08.7185107Z .................................................................................................... 700/977
2019-09-09T15:46:08.7358919Z .................................................................................................... 800/977
2019-09-09T15:46:09.2098591Z .................................................................................................... 900/977
2019-09-09T15:46:10.1755493Z .............................................................................
2019-09-09T15:46:10.1755493Z .............................................................................
2019-09-09T15:46:10.1755734Z test result: ok. 975 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
2019-09-09T15:46:10.1757201Z 
2019-09-09T15:46:10.1758154Z    Doc-tests core
2019-09-09T15:46:14.8453128Z 
2019-09-09T15:46:14.8454445Z running 2400 tests
2019-09-09T15:46:25.1103652Z ......iiiii......................................................................................... 100/2400
2019-09-09T15:46:34.8863661Z ...........................................................................ii....................... 200/2400
2019-09-09T15:46:45.9810427Z .................................................................................................i.. 300/2400
2019-09-09T15:46:58.1418701Z .................................................................................................... 400/2400
2019-09-09T15:47:07.6733089Z ............................................i..i.................iiii............................... 500/2400
2019-09-09T15:47:26.2146531Z .................................................................................................... 700/2400
2019-09-09T15:47:35.7428616Z .................................................................................................... 800/2400
2019-09-09T15:47:45.2076038Z .................................................................................................... 900/2400
2019-09-09T15:47:54.6617835Z .................................................................................................... 1000/2400
---
2019-09-09T15:52:30.0388379Z 
2019-09-09T15:52:30.0388675Z running 991 tests
2019-09-09T15:52:49.3432049Z i................................................................................................... 100/991
2019-09-09T15:52:59.8666626Z .................................................................................................... 200/991
2019-09-09T15:53:07.4350089Z .................iii......i......i...i......i....................................................... 300/991
2019-09-09T15:53:12.6263414Z .................................................................................................... 400/991
2019-09-09T15:53:19.9436860Z ..................................i..i.................................ii........................... 500/991
2019-09-09T15:53:33.6694345Z .................................................................................................... 700/991
2019-09-09T15:53:33.6694345Z .................................................................................................... 700/991
2019-09-09T15:53:41.1236004Z .................iiii............................................................................... 800/991
2019-09-09T15:53:55.0593774Z .................................................................................................... 900/991
2019-09-09T15:54:01.9959186Z .......................................iiii................................................
2019-09-09T15:54:01.9960522Z 
2019-09-09T15:54:02.0013288Z  finished in 225.263
2019-09-09T15:54:02.0015411Z Testing term stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-09T15:54:02.2774194Z    Compiling term v0.0.0 (/checkout/src/libterm)
---
2019-09-09T15:57:08.6456361Z    --> src/libsyntax/tests.rs:143:23
2019-09-09T15:57:08.6457109Z     |
2019-09-09T15:57:08.6457633Z 143 |           let emitter = EmitterWriter::new(
2019-09-09T15:57:08.6458078Z     |  _______________________^
2019-09-09T15:57:08.6458613Z 144 | |             Box::new(Shared { data: output.clone() }),
2019-09-09T15:57:08.6459210Z 145 | |             Some(source_map.clone()),
2019-09-09T15:57:08.6460468Z ...   |
2019-09-09T15:57:08.6460841Z 149 | |             None,
2019-09-09T15:57:08.6461251Z 150 | |         );
2019-09-09T15:57:08.6461640Z     | |_________^ expected 7 parameters
2019-09-09T15:57:08.6461640Z     | |_________^ expected 7 parameters
2019-09-09T15:57:08.6461775Z 
2019-09-09T15:57:11.3969353Z error[E0061]: this function takes 7 parameters but 6 parameters were supplied
2019-09-09T15:57:11.3970691Z   --> src/libsyntax/parse/lexer/tests.rs:14:19
2019-09-09T15:57:11.3971195Z    |
2019-09-09T15:57:11.3971717Z 14 |       let emitter = EmitterWriter::new(
2019-09-09T15:57:11.3972488Z    |  ___________________^
2019-09-09T15:57:11.3972963Z 15 | |         Box::new(io::sink()),
2019-09-09T15:57:11.3973419Z 16 | |         Some(sm.clone()),
2019-09-09T15:57:11.3974496Z ...  |
2019-09-09T15:57:11.3975031Z 20 | |         None,
2019-09-09T15:57:11.3975703Z 21 | |     );
2019-09-09T15:57:11.3976169Z    | |_____^ expected 7 parameters
---
2019-09-09T15:57:12.9313411Z 
2019-09-09T15:57:12.9313863Z To learn more, run the command again with --verbose.
2019-09-09T15:57:12.9322044Z 
2019-09-09T15:57:12.9322408Z 
2019-09-09T15:57:12.9323301Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
2019-09-09T15:57:12.9323453Z 
2019-09-09T15:57:12.9323481Z 
2019-09-09T15:57:12.9334545Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-09T15:57:12.9334629Z Build completed unsuccessfully in 1:25:48
2019-09-09T15:57:12.9334629Z Build completed unsuccessfully in 1:25:48
2019-09-09T15:57:12.9384581Z == clock drift check ==
2019-09-09T15:57:12.9405086Z   local time: Mon Sep  9 15:57:12 UTC 2019
2019-09-09T15:57:13.0961705Z   network time: Mon, 09 Sep 2019 15:57:13 GMT
2019-09-09T15:57:13.0967522Z == end clock drift check ==
2019-09-09T15:57:13.6661908Z ##[error]Bash exited with code '1'.
2019-09-09T15:57:13.6700929Z ##[section]Starting: Checkout
2019-09-09T15:57:13.6702469Z ==============================================================================
2019-09-09T15:57:13.6702524Z Task         : Get sources
2019-09-09T15:57:13.6702577Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
