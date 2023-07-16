plain
2019-11-12T21:28:30.0318069Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T21:28:30.0466246Z ##[command]git config gc.auto 0
2019-11-12T21:28:30.0518405Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T21:28:30.0545357Z ##[command]git config --get-all http.proxy
2019-11-12T21:28:30.0751487Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66346/merge:refs/remotes/pull/66346/merge
---
2019-11-12T22:23:52.4491976Z .................................................................................................... 1500/9234
2019-11-12T22:23:58.4306159Z .................................................................................................... 1600/9234
2019-11-12T22:24:07.3465799Z .................................................................................................... 1700/9234
2019-11-12T22:24:15.5558934Z ...i................................................................................................ 1800/9234
2019-11-12T22:24:22.1204652Z .......................................................................................iiiii........ 1900/9234
2019-11-12T22:24:42.7563285Z .................................................................................................... 2100/9234
2019-11-12T22:24:45.1528340Z .................................................................................................... 2200/9234
2019-11-12T22:24:47.5176696Z .................................................................................................... 2300/9234
2019-11-12T22:24:56.8979643Z .................................................................................................... 2400/9234
---
2019-11-12T22:27:41.7438504Z ....................................................................................i............... 4700/9234
2019-11-12T22:27:48.7817283Z i................................................................................................... 4800/9234
2019-11-12T22:27:57.5832433Z .................................................................................................... 4900/9234
2019-11-12T22:28:02.7300998Z .................................................................................................... 5000/9234
2019-11-12T22:28:13.5857649Z .......................................................................................ii.ii........ 5100/9234
2019-11-12T22:28:21.6769799Z ......................i............................................................................. 5300/9234
2019-11-12T22:28:29.5406239Z .................................................................................................... 5400/9234
2019-11-12T22:28:37.7469917Z .....................................................................i.............................. 5500/9234
2019-11-12T22:28:44.6345960Z .................................................................................................... 5600/9234
2019-11-12T22:28:44.6345960Z .................................................................................................... 5600/9234
2019-11-12T22:28:51.5493354Z .................................................................................................... 5700/9234
2019-11-12T22:29:00.9381786Z .......................................................ii...i..ii...........i....................... 5800/9234
2019-11-12T22:29:21.6548360Z .................................................................................................... 6000/9234
2019-11-12T22:29:28.8186456Z .................................................................................................... 6100/9234
2019-11-12T22:29:28.8186456Z .................................................................................................... 6100/9234
2019-11-12T22:29:33.5029709Z ..........................................................................i..ii..................... 6200/9234
2019-11-12T22:30:01.3056353Z .................................................................................................... 6400/9234
2019-11-12T22:30:03.5281132Z ..........................................i......................................................... 6500/9234
2019-11-12T22:30:05.6914655Z .................................................................................................... 6600/9234
2019-11-12T22:30:08.0059640Z ..........................i......................................................................... 6700/9234
---
2019-11-12T22:35:01.6884524Z  finished in 5.298
2019-11-12T22:35:01.7053098Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T22:35:02.5734758Z 
2019-11-12T22:35:02.5734882Z running 156 tests
2019-11-12T22:35:04.6059353Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/156
2019-11-12T22:35:07.0678247Z .i.i..iiii..............i.........iii.i.........ii......
2019-11-12T22:35:07.0730216Z 
2019-11-12T22:35:07.0730285Z  finished in 4.646
2019-11-12T22:35:07.0730567Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T22:35:07.0730601Z 
---
2019-11-12T22:35:08.3563880Z  finished in 1.986
2019-11-12T22:35:08.3756849Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T22:35:08.5109366Z 
2019-11-12T22:35:08.5109556Z running 9 tests
2019-11-12T22:35:08.5113593Z iiiiiiiii
2019-11-12T22:35:08.5113940Z 
2019-11-12T22:35:08.5116813Z  finished in 0.135
2019-11-12T22:35:08.5275754Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T22:35:09.0720931Z 
---
2019-11-12T22:35:27.2350827Z  finished in 18.707
2019-11-12T22:35:27.2528371Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T22:35:27.3961649Z 
2019-11-12T22:35:27.3961827Z running 123 tests
2019-11-12T22:35:49.4155221Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-11-12T22:35:53.6709360Z i.i.i......iii.i.....ii
2019-11-12T22:35:53.6710489Z 
2019-11-12T22:35:53.6714745Z  finished in 26.418
2019-11-12T22:35:53.6723950Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-12T22:35:53.6725054Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-11-12T22:46:46.6218696Z 
2019-11-12T22:46:46.6219505Z    Doc-tests core
2019-11-12T22:46:51.2628388Z 
2019-11-12T22:46:51.2635162Z running 2418 tests
2019-11-12T22:47:01.7542786Z ......iiiii......................................................................................... 100/2418
2019-11-12T22:47:11.8221865Z ................................................................................ii.................. 200/2418
2019-11-12T22:47:35.2020647Z ..i................................................................................................. 400/2418
2019-11-12T22:47:35.2020647Z ..i................................................................................................. 400/2418
2019-11-12T22:47:44.9823712Z ..................................................i..i.................iiii......................... 500/2418
2019-11-12T22:48:03.3067027Z .................................................................................................... 700/2418
2019-11-12T22:48:12.7410564Z .................................................................................................... 800/2418
2019-11-12T22:48:22.4315879Z .................................................................................................... 900/2418
2019-11-12T22:48:32.2561908Z .................................................................................................... 1000/2418
---
2019-11-12T22:52:30.2671846Z 
2019-11-12T22:52:30.2673400Z running 1000 tests
2019-11-12T22:52:50.0361008Z i................................................................................................... 100/1000
2019-11-12T22:53:01.0401558Z .................................................................................................... 200/1000
2019-11-12T22:53:08.6753267Z ...................iii......i......i...i......i..................................................... 300/1000
2019-11-12T22:53:13.7502609Z .................................................................................................... 400/1000
2019-11-12T22:53:20.9165499Z ...........................................i..i.................................ii.................. 500/1000
2019-11-12T22:53:34.1491485Z .................................................................................................... 700/1000
2019-11-12T22:53:34.1491485Z .................................................................................................... 700/1000
2019-11-12T22:53:41.0675399Z ..........................iiii...................................................................... 800/1000
2019-11-12T22:53:55.4168539Z ................................................................................................FFF. 900/1000
2019-11-12T22:54:02.1124060Z FFFF.FF.FFFFFFFFFFFFFF.FFF.FFFFFF.FF.FFFFFFFFFF.iiii................................................ 1000/1000
2019-11-12T22:54:02.1129179Z failures:
2019-11-12T22:54:02.1129208Z 
2019-11-12T22:54:02.1148432Z ---- sys/unix/ext/mod.rs - sys::unix::ext (line 14) stdout ----
2019-11-12T22:54:02.1148432Z ---- sys/unix/ext/mod.rs - sys::unix::ext (line 14) stdout ----
2019-11-12T22:54:02.1148739Z error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
2019-11-12T22:54:02.1149038Z  --> sys/unix/ext/mod.rs:19:13
2019-11-12T22:54:02.1149993Z   |
2019-11-12T22:54:02.1150049Z 7 |     let f = File::create("foo.txt")?;
2019-11-12T22:54:02.1150094Z   |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
2019-11-12T22:54:02.1150187Z   = help: the trait `std::ops::Try` is not implemented for `()`
2019-11-12T22:54:02.1150228Z   = note: required by `std::ops::Try::from_error`
2019-11-12T22:54:02.1150255Z 
2019-11-12T22:54:02.1150304Z error: aborting due to previous error
---
2019-11-12T22:54:02.1154538Z    |
2019-11-12T22:54:02.1154800Z 10 |   fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1155285Z    |                ------------------- expected `std::result::Result<(), std::io::Error>` because of return type
2019-11-12T22:54:02.1155343Z ...
2019-11-12T22:54:02.1155407Z 13 | /     for stream in listener.incoming() {
2019-11-12T22:54:02.1155453Z 14 | |         match stream {
2019-11-12T22:54:02.1155500Z 15 | |             Ok(stream) => {
2019-11-12T22:54:02.1155569Z 16 | |                 thread::spawn(|| handle_client(stream));
2019-11-12T22:54:02.1155671Z 21 | |         }
2019-11-12T22:54:02.1155860Z 22 | |     }
2019-11-12T22:54:02.1155924Z    | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1155968Z    |
---
2019-11-12T22:54:02.1157960Z   |
2019-11-12T22:54:02.1158164Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1158714Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1158779Z   |    |
2019-11-12T22:54:02.1158823Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1158940Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1159150Z              found type `()`
2019-11-12T22:54:02.1159198Z 
2019-11-12T22:54:02.1159239Z error: aborting due to previous error
---
2019-11-12T22:54:02.1160896Z   |
2019-11-12T22:54:02.1161107Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1161335Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1161377Z   |    |
2019-11-12T22:54:02.1161416Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1161759Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1161798Z              found type `()`
2019-11-12T22:54:02.1161823Z 
2019-11-12T22:54:02.1161876Z error: aborting due to previous error
---
2019-11-12T22:54:02.1163429Z   |
2019-11-12T22:54:02.1163617Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1163841Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1163899Z   |    |
2019-11-12T22:54:02.1163949Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1164030Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1164091Z              found type `()`
2019-11-12T22:54:02.1164118Z 
2019-11-12T22:54:02.1164157Z error: aborting due to previous error
---
2019-11-12T22:54:02.1165192Z   |
2019-11-12T22:54:02.1165411Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1166406Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1166561Z   |    |
2019-11-12T22:54:02.1166619Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1166697Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1166735Z              found type `()`
2019-11-12T22:54:02.1166775Z 
2019-11-12T22:54:02.1166811Z error: aborting due to previous error
---
2019-11-12T22:54:02.1207032Z   |
2019-11-12T22:54:02.1207267Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1207506Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1207560Z   |    |
2019-11-12T22:54:02.1207601Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1207699Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1207739Z              found type `()`
2019-11-12T22:54:02.1207767Z 
2019-11-12T22:54:02.1207821Z error: aborting due to previous error
2019-11-12T22:54:02.1207821Z error: aborting due to previous error
2019-11-12T22:54:02.1207847Z 
2019-11-12T22:54:02.1208080Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1208264Z Couldn't compile the test.
2019-11-12T22:54:02.1208517Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixDatagram::connect (line 1151) stdout ----
2019-11-12T22:54:02.1208573Z error[E0069]: `return;` in a function whose return type is not `()`
2019-11-12T22:54:02.1208766Z   --> sys/unix/ext/net.rs:1160:13
2019-11-12T22:54:02.1209010Z 5  | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1209376Z    |              ------------------- expected `std::io::Result<()>` because of this return type
2019-11-12T22:54:02.1209437Z ...
2019-11-12T22:54:02.1209474Z 11 |             return
2019-11-12T22:54:02.1209474Z 11 |             return
2019-11-12T22:54:02.1209515Z    |             ^^^^^^ return type is not `()`
2019-11-12T22:54:02.1209597Z error[E0308]: mismatched types
2019-11-12T22:54:02.1209787Z  --> sys/unix/ext/net.rs:1154:14
2019-11-12T22:54:02.1209826Z   |
2019-11-12T22:54:02.1210027Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1210027Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1210256Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1210298Z   |    |
2019-11-12T22:54:02.1210365Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1210445Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1210484Z              found type `()`
2019-11-12T22:54:02.1210526Z 
2019-11-12T22:54:02.1210570Z error: aborting due to 2 previous errors
---
2019-11-12T22:54:02.1211578Z   |
2019-11-12T22:54:02.1211775Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1212191Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1212235Z   |    |
2019-11-12T22:54:02.1212365Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1212674Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1212741Z              found type `()`
2019-11-12T22:54:02.1212769Z 
2019-11-12T22:54:02.1212807Z error: aborting due to previous error
---
2019-11-12T22:54:02.1213928Z   |
2019-11-12T22:54:02.1214127Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1214463Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1214515Z   |    |
2019-11-12T22:54:02.1214559Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1214665Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1214707Z              found type `()`
2019-11-12T22:54:02.1214733Z 
2019-11-12T22:54:02.1214791Z error: aborting due to previous error
---
2019-11-12T22:54:02.1215984Z   |
2019-11-12T22:54:02.1216160Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1216377Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1216434Z   |    |
2019-11-12T22:54:02.1216474Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1216645Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1216685Z              found type `()`
2019-11-12T22:54:02.1216711Z 
2019-11-12T22:54:02.1216748Z error: aborting due to previous error
---
2019-11-12T22:54:02.1217726Z   |
2019-11-12T22:54:02.1217933Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1218162Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1218205Z   |    |
2019-11-12T22:54:02.1218262Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1218347Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1218404Z              found type `()`
2019-11-12T22:54:02.1218430Z 
2019-11-12T22:54:02.1218466Z error: aborting due to previous error
2019-11-12T22:54:02.1218466Z error: aborting due to previous error
2019-11-12T22:54:02.1218490Z 
2019-11-12T22:54:02.1218722Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1218904Z Couldn't compile the test.
2019-11-12T22:54:02.1219140Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixDatagram::recv_from (line 1249) stdout ----
2019-11-12T22:54:02.1219199Z error[E0308]: mismatched types
2019-11-12T22:54:02.1219385Z  --> sys/unix/ext/net.rs:1256:31
2019-11-12T22:54:02.1219430Z   |
2019-11-12T22:54:02.1219473Z 9 |         Ok((size, sender)) => println!("received {} bytes from {:?}", size, sender),
2019-11-12T22:54:02.1219541Z   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1219629Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1219686Z              found type `()`
2019-11-12T22:54:02.1219971Z   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T22:54:02.1220007Z 
2019-11-12T22:54:02.1220007Z 
2019-11-12T22:54:02.1220058Z error[E0308]: mismatched types
2019-11-12T22:54:02.1220247Z   --> sys/unix/ext/net.rs:1257:19
2019-11-12T22:54:02.1220441Z    |
2019-11-12T22:54:02.1220480Z 10 |         Err(e) => println!("recv_from function failed: {:?}", e),
2019-11-12T22:54:02.1220614Z    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1220696Z    = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1220749Z               found type `()`
2019-11-12T22:54:02.1221049Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T22:54:02.1221084Z 
---
2019-11-12T22:54:02.1222242Z   |
2019-11-12T22:54:02.1222598Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1222817Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1222875Z   |    |
2019-11-12T22:54:02.1222917Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1223093Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1223135Z              found type `()`
2019-11-12T22:54:02.1223161Z 
2019-11-12T22:54:02.1223198Z error: aborting due to previous error
---
2019-11-12T22:54:02.1224616Z   |
2019-11-12T22:54:02.1224828Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1225061Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1225106Z   |    |
2019-11-12T22:54:02.1225166Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1225260Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1225322Z              found type `()`
2019-11-12T22:54:02.1225349Z 
2019-11-12T22:54:02.1225389Z error: aborting due to previous error
---
2019-11-12T22:54:02.1226711Z   |
2019-11-12T22:54:02.1226888Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1227122Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1227170Z   |    |
2019-11-12T22:54:02.1227212Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1227309Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1227350Z              found type `()`
2019-11-12T22:54:02.1227376Z 
2019-11-12T22:54:02.1227431Z error: aborting due to previous error
---
2019-11-12T22:54:02.1228664Z   |
2019-11-12T22:54:02.1228838Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1229046Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1229271Z   |    |
2019-11-12T22:54:02.1229310Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1229406Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1229445Z              found type `()`
2019-11-12T22:54:02.1229470Z 
2019-11-12T22:54:02.1229505Z error: aborting due to previous error
---
2019-11-12T22:54:02.1230450Z   |
2019-11-12T22:54:02.1230647Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1230962Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1231002Z   |    |
2019-11-12T22:54:02.1231060Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1231137Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1231192Z              found type `()`
2019-11-12T22:54:02.1231217Z 
2019-11-12T22:54:02.1231252Z error: aborting due to previous error
2019-11-12T22:54:02.1231252Z error: aborting due to previous error
2019-11-12T22:54:02.1231277Z 
2019-11-12T22:54:02.1231505Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1231681Z Couldn't compile the test.
2019-11-12T22:54:02.1232101Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixDatagram::set_read_timeout (line 1388) stdout ----
2019-11-12T22:54:02.1232165Z error[E0317]: if may be missing an else clause
2019-11-12T22:54:02.1232348Z   --> sys/unix/ext/net.rs:1397:5
2019-11-12T22:54:02.1232616Z    |
2019-11-12T22:54:02.1232679Z 11 |     assert_eq!(err.kind(), io::ErrorKind::InvalidInput)
2019-11-12T22:54:02.1232760Z    |     |
2019-11-12T22:54:02.1232800Z    |     expected (), found enum `std::result::Result`
2019-11-12T22:54:02.1232855Z    |     found here
2019-11-12T22:54:02.1232891Z    |
2019-11-12T22:54:02.1232891Z    |
2019-11-12T22:54:02.1232927Z    = note: expected type `()`
2019-11-12T22:54:02.1232986Z               found type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1233029Z    = note: `if` expressions without `else` evaluate to `()`
2019-11-12T22:54:02.1233073Z    = help: consider adding an `else` block that evaluates to the expected type
2019-11-12T22:54:02.1233432Z 
2019-11-12T22:54:02.1233470Z error: aborting due to previous error
2019-11-12T22:54:02.1233496Z 
2019-11-12T22:54:02.1233738Z For more information about this error, try `rustc --explain E0317`.
2019-11-12T22:54:02.1233738Z For more information about this error, try `rustc --explain E0317`.
2019-11-12T22:54:02.1233921Z Couldn't compile the test.
2019-11-12T22:54:02.1234168Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixDatagram::set_write_timeout (line 1432) stdout ----
2019-11-12T22:54:02.1234231Z error[E0317]: if may be missing an else clause
2019-11-12T22:54:02.1234423Z   --> sys/unix/ext/net.rs:1441:5
2019-11-12T22:54:02.1234464Z    |
2019-11-12T22:54:02.1234521Z 11 |     assert_eq!(err.kind(), io::ErrorKind::InvalidInput)
2019-11-12T22:54:02.1234605Z    |     |
2019-11-12T22:54:02.1234646Z    |     expected (), found enum `std::result::Result`
2019-11-12T22:54:02.1234780Z    |     found here
2019-11-12T22:54:02.1234826Z    |
2019-11-12T22:54:02.1234826Z    |
2019-11-12T22:54:02.1234863Z    = note: expected type `()`
2019-11-12T22:54:02.1234922Z               found type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1234973Z    = note: `if` expressions without `else` evaluate to `()`
2019-11-12T22:54:02.1235020Z    = help: consider adding an `else` block that evaluates to the expected type
2019-11-12T22:54:02.1235399Z 
2019-11-12T22:54:02.1235437Z error: aborting due to previous error
2019-11-12T22:54:02.1235463Z 
2019-11-12T22:54:02.1236050Z For more information about this error, try `rustc --explain E0317`.
---
2019-11-12T22:54:02.1236939Z   |
2019-11-12T22:54:02.1237134Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1237353Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1237469Z   |    |
2019-11-12T22:54:02.1237510Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1237606Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1237648Z              found type `()`
2019-11-12T22:54:02.1237691Z 
2019-11-12T22:54:02.1237728Z error: aborting due to previous error
---
2019-11-12T22:54:02.1238752Z   |
2019-11-12T22:54:02.1238944Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1239351Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1239391Z   |    |
2019-11-12T22:54:02.1239432Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1239527Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1239566Z              found type `()`
2019-11-12T22:54:02.1239591Z 
2019-11-12T22:54:02.1239643Z error: aborting due to previous error
2019-11-12T22:54:02.1239643Z error: aborting due to previous error
2019-11-12T22:54:02.1239668Z 
2019-11-12T22:54:02.1239885Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1240062Z Couldn't compile the test.
2019-11-12T22:54:02.1240321Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixDatagram::take_error (line 1510) stdout ----
2019-11-12T22:54:02.1240365Z error[E0308]: mismatched types
2019-11-12T22:54:02.1240550Z  --> sys/unix/ext/net.rs:1515:46
2019-11-12T22:54:02.1240603Z   |
2019-11-12T22:54:02.1240650Z 7 |       if let Ok(Some(err)) = sock.take_error() {
2019-11-12T22:54:02.1240692Z   |  ______________________________________________^
2019-11-12T22:54:02.1240748Z 8 | |         println!("Got error: {:?}", err);
2019-11-12T22:54:02.1240826Z   | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1240883Z   |
2019-11-12T22:54:02.1240922Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1240962Z              found type `()`
2019-11-12T22:54:02.1240962Z              found type `()`
2019-11-12T22:54:02.1240987Z 
2019-11-12T22:54:02.1241040Z error[E0308]: mismatched types
2019-11-12T22:54:02.1241228Z  --> sys/unix/ext/net.rs:1515:5
2019-11-12T22:54:02.1241267Z   |
2019-11-12T22:54:02.1241390Z 7 | /     if let Ok(Some(err)) = sock.take_error() {
2019-11-12T22:54:02.1241459Z 8 | |         println!("Got error: {:?}", err);
2019-11-12T22:54:02.1241537Z   | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1241597Z   |
2019-11-12T22:54:02.1241637Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1241677Z              found type `()`
---
2019-11-12T22:54:02.1243098Z   |
2019-11-12T22:54:02.1243291Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1243518Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1243578Z   |    |
2019-11-12T22:54:02.1243622Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1243800Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1243843Z              found type `()`
2019-11-12T22:54:02.1243870Z 
2019-11-12T22:54:02.1243927Z error: aborting due to previous error
2019-11-12T22:54:02.1243927Z error: aborting due to previous error
2019-11-12T22:54:02.1243955Z 
2019-11-12T22:54:02.1244211Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1244402Z Couldn't compile the test.
2019-11-12T22:54:02.1244665Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixListener::accept (line 809) stdout ----
2019-11-12T22:54:02.1244712Z error[E0308]: mismatched types
2019-11-12T22:54:02.1244907Z  --> sys/unix/ext/net.rs:816:31
2019-11-12T22:54:02.1244970Z   |
2019-11-12T22:54:02.1245014Z 9 |         Ok((socket, addr)) => println!("Got a client: {:?}", addr),
2019-11-12T22:54:02.1245067Z   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1245176Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1245219Z              found type `()`
2019-11-12T22:54:02.1245516Z   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T22:54:02.1245571Z 
2019-11-12T22:54:02.1245571Z 
2019-11-12T22:54:02.1245611Z error[E0308]: mismatched types
2019-11-12T22:54:02.1270467Z   --> sys/unix/ext/net.rs:817:19
2019-11-12T22:54:02.1270537Z    |
2019-11-12T22:54:02.1270581Z 10 |         Err(e) => println!("accept function failed: {:?}", e),
2019-11-12T22:54:02.1270797Z    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1270893Z    = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1270930Z               found type `()`
2019-11-12T22:54:02.1271227Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-11-12T22:54:02.1271269Z 
---
2019-11-12T22:54:02.1272328Z    |
2019-11-12T22:54:02.1272674Z 10 |   fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1273048Z    |                ------------------- expected `std::result::Result<(), std::io::Error>` because of return type
2019-11-12T22:54:02.1273102Z ...
2019-11-12T22:54:02.1273140Z 14 | /     for stream in listener.incoming() {
2019-11-12T22:54:02.1273197Z 15 | |         match stream {
2019-11-12T22:54:02.1273243Z 16 | |             Ok(stream) => {
2019-11-12T22:54:02.1273282Z 17 | |                 /* connection succeeded */
2019-11-12T22:54:02.1273370Z 24 | |         }
2019-11-12T22:54:02.1273405Z 25 | |     }
2019-11-12T22:54:02.1273445Z    | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1273499Z    |
---
2019-11-12T22:54:02.1274892Z   |
2019-11-12T22:54:02.1275073Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1275305Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1275348Z   |    |
2019-11-12T22:54:02.1275390Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1275649Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1275842Z              found type `()`
2019-11-12T22:54:02.1275867Z 
2019-11-12T22:54:02.1275916Z error: aborting due to previous error
---
2019-11-12T22:54:02.1276796Z    |
2019-11-12T22:54:02.1276969Z 10 |   fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1277198Z    |                ------------------- expected `std::result::Result<(), std::io::Error>` because of return type
2019-11-12T22:54:02.1277253Z ...
2019-11-12T22:54:02.1277290Z 13 | /     for stream in listener.incoming() {
2019-11-12T22:54:02.1277325Z 14 | |         match stream {
2019-11-12T22:54:02.1277361Z 15 | |             Ok(stream) => {
2019-11-12T22:54:02.1277418Z 16 | |                 thread::spawn(|| handle_client(stream));
2019-11-12T22:54:02.1277494Z 21 | |         }
2019-11-12T22:54:02.1277546Z 22 | |     }
2019-11-12T22:54:02.1277583Z    | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1277618Z    |
---
2019-11-12T22:54:02.1278912Z   |
2019-11-12T22:54:02.1279105Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1279392Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1279441Z   |    |
2019-11-12T22:54:02.1279497Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1279573Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1279639Z              found type `()`
2019-11-12T22:54:02.1279665Z 
2019-11-12T22:54:02.1279702Z error: aborting due to previous error
2019-11-12T22:54:02.1279702Z error: aborting due to previous error
2019-11-12T22:54:02.1279727Z 
2019-11-12T22:54:02.1280168Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1280343Z Couldn't compile the test.
2019-11-12T22:54:02.1280748Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixListener::take_error (line 892) stdout ----
2019-11-12T22:54:02.1280793Z error[E0308]: mismatched types
2019-11-12T22:54:02.1280991Z   --> sys/unix/ext/net.rs:898:50
2019-11-12T22:54:02.1281031Z    |
2019-11-12T22:54:02.1281071Z 8  |       if let Ok(Some(err)) = listener.take_error() {
2019-11-12T22:54:02.1281141Z    |  __________________________________________________^
2019-11-12T22:54:02.1281183Z 9  | |         println!("Got error: {:?}", err);
2019-11-12T22:54:02.1281277Z    | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1281396Z    |
2019-11-12T22:54:02.1281436Z    = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1281493Z               found type `()`
2019-11-12T22:54:02.1281493Z               found type `()`
2019-11-12T22:54:02.1281518Z 
2019-11-12T22:54:02.1281555Z error[E0308]: mismatched types
2019-11-12T22:54:02.1281770Z   --> sys/unix/ext/net.rs:898:5
2019-11-12T22:54:02.1281825Z    |
2019-11-12T22:54:02.1281864Z 8  | /     if let Ok(Some(err)) = listener.take_error() {
2019-11-12T22:54:02.1281906Z 9  | |         println!("Got error: {:?}", err);
2019-11-12T22:54:02.1282178Z    | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1282216Z    |
2019-11-12T22:54:02.1282266Z    = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1282324Z               found type `()`
---
2019-11-12T22:54:02.1284007Z   |
2019-11-12T22:54:02.1284201Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1284424Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1284466Z   |    |
2019-11-12T22:54:02.1284521Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1284610Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1284651Z              found type `()`
2019-11-12T22:54:02.1284693Z 
2019-11-12T22:54:02.1284730Z error: aborting due to previous error
---
2019-11-12T22:54:02.1286047Z   |
2019-11-12T22:54:02.1286395Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1286618Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1286657Z   |    |
2019-11-12T22:54:02.1286787Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1286890Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1287087Z              found type `()`
2019-11-12T22:54:02.1287112Z 
2019-11-12T22:54:02.1287168Z error: aborting due to previous error
---
2019-11-12T22:54:02.1288046Z   |
2019-11-12T22:54:02.1288209Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1288406Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1288460Z   |    |
2019-11-12T22:54:02.1288505Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1288594Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1288632Z              found type `()`
2019-11-12T22:54:02.1288736Z 
2019-11-12T22:54:02.1288772Z error: aborting due to previous error
---
2019-11-12T22:54:02.1289784Z   |
2019-11-12T22:54:02.1289976Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1290191Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1290230Z   |    |
2019-11-12T22:54:02.1290293Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1290369Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1290427Z              found type `()`
2019-11-12T22:54:02.1290452Z 
2019-11-12T22:54:02.1290486Z error: aborting due to previous error
---
2019-11-12T22:54:02.1291403Z   |
2019-11-12T22:54:02.1291578Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1291812Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1291852Z   |    |
2019-11-12T22:54:02.1292063Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1292522Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1292701Z              found type `()`
2019-11-12T22:54:02.1292727Z 
2019-11-12T22:54:02.1292783Z error: aborting due to previous error
---
2019-11-12T22:54:02.1295722Z   |
2019-11-12T22:54:02.1295892Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1296217Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1296291Z   |    |
2019-11-12T22:54:02.1296330Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1296431Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1296472Z              found type `()`
2019-11-12T22:54:02.1296497Z 
2019-11-12T22:54:02.1296533Z error: aborting due to previous error
2019-11-12T22:54:02.1296533Z error: aborting due to previous error
2019-11-12T22:54:02.1296573Z 
2019-11-12T22:54:02.1296825Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1297001Z Couldn't compile the test.
2019-11-12T22:54:02.1297247Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixStream::set_read_timeout (line 406) stdout ----
2019-11-12T22:54:02.1297303Z error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
2019-11-12T22:54:02.1297497Z  --> sys/unix/ext/net.rs:410:14
2019-11-12T22:54:02.1297550Z   |
2019-11-12T22:54:02.1297589Z 7 | let socket = UnixStream::connect("/tmp/sock")?;
2019-11-12T22:54:02.1297636Z   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
2019-11-12T22:54:02.1297820Z   = help: the trait `std::ops::Try` is not implemented for `()`
2019-11-12T22:54:02.1297862Z   = note: required by `std::ops::Try::from_error`
2019-11-12T22:54:02.1297888Z 
2019-11-12T22:54:02.1297939Z error: aborting due to previous error
2019-11-12T22:54:02.1297939Z error: aborting due to previous error
2019-11-12T22:54:02.1297964Z 
2019-11-12T22:54:02.1298201Z For more information about this error, try `rustc --explain E0277`.
2019-11-12T22:54:02.1298378Z Couldn't compile the test.
2019-11-12T22:54:02.1298628Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixStream::set_read_timeout (line 417) stdout ----
2019-11-12T22:54:02.1298674Z error[E0317]: if may be missing an else clause
2019-11-12T22:54:02.1304216Z   --> sys/unix/ext/net.rs:426:5
2019-11-12T22:54:02.1304333Z    |
2019-11-12T22:54:02.1304375Z 11 |     assert_eq!(err.kind(), io::ErrorKind::InvalidInput)
2019-11-12T22:54:02.1304519Z    |     |
2019-11-12T22:54:02.1304566Z    |     expected (), found enum `std::result::Result`
2019-11-12T22:54:02.1304606Z    |     found here
2019-11-12T22:54:02.1304657Z    |
2019-11-12T22:54:02.1304657Z    |
2019-11-12T22:54:02.1304694Z    = note: expected type `()`
2019-11-12T22:54:02.1304736Z               found type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1304780Z    = note: `if` expressions without `else` evaluate to `()`
2019-11-12T22:54:02.1304844Z    = help: consider adding an `else` block that evaluates to the expected type
2019-11-12T22:54:02.1305231Z 
2019-11-12T22:54:02.1305295Z error: aborting due to previous error
2019-11-12T22:54:02.1305322Z 
2019-11-12T22:54:02.1305560Z For more information about this error, try `rustc --explain E0317`.
---
2019-11-12T22:54:02.1306323Z   |
2019-11-12T22:54:02.1306677Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1306888Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1306944Z   |    |
2019-11-12T22:54:02.1306982Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1307076Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1307117Z              found type `()`
2019-11-12T22:54:02.1307141Z 
2019-11-12T22:54:02.1307327Z error: aborting due to previous error
2019-11-12T22:54:02.1307327Z error: aborting due to previous error
2019-11-12T22:54:02.1307386Z 
2019-11-12T22:54:02.1307639Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1307813Z Couldn't compile the test.
2019-11-12T22:54:02.1308056Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixStream::set_write_timeout (line 461) stdout ----
2019-11-12T22:54:02.1308124Z error[E0317]: if may be missing an else clause
2019-11-12T22:54:02.1308300Z   --> sys/unix/ext/net.rs:470:5
2019-11-12T22:54:02.1308338Z    |
2019-11-12T22:54:02.1308573Z 11 |     assert_eq!(err.kind(), io::ErrorKind::InvalidInput)
2019-11-12T22:54:02.1308651Z    |     |
2019-11-12T22:54:02.1308705Z    |     expected (), found enum `std::result::Result`
2019-11-12T22:54:02.1308743Z    |     found here
2019-11-12T22:54:02.1308777Z    |
2019-11-12T22:54:02.1308777Z    |
2019-11-12T22:54:02.1308828Z    = note: expected type `()`
2019-11-12T22:54:02.1308875Z               found type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1308917Z    = note: `if` expressions without `else` evaluate to `()`
2019-11-12T22:54:02.1308976Z    = help: consider adding an `else` block that evaluates to the expected type
2019-11-12T22:54:02.1310413Z 
2019-11-12T22:54:02.1310484Z error: aborting due to previous error
2019-11-12T22:54:02.1310533Z 
2019-11-12T22:54:02.1310840Z For more information about this error, try `rustc --explain E0317`.
2019-11-12T22:54:02.1310840Z For more information about this error, try `rustc --explain E0317`.
2019-11-12T22:54:02.1311046Z Couldn't compile the test.
2019-11-12T22:54:02.1311338Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixStream::take_error (line 538) stdout ----
2019-11-12T22:54:02.1311392Z error[E0308]: mismatched types
2019-11-12T22:54:02.1311602Z  --> sys/unix/ext/net.rs:543:48
2019-11-12T22:54:02.1311648Z   |
2019-11-12T22:54:02.1311726Z 7 |       if let Ok(Some(err)) = socket.take_error() {
2019-11-12T22:54:02.1311776Z   |  ________________________________________________^
2019-11-12T22:54:02.1311825Z 8 | |         println!("Got error: {:?}", err);
2019-11-12T22:54:02.1311941Z   | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1311985Z   |
2019-11-12T22:54:02.1312048Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1312096Z              found type `()`
2019-11-12T22:54:02.1312096Z              found type `()`
2019-11-12T22:54:02.1312125Z 
2019-11-12T22:54:02.1312167Z error[E0308]: mismatched types
2019-11-12T22:54:02.1312400Z  --> sys/unix/ext/net.rs:543:5
2019-11-12T22:54:02.1312445Z   |
2019-11-12T22:54:02.1312492Z 7 | /     if let Ok(Some(err)) = socket.take_error() {
2019-11-12T22:54:02.1312558Z 8 | |         println!("Got error: {:?}", err);
2019-11-12T22:54:02.1312650Z   | |_____^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1312702Z   |
2019-11-12T22:54:02.1312768Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1312814Z              found type `()`
2019-11-12T22:54:02.1312814Z              found type `()`
2019-11-12T22:54:02.1312843Z 
2019-11-12T22:54:02.1312905Z error: aborting due to 2 previous errors
2019-11-12T22:54:02.1312942Z 
2019-11-12T22:54:02.1313195Z For more information about this error, try `rustc --explain E0308`.
2019-11-12T22:54:02.1313400Z Couldn't compile the test.
2019-11-12T22:54:02.1313687Z ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixStream::shutdown (line 566) stdout ----
2019-11-12T22:54:02.1313754Z error[E0599]: no method named `shutdown` found for type `std::result::Result<std::os::unix::net::UnixStream, std::io::Error>` in the current scope
2019-11-12T22:54:02.1313972Z  --> sys/unix/ext/net.rs:572:12
2019-11-12T22:54:02.1314034Z   |
2019-11-12T22:54:02.1314083Z 8 |     socket.shutdown(Shutdown::Both).expect("shutdown function failed");
2019-11-12T22:54:02.1314422Z   |            ^^^^^^^^ method not found in `std::result::Result<std::os::unix::net::UnixStream, std::io::Error>`
2019-11-12T22:54:02.1314527Z error[E0308]: mismatched types
2019-11-12T22:54:02.1314954Z  --> sys/unix/ext/net.rs:570:14
2019-11-12T22:54:02.1314994Z   |
2019-11-12T22:54:02.1315370Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1315370Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1315590Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1315634Z   |    |
2019-11-12T22:54:02.1315692Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1315738Z 7 |     let socket = UnixStream::connect("/tmp/sock");
2019-11-12T22:54:02.1315950Z 8 |     socket.shutdown(Shutdown::Both).expect("shutdown function failed");
2019-11-12T22:54:02.1316254Z   |
2019-11-12T22:54:02.1316301Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1316355Z              found type `()`
2019-11-12T22:54:02.1316379Z 
---
2019-11-12T22:54:02.1317472Z   |
2019-11-12T22:54:02.1317647Z 5 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1317859Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1317900Z   |    |
2019-11-12T22:54:02.1317962Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1318040Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1318095Z              found type `()`
2019-11-12T22:54:02.1318119Z 
2019-11-12T22:54:02.1318155Z error: aborting due to previous error
---
2019-11-12T22:54:02.1319137Z   |
2019-11-12T22:54:02.1319320Z 6 | fn main() -> std::io::Result<()> {
2019-11-12T22:54:02.1319568Z   |    ----      ^^^^^^^^^^^^^^^^^^^ expected enum `std::result::Result`, found ()
2019-11-12T22:54:02.1319610Z   |    |
2019-11-12T22:54:02.1319659Z   |    implicitly returns `()` as its body has no tail or `return` expression
2019-11-12T22:54:02.1319754Z   = note: expected type `std::result::Result<(), std::io::Error>`
2019-11-12T22:54:02.1319793Z              found type `()`
2019-11-12T22:54:02.1319841Z 
2019-11-12T22:54:02.1319877Z error: aborting due to previous error
---
2019-11-12T22:54:02.1379094Z   local time: Tue Nov 12 22:54:02 UTC 2019
2019-11-12T22:54:02.2102031Z   network time: Tue, 12 Nov 2019 22:54:02 GMT
2019-11-12T22:54:02.2103293Z == end clock drift check ==
2019-11-12T22:54:02.8008910Z 
2019-11-12T22:54:02.8168952Z ##[error]Bash exited with code '1'.
2019-11-12T22:54:02.8199021Z ##[section]Starting: Checkout
2019-11-12T22:54:02.8201000Z ==============================================================================
2019-11-12T22:54:02.8201051Z Task         : Get sources
2019-11-12T22:54:02.8201109Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
