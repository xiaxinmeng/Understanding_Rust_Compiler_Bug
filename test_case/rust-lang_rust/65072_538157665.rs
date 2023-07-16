plain
2019-10-03T22:32:05.7200805Z test [ui] ui/extern/extern-wrong-value-type.rs ... ok
2019-10-03T22:32:05.8912492Z test [ui] ui/extern/extern-vectorcall.rs ... ok
2019-10-03T22:32:05.9579108Z test [ui] ui/extern/external-doc-error.rs ... ok
2019-10-03T22:32:06.1131678Z test [ui] ui/extern/extern_fat_drop.rs ... ok
2019-10-03T22:32:06.6446729Z test [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#no ... FAILED
2019-10-03T22:32:07.8802463Z test [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat ... FAILED
2019-10-03T22:32:08.7419004Z test [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin ... FAILED
2019-10-03T22:32:08.8163430Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 ... FAILED
2019-10-03T22:32:10.2059654Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 ... FAILED
2019-10-03T22:32:10.4191074Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat2 ... FAILED
2019-10-03T22:32:10.8915988Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no0 ... FAILED
2019-10-03T22:32:11.4969655Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no1 ... FAILED
2019-10-03T22:32:12.0194816Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no2 ... FAILED
2019-10-03T22:32:12.2672943Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat3 ... FAILED
2019-10-03T22:32:12.6129597Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no3 ... FAILED
2019-10-03T22:32:14.6783825Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin0 ... FAILED
2019-10-03T22:32:15.7976280Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin1 ... FAILED
2019-10-03T22:32:17.2634802Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin2 ... FAILED
2019-10-03T22:32:17.7772728Z test [ui] ui/extoption_env-not-defined.rs ... ok
2019-10-03T22:32:17.8164189Z test [ui] ui/extoption_env-not-string-literal.rs ... ok
2019-10-03T22:32:17.8587634Z test [ui] ui/extoption_env-too-many-args.rs ... ok
2019-10-03T22:32:17.8587634Z test [ui] ui/extoption_env-too-many-args.rs ... ok
2019-10-03T22:32:17.9684258Z test [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin3 ... FAILED
2019-10-03T22:32:18.1136143Z test [ui] ui/fail-no-dead-code.rs ... ok
2019-10-03T22:32:18.1415706Z test [ui] ui/fact.rs ... ok
2019-10-03T22:32:18.1583005Z test [ui] ui/fail-simple.rs ... ok
2019-10-03T22:32:18.5327424Z test [ui] ui/fat-ptr-cast-rpass.rs ... ok
---
2019-10-03T22:42:03.1164912Z test [ui] ui/wrapping-int-combinations.rs ... ok
2019-10-03T22:42:03.1165160Z 
2019-10-03T22:42:03.1165260Z failures:
2019-10-03T22:42:03.1165292Z 
2019-10-03T22:42:03.1165624Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#no stdout ----
2019-10-03T22:42:03.1165680Z 
2019-10-03T22:42:03.1165752Z error in revision `no`: test run failed!
2019-10-03T22:42:03.1165813Z status: exit code: 101
2019-10-03T22:42:03.1166201Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.no/a.wasm"
2019-10-03T22:42:03.1166565Z ------------------------------------------
2019-10-03T22:42:03.1166606Z 
2019-10-03T22:42:03.1166992Z ------------------------------------------
2019-10-03T22:42:03.1167472Z stderr:
2019-10-03T22:42:03.1167472Z stderr:
2019-10-03T22:42:03.1167802Z ------------------------------------------
2019-10-03T22:42:03.1167882Z RuntimeError: unreachable
2019-10-03T22:42:03.1168166Z     at __rust_start_panic (wasm-function[133]:1)
2019-10-03T22:42:03.1168438Z     at rust_panic (wasm-function[126]:39)
2019-10-03T22:42:03.1168761Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[121]:342)
2019-10-03T22:42:03.1169112Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[120]:151)
2019-10-03T22:42:03.1169389Z     at rust_begin_unwind (wasm-function[119]:3)
2019-10-03T22:42:03.1169717Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[154]:76)
2019-10-03T22:42:03.1170441Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[175]:143)
2019-10-03T22:42:03.1171171Z     at _ZN3std6thread5spawn17h5f2101b1291ca61eE (wasm-function[25]:462)
2019-10-03T22:42:03.1171632Z     at _ZN52issue_64655_allow_unwind_when_calling_panic_directly4main17h25f7bacc4f4b8b71E (wasm-function[8]:91)
2019-10-03T22:42:03.1171922Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17head921094cf97d61E (wasm-function[19]:25)
2019-10-03T22:42:03.1172225Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[103]:8)
2019-10-03T22:42:03.1172495Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[118]:20)
2019-10-03T22:42:03.1172716Z     at __rust_maybe_catch_panic (wasm-function[132]:5)
2019-10-03T22:42:03.1172975Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[127]:250)
2019-10-03T22:42:03.1173217Z     at _ZN3std2rt10lang_start17hed6db8391449b206E (wasm-function[18]:42)
2019-10-03T22:42:03.1173444Z     at main (wasm-function[10]:11)
2019-10-03T22:42:03.1173662Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1173747Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1173827Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1173895Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1174141Z ------------------------------------------
2019-10-03T22:42:03.1174180Z 
2019-10-03T22:42:03.1174226Z 
2019-10-03T22:42:03.1174226Z 
2019-10-03T22:42:03.1174626Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#fat stdout ----
2019-10-03T22:42:03.1174691Z 
2019-10-03T22:42:03.1174739Z error in revision `fat`: test run failed!
2019-10-03T22:42:03.1174811Z status: exit code: 101
2019-10-03T22:42:03.1175146Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.fat/a.wasm"
2019-10-03T22:42:03.1175469Z ------------------------------------------
2019-10-03T22:42:03.1175527Z 
2019-10-03T22:42:03.1175719Z ------------------------------------------
2019-10-03T22:42:03.1175793Z stderr:
2019-10-03T22:42:03.1175793Z stderr:
2019-10-03T22:42:03.1175980Z ------------------------------------------
2019-10-03T22:42:03.1176185Z RuntimeError: unreachable
2019-10-03T22:42:03.1176419Z     at rust_panic (wasm-function[72]:1)
2019-10-03T22:42:03.1176866Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[71]:329)
2019-10-03T22:42:03.1177144Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[84]:87)
2019-10-03T22:42:03.1177990Z     at rust_begin_unwind (wasm-function[20]:3)
2019-10-03T22:42:03.1178392Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[17]:76)
2019-10-03T22:42:03.1178716Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[6]:143)
2019-10-03T22:42:03.1179062Z     at _ZN3std6thread5spawn17h5f2101b1291ca61eE (wasm-function[0]:577)
2019-10-03T22:42:03.1179439Z     at _ZN52issue_64655_allow_unwind_when_calling_panic_directly4main17h25f7bacc4f4b8b71E (wasm-function[57]:63)
2019-10-03T22:42:03.1179800Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17head921094cf97d61E (wasm-function[68]:6)
2019-10-03T22:42:03.1180187Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[58]:8)
2019-10-03T22:42:03.1180460Z     at main (wasm-function[56]:208)
2019-10-03T22:42:03.1180763Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1180850Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1180954Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1181236Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1181296Z     at tryModuleLoad (module.js:503:12)
2019-10-03T22:42:03.1181545Z     at Function.Module._load (module.js:495:3)
2019-10-03T22:42:03.1181732Z     at Function.Module.runMain (module.js:682:10)
2019-10-03T22:42:03.1181806Z     at startup (bootstrap_node.js:191:16)
2019-10-03T22:42:03.1181859Z     at bootstrap_node.js:613:3
2019-10-03T22:42:03.1182125Z ------------------------------------------
2019-10-03T22:42:03.1182180Z 
2019-10-03T22:42:03.1182214Z 
2019-10-03T22:42:03.1182214Z 
2019-10-03T22:42:03.1182438Z ---- [ui] ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.rs#thin stdout ----
2019-10-03T22:42:03.1182506Z 
2019-10-03T22:42:03.1182553Z error in revision `thin`: test run failed!
2019-10-03T22:42:03.1182625Z status: exit code: 101
2019-10-03T22:42:03.1182958Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-allow-unwind-when-calling-panic-directly.thin/a.wasm"
2019-10-03T22:42:03.1183268Z ------------------------------------------
2019-10-03T22:42:03.1183313Z 
2019-10-03T22:42:03.1183488Z ------------------------------------------
2019-10-03T22:42:03.1183559Z stderr:
2019-10-03T22:42:03.1183559Z stderr:
2019-10-03T22:42:03.1183748Z ------------------------------------------
2019-10-03T22:42:03.1183805Z RuntimeError: unreachable
2019-10-03T22:42:03.1184004Z     at rust_panic (wasm-function[142]:1)
2019-10-03T22:42:03.1184246Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[137]:333)
2019-10-03T22:42:03.1184536Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E.llvm.11272857713437629739 (wasm-function[136]:87)
2019-10-03T22:42:03.1184743Z     at rust_begin_unwind (wasm-function[135]:3)
2019-10-03T22:42:03.1184985Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[15]:76)
2019-10-03T22:42:03.1185233Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[17]:143)
2019-10-03T22:42:03.1185475Z     at _ZN3std6thread5spawn17h5f2101b1291ca61eE (wasm-function[67]:371)
2019-10-03T22:42:03.1185770Z     at _ZN52issue_64655_allow_unwind_when_calling_panic_directly4main17h25f7bacc4f4b8b71E (wasm-function[57]:73)
2019-10-03T22:42:03.1186059Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17head921094cf97d61E (wasm-function[63]:6)
2019-10-03T22:42:03.1186358Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[126]:8)
2019-10-03T22:42:03.1186892Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[143]:187)
2019-10-03T22:42:03.1187137Z     at main (wasm-function[59]:46)
2019-10-03T22:42:03.1187787Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1187880Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1187982Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1188062Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1188155Z     at tryModuleLoad (module.js:503:12)
2019-10-03T22:42:03.1188252Z     at Function.Module._load (module.js:495:3)
2019-10-03T22:42:03.1188345Z     at Function.Module.runMain (module.js:682:10)
2019-10-03T22:42:03.1188441Z     at startup (bootstrap_node.js:191:16)
2019-10-03T22:42:03.1188779Z ------------------------------------------
2019-10-03T22:42:03.1188831Z 
2019-10-03T22:42:03.1188868Z 
2019-10-03T22:42:03.1188868Z 
2019-10-03T22:42:03.1189188Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----
2019-10-03T22:42:03.1189253Z 
2019-10-03T22:42:03.1189336Z error in revision `fat0`: test run failed!
2019-10-03T22:42:03.1189411Z status: exit code: 101
2019-10-03T22:42:03.1189874Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a.wasm"
2019-10-03T22:42:03.1190257Z ------------------------------------------
2019-10-03T22:42:03.1190308Z 
2019-10-03T22:42:03.1190566Z ------------------------------------------
2019-10-03T22:42:03.1190758Z stderr:
2019-10-03T22:42:03.1190758Z stderr:
2019-10-03T22:42:03.1191401Z ------------------------------------------
2019-10-03T22:42:03.1191464Z RuntimeError: unreachable
2019-10-03T22:42:03.1191849Z     at __rust_start_panic (wasm-function[296]:1)
2019-10-03T22:42:03.1192050Z     at rust_panic (wasm-function[303]:61)
2019-10-03T22:42:03.1192290Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[273]:737)
2019-10-03T22:42:03.1192551Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[319]:231)
2019-10-03T22:42:03.1192753Z     at rust_begin_unwind (wasm-function[82]:3)
2019-10-03T22:42:03.1192996Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[79]:129)
2019-10-03T22:42:03.1193230Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[111]:242)
2019-10-03T22:42:03.1193496Z     at _ZN4core6result19Result$LT$T$C$E$GT$6expect17h8493cd1794774014E (wasm-function[239]:171)
2019-10-03T22:42:03.1193758Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[268]:112)
2019-10-03T22:42:03.1194039Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[265]:308)
2019-10-03T22:42:03.1194333Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[17]:12)
2019-10-03T22:42:03.1194616Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[377]:14)
2019-10-03T22:42:03.1194893Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[376]:28)
2019-10-03T22:42:03.1195132Z     at __rust_maybe_catch_panic (wasm-function[277]:11)
2019-10-03T22:42:03.1195378Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[199]:723)
2019-10-03T22:42:03.1195639Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[198]:72)
2019-10-03T22:42:03.1195842Z     at main (wasm-function[264]:17)
2019-10-03T22:42:03.1196082Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1196155Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1196239Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1196494Z ------------------------------------------
2019-10-03T22:42:03.1196534Z 
2019-10-03T22:42:03.1196579Z 
2019-10-03T22:42:03.1196579Z 
2019-10-03T22:42:03.1197058Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 stdout ----
2019-10-03T22:42:03.1197117Z 
2019-10-03T22:42:03.1197187Z error in revision `fat1`: test run failed!
2019-10-03T22:42:03.1197260Z status: exit code: 101
2019-10-03T22:42:03.1198372Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat1/a.wasm"
2019-10-03T22:42:03.1198773Z ------------------------------------------
2019-10-03T22:42:03.1198842Z 
2019-10-03T22:42:03.1199082Z ------------------------------------------
2019-10-03T22:42:03.1199187Z stderr:
2019-10-03T22:42:03.1199187Z stderr:
2019-10-03T22:42:03.1199427Z ------------------------------------------
2019-10-03T22:42:03.1199523Z RuntimeError: unreachable
2019-10-03T22:42:03.1199780Z     at __rust_start_panic (wasm-function[257]:1)
2019-10-03T22:42:03.1200049Z     at rust_panic (wasm-function[265]:39)
2019-10-03T22:42:03.1200375Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[264]:342)
2019-10-03T22:42:03.1200730Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[281]:151)
2019-10-03T22:42:03.1201027Z     at rust_begin_unwind (wasm-function[100]:3)
2019-10-03T22:42:03.1201477Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[97]:76)
2019-10-03T22:42:03.1201925Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[68]:143)
2019-10-03T22:42:03.1202351Z     at _ZN4core6result19Result$LT$T$C$E$GT$6expect17h8493cd1794774014E (wasm-function[6]:61)
2019-10-03T22:42:03.1202777Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[3]:54)
2019-10-03T22:42:03.1203174Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[240]:72)
2019-10-03T22:42:03.1203614Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[231]:6)
2019-10-03T22:42:03.1203919Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[339]:8)
2019-10-03T22:42:03.1204173Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[338]:20)
2019-10-03T22:42:03.1204410Z     at __rust_maybe_catch_panic (wasm-function[252]:5)
2019-10-03T22:42:03.1204667Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[229]:250)
2019-10-03T22:42:03.1205083Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[228]:42)
2019-10-03T22:42:03.1205483Z     at main (wasm-function[239]:11)
2019-10-03T22:42:03.1206072Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1206175Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1206247Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1206745Z ------------------------------------------
2019-10-03T22:42:03.1206809Z 
2019-10-03T22:42:03.1207096Z 
2019-10-03T22:42:03.1207096Z 
2019-10-03T22:42:03.1207372Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat2 stdout ----
2019-10-03T22:42:03.1207448Z 
2019-10-03T22:42:03.1207509Z error in revision `fat2`: test run failed!
2019-10-03T22:42:03.1207596Z status: exit code: 101
2019-10-03T22:42:03.1208492Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat2/a.wasm"
2019-10-03T22:42:03.1208902Z ------------------------------------------
2019-10-03T22:42:03.1208954Z 
2019-10-03T22:42:03.1209191Z ------------------------------------------
2019-10-03T22:42:03.1209283Z stderr:
2019-10-03T22:42:03.1209283Z stderr:
2019-10-03T22:42:03.1209551Z ------------------------------------------
2019-10-03T22:42:03.1209629Z RuntimeError: unreachable
2019-10-03T22:42:03.1209893Z     at rust_panic (wasm-function[73]:1)
2019-10-03T22:42:03.1210210Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[72]:329)
2019-10-03T22:42:03.1210664Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[84]:87)
2019-10-03T22:42:03.1210979Z     at rust_begin_unwind (wasm-function[20]:3)
2019-10-03T22:42:03.1211547Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[17]:76)
2019-10-03T22:42:03.1212203Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[6]:143)
2019-10-03T22:42:03.1212801Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[0]:577)
2019-10-03T22:42:03.1213451Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[60]:63)
2019-10-03T22:42:03.1213733Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[57]:6)
2019-10-03T22:42:03.1214048Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[61]:8)
2019-10-03T22:42:03.1214279Z     at main (wasm-function[59]:208)
2019-10-03T22:42:03.1214506Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1214599Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1214665Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1214744Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1214803Z     at tryModuleLoad (module.js:503:12)
2019-10-03T22:42:03.1214883Z     at Function.Module._load (module.js:495:3)
2019-10-03T22:42:03.1214964Z     at Function.Module.runMain (module.js:682:10)
2019-10-03T22:42:03.1215026Z     at startup (bootstrap_node.js:191:16)
2019-10-03T22:42:03.1215104Z     at bootstrap_node.js:613:3
2019-10-03T22:42:03.1215341Z ------------------------------------------
2019-10-03T22:42:03.1215478Z 
2019-10-03T22:42:03.1215508Z 
2019-10-03T22:42:03.1215508Z 
2019-10-03T22:42:03.1215782Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no0 stdout ----
2019-10-03T22:42:03.1215834Z 
2019-10-03T22:42:03.1215903Z error in revision `no0`: test run failed!
2019-10-03T22:42:03.1215962Z status: exit code: 101
2019-10-03T22:42:03.1216342Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no0/a.wasm"
2019-10-03T22:42:03.1216650Z ------------------------------------------
2019-10-03T22:42:03.1216690Z 
2019-10-03T22:42:03.1217300Z ------------------------------------------
2019-10-03T22:42:03.1217363Z stderr:
2019-10-03T22:42:03.1217363Z stderr:
2019-10-03T22:42:03.1217726Z ------------------------------------------
2019-10-03T22:42:03.1218121Z RuntimeError: unreachable
2019-10-03T22:42:03.1218455Z     at __rust_start_panic (wasm-function[344]:1)
2019-10-03T22:42:03.1218723Z     at rust_panic (wasm-function[336]:39)
2019-10-03T22:42:03.1219061Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[331]:342)
2019-10-03T22:42:03.1219410Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[330]:151)
2019-10-03T22:42:03.1219686Z     at rust_begin_unwind (wasm-function[329]:3)
2019-10-03T22:42:03.1220023Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[365]:76)
2019-10-03T22:42:03.1220343Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[387]:143)
2019-10-03T22:42:03.1220700Z     at _ZN4core6result19Result$LT$T$C$E$GT$6expect17h8493cd1794774014E (wasm-function[112]:146)
2019-10-03T22:42:03.1221030Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[153]:124)
2019-10-03T22:42:03.1221681Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[140]:582)
2019-10-03T22:42:03.1222169Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[59]:12)
2019-10-03T22:42:03.1222495Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[313]:8)
2019-10-03T22:42:03.1222801Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[328]:20)
2019-10-03T22:42:03.1223185Z     at __rust_maybe_catch_panic (wasm-function[343]:5)
2019-10-03T22:42:03.1223495Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[337]:250)
2019-10-03T22:42:03.1223783Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[58]:72)
2019-10-03T22:42:03.1224009Z     at main (wasm-function[144]:17)
2019-10-03T22:42:03.1224600Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1224673Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1224760Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1225108Z ------------------------------------------
2019-10-03T22:42:03.1225161Z 
2019-10-03T22:42:03.1225192Z 
2019-10-03T22:42:03.1225192Z 
2019-10-03T22:42:03.1225610Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no1 stdout ----
2019-10-03T22:42:03.1225662Z 
2019-10-03T22:42:03.1225731Z error in revision `no1`: test run failed!
2019-10-03T22:42:03.1225968Z status: exit code: 101
2019-10-03T22:42:03.1226366Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no1/a.wasm"
2019-10-03T22:42:03.1226695Z ------------------------------------------
2019-10-03T22:42:03.1226936Z 
2019-10-03T22:42:03.1227144Z ------------------------------------------
2019-10-03T22:42:03.1227239Z stderr:
2019-10-03T22:42:03.1227239Z stderr:
2019-10-03T22:42:03.1227444Z ------------------------------------------
2019-10-03T22:42:03.1227694Z RuntimeError: unreachable
2019-10-03T22:42:03.1228235Z     at __rust_start_panic (wasm-function[300]:1)
2019-10-03T22:42:03.1228660Z     at rust_panic (wasm-function[293]:39)
2019-10-03T22:42:03.1228978Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[288]:342)
2019-10-03T22:42:03.1229328Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[287]:151)
2019-10-03T22:42:03.1229620Z     at rust_begin_unwind (wasm-function[286]:3)
2019-10-03T22:42:03.1229937Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[321]:76)
2019-10-03T22:42:03.1230278Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[343]:143)
2019-10-03T22:42:03.1230618Z     at _ZN4core6result19Result$LT$T$C$E$GT$6expect17h8493cd1794774014E (wasm-function[129]:61)
2019-10-03T22:42:03.1230950Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[127]:54)
2019-10-03T22:42:03.1231453Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[86]:92)
2019-10-03T22:42:03.1231896Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[63]:6)
2019-10-03T22:42:03.1232200Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[270]:8)
2019-10-03T22:42:03.1232454Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[285]:20)
2019-10-03T22:42:03.1232698Z     at __rust_maybe_catch_panic (wasm-function[299]:5)
2019-10-03T22:42:03.1232944Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[294]:250)
2019-10-03T22:42:03.1233204Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[62]:42)
2019-10-03T22:42:03.1233418Z     at main (wasm-function[88]:11)
2019-10-03T22:42:03.1233636Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1233719Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1233782Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1234027Z ------------------------------------------
2019-10-03T22:42:03.1234090Z 
2019-10-03T22:42:03.1234119Z 
2019-10-03T22:42:03.1234119Z 
2019-10-03T22:42:03.1234344Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no2 stdout ----
2019-10-03T22:42:03.1234409Z 
2019-10-03T22:42:03.1234459Z error in revision `no2`: test run failed!
2019-10-03T22:42:03.1234534Z status: exit code: 101
2019-10-03T22:42:03.1235127Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no2/a.wasm"
2019-10-03T22:42:03.1235476Z ------------------------------------------
2019-10-03T22:42:03.1235518Z 
2019-10-03T22:42:03.1235698Z ------------------------------------------
2019-10-03T22:42:03.1235771Z stderr:
2019-10-03T22:42:03.1235771Z stderr:
2019-10-03T22:42:03.1235965Z ------------------------------------------
2019-10-03T22:42:03.1236023Z RuntimeError: unreachable
2019-10-03T22:42:03.1236234Z     at __rust_start_panic (wasm-function[142]:1)
2019-10-03T22:42:03.1236437Z     at rust_panic (wasm-function[135]:39)
2019-10-03T22:42:03.1236865Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[130]:342)
2019-10-03T22:42:03.1237124Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[129]:151)
2019-10-03T22:42:03.1237890Z     at rust_begin_unwind (wasm-function[128]:3)
2019-10-03T22:42:03.1238248Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[163]:76)
2019-10-03T22:42:03.1238575Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[185]:143)
2019-10-03T22:42:03.1238902Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[29]:462)
2019-10-03T22:42:03.1239248Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[14]:91)
2019-10-03T22:42:03.1239639Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[9]:25)
2019-10-03T22:42:03.1240021Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[112]:8)
2019-10-03T22:42:03.1240533Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[127]:20)
2019-10-03T22:42:03.1240859Z     at __rust_maybe_catch_panic (wasm-function[141]:5)
2019-10-03T22:42:03.1241307Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[136]:250)
2019-10-03T22:42:03.1241740Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[8]:42)
2019-10-03T22:42:03.1241936Z     at main (wasm-function[16]:11)
2019-10-03T22:42:03.1242166Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1242247Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1242309Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1242385Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1242623Z ------------------------------------------
2019-10-03T22:42:03.1242661Z 
2019-10-03T22:42:03.1242689Z 
2019-10-03T22:42:03.1242689Z 
2019-10-03T22:42:03.1242934Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat3 stdout ----
2019-10-03T22:42:03.1242981Z 
2019-10-03T22:42:03.1243046Z error in revision `fat3`: test run failed!
2019-10-03T22:42:03.1243103Z status: exit code: 101
2019-10-03T22:42:03.1243456Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat3/a.wasm"
2019-10-03T22:42:03.1243751Z ------------------------------------------
2019-10-03T22:42:03.1243790Z 
2019-10-03T22:42:03.1243984Z ------------------------------------------
2019-10-03T22:42:03.1244040Z stderr:
2019-10-03T22:42:03.1244040Z stderr:
2019-10-03T22:42:03.1244235Z ------------------------------------------
2019-10-03T22:42:03.1244292Z RuntimeError: unreachable
2019-10-03T22:42:03.1244488Z     at rust_panic (wasm-function[69]:1)
2019-10-03T22:42:03.1244743Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[68]:329)
2019-10-03T22:42:03.1245005Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[80]:87)
2019-10-03T22:42:03.1245228Z     at rust_begin_unwind (wasm-function[20]:3)
2019-10-03T22:42:03.1245458Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[17]:76)
2019-10-03T22:42:03.1245802Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[6]:143)
2019-10-03T22:42:03.1246068Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[0]:577)
2019-10-03T22:42:03.1246347Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[56]:63)
2019-10-03T22:42:03.1246651Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[53]:6)
2019-10-03T22:42:03.1247103Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[57]:8)
2019-10-03T22:42:03.1247849Z     at main (wasm-function[55]:208)
2019-10-03T22:42:03.1248175Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1248281Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1248381Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1248460Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1248552Z     at tryModuleLoad (module.js:503:12)
2019-10-03T22:42:03.1248639Z     at Function.Module._load (module.js:495:3)
2019-10-03T22:42:03.1248739Z     at Function.Module.runMain (module.js:682:10)
2019-10-03T22:42:03.1248816Z     at startup (bootstrap_node.js:191:16)
2019-10-03T22:42:03.1248908Z     at bootstrap_node.js:613:3
2019-10-03T22:42:03.1249222Z ------------------------------------------
2019-10-03T22:42:03.1249274Z 
2019-10-03T22:42:03.1249311Z 
2019-10-03T22:42:03.1249311Z 
2019-10-03T22:42:03.1249617Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#no3 stdout ----
2019-10-03T22:42:03.1249680Z 
2019-10-03T22:42:03.1249771Z error in revision `no3`: test run failed!
2019-10-03T22:42:03.1249957Z status: exit code: 101
2019-10-03T22:42:03.1250445Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.no3/a.wasm"
2019-10-03T22:42:03.1250990Z ------------------------------------------
2019-10-03T22:42:03.1251040Z 
2019-10-03T22:42:03.1251413Z ------------------------------------------
2019-10-03T22:42:03.1251472Z stderr:
2019-10-03T22:42:03.1251472Z stderr:
2019-10-03T22:42:03.1251676Z ------------------------------------------
2019-10-03T22:42:03.1251735Z RuntimeError: unreachable
2019-10-03T22:42:03.1251952Z     at __rust_start_panic (wasm-function[142]:1)
2019-10-03T22:42:03.1252168Z     at rust_panic (wasm-function[135]:39)
2019-10-03T22:42:03.1252417Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[130]:342)
2019-10-03T22:42:03.1252688Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E (wasm-function[129]:151)
2019-10-03T22:42:03.1252913Z     at rust_begin_unwind (wasm-function[128]:3)
2019-10-03T22:42:03.1253166Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[163]:76)
2019-10-03T22:42:03.1253413Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[185]:143)
2019-10-03T22:42:03.1253678Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[29]:462)
2019-10-03T22:42:03.1254129Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[14]:91)
2019-10-03T22:42:03.1254401Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[9]:25)
2019-10-03T22:42:03.1254699Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[112]:8)
2019-10-03T22:42:03.1254953Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[127]:20)
2019-10-03T22:42:03.1255191Z     at __rust_maybe_catch_panic (wasm-function[141]:5)
2019-10-03T22:42:03.1255463Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[136]:250)
2019-10-03T22:42:03.1255708Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[8]:42)
2019-10-03T22:42:03.1255925Z     at main (wasm-function[16]:11)
2019-10-03T22:42:03.1256146Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1256298Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1256369Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1256446Z     at Module.load (module.js:560:32)
2019-10-03T22:42:03.1256886Z ------------------------------------------
2019-10-03T22:42:03.1256930Z 
2019-10-03T22:42:03.1256960Z 
2019-10-03T22:42:03.1256960Z 
2019-10-03T22:42:03.1257207Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin0 stdout ----
2019-10-03T22:42:03.1257257Z 
2019-10-03T22:42:03.1258101Z error in revision `thin0`: test run failed!
2019-10-03T22:42:03.1258202Z status: exit code: 101
2019-10-03T22:42:03.1258759Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin0/a.wasm"
2019-10-03T22:42:03.1266088Z ------------------------------------------
2019-10-03T22:42:03.1266154Z 
2019-10-03T22:42:03.1266838Z ------------------------------------------
2019-10-03T22:42:03.1266940Z stderr:
2019-10-03T22:42:03.1266940Z stderr:
2019-10-03T22:42:03.1267679Z ------------------------------------------
2019-10-03T22:42:03.1267786Z RuntimeError: unreachable
2019-10-03T22:42:03.1268059Z     at __rust_start_panic (wasm-function[280]:1)
2019-10-03T22:42:03.1268333Z     at rust_panic (wasm-function[367]:61)
2019-10-03T22:42:03.1268655Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[362]:737)
2019-10-03T22:42:03.1269046Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E.llvm.11272857713437629739 (wasm-function[361]:231)
2019-10-03T22:42:03.1269605Z     at rust_begin_unwind (wasm-function[360]:3)
2019-10-03T22:42:03.1269917Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[28]:129)
2019-10-03T22:42:03.1270256Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[31]:242)
2019-10-03T22:42:03.1270608Z     at _ZN4core6result19Result$LT$T$C$E$GT$6expect17h8493cd1794774014E (wasm-function[169]:165)
2019-10-03T22:42:03.1271229Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[210]:112)
2019-10-03T22:42:03.1271500Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[197]:174)
2019-10-03T22:42:03.1271785Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[128]:12)
2019-10-03T22:42:03.1272084Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[344]:14)
2019-10-03T22:42:03.1272342Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[359]:28)
2019-10-03T22:42:03.1272593Z     at __rust_maybe_catch_panic (wasm-function[279]:11)
2019-10-03T22:42:03.1272856Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[368]:723)
2019-10-03T22:42:03.1273277Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[127]:72)
2019-10-03T22:42:03.1273508Z     at main (wasm-function[201]:17)
2019-10-03T22:42:03.1273927Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1274017Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1274088Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1274352Z ------------------------------------------
2019-10-03T22:42:03.1274410Z 
2019-10-03T22:42:03.1274441Z 
2019-10-03T22:42:03.1274441Z 
2019-10-03T22:42:03.1274679Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin1 stdout ----
2019-10-03T22:42:03.1274748Z 
2019-10-03T22:42:03.1274803Z error in revision `thin1`: test run failed!
2019-10-03T22:42:03.1274882Z status: exit code: 101
2019-10-03T22:42:03.1275260Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin1/a.wasm"
2019-10-03T22:42:03.1275596Z ------------------------------------------
2019-10-03T22:42:03.1275638Z 
2019-10-03T22:42:03.1275919Z ------------------------------------------
2019-10-03T22:42:03.1276009Z stderr:
2019-10-03T22:42:03.1276009Z stderr:
2019-10-03T22:42:03.1276247Z ------------------------------------------
2019-10-03T22:42:03.1276312Z RuntimeError: unreachable
2019-10-03T22:42:03.1276549Z     at __rust_start_panic (wasm-function[237]:1)
2019-10-03T22:42:03.1276937Z     at rust_panic (wasm-function[324]:39)
2019-10-03T22:42:03.1277257Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[319]:342)
2019-10-03T22:42:03.1277953Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E.llvm.11272857713437629739 (wasm-function[318]:151)
2019-10-03T22:42:03.1278399Z     at rust_begin_unwind (wasm-function[317]:3)
2019-10-03T22:42:03.1278727Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[28]:76)
2019-10-03T22:42:03.1279048Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[31]:143)
2019-10-03T22:42:03.1279445Z     at _ZN4core6result19Result$LT$T$C$E$GT$6expect17h8493cd1794774014E.llvm.12875966267384445257 (wasm-function[170]:61)
2019-10-03T22:42:03.1279776Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[168]:54)
2019-10-03T22:42:03.1280139Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[159]:70)
2019-10-03T22:42:03.1280515Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[148]:6)
2019-10-03T22:42:03.1280877Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[301]:8)
2019-10-03T22:42:03.1281582Z     at _ZN3std9panicking3try7do_call17h65e045ba6943076cE (wasm-function[316]:20)
2019-10-03T22:42:03.1282249Z     at __rust_maybe_catch_panic (wasm-function[236]:5)
2019-10-03T22:42:03.1282513Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[325]:250)
2019-10-03T22:42:03.1282929Z     at _ZN3std2rt10lang_start17h448279cfdf6c09bdE (wasm-function[147]:42)
2019-10-03T22:42:03.1283158Z     at main (wasm-function[161]:11)
2019-10-03T22:42:03.1283399Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1283467Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1283547Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1283795Z ------------------------------------------
2019-10-03T22:42:03.1283835Z 
2019-10-03T22:42:03.1283862Z 
2019-10-03T22:42:03.1283862Z 
2019-10-03T22:42:03.1284104Z ---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#thin2 stdout ----
2019-10-03T22:42:03.1284154Z 
2019-10-03T22:42:03.1284222Z error in revision `thin2`: test run failed!
2019-10-03T22:42:03.1284453Z status: exit code: 101
2019-10-03T22:42:03.1284821Z command: "/node-v9.2.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.thin2/a.wasm"
2019-10-03T22:42:03.1285130Z ------------------------------------------
2019-10-03T22:42:03.1285170Z 
2019-10-03T22:42:03.1285374Z ------------------------------------------
2019-10-03T22:42:03.1285436Z stderr:
2019-10-03T22:42:03.1285436Z stderr:
2019-10-03T22:42:03.1285640Z ------------------------------------------
2019-10-03T22:42:03.1285700Z RuntimeError: unreachable
2019-10-03T22:42:03.1285908Z     at rust_panic (wasm-function[144]:1)
2019-10-03T22:42:03.1286173Z     at _ZN3std9panicking20rust_panic_with_hook17h95bd8bb26eec786eE (wasm-function[139]:333)
2019-10-03T22:42:03.1286458Z     at _ZN3std9panicking18continue_panic_fmt17hb70a38b909117a00E.llvm.11272857713437629739 (wasm-function[138]:87)
2019-10-03T22:42:03.1286897Z     at rust_begin_unwind (wasm-function[137]:3)
2019-10-03T22:42:03.1287735Z     at _ZN4core9panicking9panic_fmt17h00808d6c375ce150E (wasm-function[15]:76)
2019-10-03T22:42:03.1288094Z     at _ZN4core6result13unwrap_failed17h3360d98b5655c6b3E (wasm-function[18]:143)
2019-10-03T22:42:03.1288440Z     at _ZN3std6thread5spawn17h81579634d7a5376aE (wasm-function[68]:371)
2019-10-03T22:42:03.1288918Z     at _ZN41issue_64655_extern_rust_must_allow_unwind4main17h8227ded484700a79E (wasm-function[63]:73)
2019-10-03T22:42:03.1289359Z     at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h55cfc7346c142788E (wasm-function[58]:6)
2019-10-03T22:42:03.1289745Z     at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h035b275d993bf9aeE (wasm-function[128]:8)
2019-10-03T22:42:03.1290125Z     at _ZN3std2rt19lang_start_internal17hb0408182365c6965E (wasm-function[145]:187)
2019-10-03T22:42:03.1290423Z     at main (wasm-function[65]:46)
2019-10-03T22:42:03.1290727Z     at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
2019-10-03T22:42:03.1290997Z     at Module._compile (module.js:641:30)
2019-10-03T22:42:03.1291065Z     at Object.Module._extensions..js (module.js:652:10)
2019-10-03T22:42:03.1291147Z     at Module.load (module.js:560:32)
