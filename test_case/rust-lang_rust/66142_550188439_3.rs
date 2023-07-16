\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":7513112,"line_start":12,"line_end":22,"column_start":10,"column_end":2,"is_primary":true,"text":[],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/used_underscore_binding.rs","byte_start":340,"byte_end":355,"line_start":12,"line_end":12,"column_start":10,"column_end":25,"is_primary":false,"text":[{"text":"#[derive(DeriveSomething)]","highlight_start":10,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"#[derive(DeriveSomething)]","def_site_span":{"file_name":"/checkout/src/tools/clippy/tests/ui/auxiliary/proc_macro_derive.rs","byte_start":215,"byte_end":549,"line_start":11,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":1},{"text":"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0463]: can't find crate for `clippy_lints`\n  --> tests/ui/used_underscore_binding.rs:12:10\n   |\nLL |   #[derive(DeriveSomething)]\n   |  __________^\nLL | | struct Baz;\nLL | |\nLL | | macro_rules! test_macro {\n...  |\nLL | |\nLL | | /// Tests that we lint if we use a binding with a single leading underscore\n   | |_^ can't find crate\n\n"}
2019-11-06T07:13:13.5322076Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T07:13:13.5322129Z 
2019-11-06T07:13:13.5322362Z ------------------------------------------
2019-11-06T07:13:13.5322535Z 
---
2019-11-06T07:13:13.5324246Z expected stderr:
2019-11-06T07:13:13.5324288Z error: useless lint attribute
2019-11-06T07:13:13.5324502Z   --> $DIR/useless_attribute.rs:6:1
2019-11-06T07:13:13.5324562Z    |
2019-11-06T07:13:13.5324603Z LL | #[allow(dead_code)]
2019-11-06T07:13:13.5324671Z    | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T07:13:13.5324973Z    = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T07:13:13.5325007Z 
2019-11-06T07:13:13.5325047Z error: useless lint attribute
2019-11-06T07:13:13.5325275Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T07:13:13.5325275Z   --> $DIR/useless_attribute.rs:7:1
2019-11-06T07:13:13.5325320Z    |
2019-11-06T07:13:13.5325548Z LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T07:13:13.5325884Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T07:13:13.5325971Z error: aborting due to 2 previous errors
2019-11-06T07:13:13.5325999Z 
2019-11-06T07:13:13.5326041Z 
2019-11-06T07:13:13.5326067Z 
2019-11-06T07:13:13.5326067Z 
2019-11-06T07:13:13.5326106Z diff of stderr:
2019-11-06T07:13:13.5326133Z 
2019-11-06T07:13:13.5326355Z -error: useless lint attribute
2019-11-06T07:13:13.5326843Z -  --> $DIR/useless_attribute.rs:6:1
2019-11-06T07:13:13.5327132Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T07:13:13.5327349Z +  --> $DIR/useless_attribute.rs:14:1
2019-11-06T07:13:13.5327415Z     |
2019-11-06T07:13:13.5327614Z -LL | #[allow(dead_code)]
2019-11-06T07:13:13.5327870Z -   | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
2019-11-06T07:13:13.5328308Z -   = note: `-D clippy::useless-attribute` implied by `-D warnings`
2019-11-06T07:13:13.5328357Z +LL | extern crate clippy_lints;
2019-11-06T07:13:13.5328595Z +   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
2019-11-06T07:13:13.5328640Z  
2019-11-06T07:13:13.5328640Z  
2019-11-06T07:13:13.5328842Z -error: useless lint attribute
2019-11-06T07:13:13.5329067Z -  --> $DIR/useless_attribute.rs:7:1
2019-11-06T07:13:13.5329249Z -   |
2019-11-06T07:13:13.5329479Z -LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
2019-11-06T07:13:13.5329814Z -   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
2019-11-06T07:13:13.5329931Z  
2019-11-06T07:13:13.5330146Z -error: aborting due to 2 previous errors
2019-11-06T07:13:13.5330341Z -
2019-11-06T07:13:13.5330578Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T07:13:13.5333464Z 
2019-11-06T07:13:13.5333675Z ------------------------------------------
2019-11-06T07:13:13.5333738Z stderr:
2019-11-06T07:13:13.5333949Z ------------------------------------------
2019-11-06T07:13:13.5335219Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n