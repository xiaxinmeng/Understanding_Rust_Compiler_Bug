plain
[01:31:44] 
[01:31:44] ---- [ui (nll)] ui/check-static-values-constraints.rs stdout ----
[01:31:44] diff of stderr:
[01:31:44] 
[01:31:44] 13 LL | static STATIC11: Box<MyOwned> = box MyOwned;
[01:31:44] 14    |                                 ^^^^^^^^^^^ allocation not allowed in statics
[01:31:44] 15 
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL | static STATIC11: Box<MyOwned> = box MyOwned;
[01:31:44] + 
[01:31:44] + 
[01:31:44] 16 error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:90:32
[01:31:44] 18    |
[01:31:44] 18    |
[01:31:44] 19 LL |     field2: SafeEnum::Variant4("str".to_string())
[01:31:44] 
[01:31:44] 21 
[01:31:44] 22 error[E0010]: allocations are not allowed in statics
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:94:5
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:94:5
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:95:5
[01:31:44] 24    |
[01:31:44] 25 LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] 26    |     ^^^^^^^^^^^ allocation not allowed in statics
[01:31:44] 27 
[01:31:44] 27 
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] + 
[01:31:44] 28 error[E0010]: allocations are not allowed in statics
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:95:5
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:97:5
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:97:5
[01:31:44] 30    |
[01:31:44] 31 LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] 32    |     ^^^^^^^^^^^ allocation not allowed in statics
[01:31:44] 33 
[01:31:44] 33 
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] + 
[01:31:44] 34 error[E0010]: allocations are not allowed in statics
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:99:6
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:102:6
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:102:6
[01:31:44] 36    |
[01:31:44] 37 LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] 38    |      ^^^^^^^^^^^ allocation not allowed in statics
[01:31:44] 39 
[01:31:44] 39 
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] + 
[01:31:44] 40 error[E0010]: allocations are not allowed in statics
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:100:6
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:104:6
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:104:6
[01:31:44] 42    |
[01:31:44] 43 LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] 44    |      ^^^^^^^^^^^ allocation not allowed in statics
[01:31:44] 45 
[01:31:44] 45 
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
[01:31:44] + 
[01:31:44] 46 error[E0010]: allocations are not allowed in statics
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:106:5
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:111:5
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:111:5
[01:31:44] 48    |
[01:31:44] 49 LL |     box 3;
[01:31:44] 50    |     ^^^^^ allocation not allowed in statics
[01:31:44] 
[01:31:44] 51 
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] + LL |     box 3;
[01:31:44] +    |         ^
[01:31:44] + 
[01:31:44] + 
[01:31:44] 52 error[E0507]: cannot move out of static item
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:110:45
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:116:45
[01:31:44] 54    |
[01:31:44] 55 LL |     let y = { static x: Box<isize> = box 3; x };
[01:31:44] 
[01:31:44] 59    |                                             help: consider borrowing here: `&x`
[01:31:44] 60 
[01:31:44] 61 error[E0010]: allocations are not allowed in statics
[01:31:44] 61 error[E0010]: allocations are not allowed in statics
[01:31:44] -   --> $DIR/check-static-values-constraints.rs:110:38
[01:31:44] +   --> $DIR/check-static-values-constraints.rs:116:38
[01:31:44] 63    |
[01:31:44] 64 LL |     let y = { static x: Box<isize> = box 3; x };
[01:31:44] 65    |                                      ^^^^^ allocation not allowed in statics
[01:31:44] 66 
[01:31:44] - error: aborting due to 10 previous errors
[01:31:44] - error: aborting due to 10 previous errors
[01:31:44] + error[E0019]: static contains unimplemented expression type
[01:31:44] +    |
[01:31:44] +    |
[01:31:44] + LL |     let y = { static x: Box<isize> = box 3; x };
[01:31:44] 68 
[01:31:44] - Some errors occurred: E0010, E0015, E0493, E0507.
[01:31:44] + error: aborting due to 17 previous errors
[01:31:44] + 
[01:31:44] + 
[01:31:44] + Some errors occurred: E0010, E0015, E0019, E0493, E0507.
[01:31:44] 70 For more information about an error, try `rustc --explain E0010`.
[01:31:44] 71 
[01:31:44] 
[01:31:44] 
[01:31:44] The actual stderr differed from the expected stderr.
[01:31:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints.nll/check-static-values-constraints.nll.stderr
[01:31:44] To update references, rerun the tests and pass the `--bless` flag
[01:31:44] To only update this specific test, also pass `--test-args check-static-values-constraints.rs`
[01:31:44] error: 1 errors occurred comparing output.
[01:31:44] status: exit code: 1
[01:31:44] status: exit code: 1
[01:31:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints.nll/auxiliary" "-A" "unused"
[01:31:44] ------------------------------------------
[01:31:44] 
[01:31:44] ------------------------------------------
[01:31:44] stderr:
[01:31:44] stderr:
[01:31:44] ------------------------------------------
[01:31:44] {"message":"destructors cannot be evaluated at compile-time","code":{"code":"E0493","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/check-static-values-constraints.rs","byte_start":1896,"byte_end":2084,"line_start":65,"line_end":67,"column_start":43,"column_end":81,"is_primary":true,"text":[{"text":"                                        ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),","highlight_start":43,"highlight_end":91},{"text":"//~^ ERROR destructors cannot be evaluated at compile-time","highlight_start":1,"highlight_end":59},{"text":"                                                     field2: SafeEnum::Variant1}};","highlight_start":1,"highlight_end":81}],"label":"statics cannot evaluate destructors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0493]: destructors cannot be evaluated at compile-time\n  --> /checkout/src/test/ui/check-static-values-constraints.rs:65:43\n   |\nLL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),\n   |  ___________________________________________^\nLL | | //~^ ERROR destructors cannot be evaluated at compile-time\nLL | |                                                      field2: SafeEnum::Variant1}};\n   | |________________________________________________________________________________^ statics cannot evaluate destructors\n\n"}
[01:31:44] {"message":"allocations are not allowed in statics","code":{"code":"E0010","explanation":"\nThe value of statics and constants must be known at compile time, and they live\nfor the entire lifetime of a program. Creating a boxed value allocates memory on\nthe heap at runtime, and therefore cannot be done at compile time. Erroneous\ncode example:\n\n