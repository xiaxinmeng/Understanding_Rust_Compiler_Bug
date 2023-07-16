\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs","byte_start":119,"byte_end":140,"line_start":6,"line_end":6,"column_start":39,"column_end":60,"is_primary":true,"text":[{"text":"    let _: &I32Iterator<Item = u32> = &vec![42].into_iter();","highlight_start":39,"highlight_end":60}],"label":"expected u32, found i32","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `u32`\n   found type `i32`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"required for the cast to the object type `dyn std::iter::Iterator<Item = u32, Item = i32>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0271]: type mismatch resolving `<std::vec::IntoIter<u32> as std::iter::Iterator>::Item == i32`\n  --> /checkout/src/test/ui/associated-types/associated-types-overridden-binding-2.rs:6:39\n   |\nLL |     let _: &I32Iterator<Item = u32> = &vec![42].into_iter();\n   |                                       ^^^^^^^^^^^^^^^^^^^^^ expected u32, found i32\n   |\n   = note: expected type `u32`\n              found type `i32`\n   = note: required for the cast to the object type `dyn std::iter::Iterator<Item = u32, Item = i32>`\n\n"}
[01:15:35] {"message":"For more information about this error, try `rustc --explain E0271`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0271`.\n"}
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] 
---
[01:15:35] diff of stderr:
[01:15:35] 
[01:15:35] 2   --> $DIR/bad-sized.rs:4:24
[01:15:35] 3    |
[01:15:35] 4 LL |     let x: Vec<Trait + Sized> = Vec::new();
[01:15:35] -    |                        ^^^^^ non-auto additional trait
[01:15:35] +    |                -----   ^^^^^ additional non-auto trait
[01:15:35] +    |                first non-auto trait
[01:15:35] 6 
[01:15:35] 6 
[01:15:35] 7 error[E0277]: the size for values of type `dyn Trait` cannot be known at compilation time
[01:15:35] 
[01:15:35] 
[01:15:35] The actual stderr differed from the expected stderr.
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/bad-sized.stderr
[01:15:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/bad-sized.stderr
[01:15:35] To update references, rerun the tests and pass the `--bless` flag
[01:15:35] To only update this specific test, also pass `--test-args bad/bad-sized.rs`
[01:15:35] error: 1 errors occurred comparing output.
[01:15:35] status: exit code: 1
[01:15:35] status: exit code: 1
[01:15:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bad/bad-sized.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bad/bad-sized/auxiliary" "-A" "unused"
[01:15:35] ------------------------------------------
[01:15:35] 
[01:15:35] ------------------------------------------
[01:15:35] stderr:
[01:15:35] stderr:
[01:15:35] ------------------------------------------
[01:15:35] {"message":"only auto traits can be used as additional traits in a trait object","code":{"code":"E0225","explanation":"\nYou attempted to use multiple types as bounds for a closure or trait object.\nRust does not currently support this. A simple example that causes this error:\n\n