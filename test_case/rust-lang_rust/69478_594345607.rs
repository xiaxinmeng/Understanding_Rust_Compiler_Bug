plain
2020-03-04T04:39:20.2639528Z ========================== Starting Command Output ===========================
2020-03-04T04:39:20.2644894Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/574eacb3-1e53-45b5-82ee-29dcd19afc87.sh
2020-03-04T04:39:20.2645826Z 
2020-03-04T04:39:20.2650209Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-04T04:39:20.2671023Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-04T04:39:20.2674813Z Task         : Get sources
2020-03-04T04:39:20.2675143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T04:39:20.2675457Z Version      : 1.0.0
2020-03-04T04:39:20.2675693Z Author       : Microsoft
---
2020-03-04T04:39:21.8502222Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-04T04:39:21.8513039Z ##[command]git config gc.auto 0
2020-03-04T04:39:21.8518359Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-04T04:39:21.8522740Z ##[command]git config --get-all http.proxy
2020-03-04T04:39:21.8535187Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-03-04T05:56:29.0778264Z .................................................................................................... 1700/9728
2020-03-04T05:56:34.6820521Z .................................................................................................... 1800/9728
2020-03-04T05:56:49.2077211Z ......................................................i............................................. 1900/9728
2020-03-04T05:56:59.4372206Z .................................................................................................... 2000/9728
2020-03-04T05:57:17.1465350Z ............................................iiiii................................................... 2100/9728
2020-03-04T05:57:30.6337398Z .................................................................................................... 2300/9728
2020-03-04T05:57:33.7363115Z .................................................................................................... 2400/9728
2020-03-04T05:57:38.7697144Z .................................................................................................... 2500/9728
2020-03-04T05:58:06.5696010Z .................................................................................................... 2600/9728
---
2020-03-04T06:01:21.8317426Z ......i...............i............................................................................. 5000/9728
2020-03-04T06:01:34.0233588Z .................................................................................................... 5100/9728
2020-03-04T06:01:40.4460554Z .................................................i.................................................. 5200/9728
2020-03-04T06:01:51.2120997Z .................................................................................................... 5300/9728
2020-03-04T06:02:00.2528283Z ............................ii.ii........i...i...................................................... 5400/9728
2020-03-04T06:02:10.5270375Z .................................................................................................... 5600/9728
2020-03-04T06:02:22.5524214Z .................................................................................................... 5700/9728
2020-03-04T06:02:31.6642048Z ...................i................................................................................ 5800/9728
2020-03-04T06:02:38.5268263Z .................................................................................................... 5900/9728
2020-03-04T06:02:38.5268263Z .................................................................................................... 5900/9728
2020-03-04T06:02:52.1789487Z .................................................................................................... 6000/9728
2020-03-04T06:03:05.3407613Z ...........ii...i..ii...........i................................................................... 6100/9728
2020-03-04T06:03:24.8746412Z .................................................................................................... 6300/9728
2020-03-04T06:03:33.2943863Z .................................................................................................... 6400/9728
2020-03-04T06:03:33.2943863Z .................................................................................................... 6400/9728
2020-03-04T06:03:53.6062439Z ..........................................i..ii..................................................... 6500/9728
2020-03-04T06:04:21.0021298Z .................................................................................................... 6700/9728
2020-03-04T06:04:23.4367877Z ..................................i................................................................. 6800/9728
2020-03-04T06:04:25.9230648Z .................................................................................................... 6900/9728
2020-03-04T06:04:28.6448066Z ................................................................i................................... 7000/9728
---
2020-03-04T06:06:32.8995699Z .................................................................................................... 7700/9728
2020-03-04T06:06:39.3960369Z .................................................................................................... 7800/9728
2020-03-04T06:06:46.0295012Z .................................................................................................... 7900/9728
2020-03-04T06:06:56.5599702Z ..........i......................................................................................... 8000/9728
2020-03-04T06:07:07.6109629Z ...........................................................iiiiiiiii.i.............................. 8100/9728
2020-03-04T06:07:26.2539249Z ..i......i.......................................................................................... 8300/9728
2020-03-04T06:07:33.2224251Z .................................................................................................... 8400/9728
2020-03-04T06:07:52.0502237Z .................................................................................................... 8500/9728
2020-03-04T06:08:03.0933229Z .................................................................................................... 8600/9728
---
2020-03-04T06:10:28.8245826Z 
2020-03-04T06:10:28.8246944Z ---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
2020-03-04T06:10:28.8247460Z diff of stderr:
2020-03-04T06:10:28.8247751Z 
2020-03-04T06:10:28.8248475Z - error: symbol-name(_ZN5basic4main17h81759b0695851718E)
2020-03-04T06:10:28.8249275Z + error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-03-04T06:10:28.8250361Z 3    |
2020-03-04T06:10:28.8250682Z 4 LL | #[rustc_symbol_name]
2020-03-04T06:10:28.8251243Z 
2020-03-04T06:10:28.8254922Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8254922Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8255234Z 6 
2020-03-04T06:10:28.8255977Z - error: demangling(basic::main::h81759b0695851718)
2020-03-04T06:10:28.8256542Z + error: demangling(basic::main::h7bbff4a01206d8c2)
2020-03-04T06:10:28.8257593Z 9    |
2020-03-04T06:10:28.8257924Z 10 LL | #[rustc_symbol_name]
2020-03-04T06:10:28.8258209Z 
2020-03-04T06:10:28.8258436Z 
2020-03-04T06:10:28.8258436Z 
2020-03-04T06:10:28.8258815Z The actual stderr differed from the expected stderr.
2020-03-04T06:10:28.8259735Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
2020-03-04T06:10:28.8260677Z To update references, rerun the tests and pass the `--bless` flag
2020-03-04T06:10:28.8261523Z To only update this specific test, also pass `--test-args symbol-names/basic.rs`
2020-03-04T06:10:28.8261954Z 
2020-03-04T06:10:28.8262397Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-04T06:10:28.8262859Z status: exit code: 1
2020-03-04T06:10:28.8265525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
2020-03-04T06:10:28.8268860Z ------------------------------------------
2020-03-04T06:10:28.8269336Z 
2020-03-04T06:10:28.8269975Z ------------------------------------------
2020-03-04T06:10:28.8270416Z stderr:
2020-03-04T06:10:28.8270416Z stderr:
2020-03-04T06:10:28.8271029Z ------------------------------------------
2020-03-04T06:10:28.8271774Z error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-03-04T06:10:28.8273042Z    |
2020-03-04T06:10:28.8273367Z LL | #[rustc_symbol_name]
2020-03-04T06:10:28.8273719Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8274024Z 
2020-03-04T06:10:28.8274024Z 
2020-03-04T06:10:28.8274418Z error: demangling(basic::main::h7bbff4a01206d8c2)
2020-03-04T06:10:28.8275594Z    |
2020-03-04T06:10:28.8275928Z LL | #[rustc_symbol_name]
2020-03-04T06:10:28.8276286Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8276563Z 
2020-03-04T06:10:28.8276563Z 
2020-03-04T06:10:28.8277158Z error: demangling-alt(basic::main)
2020-03-04T06:10:28.8281012Z    |
2020-03-04T06:10:28.8281579Z LL | #[rustc_symbol_name]
2020-03-04T06:10:28.8282415Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8282779Z 
---
2020-03-04T06:10:28.8293891Z 
2020-03-04T06:10:28.8294860Z ---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
2020-03-04T06:10:28.8295473Z diff of stderr:
2020-03-04T06:10:28.8298973Z 
2020-03-04T06:10:28.8300897Z - error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2020-03-04T06:10:28.8302984Z + error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-03-04T06:10:28.8304777Z 3    |
2020-03-04T06:10:28.8305239Z 4 LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8305420Z 
2020-03-04T06:10:28.8305649Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8305649Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8305849Z 6 
2020-03-04T06:10:28.8306490Z - error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2020-03-04T06:10:28.8306942Z + error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-03-04T06:10:28.8307702Z 9    |
2020-03-04T06:10:28.8307934Z 10 LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8308113Z 
2020-03-04T06:10:28.8308307Z 22 LL |         #[rustc_def_path]
2020-03-04T06:10:28.8308307Z 22 LL |         #[rustc_def_path]
2020-03-04T06:10:28.8308562Z 23    |         ^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8308778Z 24 
2020-03-04T06:10:28.8309394Z - error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2020-03-04T06:10:28.8317004Z + error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-03-04T06:10:28.8318142Z 27    |
2020-03-04T06:10:28.8318358Z 28 LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8318563Z 
2020-03-04T06:10:28.8318763Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8318763Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8319165Z 30 
2020-03-04T06:10:28.8319881Z - error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2020-03-04T06:10:28.8320395Z + error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-03-04T06:10:28.8321208Z 33    |
2020-03-04T06:10:28.8321423Z 34 LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8321603Z 
2020-03-04T06:10:28.8321800Z 46 LL |         #[rustc_def_path]
2020-03-04T06:10:28.8321800Z 46 LL |         #[rustc_def_path]
2020-03-04T06:10:28.8322075Z 47    |         ^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8322271Z 48 
2020-03-04T06:10:28.8323391Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17SYMBOL_HASHE)
2020-03-04T06:10:28.8325456Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-03-04T06:10:28.8326731Z 51    |
2020-03-04T06:10:28.8326953Z 52 LL |             #[rustc_symbol_name]
2020-03-04T06:10:28.8327140Z 
2020-03-04T06:10:28.8327363Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8327363Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8327573Z 54 
2020-03-04T06:10:28.8328406Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::SYMBOL_HASH)
2020-03-04T06:10:28.8329289Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-03-04T06:10:28.8330285Z 57    |
2020-03-04T06:10:28.8330523Z 58 LL |             #[rustc_symbol_name]
2020-03-04T06:10:28.8330710Z 
2020-03-04T06:10:28.8330816Z 
2020-03-04T06:10:28.8330816Z 
2020-03-04T06:10:28.8331053Z The actual stderr differed from the expected stderr.
2020-03-04T06:10:28.8331844Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2020-03-04T06:10:28.8332580Z To update references, rerun the tests and pass the `--bless` flag
2020-03-04T06:10:28.8333250Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2020-03-04T06:10:28.8333518Z 
2020-03-04T06:10:28.8333806Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-04T06:10:28.8334120Z status: exit code: 1
2020-03-04T06:10:28.8336443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary"
2020-03-04T06:10:28.8338552Z ------------------------------------------
2020-03-04T06:10:28.8340088Z 
2020-03-04T06:10:28.8347970Z ------------------------------------------
2020-03-04T06:10:28.8348288Z stderr:
2020-03-04T06:10:28.8348288Z stderr:
2020-03-04T06:10:28.8348913Z ------------------------------------------
2020-03-04T06:10:28.8349511Z error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-03-04T06:10:28.8350414Z    |
2020-03-04T06:10:28.8350633Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8350888Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8351083Z 
2020-03-04T06:10:28.8351083Z 
2020-03-04T06:10:28.8351349Z error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-03-04T06:10:28.8352455Z    |
2020-03-04T06:10:28.8352658Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8352915Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8353087Z 
2020-03-04T06:10:28.8353087Z 
2020-03-04T06:10:28.8353625Z error: demangling-alt(impl1::foo::Foo::bar)
2020-03-04T06:10:28.8354466Z    |
2020-03-04T06:10:28.8354685Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8354937Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8355107Z 
2020-03-04T06:10:28.8355107Z 
2020-03-04T06:10:28.8355518Z error: def-path(foo::Foo::bar)
2020-03-04T06:10:28.8356353Z    |
2020-03-04T06:10:28.8356546Z LL |         #[rustc_def_path]
2020-03-04T06:10:28.8356814Z    |         ^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8356982Z 
2020-03-04T06:10:28.8356982Z 
2020-03-04T06:10:28.8357584Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-03-04T06:10:28.8358940Z    |
2020-03-04T06:10:28.8359146Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8359422Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8359593Z 
2020-03-04T06:10:28.8359593Z 
2020-03-04T06:10:28.8359905Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-03-04T06:10:28.8360929Z    |
2020-03-04T06:10:28.8361130Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8361386Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8361577Z 
2020-03-04T06:10:28.8361577Z 
2020-03-04T06:10:28.8362108Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2020-03-04T06:10:28.8363006Z    |
2020-03-04T06:10:28.8363226Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8363485Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8363657Z 
2020-03-04T06:10:28.8363657Z 
2020-03-04T06:10:28.8364132Z error: def-path(bar::<impl foo::Foo>::baz)
2020-03-04T06:10:28.8365179Z    |
2020-03-04T06:10:28.8365400Z LL |         #[rustc_def_path]
2020-03-04T06:10:28.8365793Z    |         ^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8365962Z 
2020-03-04T06:10:28.8365962Z 
2020-03-04T06:10:28.8367165Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-03-04T06:10:28.8368635Z    |
2020-03-04T06:10:28.8368867Z LL |             #[rustc_symbol_name]
2020-03-04T06:10:28.8369138Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8369318Z 
2020-03-04T06:10:28.8369318Z 
2020-03-04T06:10:28.8369823Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-03-04T06:10:28.8371005Z    |
2020-03-04T06:10:28.8371212Z LL |             #[rustc_symbol_name]
2020-03-04T06:10:28.8371503Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8371681Z 
2020-03-04T06:10:28.8371681Z 
2020-03-04T06:10:28.8372464Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
2020-03-04T06:10:28.8373549Z    |
2020-03-04T06:10:28.8373756Z LL |             #[rustc_symbol_name]
2020-03-04T06:10:28.8374043Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8374229Z 
2020-03-04T06:10:28.8374229Z 
2020-03-04T06:10:28.8374969Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{{closure}}#1::Bar>::method)
2020-03-04T06:10:28.8376146Z    |
2020-03-04T06:10:28.8376350Z LL |             #[rustc_def_path]
2020-03-04T06:10:28.8376668Z    |             ^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8376863Z 
---
2020-03-04T06:10:28.8394728Z 
2020-03-04T06:10:28.8395249Z ---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
2020-03-04T06:10:28.8395539Z diff of stderr:
2020-03-04T06:10:28.8395703Z 
2020-03-04T06:10:28.8396351Z - error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2020-03-04T06:10:28.8397226Z + error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-03-04T06:10:28.8399250Z 3    |
2020-03-04T06:10:28.8399456Z 4 LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8399658Z 
2020-03-04T06:10:28.8399848Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8399848Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8400038Z 6 
2020-03-04T06:10:28.8400718Z - error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2020-03-04T06:10:28.8401291Z + error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-03-04T06:10:28.8402271Z 9    |
2020-03-04T06:10:28.8402486Z 10 LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8402665Z 
2020-03-04T06:10:28.8402774Z 
2020-03-04T06:10:28.8402774Z 
2020-03-04T06:10:28.8403025Z The actual stderr differed from the expected stderr.
2020-03-04T06:10:28.8403871Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
2020-03-04T06:10:28.8408289Z To update references, rerun the tests and pass the `--bless` flag
2020-03-04T06:10:28.8409432Z To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`
2020-03-04T06:10:28.8409728Z 
2020-03-04T06:10:28.8410193Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-04T06:10:28.8410554Z status: exit code: 1
2020-03-04T06:10:28.8423807Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary"
2020-03-04T06:10:28.8429772Z ------------------------------------------
2020-03-04T06:10:28.8429986Z 
2020-03-04T06:10:28.8430486Z ------------------------------------------
2020-03-04T06:10:28.8430721Z stderr:
2020-03-04T06:10:28.8430721Z stderr:
2020-03-04T06:10:28.8437688Z ------------------------------------------
2020-03-04T06:10:28.8438519Z error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-03-04T06:10:28.8439540Z    |
2020-03-04T06:10:28.8439766Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8440024Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8440198Z 
2020-03-04T06:10:28.8440198Z 
2020-03-04T06:10:28.8440545Z error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-03-04T06:10:28.8441582Z    |
2020-03-04T06:10:28.8441784Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8442248Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8442436Z 
2020-03-04T06:10:28.8442436Z 
2020-03-04T06:10:28.8443067Z error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
2020-03-04T06:10:28.8444051Z    |
2020-03-04T06:10:28.8444254Z LL |         #[rustc_symbol_name]
2020-03-04T06:10:28.8445096Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-04T06:10:28.8445307Z 
---
2020-03-04T06:10:28.8450193Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-04T06:10:28.8450660Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-04T06:10:28.8450919Z 
2020-03-04T06:10:28.8451027Z 
2020-03-04T06:10:28.8455204Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-04T06:10:28.8458359Z 
2020-03-04T06:10:28.8458493Z 
2020-03-04T06:10:28.8458759Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-04T06:10:28.8459123Z Build completed unsuccessfully in 1:23:52
2020-03-04T06:10:28.8459123Z Build completed unsuccessfully in 1:23:52
2020-03-04T06:10:28.8459424Z == clock drift check ==
2020-03-04T06:10:28.8459750Z   local time: Wed Mar  4 06:10:28 UTC 2020
2020-03-04T06:10:29.3827905Z   network time: Wed, 04 Mar 2020 06:10:29 GMT
2020-03-04T06:10:29.3832266Z == end clock drift check ==
2020-03-04T06:10:29.7719215Z 
2020-03-04T06:10:29.7776764Z ##[error]Bash exited with code '1'.
2020-03-04T06:10:29.7808397Z ##[section]Finishing: Run build
2020-03-04T06:10:29.7876849Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-04T06:10:29.7883175Z Task         : Get sources
2020-03-04T06:10:29.7883597Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-04T06:10:29.7883980Z Version      : 1.0.0
2020-03-04T06:10:29.7884238Z Author       : Microsoft
2020-03-04T06:10:29.7884238Z Author       : Microsoft
2020-03-04T06:10:29.7884662Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-04T06:10:29.7885633Z ==============================================================================
2020-03-04T06:10:30.2328083Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-04T06:10:30.2387681Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-04T06:10:30.2494269Z Cleaning up task key
2020-03-04T06:10:30.2495734Z Start cleaning up orphan processes.
2020-03-04T06:10:30.2966933Z Terminate orphan process: pid (4393) (python)
2020-03-04T06:10:30.3028120Z ##[section]Finishing: Finalize Job
