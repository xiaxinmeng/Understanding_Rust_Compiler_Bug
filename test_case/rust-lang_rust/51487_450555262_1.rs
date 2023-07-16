\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs","byte_start":76,"byte_end":86,"line_start":3,"line_end":3,"column_start":19,"column_end":29,"is_primary":true,"text":[{"text":"trait Chromosome: Chromosome {","highlight_start":19,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires computing the supertraits of `Chromosome`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing ``","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/cycle-trait/cycle-trait-supertrait-direct.rs","byte_start":58,"byte_end":86,"line_start":3,"line_end":3,"column_start":1,"column_end":29,"is_primary":true,"text":[{"text":"trait Chromosome: Chromosome {","highlight_start":1,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when1:01:44] + LL | | existential type Foo: Copy; //~ cycle detected
[01:01:44] + LL | |
[01:01:44] + ...  |
[01:01:44] + LL | |     let _: Foo = std::mem::transmute(0u8);
[01:01:44] + LL | | }
[01:01:44] 13 
[01:01:44] 14 error: aborting due to previous error
[01:01:44] 15 
[01:01:44] 
[01:01:44] 
[01:01:44] 
[01:01:44] The actual stderr differed from the expected stderr.
[01:01:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/no_inferrable_concrete_type.stderr
[01:01:44] To update references, rerun the tests and pass the `--bless` flag
[01:01:44] To only update this specific test, also pass `--test-args existential_types/no_inferrable_concrete_type.rs`
[01:01:44] error: 1 errors occurred comparing output.
[01:01:44] status: exit code: 1
[01:01:44] status: exit code: 1
[01:01:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/existential_types/no_inferrable_concrete_type/auxiliary" "-A" "unused"
[01:01:44] ------------------------------------------
[01:01:44] 
[01:01:44] ------------------------------------------
[01:01:44] stderr:
[01:01:44] stderr:
[01:01:44] -------le_concrete_type.rs","byte_start":200,"byte_end":405,"line_start":4,"line_end":13,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"#![feature(existential_type)]","highlight_start":1,"highlight_end":30},{"text":"","highlight_start":1,"highlight_end":1},{"text":"existential type Foo: Copy; //~ cycle detected","highlight_start":1,"highlight_end":47},{"text":"","highlight_start":1,"highlight_end":1},{"text":"// make compiler happy about using 'Foo'","highlight_start":1,"highlight_end":41},{"text":"fn bar(x: Foo) -> Foo { x }","highlight_start":1,"highlight_end":28},{"text":"","highlight_start":1,"highlight_end":1},{"text":"fn main() {","highlight_start":1,"highlight_end":12},{"text":"    let _: Foo = std::mem::transmute(0u8);","highlight_start":1,"highlight_end":43},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when processing `Foo`\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:6:1\n   |\nLL | existential type Foo: Copy; //~ cycle detected\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: ...which requires processing `bar`...\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:9:23\n   |\nLL | fn bar(x: Foo) -> Foo { x }\n   |                       ^^^^^\n   = note: ...which again requires processing `Foo`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/existential_types/no_inferrable_concrete_type.rs:4:1\n   |\nLL | / #![feature(existential_type)]\nessing `cycle2::{{impl-Trait}}`...
[01:01:44] 
[01:01:44] 
[01:01:44] 21 LL | fn cycle2() -> impl Clone {
[01:01:44] 22    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:44] 23 note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
[01:01:44] -    = note: ...which again requires processing `cycle1::{{impl-Trait}}`, completing the cycle
[01:01:44] - 
[01:01:44] - error[E0391]: cycle detected when processing `cycle1::{{impl-Trait}}`
[01:01:44] + note: ...which requires processing `cycle1::{{impl-Trait}}`...
[01:01:44] 28    |
[01:01:44] 28    |
[01:01:44] 29 LL | fn cycle1() -> impl Clone {
[01:01:44] 30    |                ^^^^^^^^^^
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - note: ...which requires processing `cycle1`...
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL | fn cycle1() -> impl Clone {
[01:01:44] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:44] - note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
[01:01:44] - note: ...which requires processing `cycle2::{{impl-Trait}}`...
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL | fn cycle2() -> impl Clone {
[01:01:44] -    |                ^^^^^^^^^^
[01:01:44] - note: ...which requires processing `cycle2`...
[01:01:44] -    |
[01:01:44] -    |
[01:01:44] - LL | fn cycle2() -> impl Clone {
[01:01:44] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:44] -    = note: ...which again requires proc `--bless` flag
[01:01:44] To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`
[01:01:44] error: 1 errors occurred comparing output.
[01:01:44] status: exit code: 1
[01:01:44] status: exit code: 1
[01:01:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary" "-A" "unused"
[01:01:44] ------------------------------------------
[01:01:44] 
[01:01:44] ------------------------------------------
[01:01:44] stderr:
[01:01:44] stderr:
[01:01:44] ------------------------------------------
[01:01:44] {"message":"cycle detected when processing `cycle1`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n