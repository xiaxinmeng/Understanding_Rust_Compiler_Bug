plain
2020-02-04T20:06:10.0427109Z ========================== Starting Command Output ===========================
2020-02-04T20:06:10.0428404Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9f7900c7-bbf2-4b76-9d06-92cb3e300171.sh
2020-02-04T20:06:10.0428433Z 
2020-02-04T20:06:10.0432885Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T20:06:10.0437421Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-04T20:06:10.0438740Z Task         : Get sources
2020-02-04T20:06:10.0438766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T20:06:10.0438797Z Version      : 1.0.0
2020-02-04T20:06:10.0438820Z Author       : Microsoft
---
2020-02-04T20:06:10.7892062Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T20:06:10.7963795Z ##[command]git config gc.auto 0
2020-02-04T20:06:10.8023483Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T20:06:10.8068576Z ##[command]git config --get-all http.proxy
2020-02-04T20:06:10.8177146Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68809/merge:refs/remotes/pull/68809/merge
---
2020-02-04T20:54:08.7593541Z .................................................................................................... 1700/9583
2020-02-04T20:54:12.8610582Z .................................................................................................... 1800/9583
2020-02-04T20:54:23.2412252Z .............................i...................................................................... 1900/9583
2020-02-04T20:54:29.3015430Z .................................................................................................... 2000/9583
2020-02-04T20:54:40.9021700Z ...................iiiii............................................................................ 2100/9583
2020-02-04T20:54:49.1294145Z .................................................................................................... 2300/9583
2020-02-04T20:54:51.1369356Z .................................................................................................... 2400/9583
2020-02-04T20:54:55.1109163Z .................................................................................................... 2500/9583
2020-02-04T20:55:12.0555354Z .................................................................................................... 2600/9583
---
2020-02-04T20:57:21.1023214Z ..............................................................i...............i..................... 4900/9583
2020-02-04T20:57:27.2760891Z .................................................................................................... 5000/9583
2020-02-04T20:57:33.9789842Z .................................................................................................... 5100/9583
2020-02-04T20:57:38.1057507Z .....i.............................................................................................. 5200/9583
2020-02-04T20:57:47.6045132Z ...............................................................................ii.ii........i...i... 5300/9583
2020-02-04T20:57:54.6694746Z .................i.................................................................................. 5500/9583
2020-02-04T20:58:02.3849435Z .................................................................................................... 5600/9583
2020-02-04T20:58:08.2195946Z ..................................................................i................................. 5700/9583
2020-02-04T20:58:14.5719526Z .................................................................................................... 5800/9583
2020-02-04T20:58:14.5719526Z .................................................................................................... 5800/9583
2020-02-04T20:58:20.7753482Z .................................................................................................... 5900/9583
2020-02-04T20:58:28.7238335Z .........................................................ii...i..ii...........i..................... 6000/9583
2020-02-04T20:58:46.2669912Z .................................................................................................... 6200/9583
2020-02-04T20:58:52.4174751Z .................................................................................................... 6300/9583
2020-02-04T20:58:52.4174751Z .................................................................................................... 6300/9583
2020-02-04T20:58:58.6331689Z .....................................................................................i..ii.......... 6400/9583
2020-02-04T20:59:25.8937299Z .................................................................................................... 6600/9583
2020-02-04T20:59:33.6500835Z ........................................................................i........................... 6700/9583
2020-02-04T20:59:35.3853403Z .................................................................................................... 6800/9583
2020-02-04T20:59:37.1988623Z ..........................................................................i......................... 6900/9583
---
2020-02-04T21:01:00.5907013Z .................................................................................................... 7600/9583
2020-02-04T21:01:04.4622620Z .................................................................................................... 7700/9583
2020-02-04T21:01:09.9768267Z .................................................................................................... 7800/9583
2020-02-04T21:01:17.0844784Z .................................................................................................... 7900/9583
2020-02-04T21:01:23.1945661Z ....................................iiiiiiii........................................................ 8000/9583
2020-02-04T21:01:35.0078831Z .................................................................................................... 8200/9583
2020-02-04T21:01:41.5413941Z .................................................................................................... 8300/9583
2020-02-04T21:01:53.1663312Z .................................................................................................... 8400/9583
2020-02-04T21:01:59.3408638Z .................................................................................................... 8500/9583
---
2020-02-04T21:03:34.1331104Z ---- [ui] ui/consts/const-int-arithmetic.rs stdout ----
2020-02-04T21:03:34.1332222Z 
2020-02-04T21:03:34.1333193Z error: test compilation failed although it shouldn't!
2020-02-04T21:03:34.1333537Z status: exit code: 1
2020-02-04T21:03:34.1334458Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-arithmetic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-arithmetic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-arithmetic/auxiliary"
2020-02-04T21:03:34.1335289Z ------------------------------------------
2020-02-04T21:03:34.1336485Z 
2020-02-04T21:03:34.1337049Z ------------------------------------------
2020-02-04T21:03:34.1337246Z stderr:
2020-02-04T21:03:34.1337246Z stderr:
2020-02-04T21:03:34.1337529Z ------------------------------------------
2020-02-04T21:03:34.1337697Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1338000Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1338141Z    |
2020-02-04T21:03:34.1338272Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1338485Z ...
2020-02-04T21:03:34.1338485Z ...
2020-02-04T21:03:34.1338605Z LL | / suite! {
2020-02-04T21:03:34.1338712Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1338818Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1338954Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1339242Z ...  |
2020-02-04T21:03:34.1339797Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1340242Z    | |_- in this macro invocation
2020-02-04T21:03:34.1340376Z 
2020-02-04T21:03:34.1340740Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1341154Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1341154Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1341332Z    |
2020-02-04T21:03:34.1341461Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1341721Z ...
2020-02-04T21:03:34.1341721Z ...
2020-02-04T21:03:34.1341845Z LL | / suite! {
2020-02-04T21:03:34.1341965Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1342105Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1342230Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1342360Z ...  |
2020-02-04T21:03:34.1343155Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1343711Z    | |_- in this macro invocation
2020-02-04T21:03:34.1343847Z 
2020-02-04T21:03:34.1343969Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1344278Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1344278Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1344433Z    |
2020-02-04T21:03:34.1344544Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1344774Z ...
2020-02-04T21:03:34.1344774Z ...
2020-02-04T21:03:34.1344878Z LL | / suite! {
2020-02-04T21:03:34.1344982Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1345104Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1345211Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1345328Z ...  |
2020-02-04T21:03:34.1345597Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1346014Z    | |_- in this macro invocation
2020-02-04T21:03:34.1346133Z 
2020-02-04T21:03:34.1346247Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1346667Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1346667Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1346840Z    |
2020-02-04T21:03:34.1346950Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1347181Z ...
2020-02-04T21:03:34.1347181Z ...
2020-02-04T21:03:34.1347283Z LL | / suite! {
2020-02-04T21:03:34.1347404Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1347509Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1347619Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1347740Z ...  |
2020-02-04T21:03:34.1348028Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1348562Z    | |_- in this macro invocation
2020-02-04T21:03:34.1348685Z 
2020-02-04T21:03:34.1348800Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1349104Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1349104Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1349244Z    |
2020-02-04T21:03:34.1349357Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1349589Z ...
2020-02-04T21:03:34.1349589Z ...
2020-02-04T21:03:34.1349709Z LL | / suite! {
2020-02-04T21:03:34.1349817Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1349922Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1350043Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1350149Z ...  |
2020-02-04T21:03:34.1350415Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1350818Z    | |_- in this macro invocation
2020-02-04T21:03:34.1350945Z 
2020-02-04T21:03:34.1351078Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1351364Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1351364Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1351549Z    |
2020-02-04T21:03:34.1351751Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1352117Z ...
2020-02-04T21:03:34.1352117Z ...
2020-02-04T21:03:34.1352308Z LL | / suite! {
2020-02-04T21:03:34.1352466Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1352627Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1352824Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1352982Z ...  |
2020-02-04T21:03:34.1353300Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1355483Z    | |_- in this macro invocation
2020-02-04T21:03:34.1355525Z 
2020-02-04T21:03:34.1355563Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1355763Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1355763Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1355814Z    |
2020-02-04T21:03:34.1355849Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1355939Z ...
2020-02-04T21:03:34.1355939Z ...
2020-02-04T21:03:34.1355969Z LL | / suite! {
2020-02-04T21:03:34.1356002Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1356035Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1356087Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1356119Z ...  |
2020-02-04T21:03:34.1356287Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1356490Z    | |_- in this macro invocation
2020-02-04T21:03:34.1356513Z 
2020-02-04T21:03:34.1356551Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1356763Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1356763Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1356799Z    |
2020-02-04T21:03:34.1356833Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1373008Z ...
2020-02-04T21:03:34.1373008Z ...
2020-02-04T21:03:34.1373170Z LL | / suite! {
2020-02-04T21:03:34.1373227Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1373266Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1373303Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1373338Z ...  |
2020-02-04T21:03:34.1373670Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1373900Z    | |_- in this macro invocation
2020-02-04T21:03:34.1373936Z 
2020-02-04T21:03:34.1373978Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1374314Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1374314Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1374450Z    |
2020-02-04T21:03:34.1374489Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1374561Z ...
2020-02-04T21:03:34.1374561Z ...
2020-02-04T21:03:34.1374602Z LL | / suite! {
2020-02-04T21:03:34.1374645Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1374683Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1374732Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1374767Z ...  |
2020-02-04T21:03:34.1375064Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1375890Z    | |_- in this macro invocation
2020-02-04T21:03:34.1375923Z 
2020-02-04T21:03:34.1375963Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1376206Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1376206Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1376244Z    |
2020-02-04T21:03:34.1376288Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1376365Z ...
2020-02-04T21:03:34.1376365Z ...
2020-02-04T21:03:34.1376396Z LL | / suite! {
2020-02-04T21:03:34.1376429Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1376476Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1376510Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1376541Z ...  |
2020-02-04T21:03:34.1376725Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1376913Z    | |_- in this macro invocation
2020-02-04T21:03:34.1376937Z 
2020-02-04T21:03:34.1376985Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1377173Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1377173Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1377209Z    |
2020-02-04T21:03:34.1377257Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1377331Z ...
2020-02-04T21:03:34.1377331Z ...
2020-02-04T21:03:34.1377377Z LL | / suite! {
2020-02-04T21:03:34.1377410Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1377444Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1377496Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1377528Z ...  |
2020-02-04T21:03:34.1377698Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1377892Z    | |_- in this macro invocation
2020-02-04T21:03:34.1377915Z 
2020-02-04T21:03:34.1377952Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1378147Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1378147Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1378183Z    |
2020-02-04T21:03:34.1378217Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1378303Z ...
2020-02-04T21:03:34.1378303Z ...
2020-02-04T21:03:34.1378334Z LL | / suite! {
2020-02-04T21:03:34.1378366Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1378409Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1378442Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1378473Z ...  |
2020-02-04T21:03:34.1378748Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1378970Z    | |_- in this macro invocation
2020-02-04T21:03:34.1378993Z 
2020-02-04T21:03:34.1379037Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1379223Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1379223Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1379258Z    |
2020-02-04T21:03:34.1379297Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1379365Z ...
2020-02-04T21:03:34.1379365Z ...
2020-02-04T21:03:34.1379401Z LL | / suite! {
2020-02-04T21:03:34.1379501Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1379535Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1379567Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1379612Z ...  |
2020-02-04T21:03:34.1380156Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1380389Z    | |_- in this macro invocation
2020-02-04T21:03:34.1380414Z 
2020-02-04T21:03:34.1380451Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1382277Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1382277Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1382332Z    |
2020-02-04T21:03:34.1382365Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1382438Z ...
2020-02-04T21:03:34.1382438Z ...
2020-02-04T21:03:34.1382468Z LL | / suite! {
2020-02-04T21:03:34.1382500Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1382547Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1382581Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1382611Z ...  |
2020-02-04T21:03:34.1382789Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1383123Z    | |_- in this macro invocation
2020-02-04T21:03:34.1383175Z 
2020-02-04T21:03:34.1383225Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1383462Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1383462Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1383498Z    |
2020-02-04T21:03:34.1383546Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1383612Z ...
2020-02-04T21:03:34.1383612Z ...
2020-02-04T21:03:34.1383643Z LL | / suite! {
2020-02-04T21:03:34.1383690Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1385419Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1385499Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1385549Z ...  |
2020-02-04T21:03:34.1385824Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1386024Z    | |_- in this macro invocation
2020-02-04T21:03:34.1386049Z 
2020-02-04T21:03:34.1386095Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1386285Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1386285Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1386334Z    |
2020-02-04T21:03:34.1386368Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1386443Z ...
2020-02-04T21:03:34.1386443Z ...
2020-02-04T21:03:34.1386474Z LL | / suite! {
2020-02-04T21:03:34.1386507Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1386556Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1386589Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1386620Z ...  |
2020-02-04T21:03:34.1386795Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1386990Z    | |_- in this macro invocation
2020-02-04T21:03:34.1387014Z 
2020-02-04T21:03:34.1387056Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1387364Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1387364Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1387410Z    |
2020-02-04T21:03:34.1387483Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1387550Z ...
2020-02-04T21:03:34.1387550Z ...
2020-02-04T21:03:34.1387580Z LL | / suite! {
2020-02-04T21:03:34.1387626Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1387659Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1387692Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1387732Z ...  |
2020-02-04T21:03:34.1387932Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1388216Z    | |_- in this macro invocation
2020-02-04T21:03:34.1388241Z 
2020-02-04T21:03:34.1388278Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1388470Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1388470Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1388512Z    |
2020-02-04T21:03:34.1388546Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1388617Z ...
2020-02-04T21:03:34.1388617Z ...
2020-02-04T21:03:34.1388648Z LL | / suite! {
2020-02-04T21:03:34.1388680Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1388713Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1388755Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1388786Z ...  |
2020-02-04T21:03:34.1388955Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1389154Z    | |_- in this macro invocation
2020-02-04T21:03:34.1389183Z 
2020-02-04T21:03:34.1389220Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1389422Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1389422Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1389457Z    |
2020-02-04T21:03:34.1389497Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1389574Z ...
2020-02-04T21:03:34.1389574Z ...
2020-02-04T21:03:34.1389604Z LL | / suite! {
2020-02-04T21:03:34.1389644Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1389677Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1389710Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1389748Z ...  |
2020-02-04T21:03:34.1389917Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1390103Z    | |_- in this macro invocation
2020-02-04T21:03:34.1390138Z 
2020-02-04T21:03:34.1390182Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1390369Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1390369Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1390413Z    |
2020-02-04T21:03:34.1390446Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1390533Z ...
2020-02-04T21:03:34.1390533Z ...
2020-02-04T21:03:34.1390564Z LL | / suite! {
2020-02-04T21:03:34.1390595Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1390629Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1390668Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1390699Z ...  |
2020-02-04T21:03:34.1390866Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1391061Z    | |_- in this macro invocation
2020-02-04T21:03:34.1391084Z 
2020-02-04T21:03:34.1391120Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1391744Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1391744Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1391792Z    |
2020-02-04T21:03:34.1391827Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1391986Z ...
2020-02-04T21:03:34.1391986Z ...
2020-02-04T21:03:34.1392023Z LL | / suite! {
2020-02-04T21:03:34.1392069Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1392104Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1392138Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1392170Z ...  |
2020-02-04T21:03:34.1392412Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1392611Z    | |_- in this macro invocation
2020-02-04T21:03:34.1392645Z 
2020-02-04T21:03:34.1392685Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1392884Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1392884Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1392991Z    |
2020-02-04T21:03:34.1393027Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1393096Z ...
2020-02-04T21:03:34.1393096Z ...
2020-02-04T21:03:34.1393134Z LL | / suite! {
2020-02-04T21:03:34.1393174Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1393210Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1393251Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1393284Z ...  |
2020-02-04T21:03:34.1393482Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1393692Z    | |_- in this macro invocation
2020-02-04T21:03:34.1393716Z 
2020-02-04T21:03:34.1393755Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1393965Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1393965Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1394010Z    |
2020-02-04T21:03:34.1394045Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1394130Z ...
2020-02-04T21:03:34.1394130Z ...
2020-02-04T21:03:34.1394162Z LL | / suite! {
2020-02-04T21:03:34.1394196Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1394248Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1394285Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1394494Z ...  |
2020-02-04T21:03:34.1394673Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1394860Z    | |_- in this macro invocation
2020-02-04T21:03:34.1394884Z 
2020-02-04T21:03:34.1394932Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1395118Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1395118Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1395152Z    |
2020-02-04T21:03:34.1395198Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1395269Z ...
2020-02-04T21:03:34.1395269Z ...
2020-02-04T21:03:34.1395309Z LL | / suite! {
2020-02-04T21:03:34.1395341Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1395374Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1395426Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1395457Z ...  |
2020-02-04T21:03:34.1395624Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1395817Z    | |_- in this macro invocation
2020-02-04T21:03:34.1395840Z 
2020-02-04T21:03:34.1395877Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1396066Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1396066Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1396101Z    |
2020-02-04T21:03:34.1396135Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1396215Z ...
2020-02-04T21:03:34.1396215Z ...
2020-02-04T21:03:34.1396245Z LL | / suite! {
2020-02-04T21:03:34.1396276Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1396323Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1396356Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1396443Z ...  |
2020-02-04T21:03:34.1396650Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1397028Z    | |_- in this macro invocation
2020-02-04T21:03:34.1397052Z 
2020-02-04T21:03:34.1397099Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1397294Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1397294Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1397330Z    |
2020-02-04T21:03:34.1397373Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1397442Z ...
2020-02-04T21:03:34.1397442Z ...
2020-02-04T21:03:34.1397535Z LL | / suite! {
2020-02-04T21:03:34.1397569Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1397605Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1397639Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1397679Z ...  |
2020-02-04T21:03:34.1397878Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1398093Z    | |_- in this macro invocation
2020-02-04T21:03:34.1398119Z 
2020-02-04T21:03:34.1398157Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1398354Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1398354Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1398403Z    |
2020-02-04T21:03:34.1398440Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1398524Z ...
2020-02-04T21:03:34.1398524Z ...
2020-02-04T21:03:34.1398556Z LL | / suite! {
2020-02-04T21:03:34.1398590Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1398640Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1398676Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1398708Z ...  |
2020-02-04T21:03:34.1398896Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1399102Z    | |_- in this macro invocation
2020-02-04T21:03:34.1399127Z 
2020-02-04T21:03:34.1399127Z 
2020-02-04T21:03:34.1399176Z error: `core::num::<impl i8>::saturating_abs` is not yet stable as a const fn
2020-02-04T21:03:34.1399584Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:84:5
2020-02-04T21:03:34.1399630Z    |
2020-02-04T21:03:34.1399674Z LL |     100i8.saturating_abs(), 100;
2020-02-04T21:03:34.1399918Z    |
2020-02-04T21:03:34.1400605Z    = help: add `#![feature(saturating_int_methods)]` to the crate attributes to enable
2020-02-04T21:03:34.1400666Z 
2020-02-04T21:03:34.1400666Z 
2020-02-04T21:03:34.1400707Z error: `core::num::<impl i8>::saturating_abs` is not yet stable as a const fn
2020-02-04T21:03:34.1401201Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:85:5
2020-02-04T21:03:34.1401249Z    |
2020-02-04T21:03:34.1401419Z LL |     (-100i8).saturating_abs(), 100;
2020-02-04T21:03:34.1401486Z    |
2020-02-04T21:03:34.1401691Z    = help: add `#![feature(saturating_int_methods)]` to the crate attributes to enable
2020-02-04T21:03:34.1401721Z 
2020-02-04T21:03:34.1401721Z 
2020-02-04T21:03:34.1401756Z error: `core::num::<impl i8>::saturating_abs` is not yet stable as a const fn
2020-02-04T21:03:34.1402557Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:86:5
2020-02-04T21:03:34.1402609Z    |
2020-02-04T21:03:34.1402643Z LL |     i8::min_value().saturating_abs(), i8::max_value();
2020-02-04T21:03:34.1402725Z    |
2020-02-04T21:03:34.1402761Z    = help: add `#![feature(saturating_int_methods)]` to the crate attributes to enable
2020-02-04T21:03:34.1402797Z 
2020-02-04T21:03:34.1402797Z 
2020-02-04T21:03:34.1402843Z error: `core::num::<impl i8>::saturating_abs` is not yet stable as a const fn
2020-02-04T21:03:34.1403099Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:87:5
2020-02-04T21:03:34.1403134Z    |
2020-02-04T21:03:34.1403275Z LL |     (i8::min_value() + 1).saturating_abs(), i8::max_value();
2020-02-04T21:03:34.1403350Z    |
2020-02-04T21:03:34.1403392Z    = help: add `#![feature(saturating_int_methods)]` to the crate attributes to enable
2020-02-04T21:03:34.1403417Z 
2020-02-04T21:03:34.1403454Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1403454Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1403672Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1403714Z    |
2020-02-04T21:03:34.1403749Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1403881Z ...
2020-02-04T21:03:34.1403881Z ...
2020-02-04T21:03:34.1403911Z LL | / suite! {
2020-02-04T21:03:34.1403943Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1403988Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1404022Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1404053Z ...  |
2020-02-04T21:03:34.1404247Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1404457Z    | |_- in this macro invocation
2020-02-04T21:03:34.1404481Z 
2020-02-04T21:03:34.1404530Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1404716Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1404716Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1404752Z    |
2020-02-04T21:03:34.1404785Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1404857Z ...
2020-02-04T21:03:34.1404857Z ...
2020-02-04T21:03:34.1404887Z LL | / suite! {
2020-02-04T21:03:34.1404938Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1404971Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1405004Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1405047Z ...  |
2020-02-04T21:03:34.1405215Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1410670Z    | |_- in this macro invocation
2020-02-04T21:03:34.1410719Z 
2020-02-04T21:03:34.1410762Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1410973Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1410973Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1411026Z    |
2020-02-04T21:03:34.1411063Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1411140Z ...
2020-02-04T21:03:34.1411140Z ...
2020-02-04T21:03:34.1411173Z LL | / suite! {
2020-02-04T21:03:34.1411209Z LL | |     // `const_checked_int_methods`
2020-02-04T21:03:34.1416467Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T21:03:34.1416522Z LL | |     127i8.checked_add(2), None;
2020-02-04T21:03:34.1416556Z ...  |
2020-02-04T21:03:34.1416890Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T21:03:34.1417288Z    | |_- in this macro invocation
2020-02-04T21:03:34.1417316Z 
2020-02-04T21:03:34.1417533Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T21:03:34.1417743Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1417743Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T21:03:34.1417779Z    |
2020-02-04T21:03:34.1417813Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T21:03:34.1417896Z ...
2020-02-04T21:03:34.1417896Z ...
2020-02-04T21:03:34.1417926Z LL | / suite! {
---
2020-02-04T21:03:34.1427927Z test result: FAILED. 9530 passed; 1 failed; 52 ignored; 0 measured; 0 filtered out
2020-02-04T21:03:34.1427958Z 
2020-02-04T21:03:34.1427994Z 
2020-02-04T21:03:34.1428015Z 
2020-02-04T21:03:34.1429294Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-04T21:03:34.1429488Z 
2020-02-04T21:03:34.1429514Z 
2020-02-04T21:03:34.1429748Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-04T21:03:34.1429816Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T21:03:34.1429816Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T21:03:34.1429865Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-04T21:03:34.1430131Z Build completed unsuccessfully in 0:52:09
2020-02-04T21:03:34.1432557Z == clock drift check ==
2020-02-04T21:03:34.1432805Z   local time: Tue Feb  4 21:03:34 UTC 2020
2020-02-04T21:03:34.6905766Z   network time: Tue, 04 Feb 2020 21:03:34 GMT
2020-02-04T21:03:34.6905863Z == end clock drift check ==
2020-02-04T21:03:35.1240371Z 
2020-02-04T21:03:35.1285643Z ##[error]Bash exited with code '1'.
2020-02-04T21:03:35.1295578Z ##[section]Finishing: Run build
2020-02-04T21:03:35.1318063Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-04T21:03:35.1319491Z Task         : Get sources
2020-02-04T21:03:35.1319678Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T21:03:35.1319712Z Version      : 1.0.0
2020-02-04T21:03:35.1319742Z Author       : Microsoft
2020-02-04T21:03:35.1319742Z Author       : Microsoft
2020-02-04T21:03:35.1319793Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T21:03:35.1319830Z ==============================================================================
2020-02-04T21:03:35.4780301Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T21:03:35.4814674Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-04T21:03:35.4903278Z Cleaning up task key
2020-02-04T21:03:35.4903919Z Start cleaning up orphan processes.
2020-02-04T21:03:35.4994513Z Terminate orphan process: pid (4004) (python)
2020-02-04T21:03:35.5211503Z ##[section]Finishing: Finalize Job
