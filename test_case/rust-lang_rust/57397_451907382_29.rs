\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/coherence/coherence-impls-send.rs","byte_start":560,"byte_end":599,"line_start":32,"line_end":32,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"unsafe impl Send for &'static [NotSync] {}","highlight_start":1,"highlight_end":40}],"label":"impl doesn't use types inside crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the impl does not reference only types defined in this crate","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"define and implement a trait or new type instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0117]: only traits defined in the current crate can be implemented for arbitrary types\n  --> /checkout/src/test/ui/coherence/coherence-impls-send.rs:32:1\n   |\nLL | unsafe impl Send for &'static [NotSync] {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate\n   |\n   = note: the impl does not reference only types defined in this crate\n   = note: define and implement a trait or new type instead\n\n"}
[01:00:21] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:00:21] {"message":"Some errors occurred: E0117, E0321.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0117, E0321.\n"}
[01:00:21] {"message":"For more information about an error, try `rustc --explain E0117`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0117`.\n"}
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] thread '[ui] ui/coherence/coherence-impls-send.rs#re' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:00:21] 
[01:00:21] 
[01:00:21] ---- [ui] ui/coherence/coherence-impls-copy.rs#old stdout ----
[01:00:21] diff of stderr:
[01:00:21] 
[01:00:21] 51 LL | impl Copy for i32 {}
[01:00:21] 52    | ^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 53    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 55    = note: define and implement a trait or new type instead
[01:00:21] 56 
[01:00:21] 57 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
[01:00:21] 
[01:00:21] 60 LL | impl Copy for (MyType, MyType) {}
[01:00:21] 61    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 62    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 64    = note: define and implement a trait or new type instead
[01:00:21] 65 
[01:00:21] 66 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
[01:00:21] 
[01:00:21] 69 LL | impl Copy for [MyType] {}
[01:00:21] 70    | ^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 71    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 73    = note: define and implement a trait or new type instead
[01:00:21] 74 
[01:00:21] 75 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
[01:00:21] 
[01:00:21] 78 LL | impl Copy for &'static [NotSync] {}
[01:00:21] 79    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 80    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 82    = note: define and implement a trait or new type instead
[01:00:21] 84 error: aborting due to 10 previous errors
[01:00:21] 
[01:00:21] 
[01:00:21] The actual stderr differed from the expected stderr.
[01:00:21] The actual stderr differed from the expected stderr.
[01:00:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-copy.old/coherence-impls-copy.old.stderr
[01:00:21] To update references, rerun the tests and pass the `--bless` flag
[01:00:21] To only update this specific test, also pass `--test-args coherence/coherence-impls-copy.rs`
[01:00:21] 
[01:00:21] error in revision `old`: 1 errors occurred comparing output.
[01:00:21] status: exit code: 1
[01:00:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impls-copy.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "old" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-copy.old/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-copy.old/auxiliary" "-A" "unused"
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] ------------------------------------------
[01:00:21] stderr:
[01:00:21] stderr:
[01:00:21] ------------------------------------------
[01:00:21] {"message":"conflicting implementations of trait `std::marker::Copy` for type `i32`:","code":{"code":"E0119","explanation":"\nThere are conflicting trait implementations for the same type.\nExample of erroneous code:\n\n