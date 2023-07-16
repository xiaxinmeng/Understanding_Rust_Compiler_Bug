\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cycle-trait-supertrait-indirect.rs","byte_start":592,"byte_end":602,"line_start":17,"line_end":17,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"trait B: C {","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"...which requires computing the supertraits of `C`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/cycle-trait-supertrait-indirect.rs","byte_start":608,"byte_end":618,"line_start":20,"line_end":20,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"trait C: B { }","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires computing the supertraits of `B`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when computing the supertraits of `A`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/cycle-trait-supertrait-indirect.rs","byte_start":576,"byte_end":586,"line_start":14,"line_end":14,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"trait A: B {","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `B`\n  --> /checkout/src/test/ui/cycle-trait-supertrait-indirect.rs:17:1\n   |\nLL | trait B: C {\n   | ^^^^^^^^^^\n   |\nnote: ...which requires computing the supertraits of `C`...\n  --> /checkout/src/test/ui/cycle-trait-supertrait-indirect.rs:20:1\n   |\nLL | trait C: B { }\n   | ^^^^^^^^^^\n   = note: ...which again requires computing the supertraits of `B`, completing the cycle\nnote: cycle used when computing the supertraits of `A`\n  --> /checkout/src/test/ui/cycle-trait-supertrait-indirect.rs:14:1\n   |\nLL | trait A: B {\n   | ^^^^^^^^^^\n\n"}
[00:44:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:44:35] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
---
[00:44:35] 31 error[E0391]: cycle detected when processing `cycle1`
[00:44:35] -    |
[00:44:35] - note: ...which requires processing `cycle2::{{impl-Trait}}`...
[00:44:35] 34   --> $DIR/auto-trait-leak.rs:48:16
[00:44:35] 35    |
[00:44:35] 36 LL | fn cycle2() -> impl Clone {
[00:44:35]
[00:44:35] 37    |                ^^^^^^^^^^
[00:44:35] - note: ...which requires processing `cycle2`...
[00:44:35] +    |
[00:44:35] + note: ...which requires processing `cycle2::{{impl-Trait}}`...
[00:44:35] 39   --> $DIR/auto-trait-leak.rs:48:1
[00:44:35] 40    |
[00:44:35] 41 LL | fn cycle2() -> impl Clone {
[00:44:35]
[00:44:35] 42    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:44:35] - note: ...which requires processing `cycle1::{{impl-Trait}}`...
[00:44:35] + note: ...which requires processing `cycle2`...
[00:44:35] 44   --> $DIR/auto-trait-leak.rs:42:16
[00:44:35] 45    |
[00:44:35] 46 LL | fn cycle1() -> impl Clone {
[00:44:35]
[00:44:35] 47    |                ^^^^^^^^^^
[00:44:35] - note: ...which again requires processing `cycle1`, completing the cycle
[00:44:35] + note: ...which requires processing `cycle1::{{impl-Trait}}`...
[00:44:35] 49   --> $DIR/auto-trait-leak.rs:42:16
[00:44:35] 50    |
[00:44:35] 51 LL | fn cycle1() -> impl Clone {
[00:44:35]
[00:44:35] 52    |                ^^^^^^^^^^
[00:44:35] +    = note: ...which again requires processing `cycle1`, completing the cycle
[00:44:35] 53 note: cycle used when type-checking all item bodies
---
[00:44:35] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'impl-trait/auto-trait-leak.rs'
[00:44:35]
[00:44:35] error: 1 errors occurred comparing output.
[00:44:35] status: exit code: 101
[00:44:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:44:35] {"message":"the trait bound `std::rc::Rc<std::cell::Cell<i32>>: std::marker::Send` is not satisfied in `impl std::ops::Fn<(i32,)>`","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n