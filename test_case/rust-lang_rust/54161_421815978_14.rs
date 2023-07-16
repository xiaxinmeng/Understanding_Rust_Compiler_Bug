\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/suggest-methods.rs","byte_start":1008,"byte_end":1015,"line_start":38,"line_end":38,"column_start":19,"column_end":26,"is_primary":true,"text":[{"text":"    let _ = 63u32.count_o(); //~ ERROR no method named","highlight_start":19,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no method named `count_o` found for type `u32` in the current scope\n  --> /checkout/src/test/ui/suggestions/suggest-methods.rs:38:19\n   |\nLL |     let _ = 63u32.count_o(); //~ ERROR no method named\n   |                   ^^^^^^^\n\n"}
[00:51:56] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:51:56] {"message":"For more information about this error, try `rustc --explain E0599`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0599`.\n"}
[00:51:56] ------------------------------------------
[00:51:56] 
[00:51:56] thread '[ui] ui/suggestions/suggest-methods.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:51:56] 
[00:51:56] 
[00:51:56] ---- [ui] ui/traits/trait-item-privacy.rs stdout ----
[00:51:56] diff of stderr:
[00:51:56] 
[00:51:56] 10    = help: items from traits can only be used if the trait is implemented and in scope
[00:51:56] 11    = note: the following trait defines an item `a`, perhaps you need to implement it:
[00:51:56] 12            candidate #1: `method::A`
[00:51:56] +    = help: did you mean `b`?
[00:51:56] 13 
[00:51:56] 14 error[E0599]: no method named `b` found for type `S` in the current scope
[00:51:56] 
[00:51:56] 21    |       ^
[00:51:56] 22    |
[00:51:56] 23    = help: items from traits can only be used if the trait is in scope
[00:51:56] 23    = help: items from traits can only be used if the trait is in scope
[00:51:56] +    = help: did you mean `c`?
[00:51:56] 24 help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:51:56] 25    |
[00:51:56] 26 LL | use method::B;
[00:51:56] 44    = help: items from traits can only be used if the trait is implemented and in scope
[00:51:56] 44    = help: items from traits can only be used if the trait is implemented and in scope
[00:51:56] 45    = note: the following trait defines an item `a`, perhaps you need to implement it:
[00:51:56] 46            candidate #1: `method::A`
[00:51:56] +    = help: did you mean `B`?
[00:51:56] 47 
[00:51:56] 48 error[E0599]: no function or associated item named `b` found for type `S` in the current scope
[00:51:56] 
[00:51:56] 
[00:51:56] 55    |     ^^^^ function or associated item not found in `S`
[00:51:56] 57    = help: items from traits can only be used if the trait is in scope
[00:51:56] +    = help: did you mean `B`?
[00:51:56] 58 help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:51:56] 59    |
[00:51:56] 59    |
[00:51:56] 60 LL | use method::B;
[00:51:56] 
[00:ui/traits/trait-item-privacy/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-item-privacy/auxiliary" "-A" "unused"
[00:51:56] ------------------------------------------
[00:51:56] 
[00:51:56] ------------------------------------------
[00:51:56] stderr:
[00:51:56] stderr:
[00:51:56] ------------------------------------------
[00:51:56] {"message":"no method named `a` found for type `S` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n