plain
2020-01-03T13:26:36.1417557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-03T13:26:36.1661145Z ##[command]git config gc.auto 0
2020-01-03T13:26:36.1726462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-03T13:26:36.1793173Z ##[command]git config --get-all http.proxy
2020-01-03T13:26:36.1948783Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67834/merge:refs/remotes/pull/67834/merge
---
2020-01-03T14:25:14.3871746Z .................................................................................................... 1500/9470
2020-01-03T14:25:20.8797890Z .................................................................................................... 1600/9470
2020-01-03T14:25:26.3182840Z .................................................................................................... 1700/9470
2020-01-03T14:25:36.6596557Z .................................................................................................... 1800/9470
2020-01-03T14:25:45.5782139Z i................................................................................................... 1900/9470
2020-01-03T14:25:53.0627047Z ......................................................................................iiiii......... 2000/9470
2020-01-03T14:26:16.6943635Z .................................................................................................... 2200/9470
2020-01-03T14:26:19.5563956Z .................................................................................................... 2300/9470
2020-01-03T14:26:22.0984962Z .................................................................................................... 2400/9470
2020-01-03T14:26:28.7511716Z .................................................................................................... 2500/9470
---
2020-01-03T14:29:42.1792484Z ..................i...............i................................................................. 4900/9470
2020-01-03T14:29:52.9802017Z .................................................................................................... 5000/9470
2020-01-03T14:29:59.3615440Z ...............................................................i.................................... 5100/9470
2020-01-03T14:30:08.3294180Z .................................................................................................... 5200/9470
2020-01-03T14:30:16.4830968Z ..............................ii.ii...........i..................................................... 5300/9470
2020-01-03T14:30:26.5540369Z .................................................................................................... 5500/9470
2020-01-03T14:30:37.7158062Z .................................................................................................... 5600/9470
2020-01-03T14:30:45.4194189Z .............i...................................................................................... 5700/9470
2020-01-03T14:30:52.1101169Z .................................................................................................... 5800/9470
2020-01-03T14:30:52.1101169Z .................................................................................................... 5800/9470
2020-01-03T14:31:03.7215976Z .................................................................................................... 5900/9470
2020-01-03T14:31:16.3351163Z ..ii...i..ii...........i............................................................................ 6000/9470
2020-01-03T14:31:35.0422931Z .................................................................................................... 6200/9470
2020-01-03T14:31:43.6519475Z .................................................................................................... 6300/9470
2020-01-03T14:31:43.6519475Z .................................................................................................... 6300/9470
2020-01-03T14:32:09.6667147Z .............................i..ii.................................................................. 6400/9470
2020-01-03T14:32:30.6781882Z .................................................................................................... 6600/9470
2020-01-03T14:32:33.0543350Z ....i............................................................................................... 6700/9470
2020-01-03T14:32:35.6154526Z .................................................................................................... 6800/9470
2020-01-03T14:32:38.4380216Z ....i............................................................................................... 6900/9470
---
2020-01-03T14:34:24.2868296Z .................................................................................................... 7500/9470
2020-01-03T14:34:29.2976217Z .................................................................................................... 7600/9470
2020-01-03T14:34:34.8407989Z .................................................................................................... 7700/9470
2020-01-03T14:34:46.5245534Z .................................................................................................... 7800/9470
2020-01-03T14:34:54.6916425Z .......................................i.iii........................................................ 7900/9470
2020-01-03T14:35:11.0037145Z .................................................................................................... 8100/9470
2020-01-03T14:35:20.3322977Z .................................................................................................... 8200/9470
2020-01-03T14:35:35.5190437Z .................................................................................................... 8300/9470
2020-01-03T14:35:43.9877991Z .................................................................................................... 8400/9470
---
2020-01-03T14:38:17.0086707Z  finished in 7.472
2020-01-03T14:38:17.0308611Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-03T14:38:17.2378141Z 
2020-01-03T14:38:17.2378681Z running 166 tests
2020-01-03T14:38:20.5489994Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-03T14:38:23.0626214Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-03T14:38:23.0632194Z 
2020-01-03T14:38:23.0636252Z  finished in 6.032
2020-01-03T14:38:23.0835791Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-03T14:38:23.2675567Z 
2020-01-03T14:38:23.2675567Z 
2020-01-03T14:38:23.2675884Z running 39 tests
2020-01-03T14:38:25.4158605Z i.........i...............FFFFFFFiFFFFFthread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-03T14:38:25.4159067Z failures:
2020-01-03T14:38:25.4159097Z 
2020-01-03T14:38:25.4159537Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2020-01-03T14:38:25.4159578Z 
2020-01-03T14:38:25.4159578Z 
2020-01-03T14:38:25.4159645Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4159676Z 
2020-01-03T14:38:25.4159722Z fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
2020-01-03T14:38:25.4160012Z   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
2020-01-03T14:38:25.4160299Z   actual:   extern_drop_glue-cgu.0[Internal] extern_drop_glue-cgu.1[Internal] 
2020-01-03T14:38:25.4160336Z 
2020-01-03T14:38:25.4160382Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
2020-01-03T14:38:25.4160443Z   expected: extern_drop_glue[Internal] 
2020-01-03T14:38:25.4160800Z   actual:   extern_drop_glue-cgu.0[Internal] 
2020-01-03T14:38:25.4160832Z 
2020-01-03T14:38:25.4160879Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
2020-01-03T14:38:25.4161115Z   expected: extern_drop_glue-mod1[Internal] 
2020-01-03T14:38:25.4161466Z   actual:   extern_drop_glue-cgu.1[Internal] 
2020-01-03T14:38:25.4161496Z 
2020-01-03T14:38:25.4161551Z fn extern_drop_glue::mod1[0]::user[0]
2020-01-03T14:38:25.4161759Z   expected: extern_drop_glue-mod1[External] 
2020-01-03T14:38:25.4161977Z   actual:   extern_drop_glue-cgu.1[External] 
2020-01-03T14:38:25.4162007Z 
2020-01-03T14:38:25.4162059Z fn extern_drop_glue::user[0]
2020-01-03T14:38:25.4162102Z   expected: extern_drop_glue[External] 
2020-01-03T14:38:25.4162310Z   actual:   extern_drop_glue-cgu.0[External] 
2020-01-03T14:38:25.4162653Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4162713Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-03T14:38:25.4162759Z 
2020-01-03T14:38:25.4162996Z ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
2020-01-03T14:38:25.4162996Z ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
2020-01-03T14:38:25.4163029Z 
2020-01-03T14:38:25.4163079Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4163120Z 
2020-01-03T14:38:25.4163160Z fn cgu_generic_function::bar[0]<&str>
2020-01-03T14:38:25.4163390Z   expected: cgu_generic_function-in-extern_generic.volatile[External] 
2020-01-03T14:38:25.4163910Z   actual:   extern_generic-cgu.0[External] 
2020-01-03T14:38:25.4163962Z 
2020-01-03T14:38:25.4164001Z fn cgu_generic_function::foo[0]<&str>
2020-01-03T14:38:25.4164233Z   expected: cgu_generic_function-in-extern_generic.volatile[External] 
2020-01-03T14:38:25.4164454Z   actual:   extern_generic-cgu.0[External] 
2020-01-03T14:38:25.4164485Z 
2020-01-03T14:38:25.4164525Z fn extern_generic::mod1[0]::mod1[0]::user[0]
2020-01-03T14:38:25.4164737Z   expected: extern_generic-mod1-mod1[Internal] 
2020-01-03T14:38:25.4164959Z   actual:   extern_generic-cgu.3[Internal] 
2020-01-03T14:38:25.4164988Z 
2020-01-03T14:38:25.4165028Z fn extern_generic::mod1[0]::user[0]
2020-01-03T14:38:25.4165356Z   expected: extern_generic-mod1[Internal] 
2020-01-03T14:38:25.4165643Z   actual:   extern_generic-cgu.2[Internal] 
2020-01-03T14:38:25.4165674Z 
2020-01-03T14:38:25.4165712Z fn extern_generic::mod2[0]::user[0]
2020-01-03T14:38:25.4165928Z   expected: extern_generic-mod2[Internal] 
2020-01-03T14:38:25.4166143Z   actual:   extern_generic-cgu.4[Internal] 
2020-01-03T14:38:25.4166172Z 
2020-01-03T14:38:25.4166211Z fn extern_generic::mod3[0]::non_user[0]
2020-01-03T14:38:25.4166424Z   expected: extern_generic-mod3[Internal] 
2020-01-03T14:38:25.4166623Z   actual:   extern_generic-cgu.5[Internal] 
2020-01-03T14:38:25.4166652Z 
2020-01-03T14:38:25.4166690Z fn extern_generic::user[0]
2020-01-03T14:38:25.4166744Z   expected: extern_generic[Internal] 
2020-01-03T14:38:25.4167069Z   actual:   extern_generic-cgu.1[Internal] 
2020-01-03T14:38:25.4168033Z thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4168098Z 
2020-01-03T14:38:25.4168361Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2020-01-03T14:38:25.4168397Z 
2020-01-03T14:38:25.4168397Z 
2020-01-03T14:38:25.4168458Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4168497Z 
2020-01-03T14:38:25.4168542Z fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
2020-01-03T14:38:25.4168767Z   expected: local_drop_glue-mod1[Internal] 
2020-01-03T14:38:25.4169009Z   actual:   local_drop_glue-cgu.1[Internal] 
2020-01-03T14:38:25.4169041Z 
2020-01-03T14:38:25.4169087Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]>
2020-01-03T14:38:25.4169150Z   expected: local_drop_glue[Internal] 
2020-01-03T14:38:25.4169370Z   actual:   local_drop_glue-cgu.0[Internal] 
2020-01-03T14:38:25.4169401Z 
2020-01-03T14:38:25.4169447Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]>
2020-01-03T14:38:25.4169705Z   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
2020-01-03T14:38:25.4169966Z   actual:   local_drop_glue-cgu.0[Internal] local_drop_glue-cgu.1[Internal] 
2020-01-03T14:38:25.4170001Z 
2020-01-03T14:38:25.4170062Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
2020-01-03T14:38:25.4170296Z   expected: local_drop_glue-mod1[Internal] 
2020-01-03T14:38:25.4170511Z   actual:   local_drop_glue-cgu.1[Internal] 
2020-01-03T14:38:25.4170667Z 
2020-01-03T14:38:25.4170723Z fn local_drop_glue::mod1[0]::user[0]
2020-01-03T14:38:25.4170931Z   expected: local_drop_glue-mod1[External] 
2020-01-03T14:38:25.4171139Z   actual:   local_drop_glue-cgu.1[External] 
2020-01-03T14:38:25.4171169Z 
2020-01-03T14:38:25.4171224Z fn local_drop_glue::user[0]
2020-01-03T14:38:25.4171267Z   expected: local_drop_glue[External] 
2020-01-03T14:38:25.4171479Z   actual:   local_drop_glue-cgu.0[External] 
2020-01-03T14:38:25.4171522Z 
2020-01-03T14:38:25.4171564Z fn local_drop_glue::{{impl}}[0]::drop[0]
2020-01-03T14:38:25.4171607Z   expected: local_drop_glue[External] 
2020-01-03T14:38:25.4171825Z   actual:   local_drop_glue-cgu.0[External] 
2020-01-03T14:38:25.4172179Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4172378Z 
2020-01-03T14:38:25.4172694Z ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
2020-01-03T14:38:25.4172729Z 
2020-01-03T14:38:25.4172729Z 
2020-01-03T14:38:25.4172773Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4172802Z 
2020-01-03T14:38:25.4172857Z fn cgu_explicit_inlining::always_inlined[0]
2020-01-03T14:38:25.4173113Z   expected: inlining_from_extern_crate-mod2[Internal] inlining_from_extern_crate[Internal] 
2020-01-03T14:38:25.4173387Z   actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.2[Internal] 
2020-01-03T14:38:25.4173422Z 
2020-01-03T14:38:25.4173476Z fn cgu_explicit_inlining::inlined[0]
2020-01-03T14:38:25.4173958Z   expected: inlining_from_extern_crate-mod1[Internal] inlining_from_extern_crate[Internal] 
2020-01-03T14:38:25.4174270Z   actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.1[Internal] 
2020-01-03T14:38:25.4174322Z 
2020-01-03T14:38:25.4174372Z fn inlining_from_extern_crate::mod1[0]::user[0]
2020-01-03T14:38:25.4174588Z   expected: inlining_from_extern_crate-mod1[External] 
2020-01-03T14:38:25.4174819Z   actual:   inlining_from_extern_crate-cgu.1[External] 
2020-01-03T14:38:25.4174849Z 
2020-01-03T14:38:25.4174889Z fn inlining_from_extern_crate::mod2[0]::user[0]
2020-01-03T14:38:25.4175148Z   expected: inlining_from_extern_crate-mod2[External] 
2020-01-03T14:38:25.4175392Z   actual:   inlining_from_extern_crate-cgu.2[External] 
2020-01-03T14:38:25.4175461Z fn inlining_from_extern_crate::user[0]
2020-01-03T14:38:25.4175461Z fn inlining_from_extern_crate::user[0]
2020-01-03T14:38:25.4175504Z   expected: inlining_from_extern_crate[External] 
2020-01-03T14:38:25.4175733Z   actual:   inlining_from_extern_crate-cgu.0[External] 
2020-01-03T14:38:25.4176207Z thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4176260Z 
2020-01-03T14:38:25.4176507Z ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
2020-01-03T14:38:25.4176541Z 
2020-01-03T14:38:25.4176541Z 
2020-01-03T14:38:25.4176584Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4176626Z 
2020-01-03T14:38:25.4176667Z fn local_generic::generic[0]<&str>
2020-01-03T14:38:25.4176711Z   expected: local_generic.volatile[External] 
2020-01-03T14:38:25.4177041Z   actual:   local_generic-cgu.4[External] 
2020-01-03T14:38:25.4177088Z 
2020-01-03T14:38:25.4177129Z fn local_generic::generic[0]<char>
2020-01-03T14:38:25.4177499Z   expected: local_generic.volatile[External] 
2020-01-03T14:38:25.4177864Z   actual:   local_generic-cgu.4[External] 
2020-01-03T14:38:25.4177897Z 
2020-01-03T14:38:25.4177951Z fn local_generic::generic[0]<u32>
2020-01-03T14:38:25.4177996Z   expected: local_generic.volatile[External] 
2020-01-03T14:38:25.4178234Z   actual:   local_generic-cgu.4[External] 
2020-01-03T14:38:25.4178265Z 
2020-01-03T14:38:25.4178306Z fn local_generic::generic[0]<u64>
2020-01-03T14:38:25.4178359Z   expected: local_generic.volatile[External] 
2020-01-03T14:38:25.4178593Z   actual:   local_generic-cgu.4[External] 
2020-01-03T14:38:25.4178625Z 
2020-01-03T14:38:25.4178667Z fn local_generic::mod1[0]::mod1[0]::user[0]
2020-01-03T14:38:25.4178899Z   expected: local_generic-mod1-mod1[Internal] 
2020-01-03T14:38:25.4179116Z   actual:   local_generic-cgu.2[Internal] 
2020-01-03T14:38:25.4179147Z 
2020-01-03T14:38:25.4179188Z fn local_generic::mod1[0]::user[0]
2020-01-03T14:38:25.4179414Z   expected: local_generic-mod1[Internal] 
2020-01-03T14:38:25.4179630Z   actual:   local_generic-cgu.1[Internal] 
2020-01-03T14:38:25.4179662Z 
2020-01-03T14:38:25.4179703Z fn local_generic::mod2[0]::user[0]
2020-01-03T14:38:25.4179941Z   expected: local_generic-mod2[Internal] 
2020-01-03T14:38:25.4180159Z   actual:   local_generic-cgu.3[Internal] 
2020-01-03T14:38:25.4180190Z 
2020-01-03T14:38:25.4180245Z fn local_generic::user[0]
2020-01-03T14:38:25.4180289Z   expected: local_generic[Internal] 
2020-01-03T14:38:25.4181818Z   actual:   local_generic-cgu.0[Internal] 
2020-01-03T14:38:25.4182197Z thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4182235Z 
2020-01-03T14:38:25.4182476Z ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
2020-01-03T14:38:25.4182508Z 
2020-01-03T14:38:25.4182508Z 
2020-01-03T14:38:25.4182564Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4182594Z 
2020-01-03T14:38:25.4182635Z fn local_inlining_but_not_all::inline[0]::inlined_function[0]
2020-01-03T14:38:25.4182871Z   expected: local_inlining_but_not_all-inline[External] 
2020-01-03T14:38:25.4183259Z   actual:   local_inlining_but_not_all-cgu.0[External] 
2020-01-03T14:38:25.4183306Z 
2020-01-03T14:38:25.4183347Z fn local_inlining_but_not_all::non_user[0]::baz[0]
2020-01-03T14:38:25.4183631Z   expected: local_inlining_but_not_all-non_user[External] 
2020-01-03T14:38:25.4183864Z   actual:   local_inlining_but_not_all-cgu.1[External] 
2020-01-03T14:38:25.4183895Z 
2020-01-03T14:38:25.4183935Z fn local_inlining_but_not_all::user1[0]::foo[0]
2020-01-03T14:38:25.4184165Z   expected: local_inlining_but_not_all-user1[External] 
2020-01-03T14:38:25.4184376Z   actual:   local_inlining_but_not_all-cgu.2[External] 
2020-01-03T14:38:25.4184406Z 
2020-01-03T14:38:25.4184459Z fn local_inlining_but_not_all::user2[0]::bar[0]
2020-01-03T14:38:25.4184673Z   expected: local_inlining_but_not_all-user2[External] 
2020-01-03T14:38:25.4184883Z   actual:   local_inlining_but_not_all-cgu.3[External] 
2020-01-03T14:38:25.4185238Z thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4185276Z 
2020-01-03T14:38:25.4185505Z ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
2020-01-03T14:38:25.4185560Z 
2020-01-03T14:38:25.4185560Z 
2020-01-03T14:38:25.4185602Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4185629Z 
2020-01-03T14:38:25.4185669Z fn local_inlining::inline[0]::inlined_function[0]
2020-01-03T14:38:25.4185919Z   expected: local_inlining-user1[Internal] local_inlining-user2[Internal] 
2020-01-03T14:38:25.4186151Z   actual:   local_inlining-cgu.1[Internal] local_inlining-cgu.2[Internal] 
2020-01-03T14:38:25.4186183Z 
2020-01-03T14:38:25.4186222Z fn local_inlining::non_user[0]::baz[0]
2020-01-03T14:38:25.4186443Z   expected: local_inlining-non_user[External] 
2020-01-03T14:38:25.4186767Z   actual:   local_inlining-cgu.0[External] 
2020-01-03T14:38:25.4186797Z 
2020-01-03T14:38:25.4186971Z fn local_inlining::user1[0]::foo[0]
2020-01-03T14:38:25.4187682Z   expected: local_inlining-user1[External] 
2020-01-03T14:38:25.4187922Z   actual:   local_inlining-cgu.1[External] 
2020-01-03T14:38:25.4187955Z 
2020-01-03T14:38:25.4188017Z fn local_inlining::user2[0]::bar[0]
2020-01-03T14:38:25.4188236Z   expected: local_inlining-user2[External] 
2020-01-03T14:38:25.4188462Z   actual:   local_inlining-cgu.2[External] 
2020-01-03T14:38:25.4188827Z thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4188868Z 
2020-01-03T14:38:25.4189121Z ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
2020-01-03T14:38:25.4189169Z 
2020-01-03T14:38:25.4189169Z 
2020-01-03T14:38:25.4189215Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4189245Z 
2020-01-03T14:38:25.4189288Z fn local_transitive_inlining::direct_user[0]::foo[0]
2020-01-03T14:38:25.4189546Z   expected: local_transitive_inlining-indirect_user[Internal] 
2020-01-03T14:38:25.4189776Z   actual:   local_transitive_inlining-cgu.0[Internal] 
2020-01-03T14:38:25.4189808Z 
2020-01-03T14:38:25.4189851Z fn local_transitive_inlining::indirect_user[0]::bar[0]
2020-01-03T14:38:25.4190101Z   expected: local_transitive_inlining-indirect_user[External] 
2020-01-03T14:38:25.4190669Z   actual:   local_transitive_inlining-cgu.0[External] 
2020-01-03T14:38:25.4190702Z 
2020-01-03T14:38:25.4190764Z fn local_transitive_inlining::inline[0]::inlined_function[0]
2020-01-03T14:38:25.4190993Z   expected: local_transitive_inlining-indirect_user[Internal] 
2020-01-03T14:38:25.4191209Z   actual:   local_transitive_inlining-cgu.0[Internal] 
2020-01-03T14:38:25.4191240Z 
2020-01-03T14:38:25.4191297Z fn local_transitive_inlining::non_user[0]::baz[0]
2020-01-03T14:38:25.4191520Z   expected: local_transitive_inlining-non_user[External] 
2020-01-03T14:38:25.4191737Z   actual:   local_transitive_inlining-cgu.1[External] 
2020-01-03T14:38:25.4192254Z thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4192310Z 
2020-01-03T14:38:25.4192596Z ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
2020-01-03T14:38:25.4192661Z 
2020-01-03T14:38:25.4192661Z 
2020-01-03T14:38:25.4192705Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4192733Z 
2020-01-03T14:38:25.4192771Z fn regular_modules::bar[0]
2020-01-03T14:38:25.4192828Z   expected: regular_modules[Internal] 
2020-01-03T14:38:25.4193045Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-03T14:38:25.4193131Z fn regular_modules::foo[0]
2020-01-03T14:38:25.4193131Z fn regular_modules::foo[0]
2020-01-03T14:38:25.4193174Z   expected: regular_modules[Internal] 
2020-01-03T14:38:25.4193387Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-03T14:38:25.4193418Z 
2020-01-03T14:38:25.4193474Z fn regular_modules::mod1[0]::bar[0]
2020-01-03T14:38:25.4193691Z   expected: regular_modules-mod1[Internal] 
2020-01-03T14:38:25.4194024Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-03T14:38:25.4194053Z 
2020-01-03T14:38:25.4194105Z fn regular_modules::mod1[0]::foo[0]
2020-01-03T14:38:25.4194308Z   expected: regular_modules-mod1[Internal] 
2020-01-03T14:38:25.4194521Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-03T14:38:25.4194564Z 
2020-01-03T14:38:25.4194603Z fn regular_modules::mod1[0]::mod1[0]::bar[0]
2020-01-03T14:38:25.4194814Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-03T14:38:25.4195017Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-03T14:38:25.4195062Z 
2020-01-03T14:38:25.4195101Z fn regular_modules::mod1[0]::mod1[0]::foo[0]
2020-01-03T14:38:25.4195309Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-03T14:38:25.4195524Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-03T14:38:25.4195554Z 
2020-01-03T14:38:25.4195640Z fn regular_modules::mod1[0]::mod2[0]::bar[0]
2020-01-03T14:38:25.4195853Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-03T14:38:25.4196080Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-03T14:38:25.4196110Z 
2020-01-03T14:38:25.4196149Z fn regular_modules::mod1[0]::mod2[0]::foo[0]
2020-01-03T14:38:25.4196369Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-03T14:38:25.4196584Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-03T14:38:25.4196614Z 
2020-01-03T14:38:25.4196653Z fn regular_modules::mod2[0]::bar[0]
2020-01-03T14:38:25.4196868Z   expected: regular_modules-mod2[Internal] 
2020-01-03T14:38:25.4197673Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-03T14:38:25.4197709Z 
2020-01-03T14:38:25.4197752Z fn regular_modules::mod2[0]::foo[0]
2020-01-03T14:38:25.4197993Z   expected: regular_modules-mod2[Internal] 
2020-01-03T14:38:25.4198838Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-03T14:38:25.4198882Z 
2020-01-03T14:38:25.4198945Z fn regular_modules::mod2[0]::mod1[0]::bar[0]
2020-01-03T14:38:25.4199194Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-03T14:38:25.4199429Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-03T14:38:25.4199462Z 
2020-01-03T14:38:25.4199519Z fn regular_modules::mod2[0]::mod1[0]::foo[0]
2020-01-03T14:38:25.4199748Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-03T14:38:25.4200198Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-03T14:38:25.4200250Z 
2020-01-03T14:38:25.4200293Z fn regular_modules::mod2[0]::mod2[0]::bar[0]
2020-01-03T14:38:25.4200520Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-03T14:38:25.4200735Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-03T14:38:25.4200782Z 
2020-01-03T14:38:25.4200950Z fn regular_modules::mod2[0]::mod2[0]::foo[0]
2020-01-03T14:38:25.4201164Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-03T14:38:25.4201384Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-03T14:38:25.4201455Z static regular_modules::BAZ[0]
2020-01-03T14:38:25.4201455Z static regular_modules::BAZ[0]
2020-01-03T14:38:25.4201497Z   expected: regular_modules[Internal] 
2020-01-03T14:38:25.4201818Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-03T14:38:25.4201860Z 
2020-01-03T14:38:25.4202027Z static regular_modules::mod1[0]::BAZ[0]
2020-01-03T14:38:25.4202270Z   expected: regular_modules-mod1[Internal] 
2020-01-03T14:38:25.4202495Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-03T14:38:25.4202534Z 
2020-01-03T14:38:25.4202574Z static regular_modules::mod1[0]::mod1[0]::BAZ[0]
2020-01-03T14:38:25.4202796Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-03T14:38:25.4203001Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-03T14:38:25.4203030Z 
2020-01-03T14:38:25.4203070Z static regular_modules::mod1[0]::mod2[0]::BAZ[0]
2020-01-03T14:38:25.4203290Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-03T14:38:25.4203491Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-03T14:38:25.4203520Z 
2020-01-03T14:38:25.4203559Z static regular_modules::mod2[0]::BAZ[0]
2020-01-03T14:38:25.4203776Z   expected: regular_modules-mod2[Internal] 
2020-01-03T14:38:25.4203986Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-03T14:38:25.4204016Z 
2020-01-03T14:38:25.4204070Z static regular_modules::mod2[0]::mod1[0]::BAZ[0]
2020-01-03T14:38:25.4204278Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-03T14:38:25.4204478Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-03T14:38:25.4204515Z 
2020-01-03T14:38:25.4204568Z static regular_modules::mod2[0]::mod2[0]::BAZ[0]
2020-01-03T14:38:25.4204777Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-03T14:38:25.4204977Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-03T14:38:25.4205315Z thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4205353Z 
2020-01-03T14:38:25.4205574Z ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
2020-01-03T14:38:25.4205620Z 
2020-01-03T14:38:25.4205620Z 
2020-01-03T14:38:25.4205662Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4205690Z 
2020-01-03T14:38:25.4205734Z fn statics::function[0]
2020-01-03T14:38:25.4205791Z   expected: statics[External] 
2020-01-03T14:38:25.4205988Z   actual:   statics-cgu.0[External] 
2020-01-03T14:38:25.4206018Z 
2020-01-03T14:38:25.4206176Z fn statics::mod1[0]::function[0]
2020-01-03T14:38:25.4206401Z   expected: statics-mod1[External] 
2020-01-03T14:38:25.4206599Z   actual:   statics-cgu.1[External] 
2020-01-03T14:38:25.4206800Z static statics::BAR[0]
2020-01-03T14:38:25.4206800Z static statics::BAR[0]
2020-01-03T14:38:25.4206843Z   expected: statics[Internal] 
2020-01-03T14:38:25.4207681Z   actual:   statics-cgu.0[Internal] 
2020-01-03T14:38:25.4207786Z static statics::FOO[0]
2020-01-03T14:38:25.4207786Z static statics::FOO[0]
2020-01-03T14:38:25.4207829Z   expected: statics[Internal] 
2020-01-03T14:38:25.4208047Z   actual:   statics-cgu.0[Internal] 
2020-01-03T14:38:25.4208079Z 
2020-01-03T14:38:25.4208136Z static statics::function[0]::BAR[0]
2020-01-03T14:38:25.4208178Z   expected: statics[Internal] 
2020-01-03T14:38:25.4208398Z   actual:   statics-cgu.0[Internal] 
2020-01-03T14:38:25.4208430Z 
2020-01-03T14:38:25.4208486Z static statics::function[0]::FOO[0]
2020-01-03T14:38:25.4208529Z   expected: statics[Internal] 
2020-01-03T14:38:25.4208740Z   actual:   statics-cgu.0[Internal] 
2020-01-03T14:38:25.4208785Z 
2020-01-03T14:38:25.4208984Z static statics::mod1[0]::BAR[0]
2020-01-03T14:38:25.4209241Z   expected: statics-mod1[Internal] 
2020-01-03T14:38:25.4209452Z   actual:   statics-cgu.1[Internal] 
2020-01-03T14:38:25.4209504Z 
2020-01-03T14:38:25.4209545Z static statics::mod1[0]::FOO[0]
2020-01-03T14:38:25.4209754Z   expected: statics-mod1[Internal] 
2020-01-03T14:38:25.4209978Z   actual:   statics-cgu.1[Internal] 
2020-01-03T14:38:25.4210010Z 
2020-01-03T14:38:25.4210053Z static statics::mod1[0]::function[0]::BAR[0]
2020-01-03T14:38:25.4210260Z   expected: statics-mod1[Internal] 
2020-01-03T14:38:25.4210484Z   actual:   statics-cgu.1[Internal] 
2020-01-03T14:38:25.4210516Z 
2020-01-03T14:38:25.4210558Z static statics::mod1[0]::function[0]::FOO[0]
2020-01-03T14:38:25.4210862Z   expected: statics-mod1[Internal] 
2020-01-03T14:38:25.4211266Z   actual:   statics-cgu.1[Internal] 
2020-01-03T14:38:25.4211595Z thread '[codegen-units] codegen-units/partitioning/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4211643Z 
2020-01-03T14:38:25.4212025Z ---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----
2020-01-03T14:38:25.4212057Z 
2020-01-03T14:38:25.4212057Z 
2020-01-03T14:38:25.4212099Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4212141Z 
2020-01-03T14:38:25.4212180Z fn shared_generics_aux::generic_fn[0]<u16>
2020-01-03T14:38:25.4212409Z   expected: shared_generics_aux-in-shared_generics.volatile[External] 
2020-01-03T14:38:25.4212648Z   actual:   shared_generics-cgu.1[External] 
2020-01-03T14:38:25.4213025Z thread '[codegen-units] codegen-units/partitioning/shared-generics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4213065Z 
2020-01-03T14:38:25.4213322Z ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
2020-01-03T14:38:25.4213372Z 
2020-01-03T14:38:25.4213372Z 
2020-01-03T14:38:25.4213418Z The following items were assigned to wrong codegen units:
2020-01-03T14:38:25.4213455Z 
2020-01-03T14:38:25.4213498Z fn core::ptr[0]::real_drop_in_place[0]<u32>
2020-01-03T14:38:25.4213558Z   expected: vtable_through_const[Internal] 
2020-01-03T14:38:25.4213793Z   actual:   vtable_through_const-cgu.0[Internal] 
2020-01-03T14:38:25.4213826Z 
2020-01-03T14:38:25.4213886Z fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32>
2020-01-03T14:38:25.4214125Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-03T14:38:25.4214355Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-03T14:38:25.4214387Z 
2020-01-03T14:38:25.4214448Z fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32>
2020-01-03T14:38:25.4214697Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-03T14:38:25.4214928Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-03T14:38:25.4214974Z 
2020-01-03T14:38:25.4215019Z fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32>
2020-01-03T14:38:25.4215268Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-03T14:38:25.4215509Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-03T14:38:25.4215542Z 
2020-01-03T14:38:25.4215588Z fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32>
2020-01-03T14:38:25.4215826Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-03T14:38:25.4216071Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-03T14:38:25.4216102Z 
2020-01-03T14:38:25.4216145Z fn vtable_through_const::mod1[0]::id[0]<char>
2020-01-03T14:38:25.4216379Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-03T14:38:25.4216627Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-03T14:38:25.4216658Z 
2020-01-03T14:38:25.4216710Z fn vtable_through_const::mod1[0]::id[0]<i64>
2020-01-03T14:38:25.4217610Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-03T14:38:25.4217914Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-03T14:38:25.4217947Z 
2020-01-03T14:38:25.4218158Z fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8>
2020-01-03T14:38:25.4218468Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-03T14:38:25.4218695Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-03T14:38:25.4218728Z 
2020-01-03T14:38:25.4218791Z fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8>
2020-01-03T14:38:25.4219028Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-03T14:38:25.4219248Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-03T14:38:25.4219281Z 
2020-01-03T14:38:25.4219340Z fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8>
2020-01-03T14:38:25.4219574Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-03T14:38:25.4219912Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-03T14:38:25.4219974Z 
2020-01-03T14:38:25.4220023Z fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8>
2020-01-03T14:38:25.4220985Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-03T14:38:25.4221503Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-03T14:38:25.4221899Z thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-03T14:38:25.4221939Z 
2020-01-03T14:38:25.4221963Z 
2020-01-03T14:38:25.4222017Z failures:
---
2020-01-03T14:38:25.4228250Z test result: FAILED. 24 passed; 12 failed; 3 ignored; 0 measured; 0 filtered out
2020-01-03T14:38:25.4228286Z 
2020-01-03T14:38:25.4228325Z 
2020-01-03T14:38:25.4228362Z 
2020-01-03T14:38:25.4229990Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-03T14:38:25.4230456Z 
2020-01-03T14:38:25.4230672Z 
2020-01-03T14:38:25.4230765Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-03T14:38:25.4230825Z Build completed unsuccessfully in 1:05:04
2020-01-03T14:38:25.4230825Z Build completed unsuccessfully in 1:05:04
2020-01-03T14:38:25.4230869Z == clock drift check ==
2020-01-03T14:38:25.4238620Z   local time: Fri Jan  3 14:38:25 UTC 2020
2020-01-03T14:38:25.9793957Z   network time: Fri, 03 Jan 2020 14:38:25 GMT
2020-01-03T14:38:25.9798952Z == end clock drift check ==
2020-01-03T14:38:31.1152205Z 
2020-01-03T14:38:31.1286860Z ##[error]Bash exited with code '1'.
2020-01-03T14:38:31.1325922Z ##[section]Starting: Checkout
2020-01-03T14:38:31.1327830Z ==============================================================================
2020-01-03T14:38:31.1327901Z Task         : Get sources
2020-01-03T14:38:31.1327962Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
