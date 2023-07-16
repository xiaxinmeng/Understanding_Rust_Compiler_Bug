\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-eq-3.rs","byte_start":837,"byte_end":844,"line_start":33,"line_end":33,"column_start":18,"column_end":25,"is_primary":true,"text":[{"text":"    let _: Bar = x.boo();","highlight_start":18,"highlight_end":25}],"label":"expected struct `Bar`, found associated type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `Bar`\n   found type `<I as Foo>::A`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/associated-types/associated-types-eq-3.rs:33:18\n   |\nLL |     let _: Bar = x.boo();\n   |                  ^^^^^^^ expected struct `Bar`, found associated type\n   |\n   = note: expected type `Bar`\n              found type `<I as Foo>::A`\n\n"}
[00:49:31] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:49:31] {"message":"Some errors occurred: E0271, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0271, E0308.\n"}
[00:49:31] {"message":"For more information about an error, try `rustc --explain E0271`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0271`.\n"}
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] thread '[ui] ui/associated-types/associated-types-eq-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:49:31] 
[00:49:31] 
[00:49:31] ---- [ui] ui/associated-types/associated-types-multiple-types-one-trait.rs stdout ----
[00:49:31] diff of stderr:
[00:49:31] 
[00:49:31] - error[E0271]: type mismatch resolving `<T as Foo>::Y == i32`
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL |     want_y(t); //~ ERROR type mismatch
[00:49:31] -    |     ^^^^^^ expected associated type, found i32
[00:49:31] -    |
[00:49:31] -    = note: expected type `<T as Foo>::Y`
[00:49:31] -               found type `i32`
[00:49:31] - note: required by `want_y`
[00:49:31] -    |
[00:49:31] -    |
[00:49:31] - LL | fn want_y<T:Foo<Y=i32>>(t: &T) { }
[00:49:31] - 
[00:49:31] - 
[00:49:31] 15 error[E0271]: type mismatch resolving `<T as Foo>::X == u32`
[00:49:31] 17    |
[00:49:31] 
[00:49:31] 24   --> $DIR/associated-types-multiple-types-one-trait.rs:52:1
[00:49:31] 25    |
[00:49:31] 25    |
[00:49:31] 26 LL | fn want_x<T:Foo<X=u32>>(t: &T) { }
[00:49:31] + 
[00:49:31] + 
[00:49:31] + error[E0271]: type mismatch resolving `<T as Foo>::Y == i32`
[00:49:31] +   --> $DIR/associated-types-multiple-types-one-trait.rs:23:5
[00:49:31] +    |
[00:49:31] + LL |     want_y(t); //~ ERROR type mismatch
[00:49:31] +    |     ^^^^^^ expected associated type, found i32
[00:49:31] +    |
[00:49:31] +    = note: expected type `<T as Foo>::Y`
[00:49:31] +               found type `i32`
[00:49:31] + note: required by `want_y`
[00:49:31] +   --> $DIR/associated-types-multiple-types-one-trait.rs:54:1
[00:49:31] +    |
[00:49:31] + LL | fn want_y<T:Foo<Y=i32>>(t: &T) { }
[00:49:31] 28 
[00:49:31] 29 error: aborting due to 2 previous errors
[00:49:31] 
[00:49:31] 
[00:49:31] 
[00:49:31] The actual stderr differed from the expected stderr.
[00:49:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-multiple-types-one-trait/associated-types-multiple-types-one-trait.stderr
[00:49:31] To update references, rerun the tests and pass the `--bless` flag
[00:49:31] To only update this specific test, also pass `--test-args associated-types/associated-types-multiple-types-one-trait.rs`
[00:49:31] error: 1 errors occurred comparing output.
[00:49:31] status: exit code: 1
[00:49:31] status: exit code: 1
[00:49:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-multiple-types-one-trait.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-multiple-types-one-trait/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-multiple-types-one-trait/auxiliary" "-A" "unused"
[00:49:31] ------------------------------------------
[00:49:31] 
[00:49:31] ------------------------------------------
[00:49:31] stderr:
[00:49:31] stderr:
[00:49:31] ------------------------------------------
[00:49:31] {"message":"type mismatch resolving `<T as Foo>::X == u32`","code":{"code":"E0271","explanation":"\nThis is because of a type mismatch between the associated type of some\ntrait (e.g., `T::Bar`, where `T` implements `trait Quux { type Bar; }`)\nand another type `U` that is required to be equal to `T::Bar`, but is not.\nExamples follow.\n\nHere is a basic example:\n\n