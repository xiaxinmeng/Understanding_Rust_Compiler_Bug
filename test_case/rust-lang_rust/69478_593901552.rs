plain
2020-03-03T10:03:36.6258292Z ========================== Starting Command Output ===========================
2020-03-03T10:03:36.6263404Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2c0dc405-e57f-402e-a35c-9f7625dd9fc0.sh
2020-03-03T10:03:36.6263914Z 
2020-03-03T10:03:36.6268136Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T10:03:36.6289615Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T10:03:36.6293246Z Task         : Get sources
2020-03-03T10:03:36.6293573Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T10:03:36.6293885Z Version      : 1.0.0
2020-03-03T10:03:36.6294099Z Author       : Microsoft
---
2020-03-03T10:03:37.6331657Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T10:03:37.6336729Z ##[command]git config gc.auto 0
2020-03-03T10:03:37.6340277Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T10:03:37.6343515Z ##[command]git config --get-all http.proxy
2020-03-03T10:03:37.6350063Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-03-03T11:09:13.2087942Z .................................................................................................... 1700/9726
2020-03-03T11:09:17.6167288Z .................................................................................................... 1800/9726
2020-03-03T11:09:29.6186366Z ......................................................i............................................. 1900/9726
2020-03-03T11:09:37.3684984Z .................................................................................................... 2000/9726
2020-03-03T11:09:51.1783985Z ............................................iiiii................................................... 2100/9726
2020-03-03T11:10:01.0862458Z .................................................................................................... 2300/9726
2020-03-03T11:10:03.3879928Z .................................................................................................... 2400/9726
2020-03-03T11:10:07.0246451Z .................................................................................................... 2500/9726
2020-03-03T11:10:28.7941681Z .................................................................................................... 2600/9726
---
2020-03-03T11:13:05.9819923Z ......i...............i............................................................................. 5000/9726
2020-03-03T11:13:15.5811339Z .................................................................................................... 5100/9726
2020-03-03T11:13:20.4029150Z .................................................i.................................................. 5200/9726
2020-03-03T11:13:29.1112305Z .................................................................................................... 5300/9726
2020-03-03T11:13:36.1897347Z ............................ii.ii........i...i...................................................... 5400/9726
2020-03-03T11:13:44.1156616Z .................................................................................................... 5600/9726
2020-03-03T11:13:53.4823605Z .................................................................................................... 5700/9726
2020-03-03T11:14:00.4218341Z ...................i................................................................................ 5800/9726
2020-03-03T11:14:05.7690766Z .................................................................................................... 5900/9726
2020-03-03T11:14:05.7690766Z .................................................................................................... 5900/9726
2020-03-03T11:14:16.4577886Z .................................................................................................... 6000/9726
2020-03-03T11:14:27.4609529Z ...........ii...i..ii...........i................................................................... 6100/9726
2020-03-03T11:14:43.3646915Z .................................................................................................... 6300/9726
2020-03-03T11:14:50.1329308Z .................................................................................................... 6400/9726
2020-03-03T11:14:50.1329308Z .................................................................................................... 6400/9726
2020-03-03T11:15:06.7954713Z ..........................................i..ii..................................................... 6500/9726
2020-03-03T11:15:29.0775313Z .................................................................................................... 6700/9726
2020-03-03T11:15:31.1397827Z ..................................i................................................................. 6800/9726
2020-03-03T11:15:33.2139548Z .................................................................................................... 6900/9726
2020-03-03T11:15:35.5397031Z ................................................................i................................... 7000/9726
---
2020-03-03T11:17:15.6473113Z .................................................................................................... 7700/9726
2020-03-03T11:17:20.8483698Z .................................................................................................... 7800/9726
2020-03-03T11:17:26.0840768Z .................................................................................................... 7900/9726
2020-03-03T11:17:34.2618584Z ..........i......................................................................................... 8000/9726
2020-03-03T11:17:43.2151679Z ...........................................................iiiiiii.i................................ 8100/9726
2020-03-03T11:17:57.3289519Z .i.....i............................................................................................ 8300/9726
2020-03-03T11:18:02.7712038Z .................................................................................................... 8400/9726
2020-03-03T11:18:17.5087757Z .................................................................................................... 8500/9726
2020-03-03T11:18:25.8133270Z .................................................................................................... 8600/9726
---
2020-03-03T11:20:10.4033955Z ..............................i..................................................................... 9700/9726
2020-03-03T11:20:20.9463978Z ..........................
2020-03-03T11:20:20.9465011Z failures:
2020-03-03T11:20:20.9523897Z 
2020-03-03T11:20:20.9525066Z ---- [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs stdout ----
2020-03-03T11:20:20.9526011Z 
2020-03-03T11:20:20.9526011Z 
2020-03-03T11:20:20.9526841Z 1 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
2020-03-03T11:20:20.9527782Z -   --> $DIR/feature-gate-abi-avr-interrupt.rs:14:1
2020-03-03T11:20:20.9528609Z +   --> $DIR/feature-gate-abi-avr-interrupt.rs:4:8
2020-03-03T11:20:20.9529094Z 3    |
2020-03-03T11:20:20.9529744Z 4 LL | extern "avr-interrupt" fn foo() {}
2020-03-03T11:20:20.9531254Z +    |        ^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9531654Z 6    |
2020-03-03T11:20:20.9531654Z 6    |
2020-03-03T11:20:20.9532426Z -    = help: add #![feature(abi_avr_interrupt)] to the crate attributes to enable
2020-03-03T11:20:20.9533550Z +    = note: see issue #69664 <***/issues/69664> for more information
2020-03-03T11:20:20.9534379Z +    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
2020-03-03T11:20:20.9535331Z 9 error: aborting due to previous error
2020-03-03T11:20:20.9536273Z 10 
2020-03-03T11:20:20.9536560Z 
2020-03-03T11:20:20.9536793Z 
2020-03-03T11:20:20.9536793Z 
2020-03-03T11:20:20.9537125Z The actual stderr differed from the expected stderr.
2020-03-03T11:20:20.9538176Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/feature-gate-abi-avr-interrupt.stderr
2020-03-03T11:20:20.9539057Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T11:20:20.9539924Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-avr-interrupt.rs`
2020-03-03T11:20:20.9540671Z error: 1 errors occurred comparing output.
2020-03-03T11:20:20.9541049Z status: exit code: 1
2020-03-03T11:20:20.9541049Z status: exit code: 1
2020-03-03T11:20:20.9543327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/auxiliary"
2020-03-03T11:20:20.9545376Z ------------------------------------------
2020-03-03T11:20:20.9545724Z 
2020-03-03T11:20:20.9546250Z ------------------------------------------
2020-03-03T11:20:20.9546601Z stderr:
2020-03-03T11:20:20.9546601Z stderr:
2020-03-03T11:20:20.9547146Z ------------------------------------------
2020-03-03T11:20:20.9549934Z error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
2020-03-03T11:20:20.9550884Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs:4:8
2020-03-03T11:20:20.9551324Z    |
2020-03-03T11:20:20.9551840Z LL | extern "avr-interrupt" fn foo() {}
2020-03-03T11:20:20.9552537Z    |
2020-03-03T11:20:20.9552537Z    |
2020-03-03T11:20:20.9553288Z    = note: see issue #69664 <***/issues/69664> for more information
2020-03-03T11:20:20.9553844Z    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
2020-03-03T11:20:20.9554538Z error: aborting due to previous error
2020-03-03T11:20:20.9554817Z 
2020-03-03T11:20:20.9555413Z For more information about this error, try `rustc --explain E0658`.
2020-03-03T11:20:20.9556441Z 
2020-03-03T11:20:20.9556441Z 
2020-03-03T11:20:20.9557138Z ------------------------------------------
2020-03-03T11:20:20.9557463Z 
2020-03-03T11:20:20.9557676Z 
2020-03-03T11:20:20.9558236Z ---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
2020-03-03T11:20:20.9558640Z diff of stderr:
2020-03-03T11:20:20.9558888Z 
2020-03-03T11:20:20.9559482Z - error: symbol-name(_ZN5basic4main17h81759b0695851718E)
2020-03-03T11:20:20.9560181Z + error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-03-03T11:20:20.9561138Z 3    |
2020-03-03T11:20:20.9561455Z 4 LL | #[rustc_symbol_name]
2020-03-03T11:20:20.9561717Z 
2020-03-03T11:20:20.9561997Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9561997Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9562281Z 6 
2020-03-03T11:20:20.9563188Z - error: demangling(basic::main::h81759b0695851718)
2020-03-03T11:20:20.9563876Z + error: demangling(basic::main::h7bbff4a01206d8c2)
2020-03-03T11:20:20.9564861Z 9    |
2020-03-03T11:20:20.9565469Z 10 LL | #[rustc_symbol_name]
2020-03-03T11:20:20.9566103Z 
2020-03-03T11:20:20.9569419Z 
2020-03-03T11:20:20.9569419Z 
2020-03-03T11:20:20.9569801Z The actual stderr differed from the expected stderr.
2020-03-03T11:20:20.9570903Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
2020-03-03T11:20:20.9571832Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T11:20:20.9572602Z To only update this specific test, also pass `--test-args symbol-names/basic.rs`
2020-03-03T11:20:20.9573429Z 
2020-03-03T11:20:20.9574154Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-03T11:20:20.9574651Z status: exit code: 1
2020-03-03T11:20:20.9580867Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
2020-03-03T11:20:20.9582596Z ------------------------------------------
2020-03-03T11:20:20.9582776Z 
2020-03-03T11:20:20.9583158Z ------------------------------------------
2020-03-03T11:20:20.9583362Z stderr:
2020-03-03T11:20:20.9583362Z stderr:
2020-03-03T11:20:20.9583734Z ------------------------------------------
2020-03-03T11:20:20.9584233Z error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-03-03T11:20:20.9584970Z    |
2020-03-03T11:20:20.9585153Z LL | #[rustc_symbol_name]
2020-03-03T11:20:20.9585359Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9585498Z 
2020-03-03T11:20:20.9585498Z 
2020-03-03T11:20:20.9585715Z error: demangling(basic::main::h7bbff4a01206d8c2)
2020-03-03T11:20:20.9586463Z    |
2020-03-03T11:20:20.9586629Z LL | #[rustc_symbol_name]
2020-03-03T11:20:20.9586849Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9586990Z 
2020-03-03T11:20:20.9586990Z 
2020-03-03T11:20:20.9587349Z error: demangling-alt(basic::main)
2020-03-03T11:20:20.9588058Z    |
2020-03-03T11:20:20.9588224Z LL | #[rustc_symbol_name]
2020-03-03T11:20:20.9588425Z    | ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9588579Z 
---
2020-03-03T11:20:20.9591194Z 
2020-03-03T11:20:20.9591586Z ---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
2020-03-03T11:20:20.9591829Z diff of stderr:
2020-03-03T11:20:20.9591956Z 
2020-03-03T11:20:20.9592409Z - error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2020-03-03T11:20:20.9592958Z + error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-03-03T11:20:20.9593598Z 3    |
2020-03-03T11:20:20.9593788Z 4 LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9593947Z 
2020-03-03T11:20:20.9594138Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9594138Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9594460Z 6 
2020-03-03T11:20:20.9594946Z - error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2020-03-03T11:20:20.9595319Z + error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-03-03T11:20:20.9595976Z 9    |
2020-03-03T11:20:20.9596167Z 10 LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9596343Z 
2020-03-03T11:20:20.9596517Z 22 LL |         #[rustc_def_path]
2020-03-03T11:20:20.9596517Z 22 LL |         #[rustc_def_path]
2020-03-03T11:20:20.9596830Z 23    |         ^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9597031Z 24 
2020-03-03T11:20:20.9597587Z - error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2020-03-03T11:20:20.9598280Z + error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-03-03T11:20:20.9598986Z 27    |
2020-03-03T11:20:20.9599179Z 28 LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9599339Z 
2020-03-03T11:20:20.9599533Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9599533Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9599723Z 30 
2020-03-03T11:20:20.9600235Z - error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2020-03-03T11:20:20.9600709Z + error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-03-03T11:20:20.9601393Z 33    |
2020-03-03T11:20:20.9601600Z 34 LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9601761Z 
2020-03-03T11:20:20.9601939Z 46 LL |         #[rustc_def_path]
2020-03-03T11:20:20.9601939Z 46 LL |         #[rustc_def_path]
2020-03-03T11:20:20.9602170Z 47    |         ^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9602363Z 48 
2020-03-03T11:20:20.9604434Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17SYMBOL_HASHE)
2020-03-03T11:20:20.9605913Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-03-03T11:20:20.9606928Z 51    |
2020-03-03T11:20:20.9607148Z 52 LL |             #[rustc_symbol_name]
2020-03-03T11:20:20.9607316Z 
2020-03-03T11:20:20.9607500Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9607500Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9607703Z 54 
2020-03-03T11:20:20.9608430Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::SYMBOL_HASH)
2020-03-03T11:20:20.9609205Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-03-03T11:20:20.9610077Z 57    |
2020-03-03T11:20:20.9610275Z 58 LL |             #[rustc_symbol_name]
2020-03-03T11:20:20.9610442Z 
2020-03-03T11:20:20.9610561Z 
2020-03-03T11:20:20.9610561Z 
2020-03-03T11:20:20.9610768Z The actual stderr differed from the expected stderr.
2020-03-03T11:20:20.9611440Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2020-03-03T11:20:20.9612087Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T11:20:20.9612665Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2020-03-03T11:20:20.9612904Z 
2020-03-03T11:20:20.9613147Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-03T11:20:20.9613447Z status: exit code: 1
2020-03-03T11:20:20.9615489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary"
2020-03-03T11:20:20.9617344Z ------------------------------------------
2020-03-03T11:20:20.9617542Z 
2020-03-03T11:20:20.9617987Z ------------------------------------------
2020-03-03T11:20:20.9618208Z stderr:
2020-03-03T11:20:20.9618208Z stderr:
2020-03-03T11:20:20.9618599Z ------------------------------------------
2020-03-03T11:20:20.9619124Z error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-03-03T11:20:20.9619882Z    |
2020-03-03T11:20:20.9620066Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9620297Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9620452Z 
2020-03-03T11:20:20.9620452Z 
2020-03-03T11:20:20.9620705Z error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-03-03T11:20:20.9621469Z    |
2020-03-03T11:20:20.9621665Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9621894Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9622048Z 
2020-03-03T11:20:20.9622048Z 
2020-03-03T11:20:20.9622434Z error: demangling-alt(impl1::foo::Foo::bar)
2020-03-03T11:20:20.9623225Z    |
2020-03-03T11:20:20.9623409Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9628465Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9628632Z 
2020-03-03T11:20:20.9628632Z 
2020-03-03T11:20:20.9629082Z error: def-path(foo::Foo::bar)
2020-03-03T11:20:20.9629799Z    |
2020-03-03T11:20:20.9629975Z LL |         #[rustc_def_path]
2020-03-03T11:20:20.9630193Z    |         ^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9630357Z 
2020-03-03T11:20:20.9630357Z 
2020-03-03T11:20:20.9630894Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-03-03T11:20:20.9631709Z    |
2020-03-03T11:20:20.9631888Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9632118Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9632272Z 
2020-03-03T11:20:20.9632272Z 
2020-03-03T11:20:20.9632569Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-03-03T11:20:20.9633368Z    |
2020-03-03T11:20:20.9633565Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9633793Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9633948Z 
2020-03-03T11:20:20.9633948Z 
2020-03-03T11:20:20.9634415Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2020-03-03T11:20:20.9635184Z    |
2020-03-03T11:20:20.9635363Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9635616Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9635770Z 
2020-03-03T11:20:20.9635770Z 
2020-03-03T11:20:20.9636153Z error: def-path(bar::<impl foo::Foo>::baz)
2020-03-03T11:20:20.9639805Z    |
2020-03-03T11:20:20.9639982Z LL |         #[rustc_def_path]
2020-03-03T11:20:20.9640218Z    |         ^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9640369Z 
2020-03-03T11:20:20.9640369Z 
2020-03-03T11:20:20.9641375Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-03-03T11:20:20.9642794Z    |
2020-03-03T11:20:20.9642987Z LL |             #[rustc_symbol_name]
2020-03-03T11:20:20.9643247Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9643408Z 
2020-03-03T11:20:20.9643408Z 
2020-03-03T11:20:20.9644004Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-03-03T11:20:20.9645060Z    |
2020-03-03T11:20:20.9645246Z LL |             #[rustc_symbol_name]
2020-03-03T11:20:20.9645488Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9645664Z 
2020-03-03T11:20:20.9645664Z 
2020-03-03T11:20:20.9646438Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
2020-03-03T11:20:20.9647785Z    |
2020-03-03T11:20:20.9647973Z LL |             #[rustc_symbol_name]
2020-03-03T11:20:20.9648218Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9648379Z 
2020-03-03T11:20:20.9648379Z 
2020-03-03T11:20:20.9649043Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{{closure}}#1::Bar>::method)
2020-03-03T11:20:20.9649960Z    |
2020-03-03T11:20:20.9650159Z LL |             #[rustc_def_path]
2020-03-03T11:20:20.9650393Z    |             ^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9650549Z 
---
2020-03-03T11:20:20.9651894Z 
2020-03-03T11:20:20.9652571Z ---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
2020-03-03T11:20:20.9652826Z diff of stderr:
2020-03-03T11:20:20.9652951Z 
2020-03-03T11:20:20.9653524Z - error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2020-03-03T11:20:20.9654260Z + error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-03-03T11:20:20.9655024Z 3    |
2020-03-03T11:20:20.9655213Z 4 LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9656668Z 
2020-03-03T11:20:20.9656904Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9656904Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9657085Z 6 
2020-03-03T11:20:20.9657749Z - error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2020-03-03T11:20:20.9659187Z + error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-03-03T11:20:20.9660071Z 9    |
2020-03-03T11:20:20.9660277Z 10 LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9660441Z 
2020-03-03T11:20:20.9660537Z 
2020-03-03T11:20:20.9660537Z 
2020-03-03T11:20:20.9660743Z The actual stderr differed from the expected stderr.
2020-03-03T11:20:20.9661776Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
2020-03-03T11:20:20.9664984Z To update references, rerun the tests and pass the `--bless` flag
2020-03-03T11:20:20.9665595Z To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`
2020-03-03T11:20:20.9671049Z 
2020-03-03T11:20:20.9671345Z error in revision `legacy`: 1 errors occurred comparing output.
2020-03-03T11:20:20.9671629Z status: exit code: 1
2020-03-03T11:20:20.9680011Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary"
2020-03-03T11:20:20.9682684Z ------------------------------------------
2020-03-03T11:20:20.9682871Z 
2020-03-03T11:20:20.9683303Z ------------------------------------------
2020-03-03T11:20:20.9683511Z stderr:
2020-03-03T11:20:20.9683511Z stderr:
2020-03-03T11:20:20.9683906Z ------------------------------------------
2020-03-03T11:20:20.9684526Z error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-03-03T11:20:20.9685569Z    |
2020-03-03T11:20:20.9685753Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9686108Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9686284Z 
2020-03-03T11:20:20.9686284Z 
2020-03-03T11:20:20.9686591Z error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-03-03T11:20:20.9687527Z    |
2020-03-03T11:20:20.9687707Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9687945Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9688100Z 
2020-03-03T11:20:20.9688100Z 
2020-03-03T11:20:20.9688610Z error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
2020-03-03T11:20:20.9689434Z    |
2020-03-03T11:20:20.9689631Z LL |         #[rustc_symbol_name]
2020-03-03T11:20:20.9689860Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-03-03T11:20:20.9690014Z 
---
2020-03-03T11:20:20.9691186Z 
2020-03-03T11:20:20.9691301Z 
2020-03-03T11:20:20.9691398Z 
2020-03-03T11:20:20.9691528Z failures:
2020-03-03T11:20:20.9691956Z     [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs
2020-03-03T11:20:20.9692858Z     [ui] ui/symbol-names/impl1.rs#legacy
2020-03-03T11:20:20.9693312Z     [ui] ui/symbol-names/issue-60925.rs#legacy
2020-03-03T11:20:20.9693510Z 
2020-03-03T11:20:20.9694021Z test result: FAILED. 9668 passed; 4 failed; 54 ignored; 0 measured; 0 filtered out
2020-03-03T11:20:20.9694021Z test result: FAILED. 9668 passed; 4 failed; 54 ignored; 0 measured; 0 filtered out
2020-03-03T11:20:20.9694291Z 
2020-03-03T11:20:20.9694780Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-03T11:20:20.9695217Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-03T11:20:20.9695452Z 
2020-03-03T11:20:20.9695548Z 
2020-03-03T11:20:20.9699455Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-03T11:20:20.9702265Z 
2020-03-03T11:20:20.9702364Z 
2020-03-03T11:20:20.9702658Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-03T11:20:20.9703110Z Build completed unsuccessfully in 1:08:51
2020-03-03T11:20:20.9703110Z Build completed unsuccessfully in 1:08:51
2020-03-03T11:20:20.9703359Z == clock drift check ==
2020-03-03T11:20:20.9703623Z   local time: Tue Mar  3 11:20:20 UTC 2020
2020-03-03T11:20:21.2632590Z   network time: Tue, 03 Mar 2020 11:20:21 GMT
2020-03-03T11:20:21.2635826Z == end clock drift check ==
2020-03-03T11:20:21.6550427Z 
2020-03-03T11:20:21.6627583Z ##[error]Bash exited with code '1'.
2020-03-03T11:20:21.6643123Z ##[section]Finishing: Run build
2020-03-03T11:20:21.6711931Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T11:20:21.6717457Z Task         : Get sources
2020-03-03T11:20:21.6717844Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T11:20:21.6718200Z Version      : 1.0.0
2020-03-03T11:20:21.6718486Z Author       : Microsoft
2020-03-03T11:20:21.6718486Z Author       : Microsoft
2020-03-03T11:20:21.6718895Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T11:20:21.6719354Z ==============================================================================
2020-03-03T11:20:22.0356477Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T11:20:22.0398294Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T11:20:22.0488344Z Cleaning up task key
2020-03-03T11:20:22.0489570Z Start cleaning up orphan processes.
2020-03-03T11:20:22.0669603Z Terminate orphan process: pid (4055) (python)
2020-03-03T11:20:22.0852135Z ##[section]Finishing: Finalize Job
