\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/serde.rs","byte_start":57,"byte_end":76,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate serde;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `serde_derive` which `serde` depends on\n  --> tests/ui/serde.rs:4:1\n   |\nLL | extern crate serde;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2020-02-07T02:04:01.3143337Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2020-02-07T02:04:01.3143402Z 
2020-02-07T02:04:01.3143621Z ------------------------------------------
2020-02-07T02:04:01.3143664Z 
2020-02-07T02:04:01.3143664Z 
2020-02-07T02:04:01.3143723Z test [ui] ui/serde.rs ... FAILED
2020-02-07T02:04:01.4730152Z test [ui] ui/result_map_unwrap_or_else.rs ... ok
2020-02-07T02:04:01.7952303Z test [ui] ui/shadow.rs ... ok
2020-02-07T02:04:02.3335047Z test [ui] ui/similar_names.rs ... ok
2020-02-07T02:04:02.4841686Z test [ui] ui/short_circuit_statement.rs ... ok
2020-02-07T02:04:02.5568838Z normalized stderr:
2020-02-07T02:04:02.5574413Z error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T02:04:02.5575139Z    |
2020-02-07T02:04:02.5575202Z LL | use regex;
2020-02-07T02:04:02.5575241Z    |     ^^^^^
2020-02-07T02:04:02.5575278Z    |
2020-02-07T02:04:02.5575278Z    |
2020-02-07T02:04:02.5575338Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T02:04:02.5575410Z    = note: the following crate versions were found:
2020-02-07T02:04:02.5575758Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta
2020-02-07T02:04:02.5576157Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib
2020-02-07T02:04:02.5576462Z            crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib
2020-02-07T02:04:02.5576563Z error: aborting due to previous error
2020-02-07T02:04:02.5576593Z 
2020-02-07T02:04:02.5580890Z 
2020-02-07T02:04:02.5580957Z 
---
2020-02-07T02:04:02.5599741Z diff of stderr:
2020-02-07T02:04:02.5599785Z 
2020-02-07T02:04:02.5604120Z -error: this import is redundant
2020-02-07T02:04:02.5625951Z -  --> $DIR/single_component_path_imports.rs:6:1
2020-02-07T02:04:02.5626051Z +error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T02:04:02.5626314Z     |
2020-02-07T02:04:02.5626368Z  LL | use regex;
2020-02-07T02:04:02.5626562Z -   | ^^^^^^^^^^ help: remove it entirely
2020-02-07T02:04:02.5626620Z +   |     ^^^^^
2020-02-07T02:04:02.5626620Z +   |     ^^^^^
2020-02-07T02:04:02.5626657Z     |
2020-02-07T02:04:02.5626906Z -   = note: `-D clippy::single-component-path-imports` implied by `-D warnings`
2020-02-07T02:04:02.5626964Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T02:04:02.5627008Z +   = note: the following crate versions were found:
2020-02-07T02:04:02.5627342Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta
2020-02-07T02:04:02.5627694Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib
2020-02-07T02:04:02.5628015Z +           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib
2020-02-07T02:04:02.5628109Z  error: aborting due to previous error
2020-02-07T02:04:02.5628166Z  
2020-02-07T02:04:02.5628212Z  
2020-02-07T02:04:02.5628238Z 
---
2020-02-07T02:04:02.5629347Z use serde as edres;
2020-02-07T02:04:02.5629388Z pub use serde;
2020-02-07T02:04:02.5629415Z 
2020-02-07T02:04:02.5629473Z fn main() {
2020-02-07T02:04:02.5629698Z     regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
2020-02-07T02:04:02.5629768Z 
2020-02-07T02:04:02.5629815Z 
2020-02-07T02:04:02.5629855Z expected fixed:
2020-02-07T02:04:02.5630038Z // run-rustfix
---
2020-02-07T02:04:02.5630461Z use serde as edres;
2020-02-07T02:04:02.5630502Z pub use serde;
2020-02-07T02:04:02.5630527Z 
2020-02-07T02:04:02.5630583Z fn main() {
2020-02-07T02:04:02.5630808Z     regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
2020-02-07T02:04:02.5630878Z 
2020-02-07T02:04:02.5630920Z 
2020-02-07T02:04:02.5630960Z diff of fixed:
2020-02-07T02:04:02.5630985Z 
2020-02-07T02:04:02.5630985Z 
2020-02-07T02:04:02.5631165Z  // run-rustfix
2020-02-07T02:04:02.5631381Z  // compile-flags: --edition 2018
2020-02-07T02:04:02.5631429Z  #![warn(clippy::single_component_path_imports)]
2020-02-07T02:04:02.5631473Z  #![allow(unused_imports)]
2020-02-07T02:04:02.5631663Z  
2020-02-07T02:04:02.5631891Z -
2020-02-07T02:04:02.5631934Z +use regex;
2020-02-07T02:04:02.5631975Z  use serde as edres;
2020-02-07T02:04:02.5632159Z  
2020-02-07T02:04:02.5632197Z  fn main() {
2020-02-07T02:04:02.5632197Z  fn main() {
2020-02-07T02:04:02.5632436Z      regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
2020-02-07T02:04:02.5632537Z  
2020-02-07T02:04:02.5632562Z 
2020-02-07T02:04:02.5632621Z The actual fixed differed from the expected fixed.
2020-02-07T02:04:02.5632621Z The actual fixed differed from the expected fixed.
2020-02-07T02:04:02.5632981Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/single_component_path_imports.fixed
2020-02-07T02:04:02.5633745Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base' 'single_component_path_imports.rs'
2020-02-07T02:04:02.5633800Z 
2020-02-07T02:04:02.5633846Z error: 2 errors occurred comparing output.
2020-02-07T02:04:02.5633915Z status: exit code: 1
2020-02-07T02:04:02.5633915Z status: exit code: 1
2020-02-07T02:04:02.5635532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/single_component_path_imports.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/single_component_path_imports.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-93be375d9bac0399.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-434bedc32a67cf7f.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-67a7525c7f333fed.rlib" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-95793b042519011d/out/test_build_base/single_component_path_imports.stage-id.aux" "-A" "unused"
2020-02-07T02:04:02.5635974Z ------------------------------------------
2020-02-07T02:04:02.5636005Z 
2020-02-07T02:04:02.5636213Z ------------------------------------------
2020-02-07T02:04:02.5636274Z stderr:
2020-02-07T02:04:02.5636274Z stderr:
2020-02-07T02:04:02.5636480Z ------------------------------------------
2020-02-07T02:04:02.5638689Z {"message":"found possibly newer version of crate `aho_corasick` which `regex` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_component_path_imports.rs","byte_start":127,"byte_end":132,"line_start":6,"line_end":6,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"use regex;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib\ncrate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on\n  --> tests/ui/single_component_path_imports.rs:6:5\n   |\nLL | use regex;\n   |     ^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rmeta\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-d8762d60575ad19b.rlib\n           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-c5e15ccc6e05778a.rlib\n\n"}
2020-02-07T02:04:02.5639175Z 
2020-02-07T02:04:02.5639437Z ------------------------------------------
2020-02-07T02:04:02.5639468Z 
2020-02-07T02:04:02.5639528Z test [ui] ui/single_component_path_imports.rs ... FAILED
---
2020-02-07T02:44:24.0013399Z test [compile-fail] compile-fail/deref_fn_ptr.rs ... ok
2020-02-07T02:44:24.0076586Z 
2020-02-07T02:44:24.0077224Z error: error pattern ' Division by 0 in unchecked_div' not found!
2020-02-07T02:44:24.0077287Z status: exit code: 1
2020-02-07T02:44:24.0078266Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/div-by-zero-1.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/div-by-zero-1.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/div-by-zero-1.stage-id.aux" "-A" "unused"
2020-02-07T02:44:24.0114315Z ------------------------------------------
2020-02-07T02:44:24.0114376Z 
2020-02-07T02:44:24.0114800Z ------------------------------------------
2020-02-07T02:44:24.0114851Z stderr:
---
2020-02-07T02:44:24.0119432Z test [compile-fail] compile-fail/div-by-zero-1.rs ... FAILED
2020-02-07T02:44:24.1363072Z 
2020-02-07T02:44:24.1364096Z error: error pattern ' Division by 0 in unchecked_rem' not found!
2020-02-07T02:44:24.1364160Z status: exit code: 1
2020-02-07T02:44:24.1364829Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/div-by-zero-3.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/div-by-zero-3.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/div-by-zero-3.stage-id.aux" "-A" "unused"
2020-02-07T02:44:24.1365177Z ------------------------------------------
2020-02-07T02:44:24.1365213Z 
2020-02-07T02:44:24.1365463Z ------------------------------------------
2020-02-07T02:44:24.1365512Z stderr:
---
2020-02-07T02:44:31.8326606Z error: tests/compile-fail/unchecked_add1.rs:4: expected error not found: Overflowing arithmetic in unchecked_add
2020-02-07T02:44:31.8326643Z 
2020-02-07T02:44:31.8326688Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-07T02:44:31.8326750Z status: exit code: 1
2020-02-07T02:44:31.8327505Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unchecked_add1.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/unchecked_add1.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/unchecked_add1.stage-id.aux" "-A" "unused"
2020-02-07T02:44:31.8327735Z     Error {
2020-02-07T02:44:31.8327776Z         line_num: 4,
2020-02-07T02:44:31.8327815Z         kind: Some(
2020-02-07T02:44:31.8327871Z             Error,
---
2020-02-07T02:44:31.9689016Z error: tests/compile-fail/unchecked_add2.rs:4: expected error not found: Overflowing arithmetic in unchecked_add
2020-02-07T02:44:31.9689139Z 
2020-02-07T02:44:31.9689192Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-07T02:44:31.9689267Z status: exit code: 1
2020-02-07T02:44:31.9689907Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unchecked_add2.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/unchecked_add2.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/unchecked_add2.stage-id.aux" "-A" "unused"
2020-02-07T02:44:31.9690071Z     Error {
2020-02-07T02:44:31.9690116Z         line_num: 4,
2020-02-07T02:44:31.9690158Z         kind: Some(
2020-02-07T02:44:31.9690217Z             Error,
---
2020-02-07T02:44:31.9702116Z error: tests/compile-fail/unchecked_mul1.rs:4: expected error not found: Overflowing arithmetic in unchecked_mul
2020-02-07T02:44:31.9702172Z 
2020-02-07T02:44:31.9702216Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-07T02:44:31.9702434Z status: exit code: 1
2020-02-07T02:44:31.9703088Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unchecked_mul1.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/unchecked_mul1.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/unchecked_mul1.stage-id.aux" "-A" "unused"
2020-02-07T02:44:31.9703334Z     Error {
2020-02-07T02:44:31.9703377Z         line_num: 4,
2020-02-07T02:44:31.9703418Z         kind: Some(
2020-02-07T02:44:31.9703504Z             Error,
---
2020-02-07T02:44:32.1127186Z error: tests/compile-fail/unchecked_sub1.rs:4: expected error not found: Overflowing arithmetic in unchecked_sub
2020-02-07T02:44:32.1127431Z 
2020-02-07T02:44:32.1128060Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-07T02:44:32.1128232Z status: exit code: 1
2020-02-07T02:44:32.1129094Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unchecked_sub1.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/unchecked_sub1.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/unchecked_sub1.stage-id.aux" "-A" "unused"
2020-02-07T02:44:32.1129511Z     Error {
2020-02-07T02:44:32.1129666Z         line_num: 4,
2020-02-07T02:44:32.1129810Z         kind: Some(
2020-02-07T02:44:32.1129986Z             Error,
---
2020-02-07T02:44:32.1143344Z error: tests/compile-fail/unchecked_mul2.rs:4: expected error not found: Overflowing arithmetic in unchecked_mul
2020-02-07T02:44:32.1143507Z 
2020-02-07T02:44:32.1143853Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-07T02:44:32.1144225Z status: exit code: 1
2020-02-07T02:44:32.1145314Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unchecked_mul2.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/unchecked_mul2.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/unchecked_mul2.stage-id.aux" "-A" "unused"
2020-02-07T02:44:32.1145983Z     Error {
2020-02-07T02:44:32.1146178Z         line_num: 4,
2020-02-07T02:44:32.1146322Z         kind: Some(
2020-02-07T02:44:32.1146465Z             Error,
---
2020-02-07T02:44:32.2608915Z error: tests/compile-fail/unchecked_sub2.rs:4: expected error not found: Overflowing arithmetic in unchecked_sub
2020-02-07T02:44:32.2611649Z 
2020-02-07T02:44:32.2611731Z error: 1 unexpected errors found, 1 expected errors not found
2020-02-07T02:44:32.2612907Z status: exit code: 1
2020-02-07T02:44:32.2615946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/unchecked_sub2.rs" "-L" "/tmp/compiletesteFTGVN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletesteFTGVN/unchecked_sub2.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletesteFTGVN/unchecked_sub2.stage-id.aux" "-A" "unused"
2020-02-07T02:44:32.2619282Z     Error {
2020-02-07T02:44:32.2621870Z         line_num: 4,
2020-02-07T02:44:32.2623702Z         kind: Some(
2020-02-07T02:44:32.2625969Z             Error,
---
2020-02-07T02:44:34.7281027Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2020-02-07T02:44:34.7281065Z 
2020-02-07T02:44:34.7281333Z We detected that this PR updated 'clippy-driver', but its tests failed.
2020-02-07T02:44:34.7281369Z 
2020-02-07T02:44:34.7281631Z If you do intend to update 'clippy-driver', please check the error messages above and
2020-02-07T02:44:34.7281704Z commit another update.
2020-02-07T02:44:34.7281735Z 
2020-02-07T02:44:34.7281999Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2020-02-07T02:44:34.7282260Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2020-02-07T02:44:34.7282341Z proper steps.
2020-02-07T02:44:34.7296411Z Build completed unsuccessfully in 0:00:01
2020-02-07T02:44:34.7348722Z == clock drift check ==
2020-02-07T02:44:34.7364547Z   local time: Fri Feb  7 02:44:34 UTC 2020
2020-02-07T02:44:35.0240244Z   network time: Fri, 07 Feb 2020 02:44:35 GMT
2020-02-07T02:44:35.0240244Z   network time: Fri, 07 Feb 2020 02:44:35 GMT
2020-02-07T02:44:35.0243923Z == end clock drift check ==
2020-02-07T02:44:35.3512360Z 
2020-02-07T02:44:35.3576542Z ##[error]Bash exited with code '1'.
2020-02-07T02:44:35.3587723Z ##[section]Finishing: Run build
2020-02-07T02:44:35.3609332Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-07T02:44:35.3611520Z Task         : Get sources
2020-02-07T02:44:35.3611563Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T02:44:35.3611625Z Version      : 1.0.0
2020-02-07T02:44:35.3611663Z Author       : Microsoft
2020-02-07T02:44:35.3611663Z Author       : Microsoft
2020-02-07T02:44:35.3611705Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-07T02:44:35.3611926Z ==============================================================================
2020-02-07T02:44:35.7482030Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-07T02:44:35.7555310Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-07T02:44:35.7658235Z Cleaning up task key
2020-02-07T02:44:35.7658913Z Start cleaning up orphan processes.
2020-02-07T02:44:35.7759650Z Terminate orphan process: pid (3850) (python)
2020-02-07T02:44:35.7980191Z ##[section]Finishing: Finalize Job
