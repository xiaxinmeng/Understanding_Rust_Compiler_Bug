plain
2020-02-07T00:05:13.5741968Z ========================== Starting Command Output ===========================
2020-02-07T00:05:13.5788363Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/101e4981-abd7-466e-a2ed-a1eff63e09b1.sh
2020-02-07T00:05:13.5788401Z 
2020-02-07T00:05:13.5790781Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-07T00:05:13.5797037Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-07T00:05:13.5798426Z Task         : Get sources
2020-02-07T00:05:13.5798456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T00:05:13.5798491Z Version      : 1.0.0
2020-02-07T00:05:13.5798519Z Author       : Microsoft
---
2020-02-07T00:05:14.3791537Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-07T00:05:14.3894320Z ##[command]git config gc.auto 0
2020-02-07T00:05:14.3944054Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-07T00:05:14.3996719Z ##[command]git config --get-all http.proxy
2020-02-07T00:05:14.4139864Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68823/merge:refs/remotes/pull/68823/merge
---
2020-02-07T02:02:56.6855037Z test [ui] ui/fn_to_numeric_cast.rs ... ok
2020-02-07T02:02:56.8929334Z test [ui] ui/for_kv_map.rs ... ok
2020-02-07T02:02:57.4052868Z test [ui] ui/for_loop_over_option_result.rs ... ok
2020-02-07T02:02:57.5844737Z normalized stderr:
2020-02-07T02:02:57.5845326Z error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T02:02:57.5846469Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T02:02:57.5846542Z    = note: the following crate versions were found:
2020-02-07T02:02:57.5846542Z    = note: the following crate versions were found:
2020-02-07T02:02:57.5847394Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta
2020-02-07T02:02:57.5847767Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib
2020-02-07T02:02:57.5848133Z            crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib
2020-02-07T02:02:57.5848237Z error: aborting due to previous error
2020-02-07T02:02:57.5848286Z 
2020-02-07T02:02:57.5849534Z 
2020-02-07T02:02:57.5849734Z 
2020-02-07T02:02:57.5849734Z 
2020-02-07T02:02:57.5850665Z expected stderr:
2020-02-07T02:02:57.5853547Z error[E0425]: cannot find function `f` in this scope
2020-02-07T02:02:57.5855223Z   --> $DIR/for_loop_unfixable.rs:37:12
2020-02-07T02:02:57.5855333Z    |
2020-02-07T02:02:57.5855402Z LL |         if f(&vec[i], &vec[i]) {
2020-02-07T02:02:57.5856156Z 
2020-02-07T02:02:57.5856576Z error: aborting due to previous error
2020-02-07T02:02:57.5856633Z 
2020-02-07T02:02:57.5857013Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:02:57.5857013Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:02:57.5857049Z 
2020-02-07T02:02:57.5857075Z 
2020-02-07T02:02:57.5862075Z diff of stderr:
2020-02-07T02:02:57.5863062Z 
2020-02-07T02:02:57.5863670Z -error[E0425]: cannot find function `f` in this scope
2020-02-07T02:02:57.5868903Z -  --> $DIR/for_loop_unfixable.rs:37:12
2020-02-07T02:02:57.5869060Z +error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T02:02:57.5875007Z     |
2020-02-07T02:02:57.5876427Z -LL |         if f(&vec[i], &vec[i]) {
2020-02-07T02:02:57.5881727Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T02:02:57.5900816Z +   = note: the following crate versions were found:
2020-02-07T02:02:57.5900816Z +   = note: the following crate versions were found:
2020-02-07T02:02:57.5903434Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta
2020-02-07T02:02:57.5903890Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib
2020-02-07T02:02:57.5904247Z +           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib
2020-02-07T02:02:57.5904367Z  error: aborting due to previous error
2020-02-07T02:02:57.5904410Z  
2020-02-07T02:02:57.5904662Z -For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:02:57.5904727Z  
---
2020-02-07T02:02:57.5905627Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base' 'for_loop_unfixable.rs'
2020-02-07T02:02:57.5905673Z 
2020-02-07T02:02:57.5906009Z error: 1 errors occurred comparing output.
2020-02-07T02:02:57.5906068Z status: exit code: 1
2020-02-07T02:02:57.5907813Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/for_loop_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/for_loop_unfixable.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-93be375d9bac0399.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-434bedc32a67cf7f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/for_loop_unfixable.stage-id.aux" "-A" "unused"
2020-02-07T02:02:57.5908373Z ------------------------------------------
2020-02-07T02:02:57.5908425Z 
2020-02-07T02:02:57.5908642Z ------------------------------------------
2020-02-07T02:02:57.5908688Z stderr:
2020-02-07T02:02:57.5908688Z stderr:
2020-02-07T02:02:57.5908894Z ------------------------------------------
2020-02-07T02:02:57.5911321Z {"message":"found possibly newer version of crate `if_chain` which `clippy_lints` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Tests from for_loop.rs that don't have suggestions","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib\ncrate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib\n           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib\n\n"}
2020-02-07T02:02:57.5911746Z 
2020-02-07T02:02:57.5911994Z ------------------------------------------
2020-02-07T02:02:57.5912028Z 
2020-02-07T02:02:57.5912152Z test [ui] ui/for_loop_unfixable.rs ... FAILED
---
2020-02-07T02:03:45.2802104Z test [ui] ui/option_and_then_some.rs ... ok
2020-02-07T02:03:45.6965025Z test [ui] ui/option_as_ref_deref.rs ... ok
2020-02-07T02:03:46.3925812Z test [ui] ui/option_map_or_none.rs ... ok
2020-02-07T02:03:46.5891150Z normalized stderr:
2020-02-07T02:03:46.5892631Z error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T02:03:46.5893304Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T02:03:46.5893489Z    = note: the following crate versions were found:
2020-02-07T02:03:46.5893489Z    = note: the following crate versions were found:
2020-02-07T02:03:46.5894358Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta
2020-02-07T02:03:46.5894922Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib
2020-02-07T02:03:46.5895393Z            crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib
2020-02-07T02:03:46.5895704Z error: aborting due to previous error
2020-02-07T02:03:46.5895813Z 
2020-02-07T02:03:46.5895920Z 
2020-02-07T02:03:46.5896046Z 
2020-02-07T02:03:46.5896046Z 
2020-02-07T02:03:46.5896170Z expected stderr:
2020-02-07T02:03:46.5896301Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5896664Z   --> $DIR/option_map_unit_fn_unfixable.rs:17:5
2020-02-07T02:03:46.5896834Z    |
2020-02-07T02:03:46.5896980Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T02:03:46.5897307Z 
2020-02-07T02:03:46.5897444Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5897833Z   --> $DIR/option_map_unit_fn_unfixable.rs:19:5
2020-02-07T02:03:46.5898089Z    |
2020-02-07T02:03:46.5898089Z    |
2020-02-07T02:03:46.5898237Z LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T02:03:46.5898515Z 
2020-02-07T02:03:46.5898652Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5899038Z   --> $DIR/option_map_unit_fn_unfixable.rs:23:5
2020-02-07T02:03:46.5899211Z    |
2020-02-07T02:03:46.5899211Z    |
2020-02-07T02:03:46.5899342Z LL |     x.field.map(|value| {
2020-02-07T02:03:46.5899615Z 
2020-02-07T02:03:46.5899750Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5900124Z   --> $DIR/option_map_unit_fn_unfixable.rs:27:5
2020-02-07T02:03:46.5900300Z    |
2020-02-07T02:03:46.5900300Z    |
2020-02-07T02:03:46.5900439Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T02:03:46.5901570Z 
2020-02-07T02:03:46.5901759Z error: aborting due to 4 previous errors
2020-02-07T02:03:46.5901896Z 
2020-02-07T02:03:46.5902338Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:03:46.5902338Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:03:46.5902497Z 
2020-02-07T02:03:46.5902610Z 
2020-02-07T02:03:46.5902768Z diff of stderr:
2020-02-07T02:03:46.5902880Z 
2020-02-07T02:03:46.5903231Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5903625Z -  --> $DIR/option_map_unit_fn_unfixable.rs:17:5
2020-02-07T02:03:46.5903801Z +error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T02:03:46.5903938Z     |
2020-02-07T02:03:46.5904505Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T02:03:46.5905121Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T02:03:46.5905445Z +   = note: the following crate versions were found:
2020-02-07T02:03:46.5905445Z +   = note: the following crate versions were found:
2020-02-07T02:03:46.5905924Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta
2020-02-07T02:03:46.5906450Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib
2020-02-07T02:03:46.5906929Z +           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib
2020-02-07T02:03:46.5907467Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5907830Z -  --> $DIR/option_map_unit_fn_unfixable.rs:19:5
2020-02-07T02:03:46.5908176Z -   |
2020-02-07T02:03:46.5908176Z -   |
2020-02-07T02:03:46.5908585Z -LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T02:03:46.5909141Z +error: aborting due to previous error
2020-02-07T02:03:46.5909295Z  
2020-02-07T02:03:46.5909637Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5910026Z -  --> $DIR/option_map_unit_fn_unfixable.rs:23:5
2020-02-07T02:03:46.5910026Z -  --> $DIR/option_map_unit_fn_unfixable.rs:23:5
2020-02-07T02:03:46.5910349Z -   |
2020-02-07T02:03:46.5910694Z -LL |     x.field.map(|value| {
2020-02-07T02:03:46.5911377Z -
2020-02-07T02:03:46.5911737Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:03:46.5912122Z -  --> $DIR/option_map_unit_fn_unfixable.rs:27:5
2020-02-07T02:03:46.5912440Z -   |
2020-02-07T02:03:46.5912440Z -   |
2020-02-07T02:03:46.5912839Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T02:03:46.5913928Z -
2020-02-07T02:03:46.5914614Z -error: aborting due to 4 previous errors
2020-02-07T02:03:46.5914853Z -
2020-02-07T02:03:46.5915081Z -For more information about this error, try `rustc --explain E0425`.
---
2020-02-07T02:03:46.5918477Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base' 'option_map_unit_fn_unfixable.rs'
2020-02-07T02:03:46.5918522Z 
2020-02-07T02:03:46.5918566Z error: 1 errors occurred comparing output.
2020-02-07T02:03:46.5918642Z status: exit code: 1
2020-02-07T02:03:46.5920350Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/option_map_unit_fn_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/option_map_unit_fn_unfixable.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-93be375d9bac0399.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-434bedc32a67cf7f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/option_map_unit_fn_unfixable.stage-id.aux" "-A" "unused"
2020-02-07T02:03:46.5920904Z ------------------------------------------
2020-02-07T02:03:46.5920938Z 
2020-02-07T02:03:46.5921147Z ------------------------------------------
2020-02-07T02:03:46.5921211Z stderr:
2020-02-07T02:03:46.5921211Z stderr:
2020-02-07T02:03:46.5921414Z ------------------------------------------
2020-02-07T02:03:46.5923447Z {"message":"found possibly newer version of crate `if_chain` which `clippy_lints` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_map_unit_fn_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(clippy::option_map_unit_fn)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib\ncrate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib\n           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib\n\n"}
2020-02-07T02:03:46.5923844Z 
2020-02-07T02:03:46.5924066Z ------------------------------------------
2020-02-07T02:03:46.5924097Z 
2020-02-07T02:03:46.5924166Z test [ui] ui/option_map_unit_fn_unfixable.rs ... FAILED
---
2020-02-07T02:03:57.0362467Z test [ui] ui/redundant_field_names.rs ... ok
2020-02-07T02:03:57.7281271Z test [ui] ui/redundant_pattern_matching.rs ... ok
2020-02-07T02:03:57.9841192Z test [ui] ui/redundant_static_lifetimes.rs ... ok
2020-02-07T02:03:58.0523497Z normalized stderr:
2020-02-07T02:03:58.0523765Z error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T02:03:58.0525369Z    |
2020-02-07T02:03:58.0525535Z LL | extern crate regex;
2020-02-07T02:03:58.0525973Z    | ^^^^^^^^^^^^^^^^^^^
2020-02-07T02:03:58.0526216Z    |
2020-02-07T02:03:58.0526216Z    |
2020-02-07T02:03:58.0526291Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T02:03:58.0526383Z    = note: the following crate versions were found:
2020-02-07T02:03:58.0527182Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta
2020-02-07T02:03:58.0527649Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib
2020-02-07T02:03:58.0528003Z            crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib
2020-02-07T02:03:58.0528125Z error: aborting due to previous error
2020-02-07T02:03:58.0528156Z 
2020-02-07T02:03:58.0528608Z 
2020-02-07T02:03:58.0528746Z 
2020-02-07T02:03:58.0528746Z 
2020-02-07T02:03:58.0531574Z expected stderr:
2020-02-07T02:03:58.0539179Z error: trivial regex
2020-02-07T02:03:58.0540441Z   --> $DIR/regex.rs:13:45
2020-02-07T02:03:58.0540599Z    |
2020-02-07T02:03:58.0540644Z LL |     let pipe_in_wrong_position = Regex::new("|");
2020-02-07T02:03:58.0540733Z    |
2020-02-07T02:03:58.0540994Z    = note: `-D clippy::trivial-regex` implied by `-D warnings`
2020-02-07T02:03:58.0541045Z    = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0541074Z 
2020-02-07T02:03:58.0541074Z 
2020-02-07T02:03:58.0541145Z error: trivial regex
2020-02-07T02:03:58.0541336Z   --> $DIR/regex.rs:14:60
2020-02-07T02:03:58.0541377Z    |
2020-02-07T02:03:58.0541421Z LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2020-02-07T02:03:58.0541540Z    |
2020-02-07T02:03:58.0541581Z    = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0541627Z 
2020-02-07T02:03:58.0541627Z 
2020-02-07T02:03:58.0541673Z error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T02:03:58.0541922Z    |
2020-02-07T02:03:58.0541922Z    |
2020-02-07T02:03:58.0542131Z LL |     let wrong_char_ranice = Regex::new("[z-a]");
2020-02-07T02:03:58.0542218Z    |
2020-02-07T02:03:58.0542452Z    = note: `-D clippy::invalid-regex` implied by `-D warnings`
2020-02-07T02:03:58.0542483Z 
2020-02-07T02:03:58.0542483Z 
2020-02-07T02:03:58.0542697Z error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T02:03:58.0542989Z    |
2020-02-07T02:03:58.0542989Z    |
2020-02-07T02:03:58.0543318Z LL |     let some_unicode = Regex::new("[é-è]");
2020-02-07T02:03:58.0543410Z 
2020-02-07T02:03:58.0543452Z error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0543637Z   --> $DIR/regex.rs:18:33
2020-02-07T02:03:58.0543693Z    |
2020-02-07T02:03:58.0543693Z    |
2020-02-07T02:03:58.0543736Z LL |     let some_regex = Regex::new(OPENING_PAREN);
2020-02-07T02:03:58.0543780Z    |                                 ^^^^^^^^^^^^^
2020-02-07T02:03:58.0543807Z 
2020-02-07T02:03:58.0543862Z error: trivial regex
2020-02-07T02:03:58.0544047Z   --> $DIR/regex.rs:20:53
2020-02-07T02:03:58.0544086Z    |
2020-02-07T02:03:58.0544144Z LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2020-02-07T02:03:58.0544240Z    |
2020-02-07T02:03:58.0544283Z    = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0544327Z 
2020-02-07T02:03:58.0544377Z error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0544377Z error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0544566Z   --> $DIR/regex.rs:21:41
2020-02-07T02:03:58.0544623Z    |
2020-02-07T02:03:58.0544665Z LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2020-02-07T02:03:58.0544737Z 
2020-02-07T02:03:58.0544796Z error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0544981Z   --> $DIR/regex.rs:22:56
2020-02-07T02:03:58.0545021Z    |
2020-02-07T02:03:58.0545021Z    |
2020-02-07T02:03:58.0545081Z LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2020-02-07T02:03:58.0545161Z 
2020-02-07T02:03:58.0545202Z error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0545414Z   --> $DIR/regex.rs:34:37
2020-02-07T02:03:58.0545454Z    |
2020-02-07T02:03:58.0545454Z    |
2020-02-07T02:03:58.0545692Z LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T02:03:58.0545794Z 
2020-02-07T02:03:58.0545835Z error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0546019Z   --> $DIR/regex.rs:35:39
2020-02-07T02:03:58.0546074Z    |
2020-02-07T02:03:58.0546074Z    |
2020-02-07T02:03:58.0546310Z LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T02:03:58.0546403Z 
2020-02-07T02:03:58.0546444Z error: regex syntax error: unrecognized escape sequence
2020-02-07T02:03:58.0546627Z   --> $DIR/regex.rs:37:45
2020-02-07T02:03:58.0546667Z    |
2020-02-07T02:03:58.0546667Z    |
2020-02-07T02:03:58.0546725Z LL |     let raw_string_error = Regex::new(r"[...//...]");
2020-02-07T02:03:58.0546805Z 
2020-02-07T02:03:58.0546864Z error: regex syntax error: unrecognized escape sequence
2020-02-07T02:03:58.0547059Z   --> $DIR/regex.rs:38:46
2020-02-07T02:03:58.0547099Z    |
2020-02-07T02:03:58.0547099Z    |
2020-02-07T02:03:58.0547142Z LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2020-02-07T02:03:58.0547229Z 
2020-02-07T02:03:58.0547266Z error: trivial regex
2020-02-07T02:03:58.0547469Z   --> $DIR/regex.rs:42:33
2020-02-07T02:03:58.0547509Z    |
2020-02-07T02:03:58.0547509Z    |
2020-02-07T02:03:58.0547549Z LL |     let trivial_eq = Regex::new("^foobar$");
2020-02-07T02:03:58.0547649Z    |
2020-02-07T02:03:58.0547689Z    = help: consider using `==` on `str`s
2020-02-07T02:03:58.0547716Z 
2020-02-07T02:03:58.0547771Z error: trivial regex
2020-02-07T02:03:58.0547771Z error: trivial regex
2020-02-07T02:03:58.0547955Z   --> $DIR/regex.rs:44:48
2020-02-07T02:03:58.0547995Z    |
2020-02-07T02:03:58.0548110Z LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2020-02-07T02:03:58.0548273Z    |
2020-02-07T02:03:58.0548313Z    = help: consider using `==` on `str`s
2020-02-07T02:03:58.0548357Z 
2020-02-07T02:03:58.0548395Z error: trivial regex
2020-02-07T02:03:58.0548395Z error: trivial regex
2020-02-07T02:03:58.0548602Z   --> $DIR/regex.rs:46:42
2020-02-07T02:03:58.0548642Z    |
2020-02-07T02:03:58.0548699Z LL |     let trivial_starts_with = Regex::new("^foobar");
2020-02-07T02:03:58.0548783Z    |
2020-02-07T02:03:58.0548842Z    = help: consider using `str::starts_with`
2020-02-07T02:03:58.0548869Z 
2020-02-07T02:03:58.0548906Z error: trivial regex
2020-02-07T02:03:58.0548906Z error: trivial regex
2020-02-07T02:03:58.0549090Z   --> $DIR/regex.rs:48:40
2020-02-07T02:03:58.0549146Z    |
2020-02-07T02:03:58.0549187Z LL |     let trivial_ends_with = Regex::new("foobar$");
2020-02-07T02:03:58.0549296Z    |
2020-02-07T02:03:58.0549337Z    = help: consider using `str::ends_with`
2020-02-07T02:03:58.0549370Z 
2020-02-07T02:03:58.0549408Z error: trivial regex
---
2020-02-07T02:03:58.0549863Z 
2020-02-07T02:03:58.0549962Z error: trivial regex
2020-02-07T02:03:58.0550189Z   --> $DIR/regex.rs:52:39
2020-02-07T02:03:58.0550235Z    |
2020-02-07T02:03:58.0550297Z LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2020-02-07T02:03:58.0550391Z    |
2020-02-07T02:03:58.0550462Z    = help: consider using `str::contains`
2020-02-07T02:03:58.0550491Z 
2020-02-07T02:03:58.0550533Z error: trivial regex
2020-02-07T02:03:58.0550533Z error: trivial regex
2020-02-07T02:03:58.0550744Z   --> $DIR/regex.rs:54:40
2020-02-07T02:03:58.0550814Z    |
2020-02-07T02:03:58.0550858Z LL |     let trivial_backslash = Regex::new("a/.b");
2020-02-07T02:03:58.0550968Z    |
2020-02-07T02:03:58.0551012Z    = help: consider using `str::contains`
2020-02-07T02:03:58.0551040Z 
2020-02-07T02:03:58.0551080Z error: trivial regex
---
2020-02-07T02:03:58.0551573Z 
2020-02-07T02:03:58.0551631Z error: trivial regex
2020-02-07T02:03:58.0554698Z   --> $DIR/regex.rs:59:36
2020-02-07T02:03:58.0555018Z    |
2020-02-07T02:03:58.0555492Z LL |     let trivial_empty = Regex::new("^");
2020-02-07T02:03:58.0555614Z    |
2020-02-07T02:03:58.0555679Z    = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0555713Z 
2020-02-07T02:03:58.0555755Z error: trivial regex
2020-02-07T02:03:58.0555755Z error: trivial regex
2020-02-07T02:03:58.0556069Z   --> $DIR/regex.rs:61:36
2020-02-07T02:03:58.0556136Z    |
2020-02-07T02:03:58.0556181Z LL |     let trivial_empty = Regex::new("^$");
2020-02-07T02:03:58.0556290Z    |
2020-02-07T02:03:58.0556334Z    = help: consider using `str::is_empty`
2020-02-07T02:03:58.0556365Z 
2020-02-07T02:03:58.0556411Z error: trivial regex
2020-02-07T02:03:58.0556411Z error: trivial regex
2020-02-07T02:03:58.0556661Z   --> $DIR/regex.rs:63:44
2020-02-07T02:03:58.0556710Z    |
2020-02-07T02:03:58.0556760Z LL |     let binary_trivial_empty = BRegex::new("^$");
2020-02-07T02:03:58.0557052Z    |
2020-02-07T02:03:58.0557101Z    = help: consider using `str::is_empty`
2020-02-07T02:03:58.0557227Z 
2020-02-07T02:03:58.0557293Z error: aborting due to 23 previous errors
2020-02-07T02:03:58.0557293Z error: aborting due to 23 previous errors
2020-02-07T02:03:58.0557326Z 
2020-02-07T02:03:58.0557356Z 
2020-02-07T02:03:58.0557385Z 
2020-02-07T02:03:58.0561216Z diff of stderr:
2020-02-07T02:03:58.0561315Z 
2020-02-07T02:03:58.0566438Z -error: trivial regex
2020-02-07T02:03:58.0567773Z -  --> $DIR/regex.rs:13:45
2020-02-07T02:03:58.0568880Z +error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T02:03:58.0571179Z     |
2020-02-07T02:03:58.0571179Z     |
2020-02-07T02:03:58.0571592Z -LL |     let pipe_in_wrong_position = Regex::new("|");
2020-02-07T02:03:58.0572327Z +LL | extern crate regex;
2020-02-07T02:03:58.0572488Z +   | ^^^^^^^^^^^^^^^^^^^
2020-02-07T02:03:58.0572646Z     |
2020-02-07T02:03:58.0573013Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2020-02-07T02:03:58.0573013Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2020-02-07T02:03:58.0573607Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0573820Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T02:03:58.0573971Z +   = note: the following crate versions were found:
2020-02-07T02:03:58.0574499Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta
2020-02-07T02:03:58.0575236Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib
2020-02-07T02:03:58.0576061Z +           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib
2020-02-07T02:03:58.0576708Z -error: trivial regex
2020-02-07T02:03:58.0585314Z -  --> $DIR/regex.rs:14:60
2020-02-07T02:03:58.0585859Z -   |
2020-02-07T02:03:58.0585859Z -   |
2020-02-07T02:03:58.0586395Z -LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2020-02-07T02:03:58.0587365Z -   |
2020-02-07T02:03:58.0587865Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0588302Z -
2020-02-07T02:03:58.0588302Z -
2020-02-07T02:03:58.0590896Z -error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T02:03:58.0593886Z -   |
2020-02-07T02:03:58.0593886Z -   |
2020-02-07T02:03:58.0594151Z -LL |     let wrong_char_ranice = Regex::new("[z-a]");
2020-02-07T02:03:58.0594544Z -   |
2020-02-07T02:03:58.0594785Z -   = note: `-D clippy::invalid-regex` implied by `-D warnings`
2020-02-07T02:03:58.0594957Z -
2020-02-07T02:03:58.0594957Z -
2020-02-07T02:03:58.0595220Z -error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T02:03:58.0595613Z -   |
2020-02-07T02:03:58.0595613Z -   |
2020-02-07T02:03:58.0595830Z -LL |     let some_unicode = Regex::new("[é-è]");
2020-02-07T02:03:58.0596418Z -
2020-02-07T02:03:58.0596642Z -error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0596840Z -  --> $DIR/regex.rs:18:33
2020-02-07T02:03:58.0597035Z -   |
2020-02-07T02:03:58.0597035Z -   |
2020-02-07T02:03:58.0597257Z -LL |     let some_regex = Regex::new(OPENING_PAREN);
2020-02-07T02:03:58.0597482Z -   |                                 ^^^^^^^^^^^^^
2020-02-07T02:03:58.0597672Z -
2020-02-07T02:03:58.0597861Z -error: trivial regex
2020-02-07T02:03:58.0598056Z -  --> $DIR/regex.rs:20:53
2020-02-07T02:03:58.0598420Z -   |
2020-02-07T02:03:58.0598662Z -LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2020-02-07T02:03:58.0599257Z -   |
2020-02-07T02:03:58.0599980Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0600158Z -
2020-02-07T02:03:58.0600560Z -error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0600560Z -error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0600786Z -  --> $DIR/regex.rs:21:41
2020-02-07T02:03:58.0600969Z -   |
2020-02-07T02:03:58.0601205Z -LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2020-02-07T02:03:58.0601798Z -
2020-02-07T02:03:58.0602023Z -error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0602393Z -  --> $DIR/regex.rs:22:56
2020-02-07T02:03:58.0602564Z -   |
2020-02-07T02:03:58.0602564Z -   |
2020-02-07T02:03:58.0602799Z -LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2020-02-07T02:03:58.0603479Z -
2020-02-07T02:03:58.0603702Z -error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0603910Z -  --> $DIR/regex.rs:34:37
2020-02-07T02:03:58.0604107Z -   |
2020-02-07T02:03:58.0604107Z -   |
2020-02-07T02:03:58.0604362Z -LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T02:03:58.0604790Z -
2020-02-07T02:03:58.0605015Z -error: regex syntax error on position 0: unclosed group
2020-02-07T02:03:58.0605212Z -  --> $DIR/regex.rs:35:39
2020-02-07T02:03:58.0605386Z -   |
2020-02-07T02:03:58.0605386Z -   |
2020-02-07T02:03:58.0605659Z -LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T02:03:58.0606368Z -
2020-02-07T02:03:58.0606593Z -error: regex syntax error: unrecognized escape sequence
2020-02-07T02:03:58.0606777Z -  --> $DIR/regex.rs:37:45
2020-02-07T02:03:58.0606940Z -   |
2020-02-07T02:03:58.0606940Z -   |
2020-02-07T02:03:58.0607173Z -LL |     let raw_string_error = Regex::new(r"[...//...]");
2020-02-07T02:03:58.0607556Z -
2020-02-07T02:03:58.0607780Z -error: regex syntax error: unrecognized escape sequence
2020-02-07T02:03:58.0607978Z -  --> $DIR/regex.rs:38:46
2020-02-07T02:03:58.0608140Z -   |
2020-02-07T02:03:58.0608140Z -   |
2020-02-07T02:03:58.0608354Z -LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2020-02-07T02:03:58.0608749Z -
2020-02-07T02:03:58.0608926Z -error: trivial regex
2020-02-07T02:03:58.0609128Z -  --> $DIR/regex.rs:42:33
2020-02-07T02:03:58.0609291Z -   |
2020-02-07T02:03:58.0609291Z -   |
2020-02-07T02:03:58.0609493Z -LL |     let trivial_eq = Regex::new("^foobar$");
2020-02-07T02:03:58.0610092Z -   |
2020-02-07T02:03:58.0610344Z -   = help: consider using `==` on `str`s
2020-02-07T02:03:58.0610507Z -
2020-02-07T02:03:58.0610712Z -error: trivial regex
2020-02-07T02:03:58.0610712Z -error: trivial regex
2020-02-07T02:03:58.0610894Z -  --> $DIR/regex.rs:44:48
2020-02-07T02:03:58.0611067Z -   |
2020-02-07T02:03:58.0611306Z -LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2020-02-07T02:03:58.0611703Z -   |
2020-02-07T02:03:58.0611916Z -   = help: consider using `==` on `str`s
2020-02-07T02:03:58.0612080Z -
2020-02-07T02:03:58.0612254Z -error: trivial regex
2020-02-07T02:03:58.0612254Z -error: trivial regex
2020-02-07T02:03:58.0612438Z -  --> $DIR/regex.rs:46:42
2020-02-07T02:03:58.0612620Z -   |
2020-02-07T02:03:58.0612832Z -LL |     let trivial_starts_with = Regex::new("^foobar");
2020-02-07T02:03:58.0613224Z -   |
2020-02-07T02:03:58.0613424Z -   = help: consider using `str::starts_with`
2020-02-07T02:03:58.0613584Z -
2020-02-07T02:03:58.0616785Z -error: trivial regex
2020-02-07T02:03:58.0616785Z -error: trivial regex
2020-02-07T02:03:58.0617011Z -  --> $DIR/regex.rs:48:40
2020-02-07T02:03:58.0617521Z -   |
2020-02-07T02:03:58.0618222Z -LL |     let trivial_ends_with = Regex::new("foobar$");
2020-02-07T02:03:58.0618704Z -   |
2020-02-07T02:03:58.0619018Z -   = help: consider using `str::ends_with`
2020-02-07T02:03:58.0619213Z -
2020-02-07T02:03:58.0619403Z -error: trivial regex
---
2020-02-07T02:03:58.0621112Z -
2020-02-07T02:03:58.0621290Z -error: trivial regex
2020-02-07T02:03:58.0621491Z -  --> $DIR/regex.rs:52:39
2020-02-07T02:03:58.0621653Z -   |
2020-02-07T02:03:58.0621867Z -LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2020-02-07T02:03:58.0622280Z -   |
2020-02-07T02:03:58.0622477Z -   = help: consider using `str::contains`
2020-02-07T02:03:58.0622638Z -
2020-02-07T02:03:58.0622831Z -error: trivial regex
2020-02-07T02:03:58.0622831Z -error: trivial regex
2020-02-07T02:03:58.0623024Z -  --> $DIR/regex.rs:54:40
2020-02-07T02:03:58.0623187Z -   |
2020-02-07T02:03:58.0623408Z -LL |     let trivial_backslash = Regex::new("a/.b");
2020-02-07T02:03:58.0623782Z -   |
2020-02-07T02:03:58.0623993Z -   = help: consider using `str::contains`
2020-02-07T02:03:58.0624155Z -
2020-02-07T02:03:58.0624507Z -error: trivial regex
---
2020-02-07T02:03:58.0626233Z -
2020-02-07T02:03:58.0626618Z -error: trivial regex
2020-02-07T02:03:58.0626963Z -  --> $DIR/regex.rs:59:36
2020-02-07T02:03:58.0627142Z -   |
2020-02-07T02:03:58.0627356Z -LL |     let trivial_empty = Regex::new("^");
2020-02-07T02:03:58.0627781Z -   |
2020-02-07T02:03:58.0628161Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T02:03:58.0628344Z -
2020-02-07T02:03:58.0628529Z -error: trivial regex
2020-02-07T02:03:58.0628529Z -error: trivial regex
2020-02-07T02:03:58.0628719Z -  --> $DIR/regex.rs:61:36
2020-02-07T02:03:58.0628890Z -   |
2020-02-07T02:03:58.0629116Z -LL |     let trivial_empty = Regex::new("^$");
2020-02-07T02:03:58.0629492Z -   |
2020-02-07T02:03:58.0629867Z -   = help: consider using `str::is_empty`
2020-02-07T02:03:58.0630027Z -
2020-02-07T02:03:58.0630201Z -error: trivial regex
2020-02-07T02:03:58.0630201Z -error: trivial regex
2020-02-07T02:03:58.0630399Z -  --> $DIR/regex.rs:63:44
2020-02-07T02:03:58.0630565Z -   |
2020-02-07T02:03:58.0630781Z -LL |     let binary_trivial_empty = BRegex::new("^$");
2020-02-07T02:03:58.0631184Z -   |
2020-02-07T02:03:58.0631379Z -   = help: consider using `str::is_empty`
2020-02-07T02:03:58.0631538Z -
2020-02-07T02:03:58.0631819Z -error: aborting due to 23 previous errors
---
2020-02-07T02:03:58.0632893Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base' 'regex.rs'
2020-02-07T02:03:58.0632961Z 
2020-02-07T02:03:58.0633002Z error: 1 errors occurred comparing output.
2020-02-07T02:03:58.0633100Z status: exit code: 1
2020-02-07T02:03:58.0634923Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/regex.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/regex.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-93be375d9bac0399.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-434bedc32a67cf7f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/regex.stage-id.aux" "-A" "unused"
2020-02-07T02:03:58.0635328Z ------------------------------------------
2020-02-07T02:03:58.0635378Z 
2020-02-07T02:03:58.0635583Z ------------------------------------------
2020-02-07T02:03:58.0635625Z stderr:
2020-02-07T02:03:58.0635625Z stderr:
2020-02-07T02:03:58.0636021Z ------------------------------------------
2020-02-07T02:03:58.0638207Z {"message":"found possibly newer version of crate `aho_corasick` which `regex` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/regex.rs","byte_start":95,"byte_end":114,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate regex;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib\ncrate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on\n  --> tests/ui/regex.rs:4:1\n   |\nLL | extern crate regex;\n   | ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib\n           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib\n\n"}
2020-02-07T02:03:58.0638716Z 
2020-02-07T02:03:58.0638976Z ------------------------------------------
2020-02-07T02:03:58.0639006Z 
2020-02-07T02:03:58.0639127Z test [ui] ui/regex.rs ... FAILED
2020-02-07T02:03:58.0639127Z test [ui] ui/regex.rs ... FAILED
2020-02-07T02:03:58.1322748Z test [ui] ui/redundant_static_lifetimes_multiple.rs ... ok
2020-02-07T02:03:59.1535429Z test [ui] ui/rename.rs ... ok
2020-02-07T02:03:59.2480600Z test [ui] ui/renamed_builtin_attr.rs ... ok
2020-02-07T02:03:59.6687303Z test [ui] ui/repl_uninit.rs ... ok
2020-02-07T02:04:00.4206511Z test [ui] ui/replace_consts.rs ... ok
2020-02-07T02:04:00.6004929Z normalized stderr:
2020-02-07T02:04:00.6010567Z error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T02:04:00.6010705Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T02:04:00.6010750Z    = note: the following crate versions were found:
2020-02-07T02:04:00.6010750Z    = note: the following crate versions were found:
2020-02-07T02:04:00.6011588Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta
2020-02-07T02:04:00.6011961Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib
2020-02-07T02:04:00.6012280Z            crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib
2020-02-07T02:04:00.6012377Z error: aborting due to previous error
2020-02-07T02:04:00.6012404Z 
2020-02-07T02:04:00.6016909Z 
2020-02-07T02:04:00.6016970Z 
2020-02-07T02:04:00.6016970Z 
2020-02-07T02:04:00.6020682Z expected stderr:
2020-02-07T02:04:00.6028256Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6028707Z   --> $DIR/result_map_unit_fn_unfixable.rs:17:5
2020-02-07T02:04:00.6028757Z    |
2020-02-07T02:04:00.6028805Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T02:04:00.6028928Z 
2020-02-07T02:04:00.6028978Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6029398Z   --> $DIR/result_map_unit_fn_unfixable.rs:19:5
2020-02-07T02:04:00.6029457Z    |
2020-02-07T02:04:00.6029457Z    |
2020-02-07T02:04:00.6029503Z LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T02:04:00.6029575Z 
2020-02-07T02:04:00.6029636Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6029852Z   --> $DIR/result_map_unit_fn_unfixable.rs:23:5
2020-02-07T02:04:00.6029895Z    |
2020-02-07T02:04:00.6029895Z    |
2020-02-07T02:04:00.6029950Z LL |     x.field.map(|value| {
2020-02-07T02:04:00.6030018Z 
2020-02-07T02:04:00.6030059Z error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6030446Z   --> $DIR/result_map_unit_fn_unfixable.rs:27:5
2020-02-07T02:04:00.6030486Z    |
2020-02-07T02:04:00.6030486Z    |
2020-02-07T02:04:00.6030537Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T02:04:00.6030630Z 
2020-02-07T02:04:00.6030668Z error: aborting due to 4 previous errors
2020-02-07T02:04:00.6030693Z 
2020-02-07T02:04:00.6030931Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:04:00.6030931Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T02:04:00.6036091Z 
2020-02-07T02:04:00.6036156Z 
2020-02-07T02:04:00.6063854Z diff of stderr:
2020-02-07T02:04:00.6063919Z 
2020-02-07T02:04:00.6064315Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6064533Z -  --> $DIR/result_map_unit_fn_unfixable.rs:17:5
2020-02-07T02:04:00.6064601Z +error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T02:04:00.6064645Z     |
2020-02-07T02:04:00.6064866Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T02:04:00.6065322Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T02:04:00.6065381Z +   = note: the following crate versions were found:
2020-02-07T02:04:00.6065381Z +   = note: the following crate versions were found:
2020-02-07T02:04:00.6065731Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta
2020-02-07T02:04:00.6066203Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib
2020-02-07T02:04:00.6066501Z +           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib
2020-02-07T02:04:00.6066770Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6066972Z -  --> $DIR/result_map_unit_fn_unfixable.rs:19:5
2020-02-07T02:04:00.6067149Z -   |
2020-02-07T02:04:00.6067149Z -   |
2020-02-07T02:04:00.6067385Z -LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T02:04:00.6067665Z +error: aborting due to previous error
2020-02-07T02:04:00.6067722Z  
2020-02-07T02:04:00.6067956Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6068177Z -  --> $DIR/result_map_unit_fn_unfixable.rs:23:5
2020-02-07T02:04:00.6068177Z -  --> $DIR/result_map_unit_fn_unfixable.rs:23:5
2020-02-07T02:04:00.6068374Z -   |
2020-02-07T02:04:00.6068570Z -LL |     x.field.map(|value| {
2020-02-07T02:04:00.6068955Z -
2020-02-07T02:04:00.6069175Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T02:04:00.6069392Z -  --> $DIR/result_map_unit_fn_unfixable.rs:27:5
2020-02-07T02:04:00.6069567Z -   |
2020-02-07T02:04:00.6069567Z -   |
2020-02-07T02:04:00.6069818Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T02:04:00.6070187Z -
2020-02-07T02:04:00.6070412Z -error: aborting due to 4 previous errors
2020-02-07T02:04:00.6070581Z -
2020-02-07T02:04:00.6070818Z -For more information about this error, try `rustc --explain E0425`.
---
2020-02-07T02:04:00.6071796Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base' 'result_map_unit_fn_unfixable.rs'
2020-02-07T02:04:00.6071861Z 
2020-02-07T02:04:00.6071907Z error: 1 errors occurred comparing output.
2020-02-07T02:04:00.6071951Z status: exit code: 1
2020-02-07T02:04:00.6074840Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/result_map_unit_fn_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/result_map_unit_fn_unfixable.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-93be375d9bac0399.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-434bedc32a67cf7f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/result_map_unit_fn_unfixable.stage-id.aux" "-A" "unused"
2020-02-07T02:04:00.6075431Z ------------------------------------------
2020-02-07T02:04:00.6075466Z 
2020-02-07T02:04:00.6075682Z ------------------------------------------
2020-02-07T02:04:00.6075726Z stderr:
2020-02-07T02:04:00.6075726Z stderr:
2020-02-07T02:04:00.6075948Z ------------------------------------------
2020-02-07T02:04:00.6077959Z {"message":"found possibly newer version of crate `if_chain` which `clippy_lints` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(clippy::result_map_unit_fn)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib\ncrate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rmeta\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-c235364af10680a8.rlib\n           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib\n\n"}
2020-02-07T02:04:00.6078349Z 
2020-02-07T02:04:00.6078590Z ------------------------------------------
2020-02-07T02:04:00.6078621Z 
2020-02-07T02:04:00.6078667Z test [ui] ui/result_map_unit_fn_unfixable.rs ... FAILED
---
2020-02-07T02:04:01.3132438Z expected stderr:
2020-02-07T02:04:01.3132487Z error: you should not implement `visit_string` without also implementing `visit_str`
2020-02-07T02:04:01.3132724Z   --> $DIR/serde.rs:39:5
2020-02-07T02:04:01.3132775Z    |
2020-02-07T02:04:01.3133206Z LL | /     fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
2020-02-07T02:04:01.3133331Z LL | |         E: serde::de::Error,
2020-02-07T02:04:01.3133465Z LL | |     {
2020-02-07T02:04:01.3133509Z LL | |         unimplemented!()
2020-02-07T02:04:01.3133567Z LL | |     }
---
2020-02-07T02:04:01.3134683Z -  --> $DIR/serde.rs:39:5
2020-02-07T02:04:01.3134932Z +error[E0463]: can't find crate for `serde_derive` which `serde` depends on
2020-02-07T02:04:01.3135143Z +  --> $DIR/serde.rs:4:1
2020-02-07T02:04:01.3135204Z     |
2020-02-07T02:04:01.3135450Z -LL | /     fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
2020-02-07T02:04:01.3135861Z -LL | |         E: serde::de::Error,
2020-02-07T02:04:01.3136063Z -LL | |     {
2020-02-07T02:04:01.3136266Z -LL | |         unimplemented!()
2020-02-07T02:04:01.3136451Z -LL | |     }
---
2020-02-07T02:04:01.3138689Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base' 'serde.rs'
2020-02-07T02:04:01.3138729Z 
2020-02-07T02:04:01.3138789Z error: 1 errors occurred comparing output.
2020-02-07T02:04:01.3138833Z status: exit code: 1
2020-02-07T02:04:01.3140483Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/serde.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/serde.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-93be375d9bac0399.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-434bedc32a67cf7f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/serde.stage-id.aux" "-A" "unused"
2020-02-07T02:04:01.3140975Z ------------------------------------------
2020-02-07T02:04:01.3141011Z 
2020-02-07T02:04:01.3141245Z ------------------------------------------
2020-02-07T02:04:01.3141291Z stderr:
2020-02-07T02:04:01.3141291Z stderr:
2020-02-07T02:04:01.3141494Z ------------------------------------------
2020-02-07T02:04:01.3142759Z {"message":"can't find crate for `serde_derive` which `serde` depends on","code":{"code":"E0463","explanation":"A plugin/crate was declared but cannot be found. Erroneous code example:\n\n