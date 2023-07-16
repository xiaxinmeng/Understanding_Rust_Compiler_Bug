plain
2020-02-04T22:44:26.9806911Z ========================== Starting Command Output ===========================
2020-02-04T22:44:26.9808371Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2cf7ba6a-ac3e-4fcb-98b2-4f678738fc48.sh
2020-02-04T22:44:26.9808408Z 
2020-02-04T22:44:26.9810877Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T22:44:26.9819040Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-04T22:44:26.9820775Z Task         : Get sources
2020-02-04T22:44:26.9820812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T22:44:26.9820849Z Version      : 1.0.0
2020-02-04T22:44:26.9820887Z Author       : Microsoft
---
2020-02-04T22:44:27.8255491Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T22:44:27.8344368Z ##[command]git config gc.auto 0
2020-02-04T22:44:27.8416597Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T22:44:27.8454944Z ##[command]git config --get-all http.proxy
2020-02-04T22:44:27.8633908Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68809/merge:refs/remotes/pull/68809/merge
---
2020-02-04T23:40:23.3742628Z .................................................................................................... 1700/9586
2020-02-04T23:40:28.4857172Z .................................................................................................... 1800/9586
2020-02-04T23:40:40.9300988Z .............................i...................................................................... 1900/9586
2020-02-04T23:40:47.9903292Z .................................................................................................... 2000/9586
2020-02-04T23:41:01.9328592Z ...................iiiii............................................................................ 2100/9586
2020-02-04T23:41:11.7903829Z .................................................................................................... 2300/9586
2020-02-04T23:41:14.2976883Z .................................................................................................... 2400/9586
2020-02-04T23:41:19.2004941Z .................................................................................................... 2500/9586
2020-02-04T23:41:39.7824212Z .................................................................................................... 2600/9586
---
2020-02-04T23:44:16.3892868Z ..............................................................i...............i..................... 4900/9586
2020-02-04T23:44:24.2939661Z .................................................................................................... 5000/9586
2020-02-04T23:44:32.7403768Z .................................................................................................... 5100/9586
2020-02-04T23:44:37.4332358Z .....i.............................................................................................. 5200/9586
2020-02-04T23:44:48.5006875Z ...............................................................................ii.ii........i...i... 5300/9586
2020-02-04T23:44:56.9172264Z .................i.................................................................................. 5500/9586
2020-02-04T23:45:05.8812633Z .................................................................................................... 5600/9586
2020-02-04T23:45:12.5123138Z ..................................................................i................................. 5700/9586
2020-02-04T23:45:19.4958642Z .................................................................................................... 5800/9586
2020-02-04T23:45:19.4958642Z .................................................................................................... 5800/9586
2020-02-04T23:45:26.4449478Z .................................................................................................... 5900/9586
2020-02-04T23:45:35.8695923Z .........................................................ii...i..ii...........i..................... 6000/9586
2020-02-04T23:45:57.0624974Z .................................................................................................... 6200/9586
2020-02-04T23:46:03.7417280Z .................................................................................................... 6300/9586
2020-02-04T23:46:03.7417280Z .................................................................................................... 6300/9586
2020-02-04T23:46:07.7498436Z .....................................................................................i..ii.......... 6400/9586
2020-02-04T23:46:30.3391928Z .................................................................................................... 6600/9586
2020-02-04T23:46:39.4622762Z ........................................................................i........................... 6700/9586
2020-02-04T23:46:41.6244498Z .................................................................................................... 6800/9586
2020-02-04T23:46:43.8250370Z ..........................................................................i......................... 6900/9586
---
2020-02-04T23:48:20.4251787Z .................................................................................................... 7600/9586
2020-02-04T23:48:25.2101781Z .................................................................................................... 7700/9586
2020-02-04T23:48:32.2922031Z .................................................................................................... 7800/9586
2020-02-04T23:48:41.0241391Z .................................................................................................... 7900/9586
2020-02-04T23:48:48.4257285Z ...................................iiiiiii.i........................................................ 8000/9586
2020-02-04T23:49:02.9641543Z .................................................................................................... 8200/9586
2020-02-04T23:49:11.4242756Z .................................................................................................... 8300/9586
2020-02-04T23:49:25.8659954Z .................................................................................................... 8400/9586
2020-02-04T23:49:33.5123931Z .................................................................................................... 8500/9586
---
2020-02-04T23:51:28.3864334Z ---- [ui] ui/consts/const-int-arithmetic.rs stdout ----
2020-02-04T23:51:28.3864609Z 
2020-02-04T23:51:28.3865004Z error: test compilation failed although it shouldn't!
2020-02-04T23:51:28.3865178Z status: exit code: 1
2020-02-04T23:51:28.3866148Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-arithmetic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-arithmetic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-arithmetic/auxiliary"
2020-02-04T23:51:28.3866734Z ------------------------------------------
2020-02-04T23:51:28.3866882Z 
2020-02-04T23:51:28.3867220Z ------------------------------------------
2020-02-04T23:51:28.3867400Z stderr:
2020-02-04T23:51:28.3867400Z stderr:
2020-02-04T23:51:28.3867733Z ------------------------------------------
2020-02-04T23:51:28.3867922Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3868305Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3868474Z    |
2020-02-04T23:51:28.3868643Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3868944Z ...
2020-02-04T23:51:28.3868944Z ...
2020-02-04T23:51:28.3869105Z LL | / suite! {
2020-02-04T23:51:28.3869254Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3869585Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3869767Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3869912Z ...  |
2020-02-04T23:51:28.3870280Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3870830Z    | |_- in this macro invocation
2020-02-04T23:51:28.3870975Z 
2020-02-04T23:51:28.3871268Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3874426Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3874426Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3874979Z    |
2020-02-04T23:51:28.3875146Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3875459Z ...
2020-02-04T23:51:28.3875459Z ...
2020-02-04T23:51:28.3875607Z LL | / suite! {
2020-02-04T23:51:28.3875756Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3875925Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3876086Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3876230Z ...  |
2020-02-04T23:51:28.3876626Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3877114Z    | |_- in this macro invocation
2020-02-04T23:51:28.3877274Z 
2020-02-04T23:51:28.3877432Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3877820Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3877820Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3878001Z    |
2020-02-04T23:51:28.3878152Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3878481Z ...
2020-02-04T23:51:28.3878481Z ...
2020-02-04T23:51:28.3878625Z LL | / suite! {
2020-02-04T23:51:28.3878772Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3878938Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3879086Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3879245Z ...  |
2020-02-04T23:51:28.3879591Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3880089Z    | |_- in this macro invocation
2020-02-04T23:51:28.3880233Z 
2020-02-04T23:51:28.3880391Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3880772Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3880772Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3880936Z    |
2020-02-04T23:51:28.3881086Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3881412Z ...
2020-02-04T23:51:28.3881412Z ...
2020-02-04T23:51:28.3881555Z LL | / suite! {
2020-02-04T23:51:28.3883674Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3883955Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3884106Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3884276Z ...  |
2020-02-04T23:51:28.3884745Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3885273Z    | |_- in this macro invocation
2020-02-04T23:51:28.3885417Z 
2020-02-04T23:51:28.3885577Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3885963Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3885963Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3886126Z    |
2020-02-04T23:51:28.3886276Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3886595Z ...
2020-02-04T23:51:28.3886595Z ...
2020-02-04T23:51:28.3886767Z LL | / suite! {
2020-02-04T23:51:28.3886918Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3887070Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3887298Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3887539Z ...  |
2020-02-04T23:51:28.3887956Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3890768Z    | |_- in this macro invocation
2020-02-04T23:51:28.3890808Z 
2020-02-04T23:51:28.3890864Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3891149Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3891149Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3891200Z    |
2020-02-04T23:51:28.3891253Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3891368Z ...
2020-02-04T23:51:28.3891368Z ...
2020-02-04T23:51:28.3891413Z LL | / suite! {
2020-02-04T23:51:28.3891479Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3891600Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3891650Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3891711Z ...  |
2020-02-04T23:51:28.3891946Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3892206Z    | |_- in this macro invocation
2020-02-04T23:51:28.3892254Z 
2020-02-04T23:51:28.3892320Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3892574Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3892574Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3892642Z    |
2020-02-04T23:51:28.3892692Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3892791Z ...
2020-02-04T23:51:28.3892791Z ...
2020-02-04T23:51:28.3892851Z LL | / suite! {
2020-02-04T23:51:28.3894565Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3894684Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3894760Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3894806Z ...  |
2020-02-04T23:51:28.3895054Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3895329Z    | |_- in this macro invocation
2020-02-04T23:51:28.3895361Z 
2020-02-04T23:51:28.3895425Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3895696Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3895696Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3895747Z    |
2020-02-04T23:51:28.3895796Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3895913Z ...
2020-02-04T23:51:28.3895913Z ...
2020-02-04T23:51:28.3895957Z LL | / suite! {
2020-02-04T23:51:28.3896019Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3896071Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3896120Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3896174Z ...  |
2020-02-04T23:51:28.3896415Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3896673Z    | |_- in this macro invocation
2020-02-04T23:51:28.3896706Z 
2020-02-04T23:51:28.3896778Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3897039Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3897039Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3897089Z    |
2020-02-04T23:51:28.3897156Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3897254Z ...
2020-02-04T23:51:28.3897254Z ...
2020-02-04T23:51:28.3897315Z LL | / suite! {
2020-02-04T23:51:28.3897364Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3897415Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3897480Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3897526Z ...  |
2020-02-04T23:51:28.3897757Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3898033Z    | |_- in this macro invocation
2020-02-04T23:51:28.3898065Z 
2020-02-04T23:51:28.3898120Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3898385Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3898385Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3898512Z    |
2020-02-04T23:51:28.3898566Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3898680Z ...
2020-02-04T23:51:28.3898680Z ...
2020-02-04T23:51:28.3898724Z LL | / suite! {
2020-02-04T23:51:28.3898773Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3898837Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3898886Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3898932Z ...  |
2020-02-04T23:51:28.3899178Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3900644Z    | |_- in this macro invocation
2020-02-04T23:51:28.3900679Z 
2020-02-04T23:51:28.3900754Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3901009Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3901009Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3901060Z    |
2020-02-04T23:51:28.3901136Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3901237Z ...
2020-02-04T23:51:28.3901237Z ...
2020-02-04T23:51:28.3901296Z LL | / suite! {
2020-02-04T23:51:28.3901344Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3901395Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3901443Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3901504Z ...  |
2020-02-04T23:51:28.3901727Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3902000Z    | |_- in this macro invocation
2020-02-04T23:51:28.3902041Z 
2020-02-04T23:51:28.3902097Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3902366Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3902366Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3902417Z    |
2020-02-04T23:51:28.3902467Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3902590Z ...
2020-02-04T23:51:28.3902590Z ...
2020-02-04T23:51:28.3902635Z LL | / suite! {
2020-02-04T23:51:28.3902682Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3902748Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3902798Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3902844Z ...  |
2020-02-04T23:51:28.3903081Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3903339Z    | |_- in this macro invocation
2020-02-04T23:51:28.3903372Z 
2020-02-04T23:51:28.3903442Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3903704Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3903704Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3903755Z    |
2020-02-04T23:51:28.3903820Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3903919Z ...
2020-02-04T23:51:28.3903919Z ...
2020-02-04T23:51:28.3903971Z LL | / suite! {
2020-02-04T23:51:28.3904036Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3904084Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3904134Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3904194Z ...  |
2020-02-04T23:51:28.3904416Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3904686Z    | |_- in this macro invocation
2020-02-04T23:51:28.3904719Z 
2020-02-04T23:51:28.3904775Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3905036Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3905036Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3905102Z    |
2020-02-04T23:51:28.3905152Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3905264Z ...
2020-02-04T23:51:28.3905264Z ...
2020-02-04T23:51:28.3905308Z LL | / suite! {
2020-02-04T23:51:28.3905905Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3905986Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3906035Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3906080Z ...  |
2020-02-04T23:51:28.3906638Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3906918Z    | |_- in this macro invocation
2020-02-04T23:51:28.3906950Z 
2020-02-04T23:51:28.3907021Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3907273Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3907273Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3907393Z    |
2020-02-04T23:51:28.3907444Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3907557Z ...
2020-02-04T23:51:28.3907557Z ...
2020-02-04T23:51:28.3907602Z LL | / suite! {
2020-02-04T23:51:28.3907666Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3907726Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3907776Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3907837Z ...  |
2020-02-04T23:51:28.3908412Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3908822Z    | |_- in this macro invocation
2020-02-04T23:51:28.3908870Z 
2020-02-04T23:51:28.3908928Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3909342Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3909342Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3910765Z    |
2020-02-04T23:51:28.3910830Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3910946Z ...
2020-02-04T23:51:28.3910946Z ...
2020-02-04T23:51:28.3910991Z LL | / suite! {
2020-02-04T23:51:28.3911040Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3911090Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3911154Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3911208Z ...  |
2020-02-04T23:51:28.3911466Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3911741Z    | |_- in this macro invocation
2020-02-04T23:51:28.3911775Z 
2020-02-04T23:51:28.3911829Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3912100Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3912100Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3912151Z    |
2020-02-04T23:51:28.3912248Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3912370Z ...
2020-02-04T23:51:28.3912370Z ...
2020-02-04T23:51:28.3912416Z LL | / suite! {
2020-02-04T23:51:28.3912479Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3912528Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3912577Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3912637Z ...  |
2020-02-04T23:51:28.3912869Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3913126Z    | |_- in this macro invocation
2020-02-04T23:51:28.3913173Z 
2020-02-04T23:51:28.3913230Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3913481Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3913481Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3913548Z    |
2020-02-04T23:51:28.3913599Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3913723Z ...
2020-02-04T23:51:28.3913723Z ...
2020-02-04T23:51:28.3913768Z LL | / suite! {
2020-02-04T23:51:28.3913817Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3913868Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3913934Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3913980Z ...  |
2020-02-04T23:51:28.3914203Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3914595Z    | |_- in this macro invocation
2020-02-04T23:51:28.3914630Z 
2020-02-04T23:51:28.3914688Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3914966Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3914966Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3915015Z    |
2020-02-04T23:51:28.3915067Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3915183Z ...
2020-02-04T23:51:28.3915183Z ...
2020-02-04T23:51:28.3915227Z LL | / suite! {
2020-02-04T23:51:28.3915352Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3915402Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3915452Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3915498Z ...  |
2020-02-04T23:51:28.3915745Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3916005Z    | |_- in this macro invocation
2020-02-04T23:51:28.3916062Z 
2020-02-04T23:51:28.3916117Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3916370Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3916370Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3916436Z    |
2020-02-04T23:51:28.3916487Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3916586Z ...
2020-02-04T23:51:28.3916586Z ...
2020-02-04T23:51:28.3916646Z LL | / suite! {
2020-02-04T23:51:28.3916693Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3916751Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3916816Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3916862Z ...  |
2020-02-04T23:51:28.3917082Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3917355Z    | |_- in this macro invocation
2020-02-04T23:51:28.3917388Z 
2020-02-04T23:51:28.3917453Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3917722Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3917722Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3917773Z    |
2020-02-04T23:51:28.3917822Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3917935Z ...
2020-02-04T23:51:28.3917935Z ...
2020-02-04T23:51:28.3917979Z LL | / suite! {
2020-02-04T23:51:28.3918026Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3918091Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3918149Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3918194Z ...  |
2020-02-04T23:51:28.3918430Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3918686Z    | |_- in this macro invocation
2020-02-04T23:51:28.3918718Z 
2020-02-04T23:51:28.3918790Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3919051Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3919051Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3919101Z    |
2020-02-04T23:51:28.3919167Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3919263Z ...
2020-02-04T23:51:28.3919263Z ...
2020-02-04T23:51:28.3919323Z LL | / suite! {
2020-02-04T23:51:28.3919371Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3919421Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3919485Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3919539Z ...  |
2020-02-04T23:51:28.3919762Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3920035Z    | |_- in this macro invocation
2020-02-04T23:51:28.3920067Z 
2020-02-04T23:51:28.3920122Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3920468Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3920468Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3920523Z    |
2020-02-04T23:51:28.3920573Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3920686Z ...
2020-02-04T23:51:28.3920686Z ...
2020-02-04T23:51:28.3920730Z LL | / suite! {
2020-02-04T23:51:28.3920778Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3920844Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3920892Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3920937Z ...  |
2020-02-04T23:51:28.3921183Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3921504Z    | |_- in this macro invocation
2020-02-04T23:51:28.3921537Z 
2020-02-04T23:51:28.3921611Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3921864Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3921864Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3921914Z    |
2020-02-04T23:51:28.3921989Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3922087Z ...
2020-02-04T23:51:28.3922087Z ...
2020-02-04T23:51:28.3922147Z LL | / suite! {
2020-02-04T23:51:28.3922195Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3922245Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3922294Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3922354Z ...  |
2020-02-04T23:51:28.3922576Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3922855Z    | |_- in this macro invocation
2020-02-04T23:51:28.3922888Z 
2020-02-04T23:51:28.3922943Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3923195Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3923195Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3923261Z    |
2020-02-04T23:51:28.3924770Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3924911Z ...
2020-02-04T23:51:28.3924911Z ...
2020-02-04T23:51:28.3924956Z LL | / suite! {
2020-02-04T23:51:28.3925006Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3925071Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3925121Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3925168Z ...  |
2020-02-04T23:51:28.3925439Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3925698Z    | |_- in this macro invocation
2020-02-04T23:51:28.3925731Z 
2020-02-04T23:51:28.3925811Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3926064Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3926064Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3926116Z    |
2020-02-04T23:51:28.3926183Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3926292Z ...
2020-02-04T23:51:28.3926292Z ...
2020-02-04T23:51:28.3926338Z LL | / suite! {
2020-02-04T23:51:28.3926401Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3926450Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3926500Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3926560Z ...  |
2020-02-04T23:51:28.3926784Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3927058Z    | |_- in this macro invocation
2020-02-04T23:51:28.3927091Z 
2020-02-04T23:51:28.3927146Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3927407Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3927407Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3927473Z    |
2020-02-04T23:51:28.3927523Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3927637Z ...
2020-02-04T23:51:28.3927637Z ...
2020-02-04T23:51:28.3927681Z LL | / suite! {
2020-02-04T23:51:28.3927813Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3927882Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3927933Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3927978Z ...  |
2020-02-04T23:51:28.3928210Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3928482Z    | |_- in this macro invocation
2020-02-04T23:51:28.3928514Z 
2020-02-04T23:51:28.3928585Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3928911Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3928911Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3928962Z    |
2020-02-04T23:51:28.3929012Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3929129Z ...
2020-02-04T23:51:28.3929129Z ...
2020-02-04T23:51:28.3929175Z LL | / suite! {
2020-02-04T23:51:28.3929239Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3929297Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3929348Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3929407Z ...  |
2020-02-04T23:51:28.3929631Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3929890Z    | |_- in this macro invocation
2020-02-04T23:51:28.3929938Z 
2020-02-04T23:51:28.3929995Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3930247Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3930247Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3930321Z    |
2020-02-04T23:51:28.3930372Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3930486Z ...
2020-02-04T23:51:28.3930486Z ...
2020-02-04T23:51:28.3930531Z LL | / suite! {
2020-02-04T23:51:28.3930580Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3930630Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3930701Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3930748Z ...  |
2020-02-04T23:51:28.3930972Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3931246Z    | |_- in this macro invocation
2020-02-04T23:51:28.3931278Z 
2020-02-04T23:51:28.3931335Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3931604Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3931604Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3931656Z    |
2020-02-04T23:51:28.3931707Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3931828Z ...
2020-02-04T23:51:28.3931828Z ...
2020-02-04T23:51:28.3931872Z LL | / suite! {
2020-02-04T23:51:28.3931934Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3931984Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3932034Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3932093Z ...  |
2020-02-04T23:51:28.3932323Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3932581Z    | |_- in this macro invocation
2020-02-04T23:51:28.3932628Z 
2020-02-04T23:51:28.3932685Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3932937Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3932937Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3933003Z    |
2020-02-04T23:51:28.3933055Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3933161Z ...
2020-02-04T23:51:28.3933161Z ...
2020-02-04T23:51:28.3933220Z LL | / suite! {
2020-02-04T23:51:28.3933267Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3933317Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3933382Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3933427Z ...  |
2020-02-04T23:51:28.3933650Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3933996Z    | |_- in this macro invocation
2020-02-04T23:51:28.3934028Z 
2020-02-04T23:51:28.3934084Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3934352Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3934352Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3934403Z    |
2020-02-04T23:51:28.3934453Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3934567Z ...
2020-02-04T23:51:28.3934567Z ...
2020-02-04T23:51:28.3934668Z LL | / suite! {
2020-02-04T23:51:28.3934733Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3934785Z LL | |     5i32.checked_add(2), Some(7);
2020-02-04T23:51:28.3934835Z LL | |     127i8.checked_add(2), None;
2020-02-04T23:51:28.3934881Z ...  |
2020-02-04T23:51:28.3935127Z LL | |     (-128i8).wrapping_rem(-1), 0;
2020-02-04T23:51:28.3935394Z    | |_- in this macro invocation
2020-02-04T23:51:28.3935426Z 
2020-02-04T23:51:28.3935499Z error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
2020-02-04T23:51:28.3935753Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3935753Z   --> /checkout/src/test/ui/consts/const-int-arithmetic.rs:16:34
2020-02-04T23:51:28.3935802Z    |
2020-02-04T23:51:28.3935869Z LL |           $( const _: () = assert!($expr == $result); )+
2020-02-04T23:51:28.3935969Z ...
2020-02-04T23:51:28.3935969Z ...
2020-02-04T23:51:28.3936029Z LL | / suite! {
2020-02-04T23:51:28.3936077Z LL | |     // `const_checked_int_methods`
2020-02-04T23:51:28.3936135Z LL | |     5i32.checked_add(2), Some(7);
---
2020-02-04T23:51:28.3949397Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-04T23:51:28.3949477Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T23:51:28.3949522Z 
2020-02-04T23:51:28.3949549Z 
2020-02-04T23:51:28.3951097Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-04T23:51:28.3951330Z 
2020-02-04T23:51:28.3951494Z 
2020-02-04T23:51:28.3951550Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-04T23:51:28.3951621Z Build completed unsuccessfully in 1:00:57
2020-02-04T23:51:28.3951621Z Build completed unsuccessfully in 1:00:57
2020-02-04T23:51:28.3954168Z == clock drift check ==
2020-02-04T23:51:28.3963960Z   local time: Tue Feb  4 23:51:28 UTC 2020
2020-02-04T23:51:28.9506060Z   network time: Tue, 04 Feb 2020 23:51:28 GMT
2020-02-04T23:51:28.9512127Z == end clock drift check ==
2020-02-04T23:51:29.3446184Z 
2020-02-04T23:51:29.3545807Z ##[error]Bash exited with code '1'.
2020-02-04T23:51:29.3557740Z ##[section]Finishing: Run build
2020-02-04T23:51:29.3585588Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-04T23:51:29.3587472Z Task         : Get sources
2020-02-04T23:51:29.3587525Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T23:51:29.3587579Z Version      : 1.0.0
2020-02-04T23:51:29.3587641Z Author       : Microsoft
2020-02-04T23:51:29.3587641Z Author       : Microsoft
2020-02-04T23:51:29.3587693Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T23:51:29.3587751Z ==============================================================================
2020-02-04T23:51:29.7934820Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T23:51:29.7975438Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68809/merge to s
2020-02-04T23:51:29.8085947Z Cleaning up task key
2020-02-04T23:51:29.8086785Z Start cleaning up orphan processes.
2020-02-04T23:51:29.8306915Z Terminate orphan process: pid (4571) (python)
2020-02-04T23:51:29.8592666Z ##[section]Finishing: Finalize Job
