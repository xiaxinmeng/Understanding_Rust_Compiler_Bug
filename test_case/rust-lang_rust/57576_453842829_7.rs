\n\nOnly structs and enums are permitted to impl Send, Sync, and other opt-out\ntrait, and the struct or enum must be local to the current crate. So, for\nexample, `unsafe impl Send for Rc<Foo>` is not allowed.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs","byte_start":832,"byte_end":867,"line_start":27,"line_end":27,"column_start":1,"column_end":36,"is_primary":true,"text":[{"text":"impl !Send for dyn Object + Marker2 {} //~ ERROR E0321","highlight_start":1,"highlight_end":36}],"label":"can't implement cross-crate trait with a default impl for non-struct/enum type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0321]: cross-crate traits with a default impl, like `std::marker::Send`, can only be implemented for a struct/enum type, not `(dyn Object + Marker2 + 'static)`\n  --> /checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs:27:1\n   |\nLL | impl !Send for dyn Object + Marker2 {} //~ ERROR E0321\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl for non-struct/enum type\n\n"}
[01:03:54] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:03:54] {"message":"Some errors occurred: E0117, E0321, E0371.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0117, E0321, E0371.\n"}
[01:03:54] 
[01:03:54] ------------------------------------------
[01:03:54] 
[01:03:54] thread '[ui] ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:03:54] thread '[ui] ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:03:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:54] 
[01:03:54] ---- [ui] ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs stdout ----
[01:03:54] diff of stderr:
[01:03:54] 
[01:03:54] 16 LL | unsafe impl Send for dyn Marker2 {} //~ ERROR E0117
[01:03:54] 17    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:03:54] 18    |
[01:03:54] -    = note: the impl does not reference any types defined in this crate
[01:03:54] +    = note: the impl does not reference only types defined in this crate
[01:03:54] 20    = note: define and implement a trait or new type instead
[01:03:54] 21 
[01:03:54] 22 error[E0321]: cross-crate traits with a default impl, like `std::marker::Send`, can only be implemented for a struct/enum type, not `(dyn Object + 'static)`
[01:03:54] 
[01:03:54] The actual stderr differed from the expected stderr.
[01:03:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive/coherence-impl-trait-for-marker-trait-positive.stderr
[01:03:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive/coherence-impl-trait-for-marker-trait-positive.stderr
[01:03:54] To update references, rerun the tests and pass the `--bless` flag
[01:03:54] To only update this specific test, also pass `--test-args coherence/coherence-impl-trait-for-marker-trait-positive.rs`
[01:03:54] error: 1 errors occurred comparing output.
[01:03:54] status: exit code: 1
[01:03:54] status: exit code: 1
[01:03:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impl-trait-for-marker-trait-positive/auxiliary" "-A" "unused"
[01:03:54] ------------------------------------------
[01:03:54] 
[01:03:54] ------------------------------------------
[01:03:54] stderr:
[01:03:54] stderr:
[01:03:54] ------------------------------------------
[01:03:54] {"message":"the object type `(dyn Object + Marker2 + 'static)` automatically implements the trait `Marker1`","code":{"code":"E0371","explanation":"\nWhen `Trait2` is a subtrait of `Trait1` (for example, when `Trait2` has a\ndefinition like `trait Trait2: Trait1 { ... }`), it is not allowed to implement\n`Trait1` for `Trait2`. This is because `Trait2` already implements `Trait1` by\ndefinition, so it is not useful to do this.\n\nExample:\n\n