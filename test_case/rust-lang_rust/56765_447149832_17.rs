\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs","byte_start":1280,"byte_end":1284,"line_start":43,"line_end":43,"column_start":23,"column_end":27,"is_primary":true,"text":[{"text":"fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types","highlight_start":23,"highlight_end":27}],"label":"expected struct `Black`, found struct `Blue`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `Black`\n   found type `Blue`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/associated-type/associated-type-projection-from-supertrait.rs:43:23\n   |\nLL | fn b() { dent(ModelT, Blue); } //~ ERROR mismatched types\n   |                       ^^^^ expected struct `Black`, found struct `Blue`\n   |\n   = note: expected type `Black`\n              found type `Blue`\n\n"}
[00:49:31] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:49:31] {"message":"For more information about this error, try `rustc --explain E0308`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0308`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/associated-type/associated-type-projection-from-supertrait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/associated-types/associated-types-eq-3.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] - error[E0308]: mismatched types
[00:49:31] -   --> $DIR/associated-types-eq-3.rs:33:18
[00:49:31] -    |
[00:49:31] - LL |     let _: Bar = x.boo();
[00:49:31] -    |                  ^^^^^^^ expected struct `Bar`, found associated type
[00:49:31] -    = note: expected type `Bar`
[00:49:31] -    = note: expected type `Bar`
[00:49:31] -               found type `<I as Foo>::A`
[00:49:31] - 
[00:49:31] 10 error[E0271]: type mismatch resolving `<isize as Foo>::A == Bar`
[00:49:31] 12    |
[00:49:31] 
[00:49:31] 30    = note: expected type `usize`
[00:49:31] 31               found type `Bar`
[00:49:31] 31               found type `Bar`
[00:49:31] 32    = note: required for the cast to the object type `dyn Foo<A=Bar>`
[00:49:31] + error[E0308]: mismatched types
[00:49:31] +   --> $DIR/associated-types-eq-3.rs:33:18
[00:49:31] +    |
[00:49:31] +    |
[00:49:31] + LL |     let _: Bar = x.boo();
[00:49:31] +    |                  ^^^^^^^ expected struct `Bar`, found associated type
[00:49:31] +    = note: expected type `Bar`
[00:49:31] +    = note: expected type `Bar`
[00:49:31] +               found type `<I as Foo>::A`
[00:49:31] 34 error: aborting due to 3 previous errors
[00:49:31] 35 
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/associated-types-eq-3.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args associated-types/associated-types-eq-3.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-eq-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"type mismatch resolving `<isize as Foo>::A == Bar`","code":{"code":"E0271","explanation":"\nThis is because of a type mismatch between the associated type of some\ntrait (e.g., `T::Bar`, where `T` implements `trait Quux { type Bar; }`)\nand another type `U` that is required to be equal to `T::Bar`, but is not.\nExamples follow.\n\nHere is a basic example:\n\n