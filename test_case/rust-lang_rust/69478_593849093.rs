plain
2020-03-03T08:08:32.8600483Z ========================== Starting Command Output ===========================
2020-03-03T08:08:32.8605087Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/49c31096-596f-4939-bdcd-38bf7e424d44.sh
2020-03-03T08:08:32.8605558Z 
2020-03-03T08:08:32.8609376Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T08:08:32.8629127Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T08:08:32.8633498Z Task         : Get sources
2020-03-03T08:08:32.8633759Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T08:08:32.8634066Z Version      : 1.0.0
2020-03-03T08:08:32.8634237Z Author       : Microsoft
---
2020-03-03T08:08:36.0407774Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T08:08:36.0416451Z ##[command]git config gc.auto 0
2020-03-03T08:08:36.0422655Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T08:08:36.0429540Z ##[command]git config --get-all http.proxy
2020-03-03T08:08:36.0446178Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-03-03T09:10:49.5041617Z .................................................................................................... 1700/9726
2020-03-03T09:10:53.8488807Z .................................................................................................... 1800/9726
2020-03-03T09:11:05.0244088Z ......................................................i............................................. 1900/9726
2020-03-03T09:11:12.6700541Z .................................................................................................... 2000/9726
2020-03-03T09:11:26.1328267Z ............................................iiiii................................................... 2100/9726
2020-03-03T09:11:35.7880017Z .................................................................................................... 2300/9726
2020-03-03T09:11:38.0878419Z .................................................................................................... 2400/9726
2020-03-03T09:11:41.6816287Z .................................................................................................... 2500/9726
2020-03-03T09:12:02.3138128Z .................................................................................................... 2600/9726
---
2020-03-03T09:14:50.8928667Z ......i...............i............................................................................. 5000/9726
2020-03-03T09:15:00.6089350Z .................................................................................................... 5100/9726
2020-03-03T09:15:05.5732508Z .................................................i.................................................. 5200/9726
2020-03-03T09:15:14.2203187Z .................................................................................................... 5300/9726
2020-03-03T09:15:21.3544912Z ............................ii.ii........i...i...................................................... 5400/9726
2020-03-03T09:15:29.4833258Z .................................................................................................... 5600/9726
2020-03-03T09:15:38.6554307Z .................................................................................................... 5700/9726
2020-03-03T09:15:45.6666972Z ...................i................................................................................ 5800/9726
2020-03-03T09:15:51.1757092Z .................................................................................................... 5900/9726
2020-03-03T09:15:51.1757092Z .................................................................................................... 5900/9726
2020-03-03T09:16:01.6762174Z .................................................................................................... 6000/9726
2020-03-03T09:16:12.0389855Z ...........ii...i..ii...........i................................................................... 6100/9726
2020-03-03T09:16:27.7013653Z .................................................................................................... 6300/9726
2020-03-03T09:16:34.3775480Z .................................................................................................... 6400/9726
2020-03-03T09:16:34.3775480Z .................................................................................................... 6400/9726
2020-03-03T09:16:50.3182777Z ..........................................i..ii..................................................... 6500/9726
2020-03-03T09:17:11.3144366Z .................................................................................................... 6700/9726
2020-03-03T09:17:13.2825249Z ..................................i................................................................. 6800/9726
2020-03-03T09:17:15.3050153Z .................................................................................................... 6900/9726
2020-03-03T09:17:17.4670957Z ................................................................i................................... 7000/9726
---
2020-03-03T09:18:54.0452085Z .................................................................................................... 7700/9726
2020-03-03T09:18:59.0529209Z .................................................................................................... 7800/9726
2020-03-03T09:19:04.0829188Z .................................................................................................... 7900/9726
2020-03-03T09:19:12.0244383Z ..........i......................................................................................... 8000/9726
2020-03-03T09:19:20.4699418Z ...........................................................iiiiiii.i................................ 8100/9726
2020-03-03T09:19:34.2895911Z i......i............................................................................................ 8300/9726
2020-03-03T09:19:39.3363035Z .................................................................................................... 8400/9726
2020-03-03T09:19:53.2355269Z .................................................................................................... 8500/9726
2020-03-03T09:20:00.8887474Z .................................................................................................... 8600/9726
---
2020-03-03T09:21:41.3872914Z ..............................i..................................................................... 9700/9726
2020-03-03T09:21:51.6330133Z ..........................
2020-03-03T09:21:51.6330864Z failures:
2020-03-03T09:21:51.6392276Z 
2020-03-03T09:21:51.6393733Z ---- [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs stdout ----
2020-03-03T09:21:51.6394472Z 
2020-03-03T09:21:51.6394472Z 
2020-03-03T09:21:51.6395193Z - error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
2020-03-03T09:21:51.6396003Z -   --> $DIR/feature-gate-abi-avr-interrupt.rs:14:1
2020-03-03T09:21:51.6399018Z + error[E0658]: avr-interrupt ABI is experimental and subject to change
2020-03-03T09:21:51.6399845Z +   --> $DIR/feature-gate-abi-avr-interrupt.rs:4:8
2020-03-03T09:21:51.6400091Z 3    |
2020-03-03T09:21:51.6400469Z 4 LL | extern "avr-interrupt" fn foo() {}
2020-03-03T09:21:51.6401453Z +    |        ^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6401628Z 6    |
2020-03-03T09:21:51.6401628Z 6    |
2020-03-03T09:21:51.6402320Z -    = help: add #![feature(abi_avr_interrupt)] to the crate attributes to enable
2020-03-03T09:21:51.6404007Z +    = note: see issue #69664 <***/issues/69664> for more information
2020-03-03T09:21:51.6404432Z +    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
2020-03-03T09:21:51.6404930Z 9 error: aborting due to previous error
2020-03-03T09:21:51.6405128Z 10 
2020-03-03T09:21:51.6405236Z 
2020-03-03T09:21:51.6405353Z 
2020-03-03T09:21:51.6405353Z 
2020-03-03T09:21:51.6405566Z The actual stderr differed from the expected stderr.
2020-03-03T09:21:51.6406402Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/feature-gate-abi-avr-interrupt.stderr
2020-03-03T09:21:51.6407446Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T09:21:51.6408062Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-avr-interrupt.rs`
2020-03-03T09:21:51.6408518Z error: 1 errors occurred comparing output.
2020-03-03T09:21:51.6408767Z status: exit code: 1
2020-03-03T09:21:51.6408767Z status: exit code: 1
2020-03-03T09:21:51.6410816Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/auxiliary"
2020-03-03T09:21:51.6412477Z ------------------------------------------
2020-03-03T09:21:51.6412664Z 
2020-03-03T09:21:51.6413006Z ------------------------------------------
2020-03-03T09:21:51.6413199Z stderr:
2020-03-03T09:21:51.6413199Z stderr:
2020-03-03T09:21:51.6413542Z ------------------------------------------
2020-03-03T09:21:51.6414055Z error[E0658]: avr-interrupt ABI is experimental and subject to change
2020-03-03T09:21:51.6414637Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs:4:8
2020-03-03T09:21:51.6414925Z    |
2020-03-03T09:21:51.6415262Z LL | extern "avr-interrupt" fn foo() {}
2020-03-03T09:21:51.6415646Z    |
2020-03-03T09:21:51.6415646Z    |
2020-03-03T09:21:51.6416184Z    = note: see issue #69664 <***/issues/69664> for more information
2020-03-03T09:21:51.6416752Z    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
2020-03-03T09:21:51.6417193Z error: aborting due to previous error
2020-03-03T09:21:51.6417356Z 
2020-03-03T09:21:51.6417787Z For more information about this error, try `rustc --explain E0658`.
2020-03-03T09:21:51.6418003Z 
2020-03-03T09:21:51.6418003Z 
2020-03-03T09:21:51.6418569Z ------------------------------------------
2020-03-03T09:21:51.6418746Z 
2020-03-03T09:21:51.6418845Z 
2020-03-03T09:21:51.6419241Z ---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
2020-03-03T09:21:51.6425095Z diff of stderr:
2020-03-03T09:21:51.6426020Z 
2020-03-03T09:21:51.6428562Z - error: symbol-name(_ZN5basic4main17h81759b0695851718E)
2020-03-03T09:21:51.6430190Z + error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-03-03T09:21:51.6430801Z 3    |
2020-03-03T09:21:51.6430987Z 4 LL | #[rustc_symbol_name]
2020-03-03T09:21:51.6431559Z 
2020-03-03T09:21:51.6431729Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6431729Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6431886Z 6 
2020-03-03T09:21:51.6432364Z - error: demangling(basic::main::h81759b0695851718)
2020-03-03T09:21:51.6432682Z + error: demangling(basic::main::h7bbff4a01206d8c2)
2020-03-03T09:21:51.6433384Z 9    |
2020-03-03T09:21:51.6433552Z 10 LL | #[rustc_symbol_name]
2020-03-03T09:21:51.6433691Z 
2020-03-03T09:21:51.6433783Z 
2020-03-03T09:21:51.6433783Z 
2020-03-03T09:21:51.6433997Z The actual stderr differed from the expected stderr.
2020-03-03T09:21:51.6434652Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
2020-03-03T09:21:51.6435257Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T09:21:51.6435831Z To only update this specific test, also pass `--test-args symbol-names/basic.rs`
2020-03-03T09:21:51.6436059Z 
2020-03-03T09:21:51.6436286Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-03T09:21:51.6436580Z status: exit code: 1
2020-03-03T09:21:51.6438778Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
2020-03-03T09:21:51.6440593Z ------------------------------------------
2020-03-03T09:21:51.6440758Z 
2020-03-03T09:21:51.6441110Z ------------------------------------------
2020-03-03T09:21:51.6441303Z stderr:
2020-03-03T09:21:51.6441303Z stderr:
2020-03-03T09:21:51.6441652Z ------------------------------------------
2020-03-03T09:21:51.6442117Z error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-03-03T09:21:51.6442810Z    |
2020-03-03T09:21:51.6442986Z LL | #[rustc_symbol_name]
2020-03-03T09:21:51.6443183Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6443320Z 
2020-03-03T09:21:51.6443320Z 
2020-03-03T09:21:51.6443525Z error: demangling(basic::main::h7bbff4a01206d8c2)
2020-03-03T09:21:51.6444228Z    |
2020-03-03T09:21:51.6444386Z LL | #[rustc_symbol_name]
2020-03-03T09:21:51.6444593Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6444726Z 
2020-03-03T09:21:51.6444726Z 
2020-03-03T09:21:51.6445055Z error: demangling-alt(basic::main)
2020-03-03T09:21:51.6445725Z    |
2020-03-03T09:21:51.6445882Z LL | #[rustc_symbol_name]
2020-03-03T09:21:51.6446073Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6446212Z 
---
2020-03-03T09:21:51.6459018Z 
2020-03-03T09:21:51.6459421Z ---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
2020-03-03T09:21:51.6459667Z diff of stderr:
2020-03-03T09:21:51.6459794Z 
2020-03-03T09:21:51.6460253Z - error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2020-03-03T09:21:51.6460968Z + error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-03-03T09:21:51.6461607Z 3    |
2020-03-03T09:21:51.6469424Z 4 LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6469619Z 
2020-03-03T09:21:51.6469836Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6469836Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6470022Z 6 
2020-03-03T09:21:51.6470692Z - error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2020-03-03T09:21:51.6472704Z + error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-03-03T09:21:51.6473636Z 9    |
2020-03-03T09:21:51.6473819Z 10 LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6473995Z 
2020-03-03T09:21:51.6474162Z 22 LL |         #[rustc_def_path]
2020-03-03T09:21:51.6474162Z 22 LL |         #[rustc_def_path]
2020-03-03T09:21:51.6474382Z 23    |         ^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6474556Z 24 
2020-03-03T09:21:51.6475059Z - error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2020-03-03T09:21:51.6475742Z + error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-03-03T09:21:51.6476428Z 27    |
2020-03-03T09:21:51.6476613Z 28 LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6476766Z 
2020-03-03T09:21:51.6476935Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6476935Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6477129Z 30 
2020-03-03T09:21:51.6477615Z - error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2020-03-03T09:21:51.6478054Z + error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-03-03T09:21:51.6478718Z 33    |
2020-03-03T09:21:51.6478902Z 34 LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6479055Z 
2020-03-03T09:21:51.6479237Z 46 LL |         #[rustc_def_path]
2020-03-03T09:21:51.6479237Z 46 LL |         #[rustc_def_path]
2020-03-03T09:21:51.6479456Z 47    |         ^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6479623Z 48 
2020-03-03T09:21:51.6480561Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17SYMBOL_HASHE)
2020-03-03T09:21:51.6481953Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-03-03T09:21:51.6482926Z 51    |
2020-03-03T09:21:51.6483116Z 52 LL |             #[rustc_symbol_name]
2020-03-03T09:21:51.6484161Z 
2020-03-03T09:21:51.6484495Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6484495Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6484677Z 54 
2020-03-03T09:21:51.6485452Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::SYMBOL_HASH)
2020-03-03T09:21:51.6486217Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-03-03T09:21:51.6487065Z 57    |
2020-03-03T09:21:51.6487255Z 58 LL |             #[rustc_symbol_name]
2020-03-03T09:21:51.6487415Z 
2020-03-03T09:21:51.6487507Z 
2020-03-03T09:21:51.6487507Z 
2020-03-03T09:21:51.6487720Z The actual stderr differed from the expected stderr.
2020-03-03T09:21:51.6488359Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2020-03-03T09:21:51.6488965Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T09:21:51.6489533Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2020-03-03T09:21:51.6489762Z 
2020-03-03T09:21:51.6489991Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-03T09:21:51.6490260Z status: exit code: 1
2020-03-03T09:21:51.6492337Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary"
2020-03-03T09:21:51.6494062Z ------------------------------------------
2020-03-03T09:21:51.6494232Z 
2020-03-03T09:21:51.6494574Z ------------------------------------------
2020-03-03T09:21:51.6494788Z stderr:
2020-03-03T09:21:51.6494788Z stderr:
2020-03-03T09:21:51.6495133Z ------------------------------------------
2020-03-03T09:21:51.6495606Z error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-03-03T09:21:51.6496338Z    |
2020-03-03T09:21:51.6496698Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6496942Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6497095Z 
2020-03-03T09:21:51.6497095Z 
2020-03-03T09:21:51.6497328Z error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-03-03T09:21:51.6498095Z    |
2020-03-03T09:21:51.6498277Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6498503Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6498673Z 
2020-03-03T09:21:51.6498673Z 
2020-03-03T09:21:51.6499050Z error: demangling-alt(impl1::foo::Foo::bar)
2020-03-03T09:21:51.6499779Z    |
2020-03-03T09:21:51.6500117Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6500334Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6500483Z 
2020-03-03T09:21:51.6500483Z 
2020-03-03T09:21:51.6500823Z error: def-path(foo::Foo::bar)
2020-03-03T09:21:51.6501478Z    |
2020-03-03T09:21:51.6501668Z LL |         #[rustc_def_path]
2020-03-03T09:21:51.6501875Z    |         ^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6502015Z 
2020-03-03T09:21:51.6502015Z 
2020-03-03T09:21:51.6502500Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-03-03T09:21:51.6503285Z    |
2020-03-03T09:21:51.6503472Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6503690Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6503838Z 
2020-03-03T09:21:51.6503838Z 
2020-03-03T09:21:51.6504106Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-03-03T09:21:51.6504881Z    |
2020-03-03T09:21:51.6505051Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6505287Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6505433Z 
2020-03-03T09:21:51.6505433Z 
2020-03-03T09:21:51.6505853Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2020-03-03T09:21:51.6506606Z    |
2020-03-03T09:21:51.6507421Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6507718Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6507885Z 
2020-03-03T09:21:51.6507885Z 
2020-03-03T09:21:51.6508313Z error: def-path(bar::<impl foo::Foo>::baz)
2020-03-03T09:21:51.6509023Z    |
2020-03-03T09:21:51.6509191Z LL |         #[rustc_def_path]
2020-03-03T09:21:51.6509400Z    |         ^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6509541Z 
2020-03-03T09:21:51.6509541Z 
2020-03-03T09:21:51.6510484Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-03-03T09:21:51.6511725Z    |
2020-03-03T09:21:51.6513656Z LL |             #[rustc_symbol_name]
2020-03-03T09:21:51.6513907Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6514074Z 
2020-03-03T09:21:51.6514074Z 
2020-03-03T09:21:51.6514627Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-03-03T09:21:51.6515734Z    |
2020-03-03T09:21:51.6515945Z LL |             #[rustc_symbol_name]
2020-03-03T09:21:51.6516412Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6516762Z 
2020-03-03T09:21:51.6516762Z 
2020-03-03T09:21:51.6517617Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
2020-03-03T09:21:51.6518558Z    |
2020-03-03T09:21:51.6518753Z LL |             #[rustc_symbol_name]
2020-03-03T09:21:51.6521497Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6521680Z 
2020-03-03T09:21:51.6521680Z 
2020-03-03T09:21:51.6522630Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{{closure}}#1::Bar>::method)
2020-03-03T09:21:51.6523749Z    |
2020-03-03T09:21:51.6523926Z LL |             #[rustc_def_path]
2020-03-03T09:21:51.6524169Z    |             ^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6524320Z 
---
2020-03-03T09:21:51.6525383Z 
2020-03-03T09:21:51.6525784Z ---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
2020-03-03T09:21:51.6526023Z diff of stderr:
2020-03-03T09:21:51.6526142Z 
2020-03-03T09:21:51.6526845Z - error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2020-03-03T09:21:51.6528976Z + error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-03-03T09:21:51.6529863Z 3    |
2020-03-03T09:21:51.6530094Z 4 LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6530422Z 
2020-03-03T09:21:51.6530594Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6530594Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6530770Z 6 
2020-03-03T09:21:51.6531479Z - error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2020-03-03T09:21:51.6532166Z + error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-03-03T09:21:51.6532929Z 9    |
2020-03-03T09:21:51.6533110Z 10 LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6533264Z 
2020-03-03T09:21:51.6533356Z 
2020-03-03T09:21:51.6533356Z 
2020-03-03T09:21:51.6533572Z The actual stderr differed from the expected stderr.
2020-03-03T09:21:51.6534239Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
2020-03-03T09:21:51.6538857Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T09:21:51.6539757Z To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`
2020-03-03T09:21:51.6540025Z 
2020-03-03T09:21:51.6540275Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-03T09:21:51.6540891Z status: exit code: 1
2020-03-03T09:21:51.6542905Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary"
2020-03-03T09:21:51.6544721Z ------------------------------------------
2020-03-03T09:21:51.6544968Z 
2020-03-03T09:21:51.6545361Z ------------------------------------------
2020-03-03T09:21:51.6545555Z stderr:
2020-03-03T09:21:51.6545555Z stderr:
2020-03-03T09:21:51.6545903Z ------------------------------------------
2020-03-03T09:21:51.6550546Z error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-03-03T09:21:51.6553646Z    |
2020-03-03T09:21:51.6553854Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6554074Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6554223Z 
2020-03-03T09:21:51.6554223Z 
2020-03-03T09:21:51.6554534Z error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-03-03T09:21:51.6555461Z    |
2020-03-03T09:21:51.6555650Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6555869Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6556016Z 
2020-03-03T09:21:51.6556016Z 
2020-03-03T09:21:51.6556482Z error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
2020-03-03T09:21:51.6557291Z    |
2020-03-03T09:21:51.6557465Z LL |         #[rustc_symbol_name]
2020-03-03T09:21:51.6557702Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T09:21:51.6557850Z 
---
2020-03-03T09:21:51.6561133Z 
2020-03-03T09:21:51.6561233Z 
2020-03-03T09:21:51.6561330Z 
2020-03-03T09:21:51.6561484Z failures:
2020-03-03T09:21:51.6562217Z     [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs
2020-03-03T09:21:51.6563251Z     [ui] ui/symbol-names/impl1.rs#legacy
2020-03-03T09:21:51.6563710Z     [ui] ui/symbol-names/issue-60925.rs#legacy
2020-03-03T09:21:51.6563903Z 
2020-03-03T09:21:51.6564578Z test result: FAILED. 9668 passed; 4 failed; 54 ignored; 0 measured; 0 filtered out
2020-03-03T09:21:51.6564578Z test result: FAILED. 9668 passed; 4 failed; 54 ignored; 0 measured; 0 filtered out
2020-03-03T09:21:51.6564885Z 
2020-03-03T09:21:51.6565529Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-03T09:21:51.6565963Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-03T09:21:51.6566331Z 
2020-03-03T09:21:51.6566433Z 
2020-03-03T09:21:51.6570466Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-03T09:21:51.6573698Z 
2020-03-03T09:21:51.6573794Z 
2020-03-03T09:21:51.6574019Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-03T09:21:51.6574530Z Build completed unsuccessfully in 1:05:34
2020-03-03T09:21:51.6574530Z Build completed unsuccessfully in 1:05:34
2020-03-03T09:21:51.6574848Z == clock drift check ==
2020-03-03T09:21:51.6575098Z   local time: Tue Mar  3 09:21:51 UTC 2020
2020-03-03T09:21:52.2058702Z   network time: Tue, 03 Mar 2020 09:21:52 GMT
2020-03-03T09:21:52.2059045Z == end clock drift check ==
2020-03-03T09:21:52.9322034Z 
2020-03-03T09:21:52.9395347Z ##[error]Bash exited with code '1'.
2020-03-03T09:21:52.9412207Z ##[section]Finishing: Run build
2020-03-03T09:21:52.9467737Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T09:21:52.9473624Z Task         : Get sources
2020-03-03T09:21:52.9473953Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T09:21:52.9474295Z Version      : 1.0.0
2020-03-03T09:21:52.9474507Z Author       : Microsoft
2020-03-03T09:21:52.9474507Z Author       : Microsoft
2020-03-03T09:21:52.9474841Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T09:21:52.9475412Z ==============================================================================
2020-03-03T09:21:53.2595071Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T09:21:53.2638208Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T09:21:53.2724256Z Cleaning up task key
2020-03-03T09:21:53.2725612Z Start cleaning up orphan processes.
2020-03-03T09:21:53.2893942Z Terminate orphan process: pid (5297) (python)
2020-03-03T09:21:53.3189152Z ##[section]Finishing: Finalize Job
