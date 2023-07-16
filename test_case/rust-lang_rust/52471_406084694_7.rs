\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/array-break-length.rs","byte_start":586,"byte_end":594,"line_start":17,"line_end":17,"column_start":17,"column_end":25,"is_primary":true,"text":[{"text":"        |_: [_; continue]| {} //~ ERROR: `continue` outside of loop","highlight_start":17,"highlight_end":25}],"label":"cannot break outside of a loop","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0268]: `continue` outside of loop\n  --> /checkout/src/test/ui/array-break-length.rs:17:17\n   |\nLL |         |_: [_; continue]| {} //~ ERROR: `continue` outside of loop\n   |                 ^^^^^^^^ cannot break outside of a loop\n\n"}
[00:50:15] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:50:15] {"message":"For more information about this error, try `rustc --explain E0268`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0268`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/array-break-length.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/arbitrary-self-types-not-object-safe.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/arbitrary-self-types-not-object-safe.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-self-types-not-object-safe/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/arbitrary-self-types-not-object-safe/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"the trait `Foo` cannot be made into an object","code":{"code":"E0038","explanation":"\nTrait objects like `Box<Trait>` can only be constructed when certain\nrequirements are satisfied by the trait in question.\n\nTrait objects are a form of dynamic dispatch and use a dynamically sized type\nfor the inner type. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like the\nfollowing:\n\n