plain
2020-02-26T11:32:06.0591916Z ========================== Starting Command Output ===========================
2020-02-26T11:32:06.0594636Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a6a20ad7-5687-41f8-bffe-b00c1c8304f8.sh
2020-02-26T11:32:06.0594930Z 
2020-02-26T11:32:06.0600114Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T11:32:06.0670466Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T11:32:06.0674219Z Task         : Get sources
2020-02-26T11:32:06.0674546Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T11:32:06.0674857Z Version      : 1.0.0
2020-02-26T11:32:06.0675087Z Author       : Microsoft
---
2020-02-26T11:32:07.0725532Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T11:32:07.0733958Z ##[command]git config gc.auto 0
2020-02-26T11:32:07.0739428Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T11:32:07.0745720Z ##[command]git config --get-all http.proxy
2020-02-26T11:32:07.0798075Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-02-26T12:38:33.4605938Z .................................................................................................... 1700/9709
2020-02-26T12:38:38.1395511Z .................................................................................................... 1800/9709
2020-02-26T12:38:50.0001132Z ...........................................i........................................................ 1900/9709
2020-02-26T12:38:58.1294012Z .................................................................................................... 2000/9709
2020-02-26T12:39:13.3007276Z .................................iiiii.............................................................. 2100/9709
2020-02-26T12:39:23.3976018Z .................................................................................................... 2300/9709
2020-02-26T12:39:25.7556929Z .................................................................................................... 2400/9709
2020-02-26T12:39:30.2206268Z .................................................................................................... 2500/9709
2020-02-26T12:39:51.7263175Z .................................................................................................... 2600/9709
---
2020-02-26T12:42:35.2558145Z ..........i......................................................................................... 5000/9709
2020-02-26T12:42:44.7353332Z .................................................................................................... 5100/9709
2020-02-26T12:42:49.6995610Z .....................................i.............................................................. 5200/9709
2020-02-26T12:43:00.6197543Z .................................................................................................... 5300/9709
2020-02-26T12:43:06.9409834Z ..............ii.ii........i...i.................................................................... 5400/9709
2020-02-26T12:43:15.8760510Z .................................................................................................... 5600/9709
2020-02-26T12:43:27.1695860Z .................................................................................................... 5700/9709
2020-02-26T12:43:34.2543323Z .....i.............................................................................................. 5800/9709
2020-02-26T12:43:40.6788972Z .................................................................................................... 5900/9709
2020-02-26T12:43:40.6788972Z .................................................................................................... 5900/9709
2020-02-26T12:43:50.9497890Z ................................................................................................ii.. 6000/9709
2020-02-26T12:44:03.0338153Z .i..ii...........i.................................................................................. 6100/9709
2020-02-26T12:44:20.5623358Z .................................................................................................... 6300/9709
2020-02-26T12:44:27.4742484Z .................................................................................................... 6400/9709
2020-02-26T12:44:27.4742484Z .................................................................................................... 6400/9709
2020-02-26T12:44:46.2290110Z ...........................i..ii.................................................................... 6500/9709
2020-02-26T12:45:08.7088749Z .................................................................................................... 6700/9709
2020-02-26T12:45:10.9424600Z ...................i................................................................................ 6800/9709
2020-02-26T12:45:13.2024014Z ...............................................................F.................................... 6900/9709
2020-02-26T12:45:15.5181529Z .................................................i.................................................. 7000/9709
---
2020-02-26T12:46:59.6542467Z .................................................................................................... 7700/9709
2020-02-26T12:47:04.8875194Z .................................................................................................... 7800/9709
2020-02-26T12:47:12.1847940Z ..............................................................................................i..... 7900/9709
2020-02-26T12:47:20.8040616Z .................................................................................................... 8000/9709
2020-02-26T12:47:29.0137670Z ...........................................iiiiiii.i................................................ 8100/9709
2020-02-26T12:47:44.2388610Z .................................................................................................... 8300/9709
2020-02-26T12:47:50.1691835Z .................................................................................................... 8400/9709
2020-02-26T12:48:06.3608290Z .................................................................................................... 8500/9709
2020-02-26T12:48:13.6952598Z .................................................................................................... 8600/9709
---
2020-02-26T12:50:12.9351523Z 
2020-02-26T12:50:12.9352454Z ---- [ui] ui/codemap_tests/unicode.rs stdout ----
2020-02-26T12:50:12.9352876Z diff of stderr:
2020-02-26T12:50:12.9353130Z 
2020-02-26T12:50:12.9353712Z 4 LL | extern "路濫狼á́́" fn foo() {}
2020-02-26T12:50:12.9356269Z 5    |        ^^^^^^^^^ invalid ABI
2020-02-26T12:50:12.9356522Z 6    |
2020-02-26T12:50:12.9358123Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-02-26T12:50:12.9363126Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-02-26T12:50:12.9364239Z 9 error: aborting due to previous error
2020-02-26T12:50:12.9364499Z 10 
2020-02-26T12:50:12.9364632Z 
2020-02-26T12:50:12.9364753Z 
2020-02-26T12:50:12.9364753Z 
2020-02-26T12:50:12.9365021Z The actual stderr differed from the expected stderr.
2020-02-26T12:50:12.9365858Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/unicode.stderr
2020-02-26T12:50:12.9366642Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T12:50:12.9367374Z To only update this specific test, also pass `--test-args codemap_tests/unicode.rs`
2020-02-26T12:50:12.9368264Z error: 1 errors occurred comparing output.
2020-02-26T12:50:12.9368570Z status: exit code: 1
2020-02-26T12:50:12.9368570Z status: exit code: 1
2020-02-26T12:50:12.9374023Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/unicode.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/unicode/auxiliary"
2020-02-26T12:50:12.9375805Z ------------------------------------------
2020-02-26T12:50:12.9375992Z 
2020-02-26T12:50:12.9376370Z ------------------------------------------
2020-02-26T12:50:12.9376593Z stderr:
2020-02-26T12:50:12.9376593Z stderr:
2020-02-26T12:50:12.9376995Z ------------------------------------------
2020-02-26T12:50:12.9377464Z error[E0703]: invalid ABI: found `路濫狼á́́`
2020-02-26T12:50:12.9378234Z    |
2020-02-26T12:50:12.9378234Z    |
2020-02-26T12:50:12.9378657Z LL | extern "路濫狼á́́" fn foo() {} //~ ERROR invalid ABI
2020-02-26T12:50:12.9378939Z    |        ^^^^^^^^^ invalid ABI
2020-02-26T12:50:12.9379144Z    |
2020-02-26T12:50:12.9380176Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-02-26T12:50:12.9380997Z error: aborting due to previous error
2020-02-26T12:50:12.9381170Z 
2020-02-26T12:50:12.9381271Z 
2020-02-26T12:50:12.9381636Z ------------------------------------------
2020-02-26T12:50:12.9381636Z ------------------------------------------
2020-02-26T12:50:12.9382712Z 
2020-02-26T12:50:12.9382826Z 
2020-02-26T12:50:12.9383415Z ---- [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs stdout ----
2020-02-26T12:50:12.9383651Z 
2020-02-26T12:50:12.9383941Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-26T12:50:12.9384257Z status: exit code: 101
2020-02-26T12:50:12.9386764Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/auxiliary"
2020-02-26T12:50:12.9388601Z ------------------------------------------
2020-02-26T12:50:12.9388803Z 
2020-02-26T12:50:12.9389174Z ------------------------------------------
2020-02-26T12:50:12.9389384Z stderr:
2020-02-26T12:50:12.9389384Z stderr:
2020-02-26T12:50:12.9389774Z ------------------------------------------
2020-02-26T12:50:12.9390365Z error: internal compiler error: unrecognized ABI not caught in lowering: avr-interrupt
2020-02-26T12:50:12.9391043Z   --> /checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs:4:8
2020-02-26T12:50:12.9391349Z    |
2020-02-26T12:50:12.9391714Z LL | extern "avr-interrupt" fn foo() {}
2020-02-26T12:50:12.9392125Z 
2020-02-26T12:50:12.9392125Z 
2020-02-26T12:50:12.9392713Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:355:17
2020-02-26T12:50:12.9393616Z 
2020-02-26T12:50:12.9393903Z error: internal compiler error: unexpected panic
2020-02-26T12:50:12.9394111Z 
2020-02-26T12:50:12.9394334Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-26T12:50:12.9394334Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-26T12:50:12.9394557Z 
2020-02-26T12:50:12.9395293Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-26T12:50:12.9396097Z note: rustc 1.43.0-nightly (7be7d54d3 2020-02-26) running on x86_64-unknown-linux-gnu
2020-02-26T12:50:12.9396359Z 
2020-02-26T12:50:12.9396359Z 
2020-02-26T12:50:12.9397011Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-26T12:50:12.9397494Z 
2020-02-26T12:50:12.9397855Z ------------------------------------------
2020-02-26T12:50:12.9398034Z 
2020-02-26T12:50:12.9398134Z 
2020-02-26T12:50:12.9398134Z 
2020-02-26T12:50:12.9398521Z ---- [ui] ui/parser/issue-8537.rs stdout ----
2020-02-26T12:50:12.9398756Z diff of stderr:
2020-02-26T12:50:12.9398893Z 
2020-02-26T12:50:12.9399218Z 4 LL |   "invalid-ab_isize"
2020-02-26T12:50:12.9399484Z 5    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2020-02-26T12:50:12.9399685Z 6    |
2020-02-26T12:50:12.9400612Z -    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-02-26T12:50:12.9402086Z +    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-02-26T12:50:12.9402944Z 9 error: aborting due to previous error
2020-02-26T12:50:12.9403144Z 10 
2020-02-26T12:50:12.9403252Z 
2020-02-26T12:50:12.9403353Z 
2020-02-26T12:50:12.9403353Z 
2020-02-26T12:50:12.9403582Z The actual stderr differed from the expected stderr.
2020-02-26T12:50:12.9404248Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/issue-8537.stderr
2020-02-26T12:50:12.9404873Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T12:50:12.9405473Z To only update this specific test, also pass `--test-args parser/issue-8537.rs`
2020-02-26T12:50:12.9405918Z error: 1 errors occurred comparing output.
2020-02-26T12:50:12.9406178Z status: exit code: 1
2020-02-26T12:50:12.9406178Z status: exit code: 1
2020-02-26T12:50:12.9408117Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-8537.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/auxiliary"
2020-02-26T12:50:12.9409733Z ------------------------------------------
2020-02-26T12:50:12.9409914Z 
2020-02-26T12:50:12.9410291Z ------------------------------------------
2020-02-26T12:50:12.9410501Z stderr:
2020-02-26T12:50:12.9410501Z stderr:
2020-02-26T12:50:12.9410872Z ------------------------------------------
2020-02-26T12:50:12.9411368Z error[E0703]: invalid ABI: found `invalid-ab_isize`
2020-02-26T12:50:12.9412114Z    |
2020-02-26T12:50:12.9412114Z    |
2020-02-26T12:50:12.9412498Z LL |   "invalid-ab_isize" //~ ERROR invalid ABI
2020-02-26T12:50:12.9412789Z    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2020-02-26T12:50:12.9412982Z    |
2020-02-26T12:50:12.9414097Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-02-26T12:50:12.9414984Z error: aborting due to previous error
2020-02-26T12:50:12.9415154Z 
2020-02-26T12:50:12.9415271Z 
2020-02-26T12:50:12.9415672Z ------------------------------------------
2020-02-26T12:50:12.9415672Z ------------------------------------------
2020-02-26T12:50:12.9415851Z 
2020-02-26T12:50:12.9415951Z 
2020-02-26T12:50:12.9416362Z ---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
2020-02-26T12:50:12.9416609Z diff of stderr:
2020-02-26T12:50:12.9416737Z 
2020-02-26T12:50:12.9417151Z - error: symbol-name(_ZN5basic4main17h81759b0695851718E)
2020-02-26T12:50:12.9417688Z + error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-02-26T12:50:12.9418304Z 3    |
2020-02-26T12:50:12.9418503Z 4 LL | #[rustc_symbol_name]
2020-02-26T12:50:12.9418655Z 
2020-02-26T12:50:12.9418821Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9418821Z 5    | ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9418990Z 6 
2020-02-26T12:50:12.9419425Z - error: demangling(basic::main::h81759b0695851718)
2020-02-26T12:50:12.9419778Z + error: demangling(basic::main::h7bbff4a01206d8c2)
2020-02-26T12:50:12.9420414Z 9    |
2020-02-26T12:50:12.9420597Z 10 LL | #[rustc_symbol_name]
2020-02-26T12:50:12.9420750Z 
2020-02-26T12:50:12.9420851Z 
2020-02-26T12:50:12.9420851Z 
2020-02-26T12:50:12.9421079Z The actual stderr differed from the expected stderr.
2020-02-26T12:50:12.9421751Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
2020-02-26T12:50:12.9422393Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T12:50:12.9422991Z To only update this specific test, also pass `--test-args symbol-names/basic.rs`
2020-02-26T12:50:12.9423238Z 
2020-02-26T12:50:12.9423483Z error in revision `legacy`: 1 errors occurred comparing output.
2020-02-26T12:50:12.9423790Z status: exit code: 1
2020-02-26T12:50:12.9425857Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
2020-02-26T12:50:12.9427730Z ------------------------------------------
2020-02-26T12:50:12.9427912Z 
2020-02-26T12:50:12.9428292Z ------------------------------------------
2020-02-26T12:50:12.9428502Z stderr:
2020-02-26T12:50:12.9428502Z stderr:
2020-02-26T12:50:12.9428873Z ------------------------------------------
2020-02-26T12:50:12.9429373Z error: symbol-name(_ZN5basic4main17h7bbff4a01206d8c2E)
2020-02-26T12:50:12.9430118Z    |
2020-02-26T12:50:12.9430293Z LL | #[rustc_symbol_name]
2020-02-26T12:50:12.9430523Z    | ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9430668Z 
2020-02-26T12:50:12.9430668Z 
2020-02-26T12:50:12.9430892Z error: demangling(basic::main::h7bbff4a01206d8c2)
2020-02-26T12:50:12.9431648Z    |
2020-02-26T12:50:12.9431818Z LL | #[rustc_symbol_name]
2020-02-26T12:50:12.9432040Z    | ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9432186Z 
2020-02-26T12:50:12.9432186Z 
2020-02-26T12:50:12.9432544Z error: demangling-alt(basic::main)
2020-02-26T12:50:12.9433266Z    |
2020-02-26T12:50:12.9433437Z LL | #[rustc_symbol_name]
2020-02-26T12:50:12.9433643Z    | ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9433788Z 
---
2020-02-26T12:50:12.9442336Z 
2020-02-26T12:50:12.9442743Z ---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
2020-02-26T12:50:12.9442989Z diff of stderr:
2020-02-26T12:50:12.9443120Z 
2020-02-26T12:50:12.9443575Z - error: symbol-name(_ZN5impl13foo3Foo3bar17h92cf46db76791039E)
2020-02-26T12:50:12.9444136Z + error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-02-26T12:50:12.9445153Z 3    |
2020-02-26T12:50:12.9445351Z 4 LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9445515Z 
2020-02-26T12:50:12.9445696Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9445696Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9445909Z 6 
2020-02-26T12:50:12.9446371Z - error: demangling(impl1::foo::Foo::bar::h92cf46db76791039)
2020-02-26T12:50:12.9446755Z + error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-02-26T12:50:12.9447422Z 9    |
2020-02-26T12:50:12.9447619Z 10 LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9447801Z 
2020-02-26T12:50:12.9447980Z 22 LL |         #[rustc_def_path]
2020-02-26T12:50:12.9447980Z 22 LL |         #[rustc_def_path]
2020-02-26T12:50:12.9448216Z 23    |         ^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9448397Z 24 
2020-02-26T12:50:12.9448950Z - error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h90c4a800b1aa0df0E)
2020-02-26T12:50:12.9449835Z + error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-02-26T12:50:12.9450570Z 27    |
2020-02-26T12:50:12.9450766Z 28 LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9450934Z 
2020-02-26T12:50:12.9451134Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9451134Z 29    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9451329Z 30 
2020-02-26T12:50:12.9451850Z - error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h90c4a800b1aa0df0)
2020-02-26T12:50:12.9452340Z + error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-02-26T12:50:12.9454108Z 33    |
2020-02-26T12:50:12.9454328Z 34 LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9454496Z 
2020-02-26T12:50:12.9454676Z 46 LL |         #[rustc_def_path]
2020-02-26T12:50:12.9454676Z 46 LL |         #[rustc_def_path]
2020-02-26T12:50:12.9454914Z 47    |         ^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9455688Z 48 
2020-02-26T12:50:12.9456792Z - error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17SYMBOL_HASHE)
2020-02-26T12:50:12.9458295Z + error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-02-26T12:50:12.9470680Z 51    |
2020-02-26T12:50:12.9470924Z 52 LL |             #[rustc_symbol_name]
2020-02-26T12:50:12.9471100Z 
2020-02-26T12:50:12.9471293Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9471293Z 53    |             ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9471487Z 54 
2020-02-26T12:50:12.9472415Z - error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::SYMBOL_HASH)
2020-02-26T12:50:12.9473427Z + error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-02-26T12:50:12.9474490Z 57    |
2020-02-26T12:50:12.9474688Z 58 LL |             #[rustc_symbol_name]
2020-02-26T12:50:12.9475025Z 
2020-02-26T12:50:12.9475140Z 
2020-02-26T12:50:12.9475140Z 
2020-02-26T12:50:12.9475425Z The actual stderr differed from the expected stderr.
2020-02-26T12:50:12.9476153Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
2020-02-26T12:50:12.9476817Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T12:50:12.9477405Z To only update this specific test, also pass `--test-args symbol-names/impl1.rs`
2020-02-26T12:50:12.9477651Z 
2020-02-26T12:50:12.9477897Z error in revision `legacy`: 1 errors occurred comparing output.
2020-02-26T12:50:12.9478206Z status: exit code: 1
2020-02-26T12:50:12.9480281Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary"
2020-02-26T12:50:12.9482287Z ------------------------------------------
2020-02-26T12:50:12.9482470Z 
2020-02-26T12:50:12.9482860Z ------------------------------------------
2020-02-26T12:50:12.9483069Z stderr:
2020-02-26T12:50:12.9483069Z stderr:
2020-02-26T12:50:12.9483438Z ------------------------------------------
2020-02-26T12:50:12.9483963Z error: symbol-name(_ZN5impl13foo3Foo3bar17hf9d7d0e61617a4b8E)
2020-02-26T12:50:12.9484890Z    |
2020-02-26T12:50:12.9485097Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9485334Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9485495Z 
2020-02-26T12:50:12.9485495Z 
2020-02-26T12:50:12.9485765Z error: demangling(impl1::foo::Foo::bar::hf9d7d0e61617a4b8)
2020-02-26T12:50:12.9486638Z    |
2020-02-26T12:50:12.9486846Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9487227Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9487389Z 
2020-02-26T12:50:12.9487389Z 
2020-02-26T12:50:12.9487827Z error: demangling-alt(impl1::foo::Foo::bar)
2020-02-26T12:50:12.9488581Z    |
2020-02-26T12:50:12.9488767Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9489018Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9489177Z 
2020-02-26T12:50:12.9489177Z 
2020-02-26T12:50:12.9489523Z error: def-path(foo::Foo::bar)
2020-02-26T12:50:12.9490245Z    |
2020-02-26T12:50:12.9490427Z LL |         #[rustc_def_path]
2020-02-26T12:50:12.9490650Z    |         ^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9490817Z 
2020-02-26T12:50:12.9490817Z 
2020-02-26T12:50:12.9491350Z error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17h6435cd4293f0ad82E)
2020-02-26T12:50:12.9492179Z    |
2020-02-26T12:50:12.9492366Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9492600Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9492760Z 
2020-02-26T12:50:12.9492760Z 
2020-02-26T12:50:12.9493065Z error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::h6435cd4293f0ad82)
2020-02-26T12:50:12.9493874Z    |
2020-02-26T12:50:12.9494234Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9498923Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9499090Z 
2020-02-26T12:50:12.9499090Z 
2020-02-26T12:50:12.9499709Z error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
2020-02-26T12:50:12.9500505Z    |
2020-02-26T12:50:12.9500691Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9501104Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9501489Z 
2020-02-26T12:50:12.9501489Z 
2020-02-26T12:50:12.9502092Z error: def-path(bar::<impl foo::Foo>::baz)
2020-02-26T12:50:12.9502856Z    |
2020-02-26T12:50:12.9503037Z LL |         #[rustc_def_path]
2020-02-26T12:50:12.9503277Z    |         ^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9503433Z 
2020-02-26T12:50:12.9503433Z 
2020-02-26T12:50:12.9504423Z error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17hefcb557fc1aaf107E)
2020-02-26T12:50:12.9505703Z    |
2020-02-26T12:50:12.9505900Z LL |             #[rustc_symbol_name]
2020-02-26T12:50:12.9506373Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9506544Z 
2020-02-26T12:50:12.9506544Z 
2020-02-26T12:50:12.9507018Z error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::hefcb557fc1aaf107)
2020-02-26T12:50:12.9508095Z    |
2020-02-26T12:50:12.9508288Z LL |             #[rustc_symbol_name]
2020-02-26T12:50:12.9508538Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9508872Z 
2020-02-26T12:50:12.9508872Z 
2020-02-26T12:50:12.9509604Z error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
2020-02-26T12:50:12.9510573Z    |
2020-02-26T12:50:12.9510765Z LL |             #[rustc_symbol_name]
2020-02-26T12:50:12.9511016Z    |             ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9511183Z 
2020-02-26T12:50:12.9511183Z 
2020-02-26T12:50:12.9511851Z error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{{closure}}#1::Bar>::method)
2020-02-26T12:50:12.9512790Z    |
2020-02-26T12:50:12.9513140Z LL |             #[rustc_def_path]
2020-02-26T12:50:12.9513383Z    |             ^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9513544Z 
---
2020-02-26T12:50:12.9514727Z 
2020-02-26T12:50:12.9515141Z ---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
2020-02-26T12:50:12.9515398Z diff of stderr:
2020-02-26T12:50:12.9515527Z 
2020-02-26T12:50:12.9516252Z - error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hc86312d25b60f6eeE)
2020-02-26T12:50:12.9517004Z + error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-02-26T12:50:12.9517782Z 3    |
2020-02-26T12:50:12.9517976Z 4 LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9518257Z 
2020-02-26T12:50:12.9518463Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9518463Z 5    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9518646Z 6 
2020-02-26T12:50:12.9519222Z - error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hc86312d25b60f6ee)
2020-02-26T12:50:12.9519759Z + error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-02-26T12:50:12.9520510Z 9    |
2020-02-26T12:50:12.9520705Z 10 LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9520886Z 
2020-02-26T12:50:12.9520986Z 
2020-02-26T12:50:12.9520986Z 
2020-02-26T12:50:12.9521197Z The actual stderr differed from the expected stderr.
2020-02-26T12:50:12.9521914Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
2020-02-26T12:50:12.9522596Z To update references, rerun the tests and pass the `--bless` flag
2020-02-26T12:50:12.9523446Z To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`
2020-02-26T12:50:12.9523712Z 
2020-02-26T12:50:12.9523977Z error in revision `legacy`: 1 errors occurred comparing output.
2020-02-26T12:50:12.9524419Z status: exit code: 1
2020-02-26T12:50:12.9526626Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary"
2020-02-26T12:50:12.9528379Z ------------------------------------------
2020-02-26T12:50:12.9528562Z 
2020-02-26T12:50:12.9528932Z ------------------------------------------
2020-02-26T12:50:12.9529141Z stderr:
2020-02-26T12:50:12.9529141Z stderr:
2020-02-26T12:50:12.9529533Z ------------------------------------------
2020-02-26T12:50:12.9530157Z error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h91943b9c102410e4E)
2020-02-26T12:50:12.9531041Z    |
2020-02-26T12:50:12.9531228Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9531465Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9531642Z 
2020-02-26T12:50:12.9531642Z 
2020-02-26T12:50:12.9531955Z error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h91943b9c102410e4)
2020-02-26T12:50:12.9532842Z    |
2020-02-26T12:50:12.9533034Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9533272Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9533433Z 
2020-02-26T12:50:12.9533433Z 
2020-02-26T12:50:12.9533938Z error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
2020-02-26T12:50:12.9534777Z    |
2020-02-26T12:50:12.9534980Z LL |         #[rustc_symbol_name]
2020-02-26T12:50:12.9535217Z    |         ^^^^^^^^^^^^^^^^^^^^
2020-02-26T12:50:12.9535376Z 
---
2020-02-26T12:50:12.9536519Z 
2020-02-26T12:50:12.9536619Z 
2020-02-26T12:50:12.9536753Z failures:
2020-02-26T12:50:12.9536953Z     [ui] ui/codemap_tests/unicode.rs
2020-02-26T12:50:12.9537442Z     [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs
2020-02-26T12:50:12.9538315Z     [ui] ui/symbol-names/basic.rs#legacy
2020-02-26T12:50:12.9538764Z     [ui] ui/symbol-names/impl1.rs#legacy
2020-02-26T12:50:12.9539217Z     [ui] ui/symbol-names/issue-60925.rs#legacy
2020-02-26T12:50:12.9539405Z 
2020-02-26T12:50:12.9539405Z 
2020-02-26T12:50:12.9539918Z test result: FAILED. 9649 passed; 6 failed; 54 ignored; 0 measured; 0 filtered out
2020-02-26T12:50:12.9540213Z 
2020-02-26T12:50:12.9542458Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-26T12:50:12.9543058Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-26T12:50:12.9543440Z 
2020-02-26T12:50:12.9543762Z 
2020-02-26T12:50:12.9547933Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-26T12:50:12.9550751Z 
2020-02-26T12:50:12.9550861Z 
2020-02-26T12:50:12.9551334Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-26T12:50:12.9551809Z Build completed unsuccessfully in 1:10:56
2020-02-26T12:50:12.9551809Z Build completed unsuccessfully in 1:10:56
2020-02-26T12:50:12.9552182Z == clock drift check ==
2020-02-26T12:50:12.9552571Z   local time: Wed Feb 26 12:50:12 UTC 2020
2020-02-26T12:50:13.2369153Z   network time: Wed, 26 Feb 2020 12:50:13 GMT
2020-02-26T12:50:13.2373682Z == end clock drift check ==
2020-02-26T12:50:13.9063662Z 
2020-02-26T12:50:13.9136321Z ##[error]Bash exited with code '1'.
2020-02-26T12:50:13.9151172Z ##[section]Finishing: Run build
2020-02-26T12:50:13.9205493Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T12:50:13.9211109Z Task         : Get sources
2020-02-26T12:50:13.9211509Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T12:50:13.9211913Z Version      : 1.0.0
2020-02-26T12:50:13.9212190Z Author       : Microsoft
2020-02-26T12:50:13.9212190Z Author       : Microsoft
2020-02-26T12:50:13.9212602Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-26T12:50:13.9213101Z ==============================================================================
2020-02-26T12:50:14.2712405Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-26T12:50:14.2754380Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-02-26T12:50:14.2843520Z Cleaning up task key
2020-02-26T12:50:14.2844778Z Start cleaning up orphan processes.
2020-02-26T12:50:14.3019446Z Terminate orphan process: pid (3593) (python)
2020-02-26T12:50:14.3285462Z ##[section]Finishing: Finalize Job
