\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"tests/ui/regex.rs","byte_start":95,"byte_end":114,"line_start":4,"line_end":4,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate regex;","highlight_start":1,"highlight_end":20}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `regex`\n  --> tests/ui/regex.rs:4:1\n   |\nLL | extern crate regex;\n   | ^^^^^^^^^^^^^^^^^^^ can't find crate\n\n"}
2019-11-06T07:13:13.5299460Z {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
2019-11-06T07:13:13.5299698Z 
2019-11-06T07:13:13.5299952Z ------------------------------------------
2019-11-06T07:13:13.5299985Z 
2019-11-06T07:13:13.5299985Z 
2019-11-06T07:13:13.5300012Z 
2019-11-06T07:13:13.5300255Z ---- [ui] ui/used_underscore_binding.rs stdout ----
2019-11-06T07:13:13.5300304Z normalized stderr:
2019-11-06T07:13:13.5300528Z error[E0463]: can't find crate for `clippy_lints`
2019-11-06T07:13:13.5300762Z   --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T07:13:13.5300809Z    |
2019-11-06T07:13:13.5300852Z LL |   #[derive(DeriveSomething)]
2019-11-06T07:13:13.5300954Z LL | | struct Baz;
2019-11-06T07:13:13.5300994Z LL | |
2019-11-06T07:13:13.5300994Z LL | |
2019-11-06T07:13:13.5301037Z LL | | macro_rules! test_macro {
2019-11-06T07:13:13.5301133Z LL | |
2019-11-06T07:13:13.5301181Z LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T07:13:13.5301410Z    | |_^ can't find crate
2019-11-06T07:13:13.5301452Z 
2019-11-06T07:13:13.5301452Z 
2019-11-06T07:13:13.5301502Z error: aborting due to previous error
2019-11-06T07:13:13.5301531Z 
2019-11-06T07:13:13.5301790Z For more information about this error, try `rustc --explain E0463`.
2019-11-06T07:13:13.5301824Z 
2019-11-06T07:13:13.5301849Z 
2019-11-06T07:13:13.5301890Z expected stderr:
2019-11-06T07:13:13.5301960Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5302235Z    |
2019-11-06T07:13:13.5302235Z    |
2019-11-06T07:13:13.5302291Z LL |     _foo + 1
2019-11-06T07:13:13.5302372Z    |
2019-11-06T07:13:13.5302615Z    = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T07:13:13.5302665Z 
2019-11-06T07:13:13.5302665Z 
2019-11-06T07:13:13.5302718Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5303027Z    |
2019-11-06T07:13:13.5303070Z LL |     println!("{}", _foo);
2019-11-06T07:13:13.5303112Z    |                    ^^^^
2019-11-06T07:13:13.5303140Z 
2019-11-06T07:13:13.5303140Z 
2019-11-06T07:13:13.5303208Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5303479Z    |
2019-11-06T07:13:13.5303479Z    |
2019-11-06T07:13:13.5303537Z LL |     assert_eq!(_foo, _foo);
2019-11-06T07:13:13.5303607Z 
2019-11-06T07:13:13.5303607Z 
2019-11-06T07:13:13.5303658Z error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5303947Z    |
2019-11-06T07:13:13.5303947Z    |
2019-11-06T07:13:13.5303988Z LL |     assert_eq!(_foo, _foo);
2019-11-06T07:13:13.5304085Z 
2019-11-06T07:13:13.5304085Z 
2019-11-06T07:13:13.5304146Z error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5304437Z    |
2019-11-06T07:13:13.5304437Z    |
2019-11-06T07:13:13.5304479Z LL |     s._underscore_field += 1;
2019-11-06T07:13:13.5304565Z 
2019-11-06T07:13:13.5304608Z error: aborting due to 5 previous errors
2019-11-06T07:13:13.5304636Z 
2019-11-06T07:13:13.5304662Z 
2019-11-06T07:13:13.5304662Z 
2019-11-06T07:13:13.5304703Z 
2019-11-06T07:13:13.5304743Z diff of stderr:
2019-11-06T07:13:13.5304770Z 
2019-11-06T07:13:13.5305067Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5305534Z +error[E0463]: can't find crate for `clippy_lints`
2019-11-06T07:13:13.5305865Z +  --> $DIR/used_underscore_binding.rs:12:10
2019-11-06T07:13:13.5305996Z     |
2019-11-06T07:13:13.5305996Z     |
2019-11-06T07:13:13.5306218Z -LL |     _foo + 1
2019-11-06T07:13:13.5306817Z -   |
2019-11-06T07:13:13.5307133Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T07:13:13.5307133Z -   = note: `-D clippy::used-underscore-binding` implied by `-D warnings`
2019-11-06T07:13:13.5307184Z +LL |   #[derive(DeriveSomething)]
2019-11-06T07:13:13.5307286Z +LL | | struct Baz;
2019-11-06T07:13:13.5307328Z +LL | |
2019-11-06T07:13:13.5307328Z +LL | |
2019-11-06T07:13:13.5307370Z +LL | | macro_rules! test_macro {
2019-11-06T07:13:13.5307427Z +...  |
2019-11-06T07:13:13.5307513Z +LL | | /// Tests that we lint if we use a binding with a single leading underscore
2019-11-06T07:13:13.5307725Z +   | |_^ can't find crate
2019-11-06T07:13:13.5307784Z  
2019-11-06T07:13:13.5307784Z  
2019-11-06T07:13:13.5308075Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5308524Z -   |
2019-11-06T07:13:13.5308729Z -LL |     println!("{}", _foo);
2019-11-06T07:13:13.5308931Z -   |                    ^^^^
2019-11-06T07:13:13.5308994Z +error: aborting due to previous error
2019-11-06T07:13:13.5308994Z +error: aborting due to previous error
2019-11-06T07:13:13.5309035Z  
2019-11-06T07:13:13.5309327Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5309750Z -   |
2019-11-06T07:13:13.5309750Z -   |
2019-11-06T07:13:13.5309953Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T07:13:13.5310343Z -
2019-11-06T07:13:13.5310343Z -
2019-11-06T07:13:13.5310630Z -error: used binding `_foo` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5311051Z -   |
2019-11-06T07:13:13.5311051Z -   |
2019-11-06T07:13:13.5311266Z -LL |     assert_eq!(_foo, _foo);
2019-11-06T07:13:13.5311667Z -
2019-11-06T07:13:13.5311667Z -
2019-11-06T07:13:13.5311975Z -error: used binding `_underscore_field` which is prefixed with an underscore. A leading underscore signals that a binding will not be used.
2019-11-06T07:13:13.5312396Z -   |
2019-11-06T07:13:13.5312396Z -   |
2019-11-06T07:13:13.5312606Z -LL |     s._underscore_field += 1;
2019-11-06T07:13:13.5312979Z -
2019-11-06T07:13:13.5313209Z -error: aborting due to 5 previous errors
2019-11-06T07:13:13.5313384Z -
2019-11-06T07:13:13.5313623Z +For more information about this error, try `rustc --explain E0463`.
---
2019-11-06T07:13:13.5317330Z 
2019-11-06T07:13:13.5317939Z ------------------------------------------
2019-11-06T07:13:13.5317998Z stderr:
2019-11-06T07:13:13.5318221Z ------------------------------------------
2019-11-06T07:13:13.5321092Z {"message":"can't find crate for `clippy_lints`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n