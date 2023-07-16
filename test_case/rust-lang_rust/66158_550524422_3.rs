\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":7513232,"line_start":12,"line_end":22,"column_start":10,"column_end":2,"is_primary":true,"text":[],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":355,"line_start":12,"line_end":12,"column_start":10,"column_end":25,"is_primary":false,"text":[{"text":"#[derive(DeriveSomething)]","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(DeriveSomething)]","def_site_span":{"file_name":"/checkout/src/tools/clippy/tests/ui/auxiliary/proc_macro_derive.rs","byte_start":215,"byte_end":549,"line_start":11,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/used_underscore_binding.rs:12:10\n   |\nLL |   #[derive(DeriveSomething)]\n   |  __________^\nLL | | struct Baz;\nLL | |\nLL | | macro_rules! test_macro {\n...  |\nLL | |\nLL | | /// Tests that we lint if we use a binding with a single leading underscore\n   | |_^ can't find crate\n\n"}
2019-11-06T21:36:06.7166071Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T21:36:06.7166111Z 
2019-11-06T21:36:06.7166293Z ------------------------------------------
2019-11-06T21:36:06.7166320Z 
---
2019-11-06T21:36:06.7168380Z expected stderr:
2019-11-06T21:36:06.7168424Z error: useless lint attribute
2019-11-06T21:36:06.7168652Z   --> $DIR/useless_attribute.rs:6:1
2019-11-06T21:36:06.7168697Z    |
2019-11-06T21:36:06.7168739Z LL | #[allow(dead_code)]
2019-11-06T21:36:06.7168789Z    | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T21:36:06.7169086Z    = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T21:36:06.7169119Z 
2019-11-06T21:36:06.7169185Z error: useless lint attribute
2019-11-06T21:36:06.7169394Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T21:36:06.7169394Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T21:36:06.7169437Z    |
2019-11-06T21:36:06.7169662Z LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T21:36:06.7170007Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T21:36:06.7170094Z error: aborting due to 2 previous errors
2019-11-06T21:36:06.7170139Z 
2019-11-06T21:36:06.7170165Z 
2019-11-06T21:36:06.7170190Z 
2019-11-06T21:36:06.7170190Z 
2019-11-06T21:36:06.7170229Z diff of stderr:
2019-11-06T21:36:06.7170424Z 
2019-11-06T21:36:06.7170633Z -error: useless lint attribute
2019-11-06T21:36:06.7170994Z -  --> $DIR/useless_attribute.rs:6:1
2019-11-06T21:36:06.7171196Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T21:36:06.7171399Z +  --> $DIR/useless_attribute.rs:14:1
2019-11-06T21:36:06.7171440Z     |
2019-11-06T21:36:06.7171612Z -LL | #[allow(dead_code)]
2019-11-06T21:36:06.7171862Z -   | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T21:36:06.7172237Z -   = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T21:36:06.7172290Z +LL | extern crate clippy_lints;
2019-11-06T21:36:06.7172503Z +   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T21:36:06.7172543Z  
2019-11-06T21:36:06.7172543Z  
2019-11-06T21:36:06.7172717Z -error: useless lint attribute
2019-11-06T21:36:06.7172918Z -  --> $DIR/useless_attribute.rs:7:1
2019-11-06T21:36:06.7173075Z -   |
2019-11-06T21:36:06.7173440Z -LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T21:36:06.7173886Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T21:36:06.7173968Z  
2019-11-06T21:36:06.7174155Z -error: aborting due to 2 previous errors
2019-11-06T21:36:06.7174306Z -
2019-11-06T21:36:06.7174499Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T21:36:06.7177961Z 
2019-11-06T21:36:06.7178171Z ------------------------------------------
2019-11-06T21:36:06.7178232Z stderr:
2019-11-06T21:36:06.7178439Z ------------------------------------------
2019-11-06T21:36:06.7179744Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n