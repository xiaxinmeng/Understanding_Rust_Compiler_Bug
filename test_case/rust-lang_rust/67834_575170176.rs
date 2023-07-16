plain
2020-01-16T13:11:04.3054675Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T13:11:04.3162648Z ##[command]git config gc.auto 0
2020-01-16T13:11:04.3216679Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T13:11:04.3273361Z ##[command]git config --get-all http.proxy
2020-01-16T13:11:04.3410282Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67834/merge:refs/remotes/pull/67834/merge
---
2020-01-16T14:03:05.1448566Z .................................................................................................... 1700/9524
2020-01-16T14:03:13.2237053Z .................................................................................................... 1800/9524
2020-01-16T14:03:22.3131395Z ...........i........................................................................................ 1900/9524
2020-01-16T14:03:29.3129062Z .................................................................................................... 2000/9524
2020-01-16T14:03:45.4520769Z .iiiii.............................................................................................. 2100/9524
2020-01-16T14:03:53.5864065Z .................................................................................................... 2300/9524
2020-01-16T14:03:55.9485554Z .................................................................................................... 2400/9524
2020-01-16T14:04:01.4680338Z .................................................................................................... 2500/9524
2020-01-16T14:04:21.5800753Z .................................................................................................... 2600/9524
---
2020-01-16T14:06:56.6872583Z ............................................i...............i....................................... 4900/9524
2020-01-16T14:07:05.2517776Z .................................................................................................... 5000/9524
2020-01-16T14:07:12.0419783Z .......................................................................................i............ 5100/9524
2020-01-16T14:07:17.3105915Z .................................................................................................... 5200/9524
2020-01-16T14:07:27.9656064Z ...........................................................ii.ii...........i........................ 5300/9524
2020-01-16T14:07:37.1426503Z .................................................................................................... 5500/9524
2020-01-16T14:07:47.1131159Z .................................................................................................... 5600/9524
2020-01-16T14:07:53.6124445Z ............................................i....................................................... 5700/9524
2020-01-16T14:08:00.4591643Z .................................................................................................... 5800/9524
2020-01-16T14:08:00.4591643Z .................................................................................................... 5800/9524
2020-01-16T14:08:10.9202029Z .................................................................................................... 5900/9524
2020-01-16T14:08:20.1626630Z ...................................ii...i..ii...........i........................................... 6000/9524
2020-01-16T14:08:39.4831813Z .................................................................................................... 6200/9524
2020-01-16T14:08:46.9756350Z .................................................................................................... 6300/9524
2020-01-16T14:08:52.6523701Z ................................................................i.ii................................ 6400/9524
2020-01-16T14:09:05.4295072Z .................................................................................................... 6500/9524
---
2020-01-16T14:11:02.3922576Z .................................................................................................... 7500/9524
2020-01-16T14:11:06.7346364Z .................................................................................................... 7600/9524
2020-01-16T14:11:12.5984179Z .................................................................................................... 7700/9524
2020-01-16T14:11:19.2724173Z .................................................................................................... 7800/9524
2020-01-16T14:11:29.7121987Z ........................................................................................iiii........ 7900/9524
2020-01-16T14:11:46.3782093Z ......................i......i...................................................................... 8100/9524
2020-01-16T14:11:51.6671777Z .................................................................................................... 8200/9524
2020-01-16T14:12:04.9762511Z .................................................................................................... 8300/9524
2020-01-16T14:12:15.6387625Z .................................................................................................... 8400/9524
---
2020-01-16T14:14:41.7710466Z  finished in 7.653
2020-01-16T14:14:41.7879788Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T14:14:41.9460284Z 
2020-01-16T14:14:41.9460510Z running 166 tests
2020-01-16T14:14:45.2959827Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-16T14:14:47.5581001Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-16T14:14:47.5581575Z 
2020-01-16T14:14:47.5603715Z  finished in 5.772
2020-01-16T14:14:47.5775851Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-16T14:14:47.7363424Z 
2020-01-16T14:14:47.7363424Z 
2020-01-16T14:14:47.7364331Z running 39 tests
2020-01-16T14:14:49.7982801Z i.........i...............FFFFFFFiFFFFF
2020-01-16T14:14:49.7989510Z 
2020-01-16T14:14:49.7990798Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2020-01-16T14:14:49.7991434Z 
2020-01-16T14:14:49.7992036Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.7992036Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.7992230Z 
2020-01-16T14:14:49.7992382Z fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
2020-01-16T14:14:49.7993575Z   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
2020-01-16T14:14:49.7994866Z   actual:   extern_drop_glue-cgu.0[Internal] extern_drop_glue-cgu.1[Internal] 
2020-01-16T14:14:49.7995056Z 
2020-01-16T14:14:49.7995694Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
2020-01-16T14:14:49.7996274Z   expected: extern_drop_glue[Internal] 
2020-01-16T14:14:49.7997183Z   actual:   extern_drop_glue-cgu.0[Internal] 
2020-01-16T14:14:49.7997362Z 
2020-01-16T14:14:49.7997539Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
2020-01-16T14:14:49.7997903Z   expected: extern_drop_glue-mod1[Internal] 
2020-01-16T14:14:49.7998786Z   actual:   extern_drop_glue-cgu.1[Internal] 
2020-01-16T14:14:49.7999258Z 
2020-01-16T14:14:49.7999699Z fn extern_drop_glue::mod1[0]::user[0]
2020-01-16T14:14:49.8000627Z   expected: extern_drop_glue-mod1[External] 
2020-01-16T14:14:49.8001738Z   actual:   extern_drop_glue-cgu.1[External] 
2020-01-16T14:14:49.8002076Z 
2020-01-16T14:14:49.8002384Z fn extern_drop_glue::user[0]
2020-01-16T14:14:49.8002568Z   expected: extern_drop_glue[External] 
2020-01-16T14:14:49.8002985Z   actual:   extern_drop_glue-cgu.0[External] 
2020-01-16T14:14:49.8003628Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8003816Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-16T14:14:49.8003964Z 
2020-01-16T14:14:49.8004542Z ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
2020-01-16T14:14:49.8004542Z ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
2020-01-16T14:14:49.8004721Z 
2020-01-16T14:14:49.8005126Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8005354Z 
2020-01-16T14:14:49.8005500Z fn cgu_generic_function::bar[0]<&str>
2020-01-16T14:14:49.8005888Z   expected: cgu_generic_function-in-extern_generic.volatile[External] 
2020-01-16T14:14:49.8006429Z   actual:   extern_generic-cgu.0[External] 
2020-01-16T14:14:49.8006883Z 
2020-01-16T14:14:49.8007547Z fn cgu_generic_function::foo[0]<&str>
2020-01-16T14:14:49.8008495Z   expected: cgu_generic_function-in-extern_generic.volatile[External] 
2020-01-16T14:14:49.8008898Z   actual:   extern_generic-cgu.0[External] 
2020-01-16T14:14:49.8009049Z 
2020-01-16T14:14:49.8009234Z fn extern_generic::mod1[0]::mod1[0]::user[0]
2020-01-16T14:14:49.8009681Z   expected: extern_generic-mod1-mod1[Internal] 
2020-01-16T14:14:49.8010657Z   actual:   extern_generic-cgu.3[Internal] 
2020-01-16T14:14:49.8012543Z 
2020-01-16T14:14:49.8012607Z fn extern_generic::mod1[0]::user[0]
2020-01-16T14:14:49.8012916Z   expected: extern_generic-mod1[Internal] 
2020-01-16T14:14:49.8013127Z   actual:   extern_generic-cgu.2[Internal] 
2020-01-16T14:14:49.8013307Z 
2020-01-16T14:14:49.8013349Z fn extern_generic::mod2[0]::user[0]
2020-01-16T14:14:49.8013608Z   expected: extern_generic-mod2[Internal] 
2020-01-16T14:14:49.8013814Z   actual:   extern_generic-cgu.4[Internal] 
2020-01-16T14:14:49.8013844Z 
2020-01-16T14:14:49.8013914Z fn extern_generic::mod3[0]::non_user[0]
2020-01-16T14:14:49.8014122Z   expected: extern_generic-mod3[Internal] 
2020-01-16T14:14:49.8014327Z   actual:   extern_generic-cgu.5[Internal] 
2020-01-16T14:14:49.8014356Z 
2020-01-16T14:14:49.8014415Z fn extern_generic::user[0]
2020-01-16T14:14:49.8014459Z   expected: extern_generic[Internal] 
2020-01-16T14:14:49.8014666Z   actual:   extern_generic-cgu.1[Internal] 
2020-01-16T14:14:49.8015021Z thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8015058Z 
2020-01-16T14:14:49.8015296Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2020-01-16T14:14:49.8015356Z 
2020-01-16T14:14:49.8015356Z 
2020-01-16T14:14:49.8015401Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8015430Z 
2020-01-16T14:14:49.8015474Z fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
2020-01-16T14:14:49.8015714Z   expected: local_drop_glue-mod1[Internal] 
2020-01-16T14:14:49.8015925Z   actual:   local_drop_glue-cgu.1[Internal] 
2020-01-16T14:14:49.8015955Z 
2020-01-16T14:14:49.8016000Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]>
2020-01-16T14:14:49.8016068Z   expected: local_drop_glue[Internal] 
2020-01-16T14:14:49.8016276Z   actual:   local_drop_glue-cgu.0[Internal] 
2020-01-16T14:14:49.8016305Z 
2020-01-16T14:14:49.8016369Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]>
2020-01-16T14:14:49.8016773Z   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
2020-01-16T14:14:49.8017025Z   actual:   local_drop_glue-cgu.0[Internal] local_drop_glue-cgu.1[Internal] 
2020-01-16T14:14:49.8017067Z 
2020-01-16T14:14:49.8017136Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
2020-01-16T14:14:49.8017354Z   expected: local_drop_glue-mod1[Internal] 
2020-01-16T14:14:49.8017568Z   actual:   local_drop_glue-cgu.1[Internal] 
2020-01-16T14:14:49.8017618Z 
2020-01-16T14:14:49.8018089Z fn local_drop_glue::mod1[0]::user[0]
2020-01-16T14:14:49.8018408Z   expected: local_drop_glue-mod1[External] 
2020-01-16T14:14:49.8018730Z   actual:   local_drop_glue-cgu.1[External] 
2020-01-16T14:14:49.8018787Z 
2020-01-16T14:14:49.8018830Z fn local_drop_glue::user[0]
2020-01-16T14:14:49.8018876Z   expected: local_drop_glue[External] 
2020-01-16T14:14:49.8019120Z   actual:   local_drop_glue-cgu.0[External] 
2020-01-16T14:14:49.8019151Z 
2020-01-16T14:14:49.8019194Z fn local_drop_glue::{{impl}}[0]::drop[0]
2020-01-16T14:14:49.8019240Z   expected: local_drop_glue[External] 
2020-01-16T14:14:49.8019475Z   actual:   local_drop_glue-cgu.0[External] 
2020-01-16T14:14:49.8019985Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8020023Z 
2020-01-16T14:14:49.8020396Z ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
2020-01-16T14:14:49.8020439Z 
2020-01-16T14:14:49.8020439Z 
2020-01-16T14:14:49.8020484Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8020514Z 
2020-01-16T14:14:49.8020575Z fn cgu_explicit_inlining::always_inlined[0]
2020-01-16T14:14:49.8020857Z   expected: inlining_from_extern_crate-mod2[Internal] inlining_from_extern_crate[Internal] 
2020-01-16T14:14:49.8021123Z   actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.2[Internal] 
2020-01-16T14:14:49.8021177Z 
2020-01-16T14:14:49.8021218Z fn cgu_explicit_inlining::inlined[0]
2020-01-16T14:14:49.8021472Z   expected: inlining_from_extern_crate-mod1[Internal] inlining_from_extern_crate[Internal] 
2020-01-16T14:14:49.8021849Z   actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.1[Internal] 
2020-01-16T14:14:49.8021884Z 
2020-01-16T14:14:49.8021927Z fn inlining_from_extern_crate::mod1[0]::user[0]
2020-01-16T14:14:49.8022148Z   expected: inlining_from_extern_crate-mod1[External] 
2020-01-16T14:14:49.8022396Z   actual:   inlining_from_extern_crate-cgu.1[External] 
2020-01-16T14:14:49.8022427Z 
2020-01-16T14:14:49.8022469Z fn inlining_from_extern_crate::mod2[0]::user[0]
2020-01-16T14:14:49.8022704Z   expected: inlining_from_extern_crate-mod2[External] 
2020-01-16T14:14:49.8022925Z   actual:   inlining_from_extern_crate-cgu.2[External] 
2020-01-16T14:14:49.8022997Z fn inlining_from_extern_crate::user[0]
2020-01-16T14:14:49.8022997Z fn inlining_from_extern_crate::user[0]
2020-01-16T14:14:49.8023060Z   expected: inlining_from_extern_crate[External] 
2020-01-16T14:14:49.8023280Z   actual:   inlining_from_extern_crate-cgu.0[External] 
2020-01-16T14:14:49.8023627Z thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8023692Z 
2020-01-16T14:14:49.8023929Z ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
2020-01-16T14:14:49.8023961Z 
2020-01-16T14:14:49.8023961Z 
2020-01-16T14:14:49.8024031Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8024061Z 
2020-01-16T14:14:49.8024102Z fn local_generic::generic[0]<&str>
2020-01-16T14:14:49.8024147Z   expected: local_generic.volatile[External] 
2020-01-16T14:14:49.8024375Z   actual:   local_generic-cgu.4[External] 
2020-01-16T14:14:49.8024405Z 
2020-01-16T14:14:49.8024447Z fn local_generic::generic[0]<char>
2020-01-16T14:14:49.8024492Z   expected: local_generic.volatile[External] 
2020-01-16T14:14:49.8024893Z   actual:   local_generic-cgu.4[External] 
2020-01-16T14:14:49.8024923Z 
2020-01-16T14:14:49.8024966Z fn local_generic::generic[0]<u32>
2020-01-16T14:14:49.8025032Z   expected: local_generic.volatile[External] 
2020-01-16T14:14:49.8025255Z   actual:   local_generic-cgu.4[External] 
2020-01-16T14:14:49.8025286Z 
2020-01-16T14:14:49.8025327Z fn local_generic::generic[0]<u64>
2020-01-16T14:14:49.8025393Z   expected: local_generic.volatile[External] 
2020-01-16T14:14:49.8025605Z   actual:   local_generic-cgu.4[External] 
2020-01-16T14:14:49.8025643Z 
2020-01-16T14:14:49.8025687Z fn local_generic::mod1[0]::mod1[0]::user[0]
2020-01-16T14:14:49.8025928Z   expected: local_generic-mod1-mod1[Internal] 
2020-01-16T14:14:49.8026141Z   actual:   local_generic-cgu.2[Internal] 
2020-01-16T14:14:49.8026171Z 
2020-01-16T14:14:49.8026231Z fn local_generic::mod1[0]::user[0]
2020-01-16T14:14:49.8026444Z   expected: local_generic-mod1[Internal] 
2020-01-16T14:14:49.8026655Z   actual:   local_generic-cgu.1[Internal] 
2020-01-16T14:14:49.8026684Z 
2020-01-16T14:14:49.8026747Z fn local_generic::mod2[0]::user[0]
2020-01-16T14:14:49.8026958Z   expected: local_generic-mod2[Internal] 
2020-01-16T14:14:49.8027168Z   actual:   local_generic-cgu.3[Internal] 
2020-01-16T14:14:49.8027207Z 
2020-01-16T14:14:49.8027282Z fn local_generic::user[0]
2020-01-16T14:14:49.8027327Z   expected: local_generic[Internal] 
2020-01-16T14:14:49.8027543Z   actual:   local_generic-cgu.0[Internal] 
2020-01-16T14:14:49.8027976Z thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8028022Z 
2020-01-16T14:14:49.8028300Z ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
2020-01-16T14:14:49.8028354Z 
2020-01-16T14:14:49.8028354Z 
2020-01-16T14:14:49.8028399Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8028430Z 
2020-01-16T14:14:49.8028475Z fn local_inlining_but_not_all::inline[0]::inlined_function[0]
2020-01-16T14:14:49.8028725Z   expected: local_inlining_but_not_all-inline[External] 
2020-01-16T14:14:49.8028951Z   actual:   local_inlining_but_not_all-cgu.0[External] 
2020-01-16T14:14:49.8029048Z 
2020-01-16T14:14:49.8029112Z fn local_inlining_but_not_all::non_user[0]::baz[0]
2020-01-16T14:14:49.8029362Z   expected: local_inlining_but_not_all-non_user[External] 
2020-01-16T14:14:49.8029589Z   actual:   local_inlining_but_not_all-cgu.1[External] 
2020-01-16T14:14:49.8029621Z 
2020-01-16T14:14:49.8029693Z fn local_inlining_but_not_all::user1[0]::foo[0]
2020-01-16T14:14:49.8029921Z   expected: local_inlining_but_not_all-user1[External] 
2020-01-16T14:14:49.8030296Z   actual:   local_inlining_but_not_all-cgu.2[External] 
2020-01-16T14:14:49.8030326Z 
2020-01-16T14:14:49.8030388Z fn local_inlining_but_not_all::user2[0]::bar[0]
2020-01-16T14:14:49.8030847Z   expected: local_inlining_but_not_all-user2[External] 
2020-01-16T14:14:49.8031075Z   actual:   local_inlining_but_not_all-cgu.3[External] 
2020-01-16T14:14:49.8031454Z thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8031502Z 
2020-01-16T14:14:49.8031746Z ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
2020-01-16T14:14:49.8031798Z 
2020-01-16T14:14:49.8031798Z 
2020-01-16T14:14:49.8031843Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8031873Z 
2020-01-16T14:14:49.8031923Z fn local_inlining::inline[0]::inlined_function[0]
2020-01-16T14:14:49.8032189Z   expected: local_inlining-user1[Internal] local_inlining-user2[Internal] 
2020-01-16T14:14:49.8032436Z   actual:   local_inlining-cgu.1[Internal] local_inlining-cgu.2[Internal] 
2020-01-16T14:14:49.8032468Z 
2020-01-16T14:14:49.8032530Z fn local_inlining::non_user[0]::baz[0]
2020-01-16T14:14:49.8032749Z   expected: local_inlining-non_user[External] 
2020-01-16T14:14:49.8032970Z   actual:   local_inlining-cgu.0[External] 
2020-01-16T14:14:49.8033001Z 
2020-01-16T14:14:49.8033136Z fn local_inlining::user1[0]::foo[0]
2020-01-16T14:14:49.8033676Z   expected: local_inlining-user1[External] 
2020-01-16T14:14:49.8033902Z   actual:   local_inlining-cgu.1[External] 
2020-01-16T14:14:49.8033951Z 
2020-01-16T14:14:49.8033995Z fn local_inlining::user2[0]::bar[0]
2020-01-16T14:14:49.8034207Z   expected: local_inlining-user2[External] 
2020-01-16T14:14:49.8034419Z   actual:   local_inlining-cgu.2[External] 
2020-01-16T14:14:49.8034791Z thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8034830Z 
2020-01-16T14:14:49.8035103Z ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
2020-01-16T14:14:49.8035137Z 
2020-01-16T14:14:49.8035137Z 
2020-01-16T14:14:49.8035183Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8035213Z 
2020-01-16T14:14:49.8035277Z fn local_transitive_inlining::direct_user[0]::foo[0]
2020-01-16T14:14:49.8035512Z   expected: local_transitive_inlining-indirect_user[Internal] 
2020-01-16T14:14:49.8035738Z   actual:   local_transitive_inlining-cgu.0[Internal] 
2020-01-16T14:14:49.8035777Z 
2020-01-16T14:14:49.8035840Z fn local_transitive_inlining::indirect_user[0]::bar[0]
2020-01-16T14:14:49.8036075Z   expected: local_transitive_inlining-indirect_user[External] 
2020-01-16T14:14:49.8036301Z   actual:   local_transitive_inlining-cgu.0[External] 
2020-01-16T14:14:49.8036402Z 
2020-01-16T14:14:49.8036474Z fn local_transitive_inlining::inline[0]::inlined_function[0]
2020-01-16T14:14:49.8036728Z   expected: local_transitive_inlining-indirect_user[Internal] 
2020-01-16T14:14:49.8036954Z   actual:   local_transitive_inlining-cgu.0[Internal] 
2020-01-16T14:14:49.8037004Z 
2020-01-16T14:14:49.8037049Z fn local_transitive_inlining::non_user[0]::baz[0]
2020-01-16T14:14:49.8037278Z   expected: local_transitive_inlining-non_user[External] 
2020-01-16T14:14:49.8037522Z   actual:   local_transitive_inlining-cgu.1[External] 
2020-01-16T14:14:49.8037882Z thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8037994Z 
2020-01-16T14:14:49.8038280Z ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
2020-01-16T14:14:49.8038313Z 
2020-01-16T14:14:49.8038313Z 
2020-01-16T14:14:49.8038367Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8038397Z 
2020-01-16T14:14:49.8038457Z fn regular_modules::bar[0]
2020-01-16T14:14:49.8038503Z   expected: regular_modules[Internal] 
2020-01-16T14:14:49.8038722Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-16T14:14:49.8038814Z fn regular_modules::foo[0]
2020-01-16T14:14:49.8038814Z fn regular_modules::foo[0]
2020-01-16T14:14:49.8038861Z   expected: regular_modules[Internal] 
2020-01-16T14:14:49.8039078Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-16T14:14:49.8039128Z 
2020-01-16T14:14:49.8039171Z fn regular_modules::mod1[0]::bar[0]
2020-01-16T14:14:49.8039384Z   expected: regular_modules-mod1[Internal] 
2020-01-16T14:14:49.8049892Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-16T14:14:49.8049978Z 
2020-01-16T14:14:49.8050026Z fn regular_modules::mod1[0]::foo[0]
2020-01-16T14:14:49.8050276Z   expected: regular_modules-mod1[Internal] 
2020-01-16T14:14:49.8050511Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-16T14:14:49.8050589Z 
2020-01-16T14:14:49.8050645Z fn regular_modules::mod1[0]::mod1[0]::bar[0]
2020-01-16T14:14:49.8050881Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-16T14:14:49.8051120Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-16T14:14:49.8051151Z 
2020-01-16T14:14:49.8051194Z fn regular_modules::mod1[0]::mod1[0]::foo[0]
2020-01-16T14:14:49.8051415Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-16T14:14:49.8051656Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-16T14:14:49.8051687Z 
2020-01-16T14:14:49.8051730Z fn regular_modules::mod1[0]::mod2[0]::bar[0]
2020-01-16T14:14:49.8051967Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-16T14:14:49.8052184Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-16T14:14:49.8052224Z 
2020-01-16T14:14:49.8052268Z fn regular_modules::mod1[0]::mod2[0]::foo[0]
2020-01-16T14:14:49.8052508Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-16T14:14:49.8052725Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-16T14:14:49.8052756Z 
2020-01-16T14:14:49.8052825Z fn regular_modules::mod2[0]::bar[0]
2020-01-16T14:14:49.8053043Z   expected: regular_modules-mod2[Internal] 
2020-01-16T14:14:49.8053256Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-16T14:14:49.8053286Z 
2020-01-16T14:14:49.8053349Z fn regular_modules::mod2[0]::foo[0]
2020-01-16T14:14:49.8053563Z   expected: regular_modules-mod2[Internal] 
2020-01-16T14:14:49.8053778Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-16T14:14:49.8053808Z 
2020-01-16T14:14:49.8053871Z fn regular_modules::mod2[0]::mod1[0]::bar[0]
2020-01-16T14:14:49.8054089Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-16T14:14:49.8054303Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-16T14:14:49.8054359Z 
2020-01-16T14:14:49.8054402Z fn regular_modules::mod2[0]::mod1[0]::foo[0]
2020-01-16T14:14:49.8054625Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-16T14:14:49.8054838Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-16T14:14:49.8054888Z 
2020-01-16T14:14:49.8054931Z fn regular_modules::mod2[0]::mod2[0]::bar[0]
2020-01-16T14:14:49.8055286Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-16T14:14:49.8055562Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-16T14:14:49.8055594Z 
2020-01-16T14:14:49.8055638Z fn regular_modules::mod2[0]::mod2[0]::foo[0]
2020-01-16T14:14:49.8056009Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-16T14:14:49.8056236Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-16T14:14:49.8056305Z static regular_modules::BAZ[0]
2020-01-16T14:14:49.8056305Z static regular_modules::BAZ[0]
2020-01-16T14:14:49.8056349Z   expected: regular_modules[Internal] 
2020-01-16T14:14:49.8056575Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-16T14:14:49.8056762Z 
2020-01-16T14:14:49.8057098Z static regular_modules::mod1[0]::BAZ[0]
2020-01-16T14:14:49.8057506Z   expected: regular_modules-mod1[Internal] 
2020-01-16T14:14:49.8057713Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-16T14:14:49.8057743Z 
2020-01-16T14:14:49.8057785Z static regular_modules::mod1[0]::mod1[0]::BAZ[0]
2020-01-16T14:14:49.8058203Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-16T14:14:49.8058418Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-16T14:14:49.8058449Z 
2020-01-16T14:14:49.8058511Z static regular_modules::mod1[0]::mod2[0]::BAZ[0]
2020-01-16T14:14:49.8058731Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-16T14:14:49.8058945Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-16T14:14:49.8058975Z 
2020-01-16T14:14:49.8059037Z static regular_modules::mod2[0]::BAZ[0]
2020-01-16T14:14:49.8059252Z   expected: regular_modules-mod2[Internal] 
2020-01-16T14:14:49.8059466Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-16T14:14:49.8059497Z 
2020-01-16T14:14:49.8059568Z static regular_modules::mod2[0]::mod1[0]::BAZ[0]
2020-01-16T14:14:49.8059789Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-16T14:14:49.8060003Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-16T14:14:49.8060052Z 
2020-01-16T14:14:49.8060096Z static regular_modules::mod2[0]::mod2[0]::BAZ[0]
2020-01-16T14:14:49.8060324Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-16T14:14:49.8060541Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-16T14:14:49.8060961Z thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8061000Z 
2020-01-16T14:14:49.8061255Z ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
2020-01-16T14:14:49.8061288Z 
2020-01-16T14:14:49.8061288Z 
2020-01-16T14:14:49.8061491Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8061522Z 
2020-01-16T14:14:49.8061580Z fn statics::function[0]
2020-01-16T14:14:49.8061631Z   expected: statics[External] 
2020-01-16T14:14:49.8061833Z   actual:   statics-cgu.0[External] 
2020-01-16T14:14:49.8061861Z 
2020-01-16T14:14:49.8061920Z fn statics::mod1[0]::function[0]
2020-01-16T14:14:49.8062638Z   expected: statics-mod1[External] 
2020-01-16T14:14:49.8062834Z   actual:   statics-cgu.1[External] 
2020-01-16T14:14:49.8062927Z static statics::BAR[0]
2020-01-16T14:14:49.8062927Z static statics::BAR[0]
2020-01-16T14:14:49.8062968Z   expected: statics[Internal] 
2020-01-16T14:14:49.8063159Z   actual:   statics-cgu.0[Internal] 
2020-01-16T14:14:49.8063248Z static statics::FOO[0]
2020-01-16T14:14:49.8063248Z static statics::FOO[0]
2020-01-16T14:14:49.8063289Z   expected: statics[Internal] 
2020-01-16T14:14:49.8063483Z   actual:   statics-cgu.0[Internal] 
2020-01-16T14:14:49.8063529Z 
2020-01-16T14:14:49.8063569Z static statics::function[0]::BAR[0]
2020-01-16T14:14:49.8063784Z   expected: statics[Internal] 
2020-01-16T14:14:49.8064162Z   actual:   statics-cgu.0[Internal] 
2020-01-16T14:14:49.8064211Z 
2020-01-16T14:14:49.8064253Z static statics::function[0]::FOO[0]
2020-01-16T14:14:49.8064308Z   expected: statics[Internal] 
2020-01-16T14:14:49.8064533Z   actual:   statics-cgu.0[Internal] 
2020-01-16T14:14:49.8064563Z 
2020-01-16T14:14:49.8064605Z static statics::mod1[0]::BAR[0]
2020-01-16T14:14:49.8064808Z   expected: statics-mod1[Internal] 
2020-01-16T14:14:49.8065111Z   actual:   statics-cgu.1[Internal] 
2020-01-16T14:14:49.8065149Z 
2020-01-16T14:14:49.8065191Z static statics::mod1[0]::FOO[0]
2020-01-16T14:14:49.8065419Z   expected: statics-mod1[Internal] 
2020-01-16T14:14:49.8065645Z   actual:   statics-cgu.1[Internal] 
2020-01-16T14:14:49.8065674Z 
2020-01-16T14:14:49.8065718Z static statics::mod1[0]::function[0]::BAR[0]
2020-01-16T14:14:49.8066009Z   expected: statics-mod1[Internal] 
2020-01-16T14:14:49.8066213Z   actual:   statics-cgu.1[Internal] 
2020-01-16T14:14:49.8066242Z 
2020-01-16T14:14:49.8066286Z static statics::mod1[0]::function[0]::FOO[0]
2020-01-16T14:14:49.8066507Z   expected: statics-mod1[Internal] 
2020-01-16T14:14:49.8066809Z   actual:   statics-cgu.1[Internal] 
2020-01-16T14:14:49.8067147Z thread '[codegen-units] codegen-units/partitioning/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8067204Z 
2020-01-16T14:14:49.8067458Z ---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----
2020-01-16T14:14:49.8067493Z 
2020-01-16T14:14:49.8067493Z 
2020-01-16T14:14:49.8067538Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8067587Z 
2020-01-16T14:14:49.8067631Z fn shared_generics_aux::generic_fn[0]<u16>
2020-01-16T14:14:49.8067873Z   expected: shared_generics_aux-in-shared_generics.volatile[External] 
2020-01-16T14:14:49.8068110Z   actual:   shared_generics-cgu.1[External] 
2020-01-16T14:14:49.8068458Z thread '[codegen-units] codegen-units/partitioning/shared-generics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8068496Z 
2020-01-16T14:14:49.8069260Z ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
2020-01-16T14:14:49.8069304Z 
2020-01-16T14:14:49.8069304Z 
2020-01-16T14:14:49.8069350Z The following items were assigned to wrong codegen units:
2020-01-16T14:14:49.8069380Z 
2020-01-16T14:14:49.8069442Z fn core::ptr[0]::real_drop_in_place[0]<u32>
2020-01-16T14:14:49.8069498Z   expected: vtable_through_const[Internal] 
2020-01-16T14:14:49.8069728Z   actual:   vtable_through_const-cgu.0[Internal] 
2020-01-16T14:14:49.8069760Z 
2020-01-16T14:14:49.8069825Z fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32>
2020-01-16T14:14:49.8070058Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-16T14:14:49.8070618Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-16T14:14:49.8070666Z 
2020-01-16T14:14:49.8070710Z fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32>
2020-01-16T14:14:49.8070926Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-16T14:14:49.8071148Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-16T14:14:49.8071185Z 
2020-01-16T14:14:49.8071229Z fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32>
2020-01-16T14:14:49.8071445Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-16T14:14:49.8071716Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-16T14:14:49.8071752Z 
2020-01-16T14:14:49.8071796Z fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32>
2020-01-16T14:14:49.8072028Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-16T14:14:49.8072234Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-16T14:14:49.8072262Z 
2020-01-16T14:14:49.8072304Z fn vtable_through_const::mod1[0]::id[0]<char>
2020-01-16T14:14:49.8072565Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-16T14:14:49.8072795Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-16T14:14:49.8072826Z 
2020-01-16T14:14:49.8072870Z fn vtable_through_const::mod1[0]::id[0]<i64>
2020-01-16T14:14:49.8073118Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-16T14:14:49.8073349Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-16T14:14:49.8073380Z 
2020-01-16T14:14:49.8073444Z fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8>
2020-01-16T14:14:49.8073774Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-16T14:14:49.8074025Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-16T14:14:49.8074058Z 
2020-01-16T14:14:49.8074126Z fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8>
2020-01-16T14:14:49.8074359Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-16T14:14:49.8074581Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-16T14:14:49.8074630Z 
2020-01-16T14:14:49.8074677Z fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8>
2020-01-16T14:14:49.8074905Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-16T14:14:49.8075128Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-16T14:14:49.8075276Z 
2020-01-16T14:14:49.8075325Z fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8>
2020-01-16T14:14:49.8075580Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-16T14:14:49.8075824Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-16T14:14:49.8076178Z thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-16T14:14:49.8076217Z 
2020-01-16T14:14:49.8076261Z 
2020-01-16T14:14:49.8076301Z failures:
---
2020-01-16T14:14:49.8079560Z test result: FAILED. 24 passed; 12 failed; 3 ignored; 0 measured; 0 filtered out
2020-01-16T14:14:49.8079594Z 
2020-01-16T14:14:49.8079638Z 
2020-01-16T14:14:49.8079663Z 
2020-01-16T14:14:49.8081306Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-16T14:14:49.8081586Z 
2020-01-16T14:14:49.8081616Z 
2020-01-16T14:14:49.8081901Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-16T14:14:49.8084347Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-16T14:14:49.8084347Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-16T14:14:49.8084410Z Build completed unsuccessfully in 0:58:33
2020-01-16T14:14:49.8084475Z == clock drift check ==
2020-01-16T14:14:49.8115118Z   local time: Thu Jan 16 14:14:49 UTC 2020
2020-01-16T14:14:50.7983069Z   network time: Thu, 16 Jan 2020 14:14:50 GMT
2020-01-16T14:14:50.7983570Z == end clock drift check ==
2020-01-16T14:14:52.4322123Z 
2020-01-16T14:14:52.4382487Z ##[error]Bash exited with code '1'.
2020-01-16T14:14:52.4443757Z ##[section]Starting: Checkout
2020-01-16T14:14:52.4445396Z ==============================================================================
2020-01-16T14:14:52.4445449Z Task         : Get sources
2020-01-16T14:14:52.4445494Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
