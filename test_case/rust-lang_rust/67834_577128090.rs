plain
2020-01-22T09:58:03.3918673Z ========================== Starting Command Output ===========================
2020-01-22T09:58:03.3920267Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/182ed7c0-0780-4c16-94b2-37496d747765.sh
2020-01-22T09:58:03.3920305Z 
2020-01-22T09:58:03.3923003Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-22T09:58:03.3928892Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67834/merge to s
2020-01-22T09:58:03.3930671Z Task         : Get sources
2020-01-22T09:58:03.3930706Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T09:58:03.3930737Z Version      : 1.0.0
2020-01-22T09:58:03.3930768Z Author       : Microsoft
---
2020-01-22T09:58:04.4620475Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-22T09:58:04.4642098Z ##[command]git config gc.auto 0
2020-01-22T09:58:04.4643994Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-22T09:58:04.4645564Z ##[command]git config --get-all http.proxy
2020-01-22T09:58:04.4650897Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67834/merge:refs/remotes/pull/67834/merge
---
2020-01-22T10:49:31.5780384Z .................................................................................................... 1700/9545
2020-01-22T10:49:37.6617584Z .................................................................................................... 1800/9545
2020-01-22T10:49:48.3655717Z ....................i............................................................................... 1900/9545
2020-01-22T10:49:54.9872984Z .................................................................................................... 2000/9545
2020-01-22T10:50:09.0245867Z ..........iiiii..................................................................................... 2100/9545
2020-01-22T10:50:18.2001639Z .................................................................................................... 2300/9545
2020-01-22T10:50:20.5183100Z .................................................................................................... 2400/9545
2020-01-22T10:50:25.5934887Z .................................................................................................... 2500/9545
2020-01-22T10:50:44.8492818Z .................................................................................................... 2600/9545
---
2020-01-22T10:53:16.2696656Z ......................................................i...............i............................. 4900/9545
2020-01-22T10:53:24.0484935Z .................................................................................................... 5000/9545
2020-01-22T10:53:31.2439326Z .................................................................................................i.. 5100/9545
2020-01-22T10:53:36.0920339Z .................................................................................................... 5200/9545
2020-01-22T10:53:46.2656123Z .....................................................................ii.ii...........i.............. 5300/9545
2020-01-22T10:53:54.8828138Z ......i............................................................................................. 5500/9545
2020-01-22T10:54:04.4064181Z .................................................................................................... 5600/9545
2020-01-22T10:54:10.8544818Z .......................................................i............................................ 5700/9545
2020-01-22T10:54:17.3103569Z .................................................................................................... 5800/9545
2020-01-22T10:54:17.3103569Z .................................................................................................... 5800/9545
2020-01-22T10:54:26.5533292Z .................................................................................................... 5900/9545
2020-01-22T10:54:32.9944482Z ..............................................ii...i..ii...........i................................ 6000/9545
2020-01-22T10:54:54.2803554Z .................................................................................................... 6200/9545
2020-01-22T10:55:02.5012053Z .................................................................................................... 6300/9545
2020-01-22T10:55:02.5012053Z .................................................................................................... 6300/9545
2020-01-22T10:55:09.7142491Z ..........................................................................i..ii..................... 6400/9545
2020-01-22T10:55:35.2182546Z .................................................................................................... 6600/9545
2020-01-22T10:55:38.7384716Z ..................................................i................................................. 6700/9545
2020-01-22T10:55:40.7198423Z .................................................................................................... 6800/9545
2020-01-22T10:55:42.7204612Z .................................................i.................................................. 6900/9545
---
2020-01-22T10:57:15.0373992Z .................................................................................................... 7600/9545
2020-01-22T10:57:20.3586429Z .................................................................................................... 7700/9545
2020-01-22T10:57:26.5508117Z .................................................................................................... 7800/9545
2020-01-22T10:57:36.6815719Z .................................................................................................... 7900/9545
2020-01-22T10:57:42.1222573Z ......iiiiiii....................................................................................... 8000/9545
2020-01-22T10:57:55.9805487Z .................................................................................................... 8200/9545
2020-01-22T10:58:06.8680350Z .................................................................................................... 8300/9545
2020-01-22T10:58:19.3009203Z .................................................................................................... 8400/9545
2020-01-22T10:58:25.1192787Z .................................................................................................... 8500/9545
---
2020-01-22T11:00:37.7635237Z  finished in 6.748
2020-01-22T11:00:37.7817081Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-22T11:00:37.9724808Z 
2020-01-22T11:00:37.9726114Z running 166 tests
2020-01-22T11:00:40.7087570Z iiii......i........ii..iiii...i....i...........i............i..i..................i....i............ 100/166
2020-01-22T11:00:42.7178309Z i.i.i...iii..iiiiiii.......................iii............ii......
2020-01-22T11:00:42.7182631Z 
2020-01-22T11:00:42.7187254Z  finished in 4.937
2020-01-22T11:00:42.7346427Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-22T11:00:42.9019461Z 
2020-01-22T11:00:42.9019461Z 
2020-01-22T11:00:42.9020244Z running 39 tests
2020-01-22T11:00:44.6628771Z i.........i...............FFFFFFFiFFFFF
2020-01-22T11:00:44.6629099Z 
2020-01-22T11:00:44.6629382Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2020-01-22T11:00:44.6629413Z 
2020-01-22T11:00:44.6629454Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6629454Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6629479Z 
2020-01-22T11:00:44.6629534Z fn core::ptr[0]::drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
2020-01-22T11:00:44.6629755Z   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
2020-01-22T11:00:44.6629954Z   actual:   extern_drop_glue-cgu.0[Internal] extern_drop_glue-cgu.1[Internal] 
2020-01-22T11:00:44.6629997Z 
2020-01-22T11:00:44.6630035Z fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
2020-01-22T11:00:44.6630074Z   expected: extern_drop_glue[Internal] 
2020-01-22T11:00:44.6630255Z   actual:   extern_drop_glue-cgu.0[Internal] 
2020-01-22T11:00:44.6630298Z 
2020-01-22T11:00:44.6630517Z fn core::ptr[0]::drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
2020-01-22T11:00:44.6630743Z   expected: extern_drop_glue-mod1[Internal] 
2020-01-22T11:00:44.6631106Z   actual:   extern_drop_glue-cgu.1[Internal] 
2020-01-22T11:00:44.6631134Z 
2020-01-22T11:00:44.6631169Z fn extern_drop_glue::mod1[0]::user[0]
2020-01-22T11:00:44.6631343Z   expected: extern_drop_glue-mod1[External] 
2020-01-22T11:00:44.6631538Z   actual:   extern_drop_glue-cgu.1[External] 
2020-01-22T11:00:44.6631667Z 
2020-01-22T11:00:44.6631704Z fn extern_drop_glue::user[0]
2020-01-22T11:00:44.6631760Z   expected: extern_drop_glue[External] 
2020-01-22T11:00:44.6631966Z   actual:   extern_drop_glue-cgu.0[External] 
2020-01-22T11:00:44.6632248Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6632318Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-22T11:00:44.6632356Z 
2020-01-22T11:00:44.6632562Z ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
2020-01-22T11:00:44.6632562Z ---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----
2020-01-22T11:00:44.6632605Z 
2020-01-22T11:00:44.6632643Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6632669Z 
2020-01-22T11:00:44.6632704Z fn cgu_generic_function::bar[0]<&str>
2020-01-22T11:00:44.6632918Z   expected: cgu_generic_function-in-extern_generic.volatile[External] 
2020-01-22T11:00:44.6633106Z   actual:   extern_generic-cgu.0[External] 
2020-01-22T11:00:44.6633132Z 
2020-01-22T11:00:44.6633166Z fn cgu_generic_function::foo[0]<&str>
2020-01-22T11:00:44.6633377Z   expected: cgu_generic_function-in-extern_generic.volatile[External] 
2020-01-22T11:00:44.6633550Z   actual:   extern_generic-cgu.0[External] 
2020-01-22T11:00:44.6633576Z 
2020-01-22T11:00:44.6633627Z fn extern_generic::mod1[0]::mod1[0]::user[0]
2020-01-22T11:00:44.6633804Z   expected: extern_generic-mod1-mod1[Internal] 
2020-01-22T11:00:44.6633983Z   actual:   extern_generic-cgu.3[Internal] 
2020-01-22T11:00:44.6634009Z 
2020-01-22T11:00:44.6634062Z fn extern_generic::mod1[0]::user[0]
2020-01-22T11:00:44.6634237Z   expected: extern_generic-mod1[Internal] 
2020-01-22T11:00:44.6634407Z   actual:   extern_generic-cgu.2[Internal] 
2020-01-22T11:00:44.6634433Z 
2020-01-22T11:00:44.6634484Z fn extern_generic::mod2[0]::user[0]
2020-01-22T11:00:44.6634656Z   expected: extern_generic-mod2[Internal] 
2020-01-22T11:00:44.6634825Z   actual:   extern_generic-cgu.4[Internal] 
2020-01-22T11:00:44.6634874Z 
2020-01-22T11:00:44.6634910Z fn extern_generic::mod3[0]::non_user[0]
2020-01-22T11:00:44.6635081Z   expected: extern_generic-mod3[Internal] 
2020-01-22T11:00:44.6635252Z   actual:   extern_generic-cgu.5[Internal] 
2020-01-22T11:00:44.6635328Z fn extern_generic::user[0]
2020-01-22T11:00:44.6635328Z fn extern_generic::user[0]
2020-01-22T11:00:44.6635363Z   expected: extern_generic[Internal] 
2020-01-22T11:00:44.6635555Z   actual:   extern_generic-cgu.1[Internal] 
2020-01-22T11:00:44.6636063Z thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6636097Z 
2020-01-22T11:00:44.6636329Z ---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----
2020-01-22T11:00:44.6636359Z 
2020-01-22T11:00:44.6636359Z 
2020-01-22T11:00:44.6636397Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6636422Z 
2020-01-22T11:00:44.6636485Z fn cgu_explicit_inlining::always_inlined[0]
2020-01-22T11:00:44.6636705Z   expected: inlining_from_extern_crate-mod2[Internal] inlining_from_extern_crate[Internal] 
2020-01-22T11:00:44.6637281Z   actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.2[Internal] 
2020-01-22T11:00:44.6637331Z 
2020-01-22T11:00:44.6637371Z fn cgu_explicit_inlining::inlined[0]
2020-01-22T11:00:44.6637604Z   expected: inlining_from_extern_crate-mod1[Internal] inlining_from_extern_crate[Internal] 
2020-01-22T11:00:44.6637917Z   actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.1[Internal] 
2020-01-22T11:00:44.6637973Z 
2020-01-22T11:00:44.6638015Z fn inlining_from_extern_crate::mod1[0]::user[0]
2020-01-22T11:00:44.6638247Z   expected: inlining_from_extern_crate-mod1[External] 
2020-01-22T11:00:44.6638468Z   actual:   inlining_from_extern_crate-cgu.1[External] 
2020-01-22T11:00:44.6638497Z 
2020-01-22T11:00:44.6638536Z fn inlining_from_extern_crate::mod2[0]::user[0]
2020-01-22T11:00:44.6638985Z   expected: inlining_from_extern_crate-mod2[External] 
2020-01-22T11:00:44.6639356Z   actual:   inlining_from_extern_crate-cgu.2[External] 
2020-01-22T11:00:44.6639419Z fn inlining_from_extern_crate::user[0]
2020-01-22T11:00:44.6639419Z fn inlining_from_extern_crate::user[0]
2020-01-22T11:00:44.6639475Z   expected: inlining_from_extern_crate[External] 
2020-01-22T11:00:44.6639665Z   actual:   inlining_from_extern_crate-cgu.0[External] 
2020-01-22T11:00:44.6639976Z thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6640030Z 
2020-01-22T11:00:44.6640237Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2020-01-22T11:00:44.6640264Z 
2020-01-22T11:00:44.6640264Z 
2020-01-22T11:00:44.6640303Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6640346Z 
2020-01-22T11:00:44.6640542Z fn core::ptr[0]::drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
2020-01-22T11:00:44.6640731Z   expected: local_drop_glue-mod1[Internal] 
2020-01-22T11:00:44.6640923Z   actual:   local_drop_glue-cgu.1[Internal] 
2020-01-22T11:00:44.6640949Z 
2020-01-22T11:00:44.6640987Z fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Outer[0]>
2020-01-22T11:00:44.6641026Z   expected: local_drop_glue[Internal] 
2020-01-22T11:00:44.6641220Z   actual:   local_drop_glue-cgu.0[Internal] 
2020-01-22T11:00:44.6641246Z 
2020-01-22T11:00:44.6641283Z fn core::ptr[0]::drop_in_place[0]<local_drop_glue::Struct[0]>
2020-01-22T11:00:44.6641500Z   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
2020-01-22T11:00:44.6641704Z   actual:   local_drop_glue-cgu.0[Internal] local_drop_glue-cgu.1[Internal] 
2020-01-22T11:00:44.6641732Z 
2020-01-22T11:00:44.6641769Z fn core::ptr[0]::drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
2020-01-22T11:00:44.6641960Z   expected: local_drop_glue-mod1[Internal] 
2020-01-22T11:00:44.6642239Z   actual:   local_drop_glue-cgu.1[Internal] 
2020-01-22T11:00:44.6642274Z 
2020-01-22T11:00:44.6642482Z fn local_drop_glue::mod1[0]::user[0]
2020-01-22T11:00:44.6642682Z   expected: local_drop_glue-mod1[External] 
2020-01-22T11:00:44.6642861Z   actual:   local_drop_glue-cgu.1[External] 
2020-01-22T11:00:44.6642887Z 
2020-01-22T11:00:44.6642939Z fn local_drop_glue::user[0]
2020-01-22T11:00:44.6642977Z   expected: local_drop_glue[External] 
2020-01-22T11:00:44.6643153Z   actual:   local_drop_glue-cgu.0[External] 
2020-01-22T11:00:44.6643179Z 
2020-01-22T11:00:44.6643239Z fn local_drop_glue::{{impl}}[0]::drop[0]
2020-01-22T11:00:44.6643277Z   expected: local_drop_glue[External] 
2020-01-22T11:00:44.6643457Z   actual:   local_drop_glue-cgu.0[External] 
2020-01-22T11:00:44.6643760Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6643793Z 
2020-01-22T11:00:44.6643995Z ---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----
2020-01-22T11:00:44.6644047Z 
2020-01-22T11:00:44.6644047Z 
2020-01-22T11:00:44.6644085Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6644110Z 
2020-01-22T11:00:44.6644146Z fn local_generic::generic[0]<&str>
2020-01-22T11:00:44.6644204Z   expected: local_generic.volatile[External] 
2020-01-22T11:00:44.6644385Z   actual:   local_generic-cgu.4[External] 
2020-01-22T11:00:44.6644411Z 
2020-01-22T11:00:44.6644446Z fn local_generic::generic[0]<char>
2020-01-22T11:00:44.6644566Z   expected: local_generic.volatile[External] 
2020-01-22T11:00:44.6644776Z   actual:   local_generic-cgu.4[External] 
2020-01-22T11:00:44.6644802Z 
2020-01-22T11:00:44.6644855Z fn local_generic::generic[0]<u32>
2020-01-22T11:00:44.6644894Z   expected: local_generic.volatile[External] 
2020-01-22T11:00:44.6645075Z   actual:   local_generic-cgu.4[External] 
2020-01-22T11:00:44.6645100Z 
2020-01-22T11:00:44.6645303Z fn local_generic::generic[0]<u64>
2020-01-22T11:00:44.6645340Z   expected: local_generic.volatile[External] 
2020-01-22T11:00:44.6645602Z   actual:   local_generic-cgu.4[External] 
2020-01-22T11:00:44.6645627Z 
2020-01-22T11:00:44.6645680Z fn local_generic::mod1[0]::mod1[0]::user[0]
2020-01-22T11:00:44.6645858Z   expected: local_generic-mod1-mod1[Internal] 
2020-01-22T11:00:44.6646027Z   actual:   local_generic-cgu.2[Internal] 
2020-01-22T11:00:44.6646068Z 
2020-01-22T11:00:44.6646103Z fn local_generic::mod1[0]::user[0]
2020-01-22T11:00:44.6646274Z   expected: local_generic-mod1[Internal] 
2020-01-22T11:00:44.6646452Z   actual:   local_generic-cgu.1[Internal] 
2020-01-22T11:00:44.6646497Z 
2020-01-22T11:00:44.6646532Z fn local_generic::mod2[0]::user[0]
2020-01-22T11:00:44.6646704Z   expected: local_generic-mod2[Internal] 
2020-01-22T11:00:44.6646893Z   actual:   local_generic-cgu.3[Internal] 
2020-01-22T11:00:44.6646951Z fn local_generic::user[0]
2020-01-22T11:00:44.6646951Z fn local_generic::user[0]
2020-01-22T11:00:44.6646987Z   expected: local_generic[Internal] 
2020-01-22T11:00:44.6647175Z   actual:   local_generic-cgu.0[Internal] 
2020-01-22T11:00:44.6647459Z thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6647490Z 
2020-01-22T11:00:44.6647712Z ---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----
2020-01-22T11:00:44.6647741Z 
2020-01-22T11:00:44.6647741Z 
2020-01-22T11:00:44.6647778Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6647802Z 
2020-01-22T11:00:44.6647861Z fn local_inlining_but_not_all::inline[0]::inlined_function[0]
2020-01-22T11:00:44.6648048Z   expected: local_inlining_but_not_all-inline[External] 
2020-01-22T11:00:44.6648231Z   actual:   local_inlining_but_not_all-cgu.0[External] 
2020-01-22T11:00:44.6648273Z 
2020-01-22T11:00:44.6648310Z fn local_inlining_but_not_all::non_user[0]::baz[0]
2020-01-22T11:00:44.6648494Z   expected: local_inlining_but_not_all-non_user[External] 
2020-01-22T11:00:44.6648675Z   actual:   local_inlining_but_not_all-cgu.1[External] 
2020-01-22T11:00:44.6648724Z 
2020-01-22T11:00:44.6648761Z fn local_inlining_but_not_all::user1[0]::foo[0]
2020-01-22T11:00:44.6648945Z   expected: local_inlining_but_not_all-user1[External] 
2020-01-22T11:00:44.6649145Z   actual:   local_inlining_but_not_all-cgu.2[External] 
2020-01-22T11:00:44.6649171Z 
2020-01-22T11:00:44.6649386Z fn local_inlining_but_not_all::user2[0]::bar[0]
2020-01-22T11:00:44.6649621Z   expected: local_inlining_but_not_all-user2[External] 
2020-01-22T11:00:44.6649836Z   actual:   local_inlining_but_not_all-cgu.3[External] 
2020-01-22T11:00:44.6650134Z thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6650183Z 
2020-01-22T11:00:44.6650391Z ---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----
2020-01-22T11:00:44.6650420Z 
2020-01-22T11:00:44.6650420Z 
2020-01-22T11:00:44.6650458Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6650505Z 
2020-01-22T11:00:44.6650542Z fn local_transitive_inlining::direct_user[0]::foo[0]
2020-01-22T11:00:44.6650737Z   expected: local_transitive_inlining-indirect_user[Internal] 
2020-01-22T11:00:44.6650919Z   actual:   local_transitive_inlining-cgu.0[Internal] 
2020-01-22T11:00:44.6650960Z 
2020-01-22T11:00:44.6650996Z fn local_transitive_inlining::indirect_user[0]::bar[0]
2020-01-22T11:00:44.6651184Z   expected: local_transitive_inlining-indirect_user[External] 
2020-01-22T11:00:44.6651475Z   actual:   local_transitive_inlining-cgu.0[External] 
2020-01-22T11:00:44.6651508Z 
2020-01-22T11:00:44.6651548Z fn local_transitive_inlining::inline[0]::inlined_function[0]
2020-01-22T11:00:44.6651768Z   expected: local_transitive_inlining-indirect_user[Internal] 
2020-01-22T11:00:44.6651970Z   actual:   local_transitive_inlining-cgu.0[Internal] 
2020-01-22T11:00:44.6651995Z 
2020-01-22T11:00:44.6652030Z fn local_transitive_inlining::non_user[0]::baz[0]
2020-01-22T11:00:44.6652411Z   expected: local_transitive_inlining-non_user[External] 
2020-01-22T11:00:44.6652598Z   actual:   local_transitive_inlining-cgu.1[External] 
2020-01-22T11:00:44.6652885Z thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6652935Z 
2020-01-22T11:00:44.6653133Z ---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----
2020-01-22T11:00:44.6653161Z 
2020-01-22T11:00:44.6653161Z 
2020-01-22T11:00:44.6653206Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6653249Z 
2020-01-22T11:00:44.6653372Z fn local_inlining::inline[0]::inlined_function[0]
2020-01-22T11:00:44.6653606Z   expected: local_inlining-user1[Internal] local_inlining-user2[Internal] 
2020-01-22T11:00:44.6653806Z   actual:   local_inlining-cgu.1[Internal] local_inlining-cgu.2[Internal] 
2020-01-22T11:00:44.6653833Z 
2020-01-22T11:00:44.6653884Z fn local_inlining::non_user[0]::baz[0]
2020-01-22T11:00:44.6654070Z   expected: local_inlining-non_user[External] 
2020-01-22T11:00:44.6654243Z   actual:   local_inlining-cgu.0[External] 
2020-01-22T11:00:44.6654268Z 
2020-01-22T11:00:44.6654319Z fn local_inlining::user1[0]::foo[0]
2020-01-22T11:00:44.6654493Z   expected: local_inlining-user1[External] 
2020-01-22T11:00:44.6654664Z   actual:   local_inlining-cgu.1[External] 
2020-01-22T11:00:44.6654704Z 
2020-01-22T11:00:44.6654740Z fn local_inlining::user2[0]::bar[0]
2020-01-22T11:00:44.6654920Z   expected: local_inlining-user2[External] 
2020-01-22T11:00:44.6655095Z   actual:   local_inlining-cgu.2[External] 
2020-01-22T11:00:44.6655392Z thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6655424Z 
2020-01-22T11:00:44.6655636Z ---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----
2020-01-22T11:00:44.6655672Z 
2020-01-22T11:00:44.6655672Z 
2020-01-22T11:00:44.6655709Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6655733Z 
2020-01-22T11:00:44.6655784Z fn regular_modules::bar[0]
2020-01-22T11:00:44.6655820Z   expected: regular_modules[Internal] 
2020-01-22T11:00:44.6655998Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-22T11:00:44.6656073Z fn regular_modules::foo[0]
2020-01-22T11:00:44.6656073Z fn regular_modules::foo[0]
2020-01-22T11:00:44.6656110Z   expected: regular_modules[Internal] 
2020-01-22T11:00:44.6656285Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-22T11:00:44.6656318Z 
2020-01-22T11:00:44.6656370Z fn regular_modules::mod1[0]::bar[0]
2020-01-22T11:00:44.6656546Z   expected: regular_modules-mod1[Internal] 
2020-01-22T11:00:44.6656718Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-22T11:00:44.6656762Z 
2020-01-22T11:00:44.6656797Z fn regular_modules::mod1[0]::foo[0]
2020-01-22T11:00:44.6656970Z   expected: regular_modules-mod1[Internal] 
2020-01-22T11:00:44.6657316Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-22T11:00:44.6657496Z 
2020-01-22T11:00:44.6657533Z fn regular_modules::mod1[0]::mod1[0]::bar[0]
2020-01-22T11:00:44.6657719Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-22T11:00:44.6657915Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-22T11:00:44.6657941Z 
2020-01-22T11:00:44.6657978Z fn regular_modules::mod1[0]::mod1[0]::foo[0]
2020-01-22T11:00:44.6658160Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-22T11:00:44.6658355Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-22T11:00:44.6658448Z 
2020-01-22T11:00:44.6658491Z fn regular_modules::mod1[0]::mod2[0]::bar[0]
2020-01-22T11:00:44.6658743Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-22T11:00:44.6658939Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-22T11:00:44.6658965Z 
2020-01-22T11:00:44.6659001Z fn regular_modules::mod1[0]::mod2[0]::foo[0]
2020-01-22T11:00:44.6659198Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-22T11:00:44.6659377Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-22T11:00:44.6659470Z 
2020-01-22T11:00:44.6659507Z fn regular_modules::mod2[0]::bar[0]
2020-01-22T11:00:44.6659730Z   expected: regular_modules-mod2[Internal] 
2020-01-22T11:00:44.6659909Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-22T11:00:44.6659934Z 
2020-01-22T11:00:44.6659986Z fn regular_modules::mod2[0]::foo[0]
2020-01-22T11:00:44.6660163Z   expected: regular_modules-mod2[Internal] 
2020-01-22T11:00:44.6660489Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-22T11:00:44.6660514Z 
2020-01-22T11:00:44.6660574Z fn regular_modules::mod2[0]::mod1[0]::bar[0]
2020-01-22T11:00:44.6660753Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-22T11:00:44.6660925Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-22T11:00:44.6660949Z 
2020-01-22T11:00:44.6661001Z fn regular_modules::mod2[0]::mod1[0]::foo[0]
2020-01-22T11:00:44.6661178Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-22T11:00:44.6661507Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-22T11:00:44.6661557Z 
2020-01-22T11:00:44.6661591Z fn regular_modules::mod2[0]::mod2[0]::bar[0]
2020-01-22T11:00:44.6661765Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-22T11:00:44.6661932Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-22T11:00:44.6661974Z 
2020-01-22T11:00:44.6662008Z fn regular_modules::mod2[0]::mod2[0]::foo[0]
2020-01-22T11:00:44.6662347Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-22T11:00:44.6662537Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-22T11:00:44.6662604Z static regular_modules::BAZ[0]
2020-01-22T11:00:44.6662604Z static regular_modules::BAZ[0]
2020-01-22T11:00:44.6662641Z   expected: regular_modules[Internal] 
2020-01-22T11:00:44.6662833Z   actual:   regular_modules-cgu.0[Internal] 
2020-01-22T11:00:44.6662858Z 
2020-01-22T11:00:44.6662893Z static regular_modules::mod1[0]::BAZ[0]
2020-01-22T11:00:44.6663064Z   expected: regular_modules-mod1[Internal] 
2020-01-22T11:00:44.6663256Z   actual:   regular_modules-cgu.1[Internal] 
2020-01-22T11:00:44.6663281Z 
2020-01-22T11:00:44.6663323Z static regular_modules::mod1[0]::mod1[0]::BAZ[0]
2020-01-22T11:00:44.6663515Z   expected: regular_modules-mod1-mod1[Internal] 
2020-01-22T11:00:44.6663689Z   actual:   regular_modules-cgu.2[Internal] 
2020-01-22T11:00:44.6663713Z 
2020-01-22T11:00:44.6663749Z static regular_modules::mod1[0]::mod2[0]::BAZ[0]
2020-01-22T11:00:44.6663939Z   expected: regular_modules-mod1-mod2[Internal] 
2020-01-22T11:00:44.6664113Z   actual:   regular_modules-cgu.3[Internal] 
2020-01-22T11:00:44.6664137Z 
2020-01-22T11:00:44.6664194Z static regular_modules::mod2[0]::BAZ[0]
2020-01-22T11:00:44.6664371Z   expected: regular_modules-mod2[Internal] 
2020-01-22T11:00:44.6664545Z   actual:   regular_modules-cgu.4[Internal] 
2020-01-22T11:00:44.6664570Z 
2020-01-22T11:00:44.6664621Z static regular_modules::mod2[0]::mod1[0]::BAZ[0]
2020-01-22T11:00:44.6664798Z   expected: regular_modules-mod2-mod1[Internal] 
2020-01-22T11:00:44.6664970Z   actual:   regular_modules-cgu.5[Internal] 
2020-01-22T11:00:44.6664995Z 
2020-01-22T11:00:44.6665054Z static regular_modules::mod2[0]::mod2[0]::BAZ[0]
2020-01-22T11:00:44.6665385Z   expected: regular_modules-mod2-mod2[Internal] 
2020-01-22T11:00:44.6665553Z   actual:   regular_modules-cgu.6[Internal] 
2020-01-22T11:00:44.6665840Z thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6665871Z 
2020-01-22T11:00:44.6666125Z ---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----
2020-01-22T11:00:44.6666176Z 
2020-01-22T11:00:44.6666176Z 
2020-01-22T11:00:44.6666214Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6666238Z 
2020-01-22T11:00:44.6666272Z fn shared_generics_aux::generic_fn[0]<u16>
2020-01-22T11:00:44.6666505Z   expected: shared_generics_aux-in-shared_generics.volatile[External] 
2020-01-22T11:00:44.6666678Z   actual:   shared_generics-cgu.1[External] 
2020-01-22T11:00:44.6666962Z thread '[codegen-units] codegen-units/partitioning/shared-generics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6667057Z 
2020-01-22T11:00:44.6667268Z ---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----
2020-01-22T11:00:44.6667294Z 
2020-01-22T11:00:44.6667294Z 
2020-01-22T11:00:44.6667346Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6667371Z 
2020-01-22T11:00:44.6667403Z fn statics::function[0]
2020-01-22T11:00:44.6667438Z   expected: statics[External] 
2020-01-22T11:00:44.6667632Z   actual:   statics-cgu.0[External] 
2020-01-22T11:00:44.6667658Z 
2020-01-22T11:00:44.6667692Z fn statics::mod1[0]::function[0]
2020-01-22T11:00:44.6667855Z   expected: statics-mod1[External] 
2020-01-22T11:00:44.6668036Z   actual:   statics-cgu.1[External] 
2020-01-22T11:00:44.6668092Z static statics::BAR[0]
2020-01-22T11:00:44.6668092Z static statics::BAR[0]
2020-01-22T11:00:44.6668141Z   expected: statics[Internal] 
2020-01-22T11:00:44.6668303Z   actual:   statics-cgu.0[Internal] 
2020-01-22T11:00:44.6668367Z static statics::FOO[0]
2020-01-22T11:00:44.6668367Z static statics::FOO[0]
2020-01-22T11:00:44.6668417Z   expected: statics[Internal] 
2020-01-22T11:00:44.6668582Z   actual:   statics-cgu.0[Internal] 
2020-01-22T11:00:44.6668606Z 
2020-01-22T11:00:44.6668639Z static statics::function[0]::BAR[0]
2020-01-22T11:00:44.6668689Z   expected: statics[Internal] 
2020-01-22T11:00:44.6668852Z   actual:   statics-cgu.0[Internal] 
2020-01-22T11:00:44.6668876Z 
2020-01-22T11:00:44.6668925Z static statics::function[0]::FOO[0]
2020-01-22T11:00:44.6668966Z   expected: statics[Internal] 
2020-01-22T11:00:44.6669131Z   actual:   statics-cgu.0[Internal] 
2020-01-22T11:00:44.6669154Z 
2020-01-22T11:00:44.6669203Z static statics::mod1[0]::BAR[0]
2020-01-22T11:00:44.6669363Z   expected: statics-mod1[Internal] 
2020-01-22T11:00:44.6669523Z   actual:   statics-cgu.1[Internal] 
2020-01-22T11:00:44.6669546Z 
2020-01-22T11:00:44.6669595Z static statics::mod1[0]::FOO[0]
2020-01-22T11:00:44.6669754Z   expected: statics-mod1[Internal] 
2020-01-22T11:00:44.6669926Z   actual:   statics-cgu.1[Internal] 
2020-01-22T11:00:44.6669949Z 
2020-01-22T11:00:44.6670001Z static statics::mod1[0]::function[0]::BAR[0]
2020-01-22T11:00:44.6670162Z   expected: statics-mod1[Internal] 
2020-01-22T11:00:44.6670321Z   actual:   statics-cgu.1[Internal] 
2020-01-22T11:00:44.6670363Z 
2020-01-22T11:00:44.6670397Z static statics::mod1[0]::function[0]::FOO[0]
2020-01-22T11:00:44.6670557Z   expected: statics-mod1[Internal] 
2020-01-22T11:00:44.6670718Z   actual:   statics-cgu.1[Internal] 
2020-01-22T11:00:44.6671009Z thread '[codegen-units] codegen-units/partitioning/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6671039Z 
2020-01-22T11:00:44.6671248Z ---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----
2020-01-22T11:00:44.6671275Z 
2020-01-22T11:00:44.6671275Z 
2020-01-22T11:00:44.6671311Z The following items were assigned to wrong codegen units:
2020-01-22T11:00:44.6671334Z 
2020-01-22T11:00:44.6671390Z fn core::ptr[0]::drop_in_place[0]<u32>
2020-01-22T11:00:44.6671427Z   expected: vtable_through_const[Internal] 
2020-01-22T11:00:44.6671605Z   actual:   vtable_through_const-cgu.0[Internal] 
2020-01-22T11:00:44.6671631Z 
2020-01-22T11:00:44.6671684Z fn vtable_through_const::mod1[0]::Trait1[0]::do_something[0]<u32>
2020-01-22T11:00:44.6671864Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-22T11:00:44.6672036Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-22T11:00:44.6672061Z 
2020-01-22T11:00:44.6672342Z fn vtable_through_const::mod1[0]::Trait1[0]::do_something_else[0]<u32>
2020-01-22T11:00:44.6672563Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-22T11:00:44.6672742Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-22T11:00:44.6672786Z 
2020-01-22T11:00:44.6672823Z fn vtable_through_const::mod1[0]::Trait2[0]::do_something[0]<u32>
2020-01-22T11:00:44.6673011Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-22T11:00:44.6673298Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-22T11:00:44.6673326Z 
2020-01-22T11:00:44.6673364Z fn vtable_through_const::mod1[0]::Trait2[0]::do_something_else[0]<u32>
2020-01-22T11:00:44.6679982Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-22T11:00:44.6680708Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-22T11:00:44.6680746Z 
2020-01-22T11:00:44.6680787Z fn vtable_through_const::mod1[0]::id[0]<char>
2020-01-22T11:00:44.6681186Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-22T11:00:44.6681534Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-22T11:00:44.6681560Z 
2020-01-22T11:00:44.6681597Z fn vtable_through_const::mod1[0]::id[0]<i64>
2020-01-22T11:00:44.6681800Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-22T11:00:44.6681981Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-22T11:00:44.6682007Z 
2020-01-22T11:00:44.6682046Z fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something[0]<u8>
2020-01-22T11:00:44.6682603Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-22T11:00:44.6682796Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-22T11:00:44.6682823Z 
2020-01-22T11:00:44.6682883Z fn vtable_through_const::mod1[0]::{{impl}}[1]::do_something_else[0]<u8>
2020-01-22T11:00:44.6683083Z   expected: vtable_through_const-mod1.volatile[External] 
2020-01-22T11:00:44.6683272Z   actual:   vtable_through_const-cgu.1[External] 
2020-01-22T11:00:44.6683299Z 
2020-01-22T11:00:44.6683364Z fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something[0]<u8>
2020-01-22T11:00:44.6683565Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-22T11:00:44.6683755Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-22T11:00:44.6683801Z 
2020-01-22T11:00:44.6683843Z fn vtable_through_const::mod1[0]::{{impl}}[3]::do_something_else[0]<u8>
2020-01-22T11:00:44.6684209Z   expected: vtable_through_const-mod1.volatile[Internal] 
2020-01-22T11:00:44.6684423Z   actual:   vtable_through_const-cgu.1[Internal] 
2020-01-22T11:00:44.6684749Z thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2450:13
2020-01-22T11:00:44.6684786Z 
2020-01-22T11:00:44.6684809Z 
2020-01-22T11:00:44.6684863Z failures:
---
2020-01-22T11:00:44.6688682Z test result: FAILED. 24 passed; 12 failed; 3 ignored; 0 measured; 0 filtered out
2020-01-22T11:00:44.6688722Z 
2020-01-22T11:00:44.6688760Z 
2020-01-22T11:00:44.6688780Z 
2020-01-22T11:00:44.6691461Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-22T11:00:44.6691829Z 
2020-01-22T11:00:44.6691857Z 
2020-01-22T11:00:44.6691899Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-22T11:00:44.6691964Z Build completed unsuccessfully in 0:57:04
2020-01-22T11:00:44.6691964Z Build completed unsuccessfully in 0:57:04
2020-01-22T11:00:44.6692471Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-22T11:00:44.6692532Z == clock drift check ==
2020-01-22T11:00:44.6724285Z   local time: Wed Jan 22 11:00:44 UTC 2020
2020-01-22T11:00:45.2243521Z   network time: Wed, 22 Jan 2020 11:00:45 GMT
2020-01-22T11:00:45.2243623Z == end clock drift check ==
2020-01-22T11:00:47.4919801Z 
2020-01-22T11:00:47.5010546Z ##[error]Bash exited with code '1'.
2020-01-22T11:00:47.5022085Z ##[section]Finishing: Run build
2020-01-22T11:00:47.5041951Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67834/merge to s
2020-01-22T11:00:47.5044466Z Task         : Get sources
2020-01-22T11:00:47.5044521Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T11:00:47.5044557Z Version      : 1.0.0
2020-01-22T11:00:47.5044590Z Author       : Microsoft
2020-01-22T11:00:47.5044590Z Author       : Microsoft
2020-01-22T11:00:47.5044643Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-22T11:00:47.5044683Z ==============================================================================
2020-01-22T11:00:47.9163256Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-22T11:00:47.9212735Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67834/merge to s
2020-01-22T11:00:47.9347630Z Cleaning up task key
2020-01-22T11:00:47.9348336Z Start cleaning up orphan processes.
2020-01-22T11:00:47.9439652Z Terminate orphan process: pid (3624) (python)
2020-01-22T11:00:47.9699678Z ##[section]Finishing: Finalize Job
