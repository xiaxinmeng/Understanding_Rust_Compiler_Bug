\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-28344.rs","byte_start":643,"byte_end":656,"line_start":18,"line_end":18,"column_start":13,"column_end":26,"is_primary":true,"text":[{"text":"    let g = BitXor::bitor;","highlight_start":13,"highlight_end":26}],"label":"function or associated item not found in `dyn std::ops::BitXor<_>`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"did you mean `bitxor`?","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0599]: no function or associated item named `bitor` found for type `dyn std::ops::BitXor<_>` in the current scope\n  --> /checkout/src/test/ui/issues/issue-28344.rs:18:13\n   |\nLL |     let g = BitXor::bitor;\n   |             ^^^^^^^^^^^^^ function or associated item not found in `dyn std::ops::BitXor<_>`\n   |\n   = help: did you mean `bitxor`?\n\n"}
[00:48:35] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:48:35] {"message":"Some errors occurred: E0191, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0191, E0599.\n"}
[00:48:35] {"message":"For more information about an error, try `rustc --explain E0191`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0191`.\n"}
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] thread '[ui] ui/issues/issue-28344.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:48:35] 
[00:48:35] 
[00:48:35] ---- [ui] ui/traits/trait-alias-object.rs stdout ----
[00:48:35] diff of stderr:
[00:48:35] 
[00:48:35] 11    |
[00:48:35] 12 LL |     let _: &dyn IteratorAlias = &vec![123].into_iter();
[00:48:35] 13    |             ^^^^^^^^^^^^^^^^^ associated type `Item` must be specified
[00:48:35] -    | 
[00:48:35] -   ::: $SRC_DIR/libcore/iter/iterator.rs:LL:COL
[00:48:35] -    |
[00:48:35] - LL |     type Item;
[00:48:35] -    |     ---------- `Item` defined here
[00:48:35] 20 error: aborting due to 2 previous errors
[00:48:35] 21 
[00:48:35] 
[00:48:35] 
[00:48:35] 
[00:48:35] The actual stderr differed from the expected stderr.
[00:48:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/trait-alias-object.stderr
[00:48:35] To update references, rerun the tests and pass the `--bless` flag
[00:48:35] To only update this specific test, also pass `--test-args traits/trait-alias-object.rs`
[00:48:35] error: 1 errors occurred comparing output.
[00:48:35] status: exit code: 1
[00:48:35] status: exit code: 1
[00:48:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-object.rs" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-object/auxiliary" "-A" "unused"
[00:48:35] ------------------------------------------
[00:48:35] 
[00:48:35] ------------------------------------------
[00:48:35] stderr:
[00:48:35] stderr:
[00:48:35] ------------------------------------------
[00:48:35] {"message":"the trait `EqAlias` cannot be made into an object","code":{"code":"E0038","explanation":"\nTrait objects like `Box<Trait>` can only be constructed when certain\nrequirements are satisfied by the trait in question.\n\nTrait objects are a form of dynamic dispatch and use a dynamically sized type\nfor the inner type. So, for a given trait `Trait`, when `Trait` is treated as a\ntype, as in `Box<Trait>`, the inner type is 'unsized'. In such cases the boxed\npointer is a 'fat pointer' that contains an extra pointer to a table of methods\n(among other things) for dynamic dispatch. This design mandates some\nrestrictions on the types of traits that are allowed to be used in trait\nobjects, which are collectively termed as 'object safety' rules.\n\nAttempting to create a trait object for a non object-safe trait will trigger\nthis error.\n\nThere are various rules:\n\n### The trait cannot require `Self: Sized`\n\nWhen `Trait` is treated as a type, the type does not implement the special\n`Sized` trait, because the type does not have a known size at compile time and\ncan only be accessed behind a pointer. Thus, if we have a trait like the\nfollowing:\n\n