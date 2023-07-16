\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/regex.rs","byte_start":95,"byte_end":114,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate regex;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `regex`\n  --> tests/ui/regex.rs:4:1\n   |\nLL | extern crate regex;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T16:48:44.8357190Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T16:48:44.8357245Z 
2019-11-06T16:48:44.8357480Z ------------------------------------------
2019-11-06T16:48:44.8357531Z 
2019-11-06T16:48:44.8357531Z 
2019-11-06T16:48:44.8357560Z 
2019-11-06T16:48:44.8357800Z ---- [ui] ui/used_underscore_binding.rs stdout ----
2019-11-06T16:48:44.8357851Z normalized stderr:
2019-11-06T16:48:44.8358105Z error[E0463]: can't find crate for `clippy_lints`
2019-11-06T16:48:44.8358345Z   --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T16:48:44.8358392Z    |
2019-11-06T16:48:44.8358453Z LL |   #[derive(DeriveSomething)]
2019-11-06T16:48:44.8358543Z LL | | struct Baz;
2019-11-06T16:48:44.8358584Z LL | |
2019-11-06T16:48:44.8358584Z LL | |
2019-11-06T16:48:44.8358653Z LL | | macro_rules! test_macro {
2019-11-06T16:48:44.8358737Z LL | |
2019-11-06T16:48:44.8358808Z LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T16:48:44.8359036Z    | |_^ can't find crate
2019-11-06T16:48:44.8359071Z 
2019-11-06T16:48:44.8359071Z 
2019-11-06T16:48:44.8359226Z error: aborting due to previous error
2019-11-06T16:48:44.8359281Z 
2019-11-06T16:48:44.8359563Z For more information about this error, try `rustc --explain E0463`.
2019-11-06T16:48:44.8359599Z 
2019-11-06T16:48:44.8359624Z 
2019-11-06T16:48:44.8359683Z expected stderr:
2019-11-06T16:48:44.8359741Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8360054Z    |
2019-11-06T16:48:44.8360054Z    |
2019-11-06T16:48:44.8360096Z LL |     _foo + 1
2019-11-06T16:48:44.8360198Z    |
2019-11-06T16:48:44.8360561Z    = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T16:48:44.8360597Z 
2019-11-06T16:48:44.8360597Z 
2019-11-06T16:48:44.8360651Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8360971Z    |
2019-11-06T16:48:44.8361015Z LL |     println!("{}", _foo);
2019-11-06T16:48:44.8361077Z    |                    ^^^^
2019-11-06T16:48:44.8361106Z 
2019-11-06T16:48:44.8361106Z 
2019-11-06T16:48:44.8361159Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8361470Z    |
2019-11-06T16:48:44.8361470Z    |
2019-11-06T16:48:44.8361513Z LL |     assert_eq!(_foo, _foo);
2019-11-06T16:48:44.8361607Z 
2019-11-06T16:48:44.8361607Z 
2019-11-06T16:48:44.8361659Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8361977Z    |
2019-11-06T16:48:44.8361977Z    |
2019-11-06T16:48:44.8362022Z LL |     assert_eq!(_foo, _foo);
2019-11-06T16:48:44.8362093Z 
2019-11-06T16:48:44.8362093Z 
2019-11-06T16:48:44.8362176Z error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8362467Z    |
2019-11-06T16:48:44.8362467Z    |
2019-11-06T16:48:44.8362526Z LL |     s._underscore_field += 1;
2019-11-06T16:48:44.8362601Z 
2019-11-06T16:48:44.8362644Z error: aborting due to 5 previous errors
2019-11-06T16:48:44.8362690Z 
2019-11-06T16:48:44.8362716Z 
2019-11-06T16:48:44.8362716Z 
2019-11-06T16:48:44.8362741Z 
2019-11-06T16:48:44.8362782Z diff of stderr:
2019-11-06T16:48:44.8362810Z 
2019-11-06T16:48:44.8363148Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8363649Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T16:48:44.8363913Z +  --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T16:48:44.8363963Z     |
2019-11-06T16:48:44.8363963Z     |
2019-11-06T16:48:44.8364174Z -LL |     _foo + 1
2019-11-06T16:48:44.8364593Z -   |
2019-11-06T16:48:44.8364871Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T16:48:44.8364871Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T16:48:44.8364924Z +LL |   #[derive(DeriveSomething)]
2019-11-06T16:48:44.8365105Z +LL | | struct Baz;
2019-11-06T16:48:44.8365148Z +LL | |
2019-11-06T16:48:44.8365148Z +LL | |
2019-11-06T16:48:44.8365341Z +LL | | macro_rules! test_macro {
2019-11-06T16:48:44.8365383Z +...  |
2019-11-06T16:48:44.8365473Z +LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T16:48:44.8365832Z +   | |_^ can't find crate
2019-11-06T16:48:44.8365876Z  
2019-11-06T16:48:44.8365876Z  
2019-11-06T16:48:44.8366186Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8367275Z -   |
2019-11-06T16:48:44.8367496Z -LL |     println!("{}", _foo);
2019-11-06T16:48:44.8367736Z -   |                    ^^^^
2019-11-06T16:48:44.8367785Z +error: aborting due to previous error
2019-11-06T16:48:44.8367785Z +error: aborting due to previous error
2019-11-06T16:48:44.8367827Z  
2019-11-06T16:48:44.8368153Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8368598Z -   |
2019-11-06T16:48:44.8368598Z -   |
2019-11-06T16:48:44.8368835Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T16:48:44.8369349Z -
2019-11-06T16:48:44.8369349Z -
2019-11-06T16:48:44.8369660Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8370124Z -   |
2019-11-06T16:48:44.8370124Z -   |
2019-11-06T16:48:44.8370344Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T16:48:44.8370789Z -
2019-11-06T16:48:44.8370789Z -
2019-11-06T16:48:44.8371124Z -error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T16:48:44.8371581Z -   |
2019-11-06T16:48:44.8371581Z -   |
2019-11-06T16:48:44.8371806Z -LL |     s._underscore_field += 1;
2019-11-06T16:48:44.8372240Z -
2019-11-06T16:48:44.8372471Z -error: aborting due to 5 previous errors
2019-11-06T16:48:44.8372663Z -
2019-11-06T16:48:44.8372939Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T16:48:44.8375806Z 
2019-11-06T16:48:44.8376039Z ------------------------------------------
2019-11-06T16:48:44.8376085Z stderr:
2019-11-06T16:48:44.8376633Z ------------------------------------------
2019-11-06T16:48:44.8379680Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n