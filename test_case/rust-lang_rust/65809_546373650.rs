plain
2019-10-25T13:03:36.2677744Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T13:03:36.2894009Z ##[command]git config gc.auto 0
2019-10-25T13:03:36.2970680Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T13:03:36.3039190Z ##[command]git config --get-all http.proxy
2019-10-25T13:03:36.3200450Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65809/merge:refs/remotes/pull/65809/merge
---
2019-10-25T14:07:46.8653454Z .................................................................................................... 1600/9249
2019-10-25T14:07:51.9223167Z .................................................................................................... 1700/9249
2019-10-25T14:08:05.4224615Z ........................................................i...............i........................... 1800/9249
2019-10-25T14:08:13.4459726Z .................................................................................................... 1900/9249
2019-10-25T14:08:29.2979297Z ..............................................iiiii................................................. 2000/9249
2019-10-25T14:08:40.7558959Z .................................................................................................... 2200/9249
2019-10-25T14:08:43.5166542Z .................................................................................................... 2300/9249
2019-10-25T14:08:47.6266806Z .................................................................................................... 2400/9249
2019-10-25T14:09:12.7558212Z .................................................................................................... 2500/9249
---
2019-10-25T14:12:21.4866105Z ..................................................i...............i................................. 4800/9249
2019-10-25T14:12:30.9729077Z .................................................................................................... 4900/9249
2019-10-25T14:12:40.4092224Z .................................................................................................... 5000/9249
2019-10-25T14:12:47.0101517Z .................................................................................................... 5100/9249
2019-10-25T14:12:58.2091368Z ...................................................ii.ii............................................ 5200/9249
2019-10-25T14:13:08.7254789Z .................................................................................................... 5400/9249
2019-10-25T14:13:18.7151813Z .................................................................................................... 5500/9249
2019-10-25T14:13:26.9442766Z ..................i................................................................................. 5600/9249
2019-10-25T14:13:32.9596412Z .................................................................................................... 5700/9249
2019-10-25T14:13:32.9596412Z .................................................................................................... 5700/9249
2019-10-25T14:13:45.8502895Z .................................................................................................... 5800/9249
2019-10-25T14:13:58.0903573Z ...............ii...i..ii...........i............................................................... 5900/9249
2019-10-25T14:14:21.0764961Z .................................................................................................... 6100/9249
2019-10-25T14:14:28.0563115Z .................................................................................................... 6200/9249
2019-10-25T14:14:28.0563115Z .................................................................................................... 6200/9249
2019-10-25T14:14:42.0247973Z ......................................i..ii......................................................... 6300/9249
2019-10-25T14:15:05.6946583Z .................................................................................................... 6500/9249
2019-10-25T14:15:08.0529993Z ....i............................................................................................... 6600/9249
2019-10-25T14:15:10.4588478Z ...............................................................................i.................... 6700/9249
2019-10-25T14:15:13.2980274Z .................................................................................................... 6800/9249
---
2019-10-25T14:19:36.6346017Z 
2019-10-25T14:19:36.6370509Z ---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
2019-10-25T14:19:36.6370612Z diff of stderr:
2019-10-25T14:19:36.6370651Z 
2019-10-25T14:19:36.6370941Z - error: symbol-name(_ZN5basic4main17hd72940ef9669d526E)
2019-10-25T14:19:36.6371191Z + error: symbol-name(_ZN5basic4main17h81759b0695851718E)
2019-10-25T14:19:36.6371471Z 3    |
2019-10-25T14:19:36.6371701Z 4 LL | #[rustc_symbol_name]
2019-10-25T14:19:36.6371736Z 
2019-10-25T14:19:36.6371809Z 5    | ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6371809Z 5    | ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6371852Z 6 
2019-10-25T14:19:36.6372132Z - error: demangling(basic::main::hd72940ef9669d526)
2019-10-25T14:19:36.6372202Z + error: demangling(basic::main::h81759b0695851718)
2019-10-25T14:19:36.6372457Z 9    |
2019-10-25T14:19:36.6372520Z 10 LL | #[rustc_symbol_name]
2019-10-25T14:19:36.6372550Z 
2019-10-25T14:19:36.6372576Z 
2019-10-25T14:19:36.6372576Z 
2019-10-25T14:19:36.6372632Z The actual stderr differed from the expected stderr.
2019-10-25T14:19:36.6372959Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
2019-10-25T14:19:36.6373634Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T14:19:36.6373907Z To only update this specific test, also pass `--test-args symbol-names/basic.rs`
2019-10-25T14:19:36.6373962Z 
2019-10-25T14:19:36.6374013Z error in revision `legacy`: 1 errors occurred comparing output.
2019-10-25T14:19:36.6374074Z status: exit code: 1
2019-10-25T14:19:36.6374887Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary" "-A" "unused"
2019-10-25T14:19:36.6375230Z ------------------------------------------
2019-10-25T14:19:36.6375263Z 
2019-10-25T14:19:36.6375489Z ------------------------------------------
2019-10-25T14:19:36.6375536Z stderr:
2019-10-25T14:19:36.6375536Z stderr:
2019-10-25T14:19:36.6375774Z ------------------------------------------
2019-10-25T14:19:36.6376028Z error: symbol-name(_ZN5basic4main17h81759b0695851718E)
2019-10-25T14:19:36.6376332Z    |
2019-10-25T14:19:36.6376376Z LL | #[rustc_symbol_name]
2019-10-25T14:19:36.6376420Z    | ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6376449Z 
2019-10-25T14:19:36.6376449Z 
2019-10-25T14:19:36.6376512Z error: demangling(basic::main::h81759b0695851718)
2019-10-25T14:19:36.6376990Z    |
2019-10-25T14:19:36.6377062Z LL | #[rustc_symbol_name]
2019-10-25T14:19:36.6377107Z    | ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6377137Z 
2019-10-25T14:19:36.6377137Z 
2019-10-25T14:19:36.6377395Z error: demangling-alt(basic::main)
2019-10-25T14:19:36.6377698Z    |
2019-10-25T14:19:36.6377741Z LL | #[rustc_symbol_name]
2019-10-25T14:19:36.6377803Z    | ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6377832Z 
---
2019-10-25T14:19:36.6379387Z 
2019-10-25T14:19:36.6379622Z ---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
2019-10-25T14:19:36.6379670Z diff of stderr:
2019-10-25T14:19:36.6379714Z 
2019-10-25T14:19:36.6379967Z - error: symbol-name(_ZN5impl13foo3Foo3bar17he53b9bee7600ed8dE)
2019-10-25T14:19:36.6380228Z + error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2019-10-25T14:19:36.6380506Z 3    |
2019-10-25T14:19:36.6380551Z 4 LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6380589Z 
2019-10-25T14:19:36.6380650Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6380650Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6380692Z 6 
2019-10-25T14:19:36.6381677Z - error: demangling(impl1::foo::Foo::bar::he53b9bee7600ed8d)
2019-10-25T14:19:36.6381758Z + error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2019-10-25T14:19:36.6382035Z 9    |
2019-10-25T14:19:36.6382081Z 10 LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6382126Z 
2019-10-25T14:19:36.6382182Z 22 LL |         #[rustc_def_path]
2019-10-25T14:19:36.6382182Z 22 LL |         #[rustc_def_path]
2019-10-25T14:19:36.6382228Z 23    |         ^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6382288Z 24 
2019-10-25T14:19:36.6382583Z - error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h86c41f0462d901d4E)
2019-10-25T14:19:36.6382874Z + error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2019-10-25T14:19:36.6383157Z 27    |
2019-10-25T14:19:36.6383211Z 28 LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6383242Z 
2019-10-25T14:19:36.6383349Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6383349Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6383392Z 30 
2019-10-25T14:19:36.6383658Z - error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h86c41f0462d901d4)
2019-10-25T14:19:36.6383732Z + error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2019-10-25T14:19:36.6383991Z 33    |
2019-10-25T14:19:36.6384045Z 34 LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6384092Z 
2019-10-25T14:19:36.6384134Z 46 LL |         #[rustc_def_path]
2019-10-25T14:19:36.6384134Z 46 LL |         #[rustc_def_path]
2019-10-25T14:19:36.6384180Z 47    |         ^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6384240Z 48 
2019-10-25T14:19:36.6384701Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h636bc933fc62ee2fE)
2019-10-25T14:19:36.6385187Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h61b0fcb05ebeeb79E)
2019-10-25T14:19:36.6385493Z 51    |
2019-10-25T14:19:36.6385555Z 52 LL |             #[rustc_symbol_name]
2019-10-25T14:19:36.6385585Z 
2019-10-25T14:19:36.6385745Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6385745Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6385796Z 54 
2019-10-25T14:19:36.6386182Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method::h636bc933fc62ee2f)
2019-10-25T14:19:36.6386257Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method::h61b0fcb05ebeeb79)
2019-10-25T14:19:36.6386976Z 57    |
2019-10-25T14:19:36.6387410Z 58 LL |             #[rustc_symbol_name]
2019-10-25T14:19:36.6387441Z 
2019-10-25T14:19:36.6387468Z 
2019-10-25T14:19:36.6387468Z 
2019-10-25T14:19:36.6387531Z The actual stderr differed from the expected stderr.
2019-10-25T14:19:36.6387914Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2019-10-25T14:19:36.6388168Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T14:19:36.6388741Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2019-10-25T14:19:36.6388782Z 
2019-10-25T14:19:36.6388831Z error in revision `legacy`: 1 errors occurred comparing output.
2019-10-25T14:19:36.6388953Z status: exit code: 1
2019-10-25T14:19:36.6389772Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary" "-A" "unused"
2019-10-25T14:19:36.6390120Z ------------------------------------------
2019-10-25T14:19:36.6390161Z 
2019-10-25T14:19:36.6390385Z ------------------------------------------
2019-10-25T14:19:36.6391048Z stderr:
2019-10-25T14:19:36.6391048Z stderr:
2019-10-25T14:19:36.6391425Z ------------------------------------------
2019-10-25T14:19:36.6391678Z error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2019-10-25T14:19:36.6392359Z    |
2019-10-25T14:19:36.6392675Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6392754Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6392783Z 
2019-10-25T14:19:36.6392783Z 
2019-10-25T14:19:36.6392831Z error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2019-10-25T14:19:36.6393464Z    |
2019-10-25T14:19:36.6393508Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6393555Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6393603Z 
2019-10-25T14:19:36.6393603Z 
2019-10-25T14:19:36.6393835Z error: demangling-alt(impl1::foo::Foo::bar)
2019-10-25T14:19:36.6394132Z    |
2019-10-25T14:19:36.6394194Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6394240Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6394269Z 
2019-10-25T14:19:36.6394269Z 
2019-10-25T14:19:36.6394483Z error: def-path(foo::Foo::bar)
2019-10-25T14:19:36.6394783Z    |
2019-10-25T14:19:36.6394827Z LL |         #[rustc_def_path]
2019-10-25T14:19:36.6394896Z    |         ^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6394925Z 
2019-10-25T14:19:36.6394925Z 
2019-10-25T14:19:36.6395626Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2019-10-25T14:19:36.6395955Z    |
2019-10-25T14:19:36.6395999Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6396045Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6396091Z 
2019-10-25T14:19:36.6396091Z 
2019-10-25T14:19:36.6396361Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2019-10-25T14:19:36.6396711Z    |
2019-10-25T14:19:36.6396754Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6396799Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6401078Z 
2019-10-25T14:19:36.6401078Z 
2019-10-25T14:19:36.6401498Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2019-10-25T14:19:36.6402041Z    |
2019-10-25T14:19:36.6402110Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6402159Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6402189Z 
2019-10-25T14:19:36.6402189Z 
2019-10-25T14:19:36.6402485Z error: def-path(bar::<impl foo::Foo>::baz)
2019-10-25T14:19:36.6403097Z    |
2019-10-25T14:19:36.6403143Z LL |         #[rustc_def_path]
2019-10-25T14:19:36.6403210Z    |         ^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6403254Z 
2019-10-25T14:19:36.6403254Z 
2019-10-25T14:19:36.6403800Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$_$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h61b0fcb05ebeeb79E)
2019-10-25T14:19:36.6404174Z    |
2019-10-25T14:19:36.6404223Z LL |             #[rustc_symbol_name]
2019-10-25T14:19:36.6404301Z    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6404332Z 
2019-10-25T14:19:36.6404332Z 
2019-10-25T14:19:36.6404391Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method::h61b0fcb05ebeeb79)
2019-10-25T14:19:36.6404735Z    |
2019-10-25T14:19:36.6404781Z LL |             #[rustc_symbol_name]
2019-10-25T14:19:36.6404838Z    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6404888Z 
2019-10-25T14:19:36.6404888Z 
2019-10-25T14:19:36.6405754Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; _] as impl1::main::{{closure}}::Bar>::method)
2019-10-25T14:19:36.6406084Z    |
2019-10-25T14:19:36.6406130Z LL |             #[rustc_symbol_name]
2019-10-25T14:19:36.6406177Z    |             ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6406208Z 
2019-10-25T14:19:36.6406208Z 
2019-10-25T14:19:36.6406544Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; _] as main::{{closure}}#1::Bar>::method)
2019-10-25T14:19:36.6406837Z    |
2019-10-25T14:19:36.6406897Z LL |             #[rustc_def_path]
2019-10-25T14:19:36.6406943Z    |             ^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6406972Z 
---
2019-10-25T14:19:36.6407377Z 
2019-10-25T14:19:36.6407634Z ---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
2019-10-25T14:19:36.6407684Z diff of stderr:
2019-10-25T14:19:36.6407712Z 
2019-10-25T14:19:36.6408593Z - error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h059a991a004536adE)
2019-10-25T14:19:36.6409188Z + error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2019-10-25T14:19:36.6409488Z 2   --> $DIR/issue-60925.rs:21:9
2019-10-25T14:19:36.6409597Z 4 LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6409628Z 
2019-10-25T14:19:36.6409671Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6409733Z 6 
2019-10-25T14:19:36.6409733Z 6 
2019-10-25T14:19:36.6410012Z - error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h059a991a004536ad)
2019-10-25T14:19:36.6410205Z + error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2019-10-25T14:19:36.6410489Z 8   --> $DIR/issue-60925.rs:21:9
2019-10-25T14:19:36.6410580Z 10 LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6410610Z 
2019-10-25T14:19:36.6410637Z 
2019-10-25T14:19:36.6410701Z The actual stderr differed from the expected stderr.
2019-10-25T14:19:36.6410701Z The actual stderr differed from the expected stderr.
2019-10-25T14:19:36.6411022Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
2019-10-25T14:19:36.6411383Z To update references, rerun the tests and pass the `--bless` flag
2019-10-25T14:19:36.6411827Z To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`
2019-10-25T14:19:36.6411864Z 
2019-10-25T14:19:36.6411911Z error in revision `legacy`: 1 errors occurred comparing output.
2019-10-25T14:19:36.6411977Z status: exit code: 1
2019-10-25T14:19:36.6412797Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary" "-A" "unused"
2019-10-25T14:19:36.6421385Z ------------------------------------------
2019-10-25T14:19:36.6421448Z 
2019-10-25T14:19:36.6421835Z ------------------------------------------
2019-10-25T14:19:36.6421886Z stderr:
2019-10-25T14:19:36.6421886Z stderr:
2019-10-25T14:19:36.6422111Z ------------------------------------------
2019-10-25T14:19:36.6422445Z error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2019-10-25T14:19:36.6422709Z   --> /checkout/src/test/ui/symbol-names/issue-60925.rs:21:9
2019-10-25T14:19:36.6422823Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6422869Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6422899Z 
2019-10-25T14:19:36.6422899Z 
2019-10-25T14:19:36.6422948Z error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2019-10-25T14:19:36.6423217Z   --> /checkout/src/test/ui/symbol-names/issue-60925.rs:21:9
2019-10-25T14:19:36.6423318Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6423380Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6423409Z 
2019-10-25T14:19:36.6423409Z 
2019-10-25T14:19:36.6423665Z error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
2019-10-25T14:19:36.6423911Z   --> /checkout/src/test/ui/symbol-names/issue-60925.rs:21:9
2019-10-25T14:19:36.6424019Z LL |         #[rustc_symbol_name]
2019-10-25T14:19:36.6424074Z    |         ^^^^^^^^^^^^^^^^^^^^
2019-10-25T14:19:36.6424121Z 
2019-10-25T14:19:36.6424215Z error: aborting due to 3 previous errors
---
2019-10-25T14:19:36.6426040Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-25T14:19:36.6426116Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-25T14:19:36.6426307Z 
2019-10-25T14:19:36.6426344Z 
2019-10-25T14:19:36.6427883Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-25T14:19:36.6428207Z 
2019-10-25T14:19:36.6428236Z 
2019-10-25T14:19:36.6428810Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-25T14:19:36.6428903Z Build completed unsuccessfully in 1:08:56
2019-10-25T14:19:36.6428903Z Build completed unsuccessfully in 1:08:56
2019-10-25T14:19:36.6488668Z == clock drift check ==
2019-10-25T14:19:36.6501937Z   local time: Fri Oct 25 14:19:36 UTC 2019
2019-10-25T14:19:36.9424491Z   network time: Fri, 25 Oct 2019 14:19:36 GMT
2019-10-25T14:19:36.9424864Z == end clock drift check ==
2019-10-25T14:19:37.9087677Z 
2019-10-25T14:19:37.9274252Z ##[error]Bash exited with code '1'.
2019-10-25T14:19:37.9329054Z ##[section]Starting: Checkout
2019-10-25T14:19:37.9331580Z ==============================================================================
2019-10-25T14:19:37.9331638Z Task         : Get sources
2019-10-25T14:19:37.9331703Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
