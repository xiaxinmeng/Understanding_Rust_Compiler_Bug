\npub struct Foo; // you define your type in your crate\n\nimpl Drop for Foo { // and you can implement the trait on it!\n    // code of trait implementation here\n#   fn drop(&mut sined in this crate\n   = note: define and implement a trait or new type instead\n\n"}
[01:00:21] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:00:21] {"message":"For more information about this error, try `rustc --explain E0117`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0117`.\n"}
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] thread '[ui] ui/coherence/coherence-fundamental-trait-objects.rs#re' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:00:21] 
[01:00:21] 
[01:00:21] ---- [ui] ui/coherence/coherence-impls-send.rs#old stdout ----
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
[01:00:21] 20    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 21    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 23    = note: define and implement a trait or new type instead
[01:00:21] 24 
[01:00:21] 25 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
[01:00:21] 
[01:00:21] 28 LL | unsafe impl Send for &'static [NotSync] {}
[01:00:21] 29    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl doesn't use types inside crate
[01:00:21] 30    |
[01:00:21] -    = note: the impl does not reference any types defined in this crate
[01:00:21] +    = note: the impl does not reference only types defined in this crate
[01:00:21] 32    = note: define and implement a trait or new type instead
[01:00:21] 34 error: aborting due to 4 previous errors
[01:00:21] 
[01:00:21] 
[01:00:21] The actual stderr differed from the expected stderr.
[01:00:21] The actual stderr differed from the expected stderr.
[01:00:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-send.old/coherence-impls-send.old.stderr
[01:00:21] To update references, rerun the tests and pass the `--bless` flag
[01:00:21] To only update this specific test, also pass `--test-args coherence/coherence-impls-send.rs`
[01:00:21] 
[01:00:21] error in revision `old`: 1 errors occurred comparing output.
[01:00:21] status: exit code: 1
[01:00:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coherence/coherence-impls-send.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "old" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-send.old/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-impls-send.old/auxiliary" "-A" "unused"
[01:00:21] ------------------------------------------
[01:00:21] 
[01:00:21] ------------------------------------------
[01:00:21] stderr:
[01:00:21] stderr:
[01:00:21] ------------------------------------------
[01:00:21] {"message":"only traits defined in the current crate can be implemented for arbitrary types","code":{"code":"E0117","explanation":"\nThis error indicates a violation of one of Rust's orphan rules for trait\nimplementations. The rule prohibits any implementation of a foreign trait (a\ntrait defined in another crate) where\n\n - the type that is implementing the trait is foreign\n - all of the parameters being passed to the trait (if there are any) are also\n   foreign.\n\nHere's one example of this error:\n\n