\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/regex.rs","byte_start":95,"byte_end":114,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate regex;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `regex`\n  --> tests/ui/regex.rs:4:1\n   |\nLL | extern crate regex;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T21:36:06.7143625Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T21:36:06.7143684Z 
2019-11-06T21:36:06.7143851Z ------------------------------------------
2019-11-06T21:36:06.7143877Z 
2019-11-06T21:36:06.7143877Z 
2019-11-06T21:36:06.7143916Z 
2019-11-06T21:36:06.7144089Z ---- [ui] ui/used_underscore_binding.rs stdout ----
2019-11-06T21:36:06.7144198Z normalized stderr:
2019-11-06T21:36:06.7144391Z error[E0463]: can't find crate for `clippy_lints`
2019-11-06T21:36:06.7144582Z   --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T21:36:06.7144618Z    |
2019-11-06T21:36:06.7144652Z LL |   #[derive(DeriveSomething)]
2019-11-06T21:36:06.7144744Z LL | | struct Baz;
2019-11-06T21:36:06.7144777Z LL | |
2019-11-06T21:36:06.7144777Z LL | |
2019-11-06T21:36:06.7144828Z LL | | macro_rules! test_macro {
2019-11-06T21:36:06.7144892Z LL | |
2019-11-06T21:36:06.7144931Z LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T21:36:06.7145113Z    | |_^ can't find crate
2019-11-06T21:36:06.7145140Z 
2019-11-06T21:36:06.7145140Z 
2019-11-06T21:36:06.7145175Z error: aborting due to previous error
2019-11-06T21:36:06.7145197Z 
2019-11-06T21:36:06.7145407Z For more information about this error, try `rustc --explain E0463`.
2019-11-06T21:36:06.7145434Z 
2019-11-06T21:36:06.7145455Z 
2019-11-06T21:36:06.7145495Z expected stderr:
2019-11-06T21:36:06.7145556Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7145773Z    |
2019-11-06T21:36:06.7145773Z    |
2019-11-06T21:36:06.7145830Z LL |     _foo + 1
2019-11-06T21:36:06.7145896Z    |
2019-11-06T21:36:06.7146107Z    = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T21:36:06.7146134Z 
2019-11-06T21:36:06.7146134Z 
2019-11-06T21:36:06.7146351Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7146846Z    |
2019-11-06T21:36:06.7146889Z LL |     println!("{}", _foo);
2019-11-06T21:36:06.7146933Z    |                    ^^^^
2019-11-06T21:36:06.7146980Z 
2019-11-06T21:36:06.7146980Z 
2019-11-06T21:36:06.7147033Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7147330Z    |
2019-11-06T21:36:06.7147330Z    |
2019-11-06T21:36:06.7147372Z LL |     assert_eq!(_foo, _foo);
2019-11-06T21:36:06.7147452Z 
2019-11-06T21:36:06.7147452Z 
2019-11-06T21:36:06.7147523Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7147794Z    |
2019-11-06T21:36:06.7147794Z    |
2019-11-06T21:36:06.7147853Z LL |     assert_eq!(_foo, _foo);
2019-11-06T21:36:06.7147926Z 
2019-11-06T21:36:06.7147926Z 
2019-11-06T21:36:06.7147982Z error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7148278Z    |
2019-11-06T21:36:06.7148278Z    |
2019-11-06T21:36:06.7148320Z LL |     s._underscore_field += 1;
2019-11-06T21:36:06.7148409Z 
2019-11-06T21:36:06.7148452Z error: aborting due to 5 previous errors
2019-11-06T21:36:06.7148480Z 
2019-11-06T21:36:06.7148506Z 
2019-11-06T21:36:06.7148506Z 
2019-11-06T21:36:06.7148642Z 
2019-11-06T21:36:06.7148690Z diff of stderr:
2019-11-06T21:36:06.7148717Z 
2019-11-06T21:36:06.7149032Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7149501Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T21:36:06.7149721Z +  --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T21:36:06.7149784Z     |
2019-11-06T21:36:06.7149784Z     |
2019-11-06T21:36:06.7150126Z -LL |     _foo + 1
2019-11-06T21:36:06.7150474Z -   |
2019-11-06T21:36:06.7150880Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T21:36:06.7150880Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T21:36:06.7151003Z +LL |   #[derive(DeriveSomething)]
2019-11-06T21:36:06.7151104Z +LL | | struct Baz;
2019-11-06T21:36:06.7151336Z +LL | |
2019-11-06T21:36:06.7151336Z +LL | |
2019-11-06T21:36:06.7151375Z +LL | | macro_rules! test_macro {
2019-11-06T21:36:06.7151440Z +...  |
2019-11-06T21:36:06.7151524Z +LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T21:36:06.7151917Z +   | |_^ can't find crate
2019-11-06T21:36:06.7151955Z  
2019-11-06T21:36:06.7151955Z  
2019-11-06T21:36:06.7152391Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7152771Z -   |
2019-11-06T21:36:06.7152951Z -LL |     println!("{}", _foo);
2019-11-06T21:36:06.7153132Z -   |                    ^^^^
2019-11-06T21:36:06.7153191Z +error: aborting due to previous error
2019-11-06T21:36:06.7153191Z +error: aborting due to previous error
2019-11-06T21:36:06.7153236Z  
2019-11-06T21:36:06.7153498Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7153874Z -   |
2019-11-06T21:36:06.7153874Z -   |
2019-11-06T21:36:06.7154216Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T21:36:06.7154554Z -
2019-11-06T21:36:06.7154554Z -
2019-11-06T21:36:06.7154966Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7155478Z -   |
2019-11-06T21:36:06.7155478Z -   |
2019-11-06T21:36:06.7155642Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T21:36:06.7156266Z -
2019-11-06T21:36:06.7156266Z -
2019-11-06T21:36:06.7156876Z -error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T21:36:06.7157310Z -   |
2019-11-06T21:36:06.7157310Z -   |
2019-11-06T21:36:06.7157515Z -LL |     s._underscore_field += 1;
2019-11-06T21:36:06.7157905Z -
2019-11-06T21:36:06.7158127Z -error: aborting due to 5 previous errors
2019-11-06T21:36:06.7158503Z -
2019-11-06T21:36:06.7158745Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T21:36:06.7161897Z 
2019-11-06T21:36:06.7162088Z ------------------------------------------
2019-11-06T21:36:06.7162127Z stderr:
2019-11-06T21:36:06.7162476Z ------------------------------------------
2019-11-06T21:36:06.7165192Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n