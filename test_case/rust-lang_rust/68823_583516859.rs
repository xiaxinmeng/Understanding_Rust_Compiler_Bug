plain
2020-02-07T15:06:12.5629617Z ========================== Starting Command Output ===========================
2020-02-07T15:06:12.5633286Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e34684ba-c8ef-4ecc-ba1f-c06ed729c802.sh
2020-02-07T15:06:12.5633568Z 
2020-02-07T15:06:12.5637123Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-07T15:06:12.5643981Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-07T15:06:12.5645937Z Task         : Get sources
2020-02-07T15:06:12.5645975Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T15:06:12.5646067Z Version      : 1.0.0
2020-02-07T15:06:12.5646103Z Author       : Microsoft
---
2020-02-07T15:06:13.5990557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-07T15:06:13.6080550Z ##[command]git config gc.auto 0
2020-02-07T15:06:13.6155297Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-07T15:06:13.6216231Z ##[command]git config --get-all http.proxy
2020-02-07T15:06:13.6357929Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68823/merge:refs/remotes/pull/68823/merge
---
2020-02-07T17:13:03.2123696Z test [ui] ui/fn_to_numeric_cast.rs ... ok
2020-02-07T17:13:03.4281809Z test [ui] ui/for_kv_map.rs ... ok
2020-02-07T17:13:03.9603842Z test [ui] ui/for_loop_over_option_result.rs ... ok
2020-02-07T17:13:04.1320559Z normalized stderr:
2020-02-07T17:13:04.1320766Z error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T17:13:04.1320879Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T17:13:04.1322163Z    = note: the following crate versions were found:
2020-02-07T17:13:04.1322163Z    = note: the following crate versions were found:
2020-02-07T17:13:04.1323229Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta
2020-02-07T17:13:04.1323937Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib
2020-02-07T17:13:04.1324573Z            crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib
2020-02-07T17:13:04.1335342Z error: aborting due to previous error
2020-02-07T17:13:04.1335590Z 
2020-02-07T17:13:04.1376756Z 
2020-02-07T17:13:04.1376849Z 
2020-02-07T17:13:04.1376849Z 
2020-02-07T17:13:04.1376902Z expected stderr:
2020-02-07T17:13:04.1376979Z error[E0425]: cannot find function `f` in this scope
2020-02-07T17:13:04.1377388Z   --> $DIR/for_loop_unfixable.rs:37:12
2020-02-07T17:13:04.1377435Z    |
2020-02-07T17:13:04.1377508Z LL |         if f(&vec[i], &vec[i]) {
2020-02-07T17:13:04.1377585Z 
2020-02-07T17:13:04.1377624Z error: aborting due to previous error
2020-02-07T17:13:04.1377666Z 
2020-02-07T17:13:04.1381281Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:13:04.1381281Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:13:04.1381397Z 
2020-02-07T17:13:04.1381422Z 
2020-02-07T17:13:04.1381491Z diff of stderr:
2020-02-07T17:13:04.1381518Z 
2020-02-07T17:13:04.1381886Z -error[E0425]: cannot find function `f` in this scope
2020-02-07T17:13:04.1382093Z -  --> $DIR/for_loop_unfixable.rs:37:12
2020-02-07T17:13:04.1382161Z +error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T17:13:04.1382207Z     |
2020-02-07T17:13:04.1382412Z -LL |         if f(&vec[i], &vec[i]) {
2020-02-07T17:13:04.1382710Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T17:13:04.1382755Z +   = note: the following crate versions were found:
2020-02-07T17:13:04.1382755Z +   = note: the following crate versions were found:
2020-02-07T17:13:04.1383084Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta
2020-02-07T17:13:04.1383392Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib
2020-02-07T17:13:04.1383718Z +           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib
2020-02-07T17:13:04.1383804Z  error: aborting due to previous error
2020-02-07T17:13:04.1383859Z  
2020-02-07T17:13:04.1384072Z -For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:13:04.1384114Z  
---
2020-02-07T17:13:04.1387596Z 
2020-02-07T17:13:04.1387794Z ------------------------------------------
2020-02-07T17:13:04.1387852Z stderr:
2020-02-07T17:13:04.1388055Z ------------------------------------------
2020-02-07T17:13:04.1390122Z {"message":"found possibly newer version of crate `if_chain` which `clippy_lints` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Tests from for_loop.rs that don't have suggestions","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib\ncrate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib\n           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib\n\n"}
2020-02-07T17:13:04.1390735Z 
2020-02-07T17:13:04.1390992Z ------------------------------------------
2020-02-07T17:13:04.1391023Z 
2020-02-07T17:13:04.1391195Z test [ui] ui/for_loop_unfixable.rs ... FAILED
---
2020-02-07T17:13:51.7794807Z test [ui] ui/option_and_then_some.rs ... ok
2020-02-07T17:13:52.0634494Z test [ui] ui/option_as_ref_deref.rs ... ok
2020-02-07T17:13:52.7915122Z test [ui] ui/option_map_or_none.rs ... ok
2020-02-07T17:13:52.9719185Z normalized stderr:
2020-02-07T17:13:52.9723896Z error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T17:13:52.9724038Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T17:13:52.9724088Z    = note: the following crate versions were found:
2020-02-07T17:13:52.9724088Z    = note: the following crate versions were found:
2020-02-07T17:13:52.9724966Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta
2020-02-07T17:13:52.9725362Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib
2020-02-07T17:13:52.9725723Z            crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib
2020-02-07T17:13:52.9725812Z error: aborting due to previous error
2020-02-07T17:13:52.9725841Z 
2020-02-07T17:13:52.9729247Z 
2020-02-07T17:13:52.9729339Z 
2020-02-07T17:13:52.9729339Z 
2020-02-07T17:13:52.9732649Z expected stderr:
2020-02-07T17:13:52.9737068Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9737505Z   --> $DIR/option_map_unit_fn_unfixable.rs:17:5
2020-02-07T17:13:52.9737555Z    |
2020-02-07T17:13:52.9737604Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T17:13:52.9737700Z 
2020-02-07T17:13:52.9737745Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9737981Z   --> $DIR/option_map_unit_fn_unfixable.rs:19:5
2020-02-07T17:13:52.9738059Z    |
2020-02-07T17:13:52.9738059Z    |
2020-02-07T17:13:52.9738109Z LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T17:13:52.9738203Z 
2020-02-07T17:13:52.9738246Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9738475Z   --> $DIR/option_map_unit_fn_unfixable.rs:23:5
2020-02-07T17:13:52.9738531Z    |
2020-02-07T17:13:52.9738531Z    |
2020-02-07T17:13:52.9738589Z LL |     x.field.map(|value| {
2020-02-07T17:13:52.9738661Z 
2020-02-07T17:13:52.9738720Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9738947Z   --> $DIR/option_map_unit_fn_unfixable.rs:27:5
2020-02-07T17:13:52.9738991Z    |
2020-02-07T17:13:52.9738991Z    |
2020-02-07T17:13:52.9739037Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T17:13:52.9739128Z 
2020-02-07T17:13:52.9739170Z error: aborting due to 4 previous errors
2020-02-07T17:13:52.9739283Z 
2020-02-07T17:13:52.9739545Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:13:52.9739545Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:13:52.9743074Z 
2020-02-07T17:13:52.9743138Z 
2020-02-07T17:13:52.9746530Z diff of stderr:
2020-02-07T17:13:52.9746573Z 
2020-02-07T17:13:52.9750920Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9754675Z -  --> $DIR/option_map_unit_fn_unfixable.rs:17:5
2020-02-07T17:13:52.9783607Z +error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T17:13:52.9783699Z     |
2020-02-07T17:13:52.9784202Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T17:13:52.9784503Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T17:13:52.9784573Z +   = note: the following crate versions were found:
2020-02-07T17:13:52.9784573Z +   = note: the following crate versions were found:
2020-02-07T17:13:52.9785142Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta
2020-02-07T17:13:52.9785558Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib
2020-02-07T17:13:52.9785917Z +           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib
2020-02-07T17:13:52.9786685Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9788989Z -  --> $DIR/option_map_unit_fn_unfixable.rs:19:5
2020-02-07T17:13:52.9789196Z -   |
2020-02-07T17:13:52.9789196Z -   |
2020-02-07T17:13:52.9789463Z -LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T17:13:52.9789754Z +error: aborting due to previous error
2020-02-07T17:13:52.9789797Z  
2020-02-07T17:13:52.9790026Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9790289Z -  --> $DIR/option_map_unit_fn_unfixable.rs:23:5
2020-02-07T17:13:52.9790289Z -  --> $DIR/option_map_unit_fn_unfixable.rs:23:5
2020-02-07T17:13:52.9790476Z -   |
2020-02-07T17:13:52.9790683Z -LL |     x.field.map(|value| {
2020-02-07T17:13:52.9791094Z -
2020-02-07T17:13:52.9791322Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:13:52.9791567Z -  --> $DIR/option_map_unit_fn_unfixable.rs:27:5
2020-02-07T17:13:52.9791766Z -   |
2020-02-07T17:13:52.9791766Z -   |
2020-02-07T17:13:52.9792019Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T17:13:52.9792534Z -
2020-02-07T17:13:52.9792745Z -error: aborting due to 4 previous errors
2020-02-07T17:13:52.9792917Z -
2020-02-07T17:13:52.9793178Z -For more information about this error, try `rustc --explain E0425`.
---
2020-02-07T17:13:52.9796735Z 
2020-02-07T17:13:52.9796952Z ------------------------------------------
2020-02-07T17:13:52.9797015Z stderr:
2020-02-07T17:13:52.9797238Z ------------------------------------------
2020-02-07T17:13:52.9799392Z {"message":"found possibly newer version of crate `if_chain` which `clippy_lints` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/option_map_unit_fn_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(clippy::option_map_unit_fn)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib\ncrate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib\n           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib\n\n"}
2020-02-07T17:13:52.9799793Z 
2020-02-07T17:13:52.9800027Z ------------------------------------------
2020-02-07T17:13:52.9800058Z 
2020-02-07T17:13:52.9800121Z test [ui] ui/option_map_unit_fn_unfixable.rs ... FAILED
---
2020-02-07T17:14:03.6677091Z test [ui] ui/redundant_field_names.rs ... ok
2020-02-07T17:14:04.3956011Z test [ui] ui/redundant_pattern_matching.rs ... ok
2020-02-07T17:14:04.7603532Z test [ui] ui/redundant_static_lifetimes.rs ... ok
2020-02-07T17:14:04.8324362Z normalized stderr:
2020-02-07T17:14:04.8324481Z error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T17:14:04.8325332Z    |
2020-02-07T17:14:04.8325376Z LL | extern crate regex;
2020-02-07T17:14:04.8325419Z    | ^^^^^^^^^^^^^^^^^^^
2020-02-07T17:14:04.8325460Z    |
2020-02-07T17:14:04.8325460Z    |
2020-02-07T17:14:04.8325523Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T17:14:04.8325571Z    = note: the following crate versions were found:
2020-02-07T17:14:04.8325963Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta
2020-02-07T17:14:04.8326380Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib
2020-02-07T17:14:04.8326708Z            crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib
2020-02-07T17:14:04.8326810Z error: aborting due to previous error
2020-02-07T17:14:04.8326838Z 
2020-02-07T17:14:04.8326864Z 
2020-02-07T17:14:04.8326898Z 
2020-02-07T17:14:04.8326898Z 
2020-02-07T17:14:04.8326957Z expected stderr:
2020-02-07T17:14:04.8334114Z error: trivial regex
2020-02-07T17:14:04.8343115Z   --> $DIR/regex.rs:13:45
2020-02-07T17:14:04.8343180Z    |
2020-02-07T17:14:04.8343227Z LL |     let pipe_in_wrong_position = Regex::new("|");
2020-02-07T17:14:04.8343358Z    |
2020-02-07T17:14:04.8347877Z    = note: `-D clippy::trivial-regex` implied by `-D warnings`
2020-02-07T17:14:04.8347973Z    = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8348005Z 
2020-02-07T17:14:04.8348005Z 
2020-02-07T17:14:04.8348048Z error: trivial regex
2020-02-07T17:14:04.8348272Z   --> $DIR/regex.rs:14:60
2020-02-07T17:14:04.8348332Z    |
2020-02-07T17:14:04.8348379Z LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2020-02-07T17:14:04.8348491Z    |
2020-02-07T17:14:04.8348550Z    = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8348581Z 
2020-02-07T17:14:04.8348581Z 
2020-02-07T17:14:04.8348629Z error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T17:14:04.8349017Z    |
2020-02-07T17:14:04.8349017Z    |
2020-02-07T17:14:04.8349247Z LL |     let wrong_char_ranice = Regex::new("[z-a]");
2020-02-07T17:14:04.8352385Z    |
2020-02-07T17:14:04.8354301Z    = note: `-D clippy::invalid-regex` implied by `-D warnings`
2020-02-07T17:14:04.8354350Z 
2020-02-07T17:14:04.8354350Z 
2020-02-07T17:14:04.8354428Z error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T17:14:04.8354745Z    |
2020-02-07T17:14:04.8354745Z    |
2020-02-07T17:14:04.8354995Z LL |     let some_unicode = Regex::new("[é-è]");
2020-02-07T17:14:04.8355079Z 
2020-02-07T17:14:04.8355285Z error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8355572Z   --> $DIR/regex.rs:18:33
2020-02-07T17:14:04.8355617Z    |
2020-02-07T17:14:04.8355617Z    |
2020-02-07T17:14:04.8355662Z LL |     let some_regex = Regex::new(OPENING_PAREN);
2020-02-07T17:14:04.8355728Z    |                                 ^^^^^^^^^^^^^
2020-02-07T17:14:04.8355759Z 
2020-02-07T17:14:04.8355802Z error: trivial regex
2020-02-07T17:14:04.8356020Z   --> $DIR/regex.rs:20:53
2020-02-07T17:14:04.8356082Z    |
2020-02-07T17:14:04.8356128Z LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2020-02-07T17:14:04.8356240Z    |
2020-02-07T17:14:04.8356285Z    = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8356315Z 
2020-02-07T17:14:04.8356360Z error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8356360Z error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8356583Z   --> $DIR/regex.rs:21:41
2020-02-07T17:14:04.8356626Z    |
2020-02-07T17:14:04.8356681Z LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2020-02-07T17:14:04.8356779Z 
2020-02-07T17:14:04.8392175Z error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8392628Z   --> $DIR/regex.rs:22:56
2020-02-07T17:14:04.8392678Z    |
2020-02-07T17:14:04.8392678Z    |
2020-02-07T17:14:04.8392726Z LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2020-02-07T17:14:04.8392843Z 
2020-02-07T17:14:04.8392889Z error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8393108Z   --> $DIR/regex.rs:34:37
2020-02-07T17:14:04.8393168Z    |
2020-02-07T17:14:04.8393168Z    |
2020-02-07T17:14:04.8393424Z LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T17:14:04.8393508Z 
2020-02-07T17:14:04.8393578Z error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8393778Z   --> $DIR/regex.rs:35:39
2020-02-07T17:14:04.8393820Z    |
2020-02-07T17:14:04.8393820Z    |
2020-02-07T17:14:04.8394085Z LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T17:14:04.8394169Z 
2020-02-07T17:14:04.8394222Z error: regex syntax error: unrecognized escape sequence
2020-02-07T17:14:04.8394439Z   --> $DIR/regex.rs:37:45
2020-02-07T17:14:04.8394481Z    |
2020-02-07T17:14:04.8394481Z    |
2020-02-07T17:14:04.8394524Z LL |     let raw_string_error = Regex::new(r"[...//...]");
2020-02-07T17:14:04.8394617Z 
2020-02-07T17:14:04.8394660Z error: regex syntax error: unrecognized escape sequence
2020-02-07T17:14:04.8394858Z   --> $DIR/regex.rs:38:46
2020-02-07T17:14:04.8394918Z    |
2020-02-07T17:14:04.8394918Z    |
2020-02-07T17:14:04.8394963Z LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2020-02-07T17:14:04.8395066Z 
2020-02-07T17:14:04.8395107Z error: trivial regex
2020-02-07T17:14:04.8395303Z   --> $DIR/regex.rs:42:33
2020-02-07T17:14:04.8395345Z    |
2020-02-07T17:14:04.8395345Z    |
2020-02-07T17:14:04.8395404Z LL |     let trivial_eq = Regex::new("^foobar$");
2020-02-07T17:14:04.8395645Z    |
2020-02-07T17:14:04.8395708Z    = help: consider using `==` on `str`s
2020-02-07T17:14:04.8395737Z 
2020-02-07T17:14:04.8395778Z error: trivial regex
2020-02-07T17:14:04.8395778Z error: trivial regex
2020-02-07T17:14:04.8396025Z   --> $DIR/regex.rs:44:48
2020-02-07T17:14:04.8396085Z    |
2020-02-07T17:14:04.8396130Z LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2020-02-07T17:14:04.8396242Z    |
2020-02-07T17:14:04.8396284Z    = help: consider using `==` on `str`s
2020-02-07T17:14:04.8396312Z 
2020-02-07T17:14:04.8396352Z error: trivial regex
2020-02-07T17:14:04.8396352Z error: trivial regex
2020-02-07T17:14:04.8396653Z   --> $DIR/regex.rs:46:42
2020-02-07T17:14:04.8396702Z    |
2020-02-07T17:14:04.8396746Z LL |     let trivial_starts_with = Regex::new("^foobar");
2020-02-07T17:14:04.8396852Z    |
2020-02-07T17:14:04.8396895Z    = help: consider using `str::starts_with`
2020-02-07T17:14:04.8396932Z 
2020-02-07T17:14:04.8396989Z error: trivial regex
2020-02-07T17:14:04.8396989Z error: trivial regex
2020-02-07T17:14:04.8397217Z   --> $DIR/regex.rs:48:40
2020-02-07T17:14:04.8397258Z    |
2020-02-07T17:14:04.8397319Z LL |     let trivial_ends_with = Regex::new("foobar$");
2020-02-07T17:14:04.8397407Z    |
2020-02-07T17:14:04.8397467Z    = help: consider using `str::ends_with`
2020-02-07T17:14:04.8397496Z 
2020-02-07T17:14:04.8397536Z error: trivial regex
---
2020-02-07T17:14:04.8398018Z 
2020-02-07T17:14:04.8398058Z error: trivial regex
2020-02-07T17:14:04.8398274Z   --> $DIR/regex.rs:52:39
2020-02-07T17:14:04.8398325Z    |
2020-02-07T17:14:04.8398370Z LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2020-02-07T17:14:04.8398480Z    |
2020-02-07T17:14:04.8398522Z    = help: consider using `str::contains`
2020-02-07T17:14:04.8398550Z 
2020-02-07T17:14:04.8398606Z error: trivial regex
2020-02-07T17:14:04.8398606Z error: trivial regex
2020-02-07T17:14:04.8398806Z   --> $DIR/regex.rs:54:40
2020-02-07T17:14:04.8398847Z    |
2020-02-07T17:14:04.8398890Z LL |     let trivial_backslash = Regex::new("a/.b");
2020-02-07T17:14:04.8399003Z    |
2020-02-07T17:14:04.8399045Z    = help: consider using `str::contains`
2020-02-07T17:14:04.8399090Z 
2020-02-07T17:14:04.8399131Z error: trivial regex
---
2020-02-07T17:14:04.8399616Z 
2020-02-07T17:14:04.8399656Z error: trivial regex
2020-02-07T17:14:04.8399854Z   --> $DIR/regex.rs:59:36
2020-02-07T17:14:04.8399911Z    |
2020-02-07T17:14:04.8399954Z LL |     let trivial_empty = Regex::new("^");
2020-02-07T17:14:04.8400058Z    |
2020-02-07T17:14:04.8400102Z    = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8400131Z 
2020-02-07T17:14:04.8400181Z error: trivial regex
2020-02-07T17:14:04.8400181Z error: trivial regex
2020-02-07T17:14:04.8400400Z   --> $DIR/regex.rs:61:36
2020-02-07T17:14:04.8400441Z    |
2020-02-07T17:14:04.8400484Z LL |     let trivial_empty = Regex::new("^$");
2020-02-07T17:14:04.8400587Z    |
2020-02-07T17:14:04.8400628Z    = help: consider using `str::is_empty`
2020-02-07T17:14:04.8400744Z 
2020-02-07T17:14:04.8400801Z error: trivial regex
2020-02-07T17:14:04.8400801Z error: trivial regex
2020-02-07T17:14:04.8401141Z   --> $DIR/regex.rs:63:44
2020-02-07T17:14:04.8401184Z    |
2020-02-07T17:14:04.8401245Z LL |     let binary_trivial_empty = BRegex::new("^$");
2020-02-07T17:14:04.8401336Z    |
2020-02-07T17:14:04.8401378Z    = help: consider using `str::is_empty`
2020-02-07T17:14:04.8401424Z 
2020-02-07T17:14:04.8401468Z error: aborting due to 23 previous errors
2020-02-07T17:14:04.8401468Z error: aborting due to 23 previous errors
2020-02-07T17:14:04.8401497Z 
2020-02-07T17:14:04.8401523Z 
2020-02-07T17:14:04.8401548Z 
2020-02-07T17:14:04.8401606Z diff of stderr:
2020-02-07T17:14:04.8401786Z 
2020-02-07T17:14:04.8402165Z -error: trivial regex
2020-02-07T17:14:04.8402375Z -  --> $DIR/regex.rs:13:45
2020-02-07T17:14:04.8402430Z +error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T17:14:04.8402698Z     |
2020-02-07T17:14:04.8402698Z     |
2020-02-07T17:14:04.8402942Z -LL |     let pipe_in_wrong_position = Regex::new("|");
2020-02-07T17:14:04.8403238Z +LL | extern crate regex;
2020-02-07T17:14:04.8403281Z +   | ^^^^^^^^^^^^^^^^^^^
2020-02-07T17:14:04.8403340Z     |
2020-02-07T17:14:04.8403579Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2020-02-07T17:14:04.8403579Z -   = note: `-D clippy::trivial-regex` implied by `-D warnings`
2020-02-07T17:14:04.8403811Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8403863Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T17:14:04.8403930Z +   = note: the following crate versions were found:
2020-02-07T17:14:04.8404304Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta
2020-02-07T17:14:04.8404747Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib
2020-02-07T17:14:04.8405105Z +           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib
2020-02-07T17:14:04.8405369Z -error: trivial regex
2020-02-07T17:14:04.8405575Z -  --> $DIR/regex.rs:14:60
2020-02-07T17:14:04.8405754Z -   |
2020-02-07T17:14:04.8405754Z -   |
2020-02-07T17:14:04.8405997Z -LL |     let pipe_in_wrong_position_builder = RegexBuilder::new("|");
2020-02-07T17:14:04.8406441Z -   |
2020-02-07T17:14:04.8406666Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8406870Z -
2020-02-07T17:14:04.8406870Z -
2020-02-07T17:14:04.8407133Z -error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T17:14:04.8407531Z -   |
2020-02-07T17:14:04.8407531Z -   |
2020-02-07T17:14:04.8407759Z -LL |     let wrong_char_ranice = Regex::new("[z-a]");
2020-02-07T17:14:04.8408194Z -   |
2020-02-07T17:14:04.8408428Z -   = note: `-D clippy::invalid-regex` implied by `-D warnings`
2020-02-07T17:14:04.8408604Z -
2020-02-07T17:14:04.8408604Z -
2020-02-07T17:14:04.8408861Z -error: regex syntax error: invalid character class range, the start must be <= the end
2020-02-07T17:14:04.8410075Z -   |
2020-02-07T17:14:04.8410075Z -   |
2020-02-07T17:14:04.8410559Z -LL |     let some_unicode = Regex::new("[é-è]");
2020-02-07T17:14:04.8411006Z -
2020-02-07T17:14:04.8411251Z -error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8411600Z -  --> $DIR/regex.rs:18:33
2020-02-07T17:14:04.8411774Z -   |
2020-02-07T17:14:04.8411774Z -   |
2020-02-07T17:14:04.8411992Z -LL |     let some_regex = Regex::new(OPENING_PAREN);
2020-02-07T17:14:04.8412211Z -   |                                 ^^^^^^^^^^^^^
2020-02-07T17:14:04.8412400Z -
2020-02-07T17:14:04.8412585Z -error: trivial regex
2020-02-07T17:14:04.8412947Z -  --> $DIR/regex.rs:20:53
2020-02-07T17:14:04.8413142Z -   |
2020-02-07T17:14:04.8413368Z -LL |     let binary_pipe_in_wrong_position = BRegex::new("|");
2020-02-07T17:14:04.8413789Z -   |
2020-02-07T17:14:04.8414011Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8414182Z -
2020-02-07T17:14:04.8414402Z -error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8414402Z -error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8414618Z -  --> $DIR/regex.rs:21:41
2020-02-07T17:14:04.8414789Z -   |
2020-02-07T17:14:04.8415108Z -LL |     let some_binary_regex = BRegex::new(OPENING_PAREN);
2020-02-07T17:14:04.8415570Z -
2020-02-07T17:14:04.8415914Z -error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8416135Z -  --> $DIR/regex.rs:22:56
2020-02-07T17:14:04.8416312Z -   |
2020-02-07T17:14:04.8416312Z -   |
2020-02-07T17:14:04.8416560Z -LL |     let some_binary_regex_builder = BRegexBuilder::new(OPENING_PAREN);
2020-02-07T17:14:04.8417024Z -
2020-02-07T17:14:04.8417253Z -error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8417454Z -  --> $DIR/regex.rs:34:37
2020-02-07T17:14:04.8417649Z -   |
2020-02-07T17:14:04.8417649Z -   |
2020-02-07T17:14:04.8417905Z -LL |     let set_error = RegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T17:14:04.8418331Z -
2020-02-07T17:14:04.8418568Z -error: regex syntax error on position 0: unclosed group
2020-02-07T17:14:04.8418774Z -  --> $DIR/regex.rs:35:39
2020-02-07T17:14:04.8418967Z -   |
2020-02-07T17:14:04.8418967Z -   |
2020-02-07T17:14:04.8419226Z -LL |     let bset_error = BRegexSet::new(&[OPENING_PAREN, r"[a-z]+/.(com|org|net)"]);
2020-02-07T17:14:04.8419655Z -
2020-02-07T17:14:04.8419897Z -error: regex syntax error: unrecognized escape sequence
2020-02-07T17:14:04.8420099Z -  --> $DIR/regex.rs:37:45
2020-02-07T17:14:04.8420276Z -   |
2020-02-07T17:14:04.8420276Z -   |
2020-02-07T17:14:04.8420525Z -LL |     let raw_string_error = Regex::new(r"[...//...]");
2020-02-07T17:14:04.8420933Z -
2020-02-07T17:14:04.8421178Z -error: regex syntax error: unrecognized escape sequence
2020-02-07T17:14:04.8421379Z -  --> $DIR/regex.rs:38:46
2020-02-07T17:14:04.8421557Z -   |
2020-02-07T17:14:04.8421557Z -   |
2020-02-07T17:14:04.8421809Z -LL |     let raw_string_error = Regex::new(r#"[...//...]"#);
2020-02-07T17:14:04.8422225Z -
2020-02-07T17:14:04.8422434Z -error: trivial regex
2020-02-07T17:14:04.8422635Z -  --> $DIR/regex.rs:42:33
2020-02-07T17:14:04.8422812Z -   |
2020-02-07T17:14:04.8422812Z -   |
2020-02-07T17:14:04.8423034Z -LL |     let trivial_eq = Regex::new("^foobar$");
2020-02-07T17:14:04.8423473Z -   |
2020-02-07T17:14:04.8423688Z -   = help: consider using `==` on `str`s
2020-02-07T17:14:04.8423879Z -
2020-02-07T17:14:04.8424073Z -error: trivial regex
2020-02-07T17:14:04.8424073Z -error: trivial regex
2020-02-07T17:14:04.8424274Z -  --> $DIR/regex.rs:44:48
2020-02-07T17:14:04.8424453Z -   |
2020-02-07T17:14:04.8424715Z -LL |     let trivial_eq_builder = RegexBuilder::new("^foobar$");
2020-02-07T17:14:04.8425138Z -   |
2020-02-07T17:14:04.8425368Z -   = help: consider using `==` on `str`s
2020-02-07T17:14:04.8425547Z -
2020-02-07T17:14:04.8425748Z -error: trivial regex
2020-02-07T17:14:04.8425748Z -error: trivial regex
2020-02-07T17:14:04.8425969Z -  --> $DIR/regex.rs:46:42
2020-02-07T17:14:04.8426150Z -   |
2020-02-07T17:14:04.8426380Z -LL |     let trivial_starts_with = Regex::new("^foobar");
2020-02-07T17:14:04.8426955Z -   |
2020-02-07T17:14:04.8427195Z -   = help: consider using `str::starts_with`
2020-02-07T17:14:04.8427504Z -
2020-02-07T17:14:04.8427722Z -error: trivial regex
2020-02-07T17:14:04.8427722Z -error: trivial regex
2020-02-07T17:14:04.8427921Z -  --> $DIR/regex.rs:48:40
2020-02-07T17:14:04.8428098Z -   |
2020-02-07T17:14:04.8428343Z -LL |     let trivial_ends_with = Regex::new("foobar$");
2020-02-07T17:14:04.8428760Z -   |
2020-02-07T17:14:04.8428977Z -   = help: consider using `str::ends_with`
2020-02-07T17:14:04.8429173Z -
2020-02-07T17:14:04.8429366Z -error: trivial regex
---
2020-02-07T17:14:04.8431164Z -
2020-02-07T17:14:04.8431355Z -error: trivial regex
2020-02-07T17:14:04.8431589Z -  --> $DIR/regex.rs:52:39
2020-02-07T17:14:04.8431766Z -   |
2020-02-07T17:14:04.8431999Z -LL |     let trivial_contains = Regex::new(NOT_A_REAL_REGEX);
2020-02-07T17:14:04.8432441Z -   |
2020-02-07T17:14:04.8432657Z -   = help: consider using `str::contains`
2020-02-07T17:14:04.8432849Z -
2020-02-07T17:14:04.8433046Z -error: trivial regex
2020-02-07T17:14:04.8433046Z -error: trivial regex
2020-02-07T17:14:04.8433246Z -  --> $DIR/regex.rs:54:40
2020-02-07T17:14:04.8433423Z -   |
2020-02-07T17:14:04.8433667Z -LL |     let trivial_backslash = Regex::new("a/.b");
2020-02-07T17:14:04.8434089Z -   |
2020-02-07T17:14:04.8434322Z -   = help: consider using `str::contains`
2020-02-07T17:14:04.8434497Z -
2020-02-07T17:14:04.8434688Z -error: trivial regex
---
2020-02-07T17:14:04.8436386Z -
2020-02-07T17:14:04.8436596Z -error: trivial regex
2020-02-07T17:14:04.8436799Z -  --> $DIR/regex.rs:59:36
2020-02-07T17:14:04.8436976Z -   |
2020-02-07T17:14:04.8437209Z -LL |     let trivial_empty = Regex::new("^");
2020-02-07T17:14:04.8437615Z -   |
2020-02-07T17:14:04.8437853Z -   = help: the regex is unlikely to be useful as it is
2020-02-07T17:14:04.8438051Z -
2020-02-07T17:14:04.8438243Z -error: trivial regex
2020-02-07T17:14:04.8438243Z -error: trivial regex
2020-02-07T17:14:04.8438443Z -  --> $DIR/regex.rs:61:36
2020-02-07T17:14:04.8438636Z -   |
2020-02-07T17:14:04.8438859Z -LL |     let trivial_empty = Regex::new("^$");
2020-02-07T17:14:04.8439275Z -   |
2020-02-07T17:14:04.8439504Z -   = help: consider using `str::is_empty`
2020-02-07T17:14:04.8439678Z -
2020-02-07T17:14:04.8439869Z -error: trivial regex
2020-02-07T17:14:04.8439869Z -error: trivial regex
2020-02-07T17:14:04.8440086Z -  --> $DIR/regex.rs:63:44
2020-02-07T17:14:04.8440264Z -   |
2020-02-07T17:14:04.8440490Z -LL |     let binary_trivial_empty = BRegex::new("^$");
2020-02-07T17:14:04.8440917Z -   |
2020-02-07T17:14:04.8441132Z -   = help: consider using `str::is_empty`
2020-02-07T17:14:04.8441307Z -
2020-02-07T17:14:04.8441541Z -error: aborting due to 23 previous errors
---
2020-02-07T17:14:04.8442718Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base' 'regex.rs'
2020-02-07T17:14:04.8442762Z 
2020-02-07T17:14:04.8442826Z error: 1 errors occurred comparing output.
2020-02-07T17:14:04.8442870Z status: exit code: 1
2020-02-07T17:14:04.8444605Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/regex.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/regex.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-d198c1b8615cb4a6.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-1ad96b90660f59d3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/regex.stage-id.aux" "-A" "unused"
2020-02-07T17:14:04.8445073Z ------------------------------------------
2020-02-07T17:14:04.8445117Z 
2020-02-07T17:14:04.8445365Z ------------------------------------------
2020-02-07T17:14:04.8445412Z stderr:
2020-02-07T17:14:04.8445412Z stderr:
2020-02-07T17:14:04.8445623Z ------------------------------------------
2020-02-07T17:14:04.8448288Z {"message":"found possibly newer version of crate `aho_corasick` which `regex` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/regex.rs","byte_start":95,"byte_end":114,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate regex;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib\ncrate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on\n  --> tests/ui/regex.rs:4:1\n   |\nLL | extern crate regex;\n   | ^^^^^^^^^^^^^^^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib\n           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib\n\n"}
2020-02-07T17:14:04.8448815Z 
2020-02-07T17:14:04.8449125Z ------------------------------------------
2020-02-07T17:14:04.8449158Z 
2020-02-07T17:14:04.8449204Z test [ui] ui/regex.rs ... FAILED
2020-02-07T17:14:04.8449204Z test [ui] ui/regex.rs ... FAILED
2020-02-07T17:14:04.8680862Z test [ui] ui/redundant_static_lifetimes_multiple.rs ... ok
2020-02-07T17:14:05.9275742Z test [ui] ui/rename.rs ... ok
2020-02-07T17:14:06.0713633Z test [ui] ui/renamed_builtin_attr.rs ... ok
2020-02-07T17:14:06.3720199Z test [ui] ui/repl_uninit.rs ... ok
2020-02-07T17:14:07.1597836Z test [ui] ui/replace_consts.rs ... ok
2020-02-07T17:14:07.3685425Z normalized stderr:
2020-02-07T17:14:07.3691046Z error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T17:14:07.3691216Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T17:14:07.3691275Z    = note: the following crate versions were found:
2020-02-07T17:14:07.3691275Z    = note: the following crate versions were found:
2020-02-07T17:14:07.3692280Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta
2020-02-07T17:14:07.3692727Z            crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib
2020-02-07T17:14:07.3693168Z            crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib
2020-02-07T17:14:07.3693269Z error: aborting due to previous error
2020-02-07T17:14:07.3693303Z 
2020-02-07T17:14:07.3701688Z 
2020-02-07T17:14:07.3701763Z 
2020-02-07T17:14:07.3701763Z 
2020-02-07T17:14:07.3705432Z expected stderr:
2020-02-07T17:14:07.3710404Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3710907Z   --> $DIR/result_map_unit_fn_unfixable.rs:17:5
2020-02-07T17:14:07.3710965Z    |
2020-02-07T17:14:07.3711044Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T17:14:07.3711136Z 
2020-02-07T17:14:07.3711188Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3711493Z   --> $DIR/result_map_unit_fn_unfixable.rs:19:5
2020-02-07T17:14:07.3711547Z    |
2020-02-07T17:14:07.3711547Z    |
2020-02-07T17:14:07.3711603Z LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T17:14:07.3711725Z 
2020-02-07T17:14:07.3711775Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3712050Z   --> $DIR/result_map_unit_fn_unfixable.rs:23:5
2020-02-07T17:14:07.3712120Z    |
2020-02-07T17:14:07.3712120Z    |
2020-02-07T17:14:07.3712169Z LL |     x.field.map(|value| {
2020-02-07T17:14:07.3712265Z 
2020-02-07T17:14:07.3712334Z error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3712601Z   --> $DIR/result_map_unit_fn_unfixable.rs:27:5
2020-02-07T17:14:07.3712654Z    |
2020-02-07T17:14:07.3712654Z    |
2020-02-07T17:14:07.3712725Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T17:14:07.3712814Z 
2020-02-07T17:14:07.3712863Z error: aborting due to 4 previous errors
2020-02-07T17:14:07.3712915Z 
2020-02-07T17:14:07.3713204Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:14:07.3713204Z For more information about this error, try `rustc --explain E0425`.
2020-02-07T17:14:07.3717032Z 
2020-02-07T17:14:07.3717105Z 
2020-02-07T17:14:07.3720819Z diff of stderr:
2020-02-07T17:14:07.3720867Z 
2020-02-07T17:14:07.3732435Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3732776Z -  --> $DIR/result_map_unit_fn_unfixable.rs:17:5
2020-02-07T17:14:07.3732846Z +error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on
2020-02-07T17:14:07.3733096Z     |
2020-02-07T17:14:07.3733453Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2020-02-07T17:14:07.3733771Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T17:14:07.3733847Z +   = note: the following crate versions were found:
2020-02-07T17:14:07.3733847Z +   = note: the following crate versions were found:
2020-02-07T17:14:07.3734245Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta
2020-02-07T17:14:07.3734766Z +           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib
2020-02-07T17:14:07.3735213Z +           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib
2020-02-07T17:14:07.3735534Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3735832Z -  --> $DIR/result_map_unit_fn_unfixable.rs:19:5
2020-02-07T17:14:07.3736044Z -   |
2020-02-07T17:14:07.3736044Z -   |
2020-02-07T17:14:07.3736339Z -LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2020-02-07T17:14:07.3736664Z +error: aborting due to previous error
2020-02-07T17:14:07.3736713Z  
2020-02-07T17:14:07.3736988Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3737253Z -  --> $DIR/result_map_unit_fn_unfixable.rs:23:5
2020-02-07T17:14:07.3737253Z -  --> $DIR/result_map_unit_fn_unfixable.rs:23:5
2020-02-07T17:14:07.3737459Z -   |
2020-02-07T17:14:07.3737706Z -LL |     x.field.map(|value| {
2020-02-07T17:14:07.3738176Z -
2020-02-07T17:14:07.3738434Z -error[E0425]: cannot find value `x` in this scope
2020-02-07T17:14:07.3738709Z -  --> $DIR/result_map_unit_fn_unfixable.rs:27:5
2020-02-07T17:14:07.3738915Z -   |
2020-02-07T17:14:07.3738915Z -   |
2020-02-07T17:14:07.3739194Z -LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2020-02-07T17:14:07.3739669Z -
2020-02-07T17:14:07.3739923Z -error: aborting due to 4 previous errors
2020-02-07T17:14:07.3740124Z -
2020-02-07T17:14:07.3740421Z -For more information about this error, try `rustc --explain E0425`.
---
2020-02-07T17:14:07.3744500Z 
2020-02-07T17:14:07.3744772Z ------------------------------------------
2020-02-07T17:14:07.3744827Z stderr:
2020-02-07T17:14:07.3745072Z ------------------------------------------
2020-02-07T17:14:07.3747759Z {"message":"found possibly newer version of crate `if_chain` which `clippy_lints` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/result_map_unit_fn_unfixable.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"#![warn(clippy::result_map_unit_fn)]","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta\ncrate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib\ncrate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `if_chain` which `clippy_lints` depends on\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rmeta\n           crate `if_chain`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-632e1c1b50ff1f42.rlib\n           crate `clippy_lints`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib\n\n"}
2020-02-07T17:14:07.3748251Z 
2020-02-07T17:14:07.3748543Z ------------------------------------------
2020-02-07T17:14:07.3748581Z 
2020-02-07T17:14:07.3759891Z test [ui] ui/result_map_unit_fn_unfixable.rs ... FAILED
---
2020-02-07T17:14:08.1081421Z expected stderr:
2020-02-07T17:14:08.1081491Z error: you should not implement `visit_string` without also implementing `visit_str`
2020-02-07T17:14:08.1081726Z   --> $DIR/serde.rs:39:5
2020-02-07T17:14:08.1081766Z    |
2020-02-07T17:14:08.1082009Z LL | /     fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
2020-02-07T17:14:08.1082094Z LL | |         E: serde::de::Error,
2020-02-07T17:14:08.1082151Z LL | |     {
2020-02-07T17:14:08.1082190Z LL | |         unimplemented!()
2020-02-07T17:14:08.1082228Z LL | |     }
---
2020-02-07T17:14:08.1083359Z -  --> $DIR/serde.rs:39:5
2020-02-07T17:14:08.1084460Z +error[E0463]: can't find crate for `serde_derive` which `serde` depends on
2020-02-07T17:14:08.1084661Z +  --> $DIR/serde.rs:4:1
2020-02-07T17:14:08.1084701Z     |
2020-02-07T17:14:08.1084921Z -LL | /     fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
2020-02-07T17:14:08.1085360Z -LL | |         E: serde::de::Error,
2020-02-07T17:14:08.1085533Z -LL | |     {
2020-02-07T17:14:08.1085749Z -LL | |         unimplemented!()
2020-02-07T17:14:08.1085926Z -LL | |     }
---
2020-02-07T17:14:08.1097948Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base' 'serde.rs'
2020-02-07T17:14:08.1097986Z 
2020-02-07T17:14:08.1098026Z error: 1 errors occurred comparing output.
2020-02-07T17:14:08.1115396Z status: exit code: 1
2020-02-07T17:14:08.1117304Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/serde.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/serde.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-d198c1b8615cb4a6.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-1ad96b90660f59d3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/serde.stage-id.aux" "-A" "unused"
2020-02-07T17:14:08.1117952Z ------------------------------------------
2020-02-07T17:14:08.1118101Z 
2020-02-07T17:14:08.1118329Z ------------------------------------------
2020-02-07T17:14:08.1118482Z stderr:
2020-02-07T17:14:08.1118482Z stderr:
2020-02-07T17:14:08.1118737Z ------------------------------------------
2020-02-07T17:14:08.1120048Z {"message":"can't find crate for `serde_derive` which `serde` depends on","code":{"code":"E0463","explanation":"A plugin/crate was declared but cannot be found. Erroneous code example:\n\n