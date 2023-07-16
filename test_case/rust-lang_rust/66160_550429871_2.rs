\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/regex.rs","byte_start":95,"byte_end":114,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate regex;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `regex`\n  --> tests/ui/regex.rs:4:1\n   |\nLL | extern crate regex;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T17:25:15.9692219Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T17:25:15.9692298Z 
2019-11-06T17:25:15.9692517Z ------------------------------------------
2019-11-06T17:25:15.9692550Z 
2019-11-06T17:25:15.9692550Z 
2019-11-06T17:25:15.9692577Z 
2019-11-06T17:25:15.9692821Z ---- [ui] ui/used_underscore_binding.rs stdout ----
2019-11-06T17:25:15.9692869Z normalized stderr:
2019-11-06T17:25:15.9693093Z error[E0463]: can't find crate for `clippy_lints`
2019-11-06T17:25:15.9693312Z   --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T17:25:15.9693376Z    |
2019-11-06T17:25:15.9693419Z LL |   #[derive(DeriveSomething)]
2019-11-06T17:25:15.9693532Z LL | | struct Baz;
2019-11-06T17:25:15.9693573Z LL | |
2019-11-06T17:25:15.9693573Z LL | |
2019-11-06T17:25:15.9693615Z LL | | macro_rules! test_macro {
2019-11-06T17:25:15.9693716Z LL | |
2019-11-06T17:25:15.9693764Z LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T17:25:15.9693975Z    | |_^ can't find crate
2019-11-06T17:25:15.9694023Z 
2019-11-06T17:25:15.9694023Z 
2019-11-06T17:25:15.9694074Z error: aborting due to previous error
2019-11-06T17:25:15.9694102Z 
2019-11-06T17:25:15.9694345Z For more information about this error, try `rustc --explain E0463`.
2019-11-06T17:25:15.9694400Z 
2019-11-06T17:25:15.9694425Z 
2019-11-06T17:25:15.9694466Z expected stderr:
2019-11-06T17:25:15.9694521Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9694819Z    |
2019-11-06T17:25:15.9694819Z    |
2019-11-06T17:25:15.9694879Z LL |     _foo + 1
2019-11-06T17:25:15.9694968Z    |
2019-11-06T17:25:15.9695215Z    = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T17:25:15.9695266Z 
2019-11-06T17:25:15.9695266Z 
2019-11-06T17:25:15.9695318Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9695684Z    |
2019-11-06T17:25:15.9695732Z LL |     println!("{}", _foo);
2019-11-06T17:25:15.9695776Z    |                    ^^^^
2019-11-06T17:25:15.9695807Z 
2019-11-06T17:25:15.9695807Z 
2019-11-06T17:25:15.9695878Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9696169Z    |
2019-11-06T17:25:15.9696169Z    |
2019-11-06T17:25:15.9696231Z LL |     assert_eq!(_foo, _foo);
2019-11-06T17:25:15.9696302Z 
2019-11-06T17:25:15.9696302Z 
2019-11-06T17:25:15.9696429Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9697117Z    |
2019-11-06T17:25:15.9697117Z    |
2019-11-06T17:25:15.9697159Z LL |     assert_eq!(_foo, _foo);
2019-11-06T17:25:15.9697253Z 
2019-11-06T17:25:15.9697253Z 
2019-11-06T17:25:15.9697318Z error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9697625Z    |
2019-11-06T17:25:15.9697625Z    |
2019-11-06T17:25:15.9697668Z LL |     s._underscore_field += 1;
2019-11-06T17:25:15.9697758Z 
2019-11-06T17:25:15.9697799Z error: aborting due to 5 previous errors
2019-11-06T17:25:15.9697828Z 
2019-11-06T17:25:15.9697853Z 
2019-11-06T17:25:15.9697853Z 
2019-11-06T17:25:15.9697877Z 
2019-11-06T17:25:15.9697936Z diff of stderr:
2019-11-06T17:25:15.9697971Z 
2019-11-06T17:25:15.9698277Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9698752Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T17:25:15.9698973Z +  --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T17:25:15.9699026Z     |
2019-11-06T17:25:15.9699026Z     |
2019-11-06T17:25:15.9699239Z -LL |     _foo + 1
2019-11-06T17:25:15.9699608Z -   |
2019-11-06T17:25:15.9699871Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T17:25:15.9699871Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T17:25:15.9699921Z +LL |   #[derive(DeriveSomething)]
2019-11-06T17:25:15.9700025Z +LL | | struct Baz;
2019-11-06T17:25:15.9700066Z +LL | |
2019-11-06T17:25:15.9700066Z +LL | |
2019-11-06T17:25:15.9700108Z +LL | | macro_rules! test_macro {
2019-11-06T17:25:15.9700149Z +...  |
2019-11-06T17:25:15.9700264Z +LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T17:25:15.9700474Z +   | |_^ can't find crate
2019-11-06T17:25:15.9700535Z  
2019-11-06T17:25:15.9700535Z  
2019-11-06T17:25:15.9700833Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9701271Z -   |
2019-11-06T17:25:15.9701480Z -LL |     println!("{}", _foo);
2019-11-06T17:25:15.9701683Z -   |                    ^^^^
2019-11-06T17:25:15.9701729Z +error: aborting due to previous error
2019-11-06T17:25:15.9701729Z +error: aborting due to previous error
2019-11-06T17:25:15.9701788Z  
2019-11-06T17:25:15.9702083Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9702515Z -   |
2019-11-06T17:25:15.9702515Z -   |
2019-11-06T17:25:15.9702719Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T17:25:15.9703122Z -
2019-11-06T17:25:15.9703122Z -
2019-11-06T17:25:15.9703412Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9703835Z -   |
2019-11-06T17:25:15.9703835Z -   |
2019-11-06T17:25:15.9704041Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T17:25:15.9705304Z -
2019-11-06T17:25:15.9705304Z -
2019-11-06T17:25:15.9705661Z -error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T17:25:15.9706075Z -   |
2019-11-06T17:25:15.9706075Z -   |
2019-11-06T17:25:15.9706302Z -LL |     s._underscore_field += 1;
2019-11-06T17:25:15.9710396Z -
2019-11-06T17:25:15.9710659Z -error: aborting due to 5 previous errors
2019-11-06T17:25:15.9711017Z -
2019-11-06T17:25:15.9711517Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T17:25:15.9714291Z 
2019-11-06T17:25:15.9714886Z ------------------------------------------
2019-11-06T17:25:15.9714940Z stderr:
2019-11-06T17:25:15.9715162Z ------------------------------------------
2019-11-06T17:25:15.9722202Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n