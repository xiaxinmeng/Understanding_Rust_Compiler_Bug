\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/serde.rs","byte_start":57,"byte_end":76,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate serde;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `serde_derive` which `serde` depends on\n  --> tests/ui/serde.rs:4:1\n   |\nLL | extern crate serde;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2020-02-07T17:14:08.1120651Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2020-02-07T17:14:08.1120720Z 
2020-02-07T17:14:08.1121066Z ------------------------------------------
2020-02-07T17:14:08.1121098Z 
2020-02-07T17:14:08.1121098Z 
2020-02-07T17:14:08.1121142Z test [ui] ui/serde.rs ... FAILED
2020-02-07T17:14:08.4120525Z test [ui] ui/result_map_unwrap_or_else.rs ... ok
2020-02-07T17:14:08.6299557Z test [ui] ui/shadow.rs ... ok
2020-02-07T17:14:09.1527900Z test [ui] ui/similar_names.rs ... ok
2020-02-07T17:14:09.3715636Z test [ui] ui/short_circuit_statement.rs ... ok
2020-02-07T17:14:09.4364185Z normalized stderr:
2020-02-07T17:14:09.4364743Z error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T17:14:09.4366375Z    |
2020-02-07T17:14:09.4366859Z LL | use regex;
2020-02-07T17:14:09.4367577Z    |     ^^^^^
2020-02-07T17:14:09.4367651Z    |
2020-02-07T17:14:09.4367651Z    |
2020-02-07T17:14:09.4367733Z    = note: perhaps that crate needs to be recompiled?
2020-02-07T17:14:09.4367806Z    = note: the following crate versions were found:
2020-02-07T17:14:09.4368360Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta
2020-02-07T17:14:09.4368901Z            crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib
2020-02-07T17:14:09.4369629Z            crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib
2020-02-07T17:14:09.4369970Z error: aborting due to previous error
2020-02-07T17:14:09.4370001Z 
2020-02-07T17:14:09.4370051Z 
2020-02-07T17:14:09.4370439Z 
---
2020-02-07T17:14:09.4374516Z diff of stderr:
2020-02-07T17:14:09.4374544Z 
2020-02-07T17:14:09.4374821Z -error: this import is redundant
2020-02-07T17:14:09.4375055Z -  --> $DIR/single_component_path_imports.rs:6:1
2020-02-07T17:14:09.4375112Z +error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on
2020-02-07T17:14:09.4375421Z     |
2020-02-07T17:14:09.4375463Z  LL | use regex;
2020-02-07T17:14:09.4375682Z -   | ^^^^^^^^^^ help: remove it entirely
2020-02-07T17:14:09.4375745Z +   |     ^^^^^
2020-02-07T17:14:09.4375745Z +   |     ^^^^^
2020-02-07T17:14:09.4375812Z     |
2020-02-07T17:14:09.4377235Z -   = note: `-D clippy::single-component-path-imports` implied by `-D warnings`
2020-02-07T17:14:09.4380410Z +   = note: perhaps that crate needs to be recompiled?
2020-02-07T17:14:09.4384720Z +   = note: the following crate versions were found:
2020-02-07T17:14:09.4389163Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta
2020-02-07T17:14:09.4393050Z +           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib
2020-02-07T17:14:09.4396883Z +           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib
2020-02-07T17:14:09.4423507Z  error: aborting due to previous error
2020-02-07T17:14:09.4423573Z  
2020-02-07T17:14:09.4423613Z  
2020-02-07T17:14:09.4423659Z 
---
2020-02-07T17:14:09.4425088Z use serde as edres;
2020-02-07T17:14:09.4425141Z pub use serde;
2020-02-07T17:14:09.4425169Z 
2020-02-07T17:14:09.4425210Z fn main() {
2020-02-07T17:14:09.4425481Z     regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
2020-02-07T17:14:09.4425555Z 
2020-02-07T17:14:09.4425582Z 
2020-02-07T17:14:09.4425640Z expected fixed:
2020-02-07T17:14:09.4425833Z // run-rustfix
---
2020-02-07T17:14:09.4426590Z use serde as edres;
2020-02-07T17:14:09.4426650Z pub use serde;
2020-02-07T17:14:09.4426676Z 
2020-02-07T17:14:09.4426715Z fn main() {
2020-02-07T17:14:09.4427736Z     regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
2020-02-07T17:14:09.4427839Z 
2020-02-07T17:14:09.4427866Z 
2020-02-07T17:14:09.4427908Z diff of fixed:
2020-02-07T17:14:09.4428144Z 
2020-02-07T17:14:09.4428144Z 
2020-02-07T17:14:09.4428392Z  // run-rustfix
2020-02-07T17:14:09.4428611Z  // compile-flags: --edition 2018
2020-02-07T17:14:09.4428662Z  #![warn(clippy::single_component_path_imports)]
2020-02-07T17:14:09.4428727Z  #![allow(unused_imports)]
2020-02-07T17:14:09.4428767Z  
2020-02-07T17:14:09.4428944Z -
2020-02-07T17:14:09.4429005Z +use regex;
2020-02-07T17:14:09.4429048Z  use serde as edres;
2020-02-07T17:14:09.4429146Z  
2020-02-07T17:14:09.4429189Z  fn main() {
2020-02-07T17:14:09.4429189Z  fn main() {
2020-02-07T17:14:09.4429428Z      regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
2020-02-07T17:14:09.4429638Z  
2020-02-07T17:14:09.4429665Z 
2020-02-07T17:14:09.4429710Z The actual fixed differed from the expected fixed.
2020-02-07T17:14:09.4430150Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/single_component_path_imports.fixed
2020-02-07T17:14:09.4430150Z Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/single_component_path_imports.fixed
2020-02-07T17:14:09.4430226Z To update references, run this command from build directory:
2020-02-07T17:14:09.4430627Z tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base' 'single_component_path_imports.rs'
2020-02-07T17:14:09.4430688Z 
2020-02-07T17:14:09.4430733Z error: 2 errors occurred comparing output.
2020-02-07T17:14:09.4430777Z status: exit code: 1
2020-02-07T17:14:09.4432553Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/single_component_path_imports.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/single_component_path_imports.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde_derive-d198c1b8615cb4a6.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libserde-1ad96b90660f59d3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libclippy_lints-6e48cd16f0a16c6a.rlib" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-be13bf2e0a4c988e/out/test_build_base/single_component_path_imports.stage-id.aux" "-A" "unused"
2020-02-07T17:14:09.4433017Z ------------------------------------------
2020-02-07T17:14:09.4433061Z 
2020-02-07T17:14:09.4433290Z ------------------------------------------
2020-02-07T17:14:09.4449850Z stderr:
2020-02-07T17:14:09.4449850Z stderr:
2020-02-07T17:14:09.4450244Z ------------------------------------------
2020-02-07T17:14:09.4452720Z {"message":"found possibly newer version of crate `aho_corasick` which `regex` depends on","code":{"code":"E0460","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/single_component_path_imports.rs","byte_start":127,"byte_end":132,"line_start":6,"line_end":6,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"use regex;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"perhaps that crate needs to be recompiled?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"the following crate versions were found:\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta\ncrate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib\ncrate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0460]: found possibly newer version of crate `aho_corasick` which `regex` depends on\n  --> tests/ui/single_component_path_imports.rs:6:5\n   |\nLL | use regex;\n   |     ^^^^^\n   |\n   = note: perhaps that crate needs to be recompiled?\n   = note: the following crate versions were found:\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rmeta\n           crate `aho_corasick`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libaho_corasick-eefc1e66112f669c.rlib\n           crate `regex`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/debug/deps/libregex-54d9fd317d94ed04.rlib\n\n"}
2020-02-07T17:14:09.4453208Z 
2020-02-07T17:14:09.4453534Z ------------------------------------------
2020-02-07T17:14:09.4453568Z 
2020-02-07T17:14:09.4453623Z test [ui] ui/single_component_path_imports.rs ... FAILED
---
2020-02-07T17:30:48.9796187Z +For more information about this error, try `rustc --explain E0061`.
2020-02-07T17:30:48.9797273Z +
2020-02-07T17:30:48.9798206Z 
2020-02-07T17:30:48.9799164Z The actual stderr differed from the expected stderr.
2020-02-07T17:30:48.9800138Z Actual stderr saved to /tmp/compiletestP7AKzN/generator.stderr
2020-02-07T17:30:48.9801096Z To update references, run this command from build directory:
2020-02-07T17:30:48.9802452Z tests/run-pass/update-references.sh '/tmp/compiletestP7AKzN' 'generator.rs'
2020-02-07T17:30:48.9828677Z error: 1 errors occurred comparing output.
2020-02-07T17:30:48.9830915Z status: exit code: 1
2020-02-07T17:30:48.9830915Z status: exit code: 1
2020-02-07T17:30:48.9831712Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestP7AKzN" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestP7AKzN/generator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestP7AKzN/generator.stage-id.aux" "-A" "unused"
2020-02-07T17:30:48.9832070Z ------------------------------------------
2020-02-07T17:30:48.9832106Z 
2020-02-07T17:30:48.9832324Z ------------------------------------------
2020-02-07T17:30:48.9832368Z stderr:
2020-02-07T17:30:48.9832368Z stderr:
2020-02-07T17:30:48.9832590Z ------------------------------------------
2020-02-07T17:30:48.9834871Z {"message":"this function takes 1 parameter but 0 parameters were supplied","code":{"code":"E0061","explanation":"An invalid number of arguments was passed when calling a function.\n\nErroneous code example:\n\n