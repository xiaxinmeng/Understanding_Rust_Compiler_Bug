\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":7513128,"line_start":12,"line_end":22,"column_start":10,"column_end":2,"is_primary":true,"text":[],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":355,"line_start":12,"line_end":12,"column_start":10,"column_end":25,"is_primary":false,"text":[{"text":"#[derive(DeriveSomething)]","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(DeriveSomething)]","def_site_span":{"file_name":"/checkout/src/tools/clippy/tests/ui/auxiliary/proc_macro_derive.rs","byte_start":215,"byte_end":549,"line_start":11,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/used_underscore_binding.rs:12:10\n   |\nLL |   #[derive(DeriveSomething)]\n   |  __________^\nLL | | struct Baz;\nLL | |\nLL | | macro_rules! test_macro {\n...  |\nLL | |\nLL | | /// Tests that we lint if we use a binding with a single leading underscore\n   | |_^ can't find crate\n\n"}
2019-11-06T16:48:44.8380811Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T16:48:44.8380892Z 
2019-11-06T16:48:44.8381128Z ------------------------------------------
2019-11-06T16:48:44.8381163Z 
---
2019-11-06T16:48:44.8382831Z expected stderr:
2019-11-06T16:48:44.8382877Z error: useless lint attribute
2019-11-06T16:48:44.8383125Z   --> $DIR/useless_attribute.rs:6:1
2019-11-06T16:48:44.8383171Z    |
2019-11-06T16:48:44.8383215Z LL | #[allow(dead_code)]
2019-11-06T16:48:44.8383455Z    | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T16:48:44.8383791Z    = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T16:48:44.8383826Z 
2019-11-06T16:48:44.8383887Z error: useless lint attribute
2019-11-06T16:48:44.8384113Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T16:48:44.8384113Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T16:48:44.8384160Z    |
2019-11-06T16:48:44.8384429Z LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T16:48:44.8384776Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T16:48:44.8384986Z error: aborting due to 2 previous errors
2019-11-06T16:48:44.8385015Z 
2019-11-06T16:48:44.8385042Z 
2019-11-06T16:48:44.8385067Z 
2019-11-06T16:48:44.8385067Z 
2019-11-06T16:48:44.8385108Z diff of stderr:
2019-11-06T16:48:44.8385153Z 
2019-11-06T16:48:44.8385400Z -error: useless lint attribute
2019-11-06T16:48:44.8385640Z -  --> $DIR/useless_attribute.rs:6:1
2019-11-06T16:48:44.8385905Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T16:48:44.8386510Z +  --> $DIR/useless_attribute.rs:14:1
2019-11-06T16:48:44.8386568Z     |
2019-11-06T16:48:44.8387144Z -LL | #[allow(dead_code)]
2019-11-06T16:48:44.8387457Z -   | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T16:48:44.8387914Z -   = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T16:48:44.8387985Z +LL | extern crate clippy_lints;
2019-11-06T16:48:44.8388225Z +   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T16:48:44.8388273Z  
2019-11-06T16:48:44.8388273Z  
2019-11-06T16:48:44.8388519Z -error: useless lint attribute
2019-11-06T16:48:44.8388750Z -  --> $DIR/useless_attribute.rs:7:1
2019-11-06T16:48:44.8388947Z -   |
2019-11-06T16:48:44.8393223Z -LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T16:48:44.8393669Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T16:48:44.8393792Z  
2019-11-06T16:48:44.8394032Z -error: aborting due to 2 previous errors
2019-11-06T16:48:44.8394229Z -
2019-11-06T16:48:44.8395940Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T16:48:44.8399540Z 
2019-11-06T16:48:44.8399793Z ------------------------------------------
2019-11-06T16:48:44.8399841Z stderr:
2019-11-06T16:48:44.8400066Z ------------------------------------------
2019-11-06T16:48:44.8401400Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n