\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":7515520,"line_start":12,"line_end":22,"column_start":10,"column_end":2,"is_primary":true,"text":[],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":355,"line_start":12,"line_end":12,"column_start":10,"column_end":25,"is_primary":false,"text":[{"text":"#[derive(DeriveSomething)]","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(DeriveSomething)]","def_site_span":{"file_name":"/checkout/src/tools/clippy/tests/ui/auxiliary/proc_macro_derive.rs","byte_start":215,"byte_end":549,"line_start":11,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/used_underscore_binding.rs:12:10\n   |\nLL |   #[derive(DeriveSomething)]\n   |  __________^\nLL | | struct Baz;\nLL | |\nLL | | macro_rules! test_macro {\n...  |\nLL | |\nLL | | /// Tests that we lint if we use a binding with a single leading underscore\n   | |_^ can't find crate\n\n"}
2019-11-06T17:25:15.9723436Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T17:25:15.9723495Z 
2019-11-06T17:25:15.9723718Z ------------------------------------------
2019-11-06T17:25:15.9723771Z 
---
2019-11-06T17:25:15.9725359Z expected stderr:
2019-11-06T17:25:15.9725402Z error: useless lint attribute
2019-11-06T17:25:15.9725616Z   --> $DIR/useless_attribute.rs:6:1
2019-11-06T17:25:15.9725679Z    |
2019-11-06T17:25:15.9725721Z LL | #[allow(dead_code)]
2019-11-06T17:25:15.9725779Z    | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T17:25:15.9726083Z    = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T17:25:15.9726117Z 
2019-11-06T17:25:15.9726158Z error: useless lint attribute
2019-11-06T17:25:15.9726382Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T17:25:15.9726382Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T17:25:15.9726426Z    |
2019-11-06T17:25:15.9726951Z LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T17:25:15.9727309Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T17:25:15.9727411Z error: aborting due to 2 previous errors
2019-11-06T17:25:15.9727440Z 
2019-11-06T17:25:15.9727483Z 
2019-11-06T17:25:15.9727508Z 
2019-11-06T17:25:15.9727508Z 
2019-11-06T17:25:15.9727549Z diff of stderr:
2019-11-06T17:25:15.9727576Z 
2019-11-06T17:25:15.9727786Z -error: useless lint attribute
2019-11-06T17:25:15.9728017Z -  --> $DIR/useless_attribute.rs:6:1
2019-11-06T17:25:15.9728367Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T17:25:15.9728623Z +  --> $DIR/useless_attribute.rs:14:1
2019-11-06T17:25:15.9728689Z     |
2019-11-06T17:25:15.9728887Z -LL | #[allow(dead_code)]
2019-11-06T17:25:15.9729141Z -   | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T17:25:15.9729582Z -   = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T17:25:15.9729632Z +LL | extern crate clippy_lints;
2019-11-06T17:25:15.9729867Z +   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T17:25:15.9729990Z  
2019-11-06T17:25:15.9729990Z  
2019-11-06T17:25:15.9730212Z -error: useless lint attribute
2019-11-06T17:25:15.9730424Z -  --> $DIR/useless_attribute.rs:7:1
2019-11-06T17:25:15.9730627Z -   |
2019-11-06T17:25:15.9730858Z -LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T17:25:15.9731183Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T17:25:15.9731299Z  
2019-11-06T17:25:15.9731517Z -error: aborting due to 2 previous errors
2019-11-06T17:25:15.9731710Z -
2019-11-06T17:25:15.9731950Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T17:25:15.9735150Z 
2019-11-06T17:25:15.9735370Z ------------------------------------------
2019-11-06T17:25:15.9735434Z stderr:
2019-11-06T17:25:15.9735648Z ------------------------------------------
2019-11-06T17:25:15.9737252Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n