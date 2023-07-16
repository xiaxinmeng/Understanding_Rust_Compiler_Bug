\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/coherence/coherence-impls-send.rs","byte_start":482,"byte_end":511,"line_start":28,"line_end":28,"column_start":1,"column_end":30,"is_primary":true,"text":[{"text":"unsafe impl Send for [MyType] {}","highlight_start":1,"highlight_end":30}],"label":"impl doesn't use types inside crate","suggete\n   = note: define and implement a trait or new type instead\n\n"}
[01:00:21] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[01:00:21] {"message":"Some errors occurred: E0117, E0321.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0117, E0321.\n"}
[01:00:21] {"message":"For more information about an error, try `rustc --explain E0117`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0117`.\n"}
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] thread '[ui] ui/coherence/coherence-impls-send.rs#old' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:00:21] 
[01:00:21] 
[01:00:21] ---- [ui] ui/coherence/coherence-impls-send.rs#re stdout ----
[01:00:21] diff of stderr:
[01:00:21] 
[01:00:21] 4 LL | unsafe impl Send for (MyType, MyType) {}
[01:00:21] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 6    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 8    = note: define and implement a trait or new type instead
[01:00:21] 9 
[01:00:21] 10 error[E0321]: cross-crate traits with a default impl, like `std::marker::Send`, can only be implemented for a struct/enum type, not `&'static NotSync`
[01:00:21] 
[01:00:21] 19 LL | unsafe impl Send for [MyType] {}
[01:00:21] 20  d.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "re" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-send.re/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-send.re/auxiliary" "-A" "unused"
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] ------------------------------------------
[01:00:21] stderr:
[01:00:21] stderr:
[01:00:21] ------------------------------------------
[01:00:21] {"message":"only traits defined in the current crate can be implemented for arbitrary types","code":{"code":"E0117","explanation":"\nThis error indicates a violation of one of Rust's orphan rules for trait\nimplementations. The rule prohibits any implementation of a foreign trait (a\ntrait defined in another crate) where\n\n - the type that is implementing the trait is foreign\n - all of the parameters being passed to the trait (if there are any) are also\n   foreign.\n\nHere's one example of this error:\n\n