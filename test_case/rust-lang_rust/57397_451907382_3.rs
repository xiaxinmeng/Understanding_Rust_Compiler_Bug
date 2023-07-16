\n\nFor information on the design of the orphan rules, see [RFC 1023].\n\n[RFC 1023]: https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/coherence/coherence-cow.rs","byte_start":491,"byte_end":526,"line_start":18,"line_end":18,"column_start":1,"column_end":36,"is_primary":true,"text":[{"text":"impl<T> Remote for Pair<T,Cover<T>> { }","highlight_start":1,"highlight_end":36}],"label":"impl doesn't use types inside crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the impl does not reference only types defined in this crate","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"define and implement a trait or new type instead","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0117]: only traits defined in the current crate can be implemented for arbitrary types\n  --> /checkout/src/test/ui/coherence/coherence-cow.rs:18:1\n   |\nLL | impl<T> Remote for Pair<T,Cover<T>> { }\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate\n   |\n   = note: the impl does not reference only types defined in this crate\n   = note: define and implement a trait or new type instead\n\n"}
[01:00:21] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:21] {"message":"For more information about this error, try `rustc --explain E0117`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0117`.\n"}
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] thread '[ui] ui/coherence/coherence-cow.rs#re_a' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:00:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:21] 
[01:00:21] ---- [ui] ui/coherence/coherence-cow.rs#re_b stdout ----
[01:00:21] diff of stderr:
[01:00:21] 
[01:00:21] 4 LL | impl<T> Remote for Pair<Cover<T>,T> { }
[01:00:21] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 6    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 8    = note: define and implement a trait or new type instead
[01:00:21] 10 error: aborting due to previous error
[01:00:21] 
[01:00:21] 
[01:00:21] The actual stderr differed from the expected stderr.
[01:00:21] The actual stderr differed from the expected stderr.
[01:00:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-cow.re_b/coherence-cow.re_b.stderr
[01:00:21] To update references, rerun the tests and pass the `--bless` flag
[01:00:21] To only update this specific test, also pass `--test-args coherence/coherence-cow.rs`
[01:00:21] 
[01:00:21] error in revision `re_b`: 1 errors occurred comparing output.
[01:00:21] status: exit code: 1
[01:00:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-cow.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "re_b" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-cow.re_b/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-cow.re_b/auxiliary" "-A" "unused"
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] ------------------------------------------
[01:00:21] stderr:
[01:00:21] stderr:
[01:00:21] ------------------------------------------
[01:00:21] {"message":"only traits defined in the current crate can be implemented for arbitrary types","code":{"code":"E0117","explanation":"\nThis error indicates a violation of one of Rust's orphan rules for trait\nimplementations. The rule prohibits any implementation of a foreign trait (a\ntrait defined in another crate) where\n\n - the type that is implementing the trait is foreign\n - all of the parameters being passed to the trait (if there are any) are also\n   foreign.\n\nHere's one example of this error:\n\n