\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/ufcs/ufcs-partially-resolved.rs","byte_start":3306,"byte_end":3322,"line_start":65,"line_end":65,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    <u8 as Dr>::X::N; //~ ERROR no associated item named `N` found for type `<u8 as Dr>::X`","highlight_start":5,"highlight_end":21}],"label":"associated item not found in `<u8 as Dr>::X`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0599]: no associated item named `N` found for type `<u8 as Dr>::X` in the current scope\n  --> /checkout/src/test/ui/ufcs/ufcs-partially-resolved.rs:65:5\n   |\nLL |     <u8 as Dr>::X::N; //~ ERROR no associated item named `N` found for type `<u8 as Dr>::X`\n   |     ^^^^^^^^^^^^^^^^ associated item not found in `<u8 as Dr>::X`\n\n"}
[00:49:07] {"message":"aborting due to 32 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 32 previous errors\n\n"}
[00:49:07] {"message":"Some errors occurred: E0223, E0433, E0575, E0576, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0223, E0433, E0575, E0576, E0599.\n"}
[00:49:07] {"message":"For more information about an error, try `rustc --explain E0223`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0223`.\n"}
[00:49:07] ------------------------------------------
[00:49:07] 
[00:49:07] thread '[ui] ui/ufcs/ufcs-partially-resolved.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3284:9
[00:49:07] 
[00:49:07] 
[00:49:07] ---- [ui] ui/unspecified-self-in-trait-ref.rs stdout ----
[00:49:07] diff of stderr:
[00:49:07] 
[00:49:07] 1 error[E0599]: no function or associated item named `lol` found for type `dyn Foo<_>` in the current scope
[00:49:07] +   --> $DIR/unspecified-self-in-trait-ref.rs:20:13
[00:49:07] 3    |
[00:49:07] 3    |
[00:49:07] 4 LL |     let a = Foo::lol();
[00:49:07] -    |             |
[00:49:07] -    |             |
[00:49:07] -    |             function or associated item not found in `dyn Foo<_>`
[00:49:07] +    |             ^^^^^^^^ function or associated item not found in `dyn Foo<_>`
[00:49:07] 8 
[00:49:07] 9 error[E0599]: no function or associated item named `lol` found for type `dyn Foo<_>` in the current scope
[00:49:07] +   --> $DIR/unspecified-self-in-trait-ref.rs:22:13
[00:49:07] 11    |
[00:49:07] 11    |
[00:49:07] 12 LL |     let b = Foo::<_>::lol();
[00:49:07] -    |             |
[00:49:07] -    |             |
[00:49:07] -    |             function or associated item not found in `dyn Foo<_>`
[00:49:07] +    |             ^^^^^^^^^^^^^ function or associated item not found in `dyn Foo<_>`
[00:49:07] 16 
[00:49:07] 17 error[E0599]: no function or associated item named `lol` found for type `dyn Bar<_, _>` in the current scope
[00:49:07] +   --> $DIR/unspecified-self-in-trait-ref.rs:24:13
[00:49:07] 19    |
[00:49:07] 19    |
[00:49:07] 20 LL |     let c = Bar::lol();
[00:49:07] -    |             |
[00:49:07] -    |             |
[00:49:07] -    |             function or associated item not found in `dyn Bar<_, _>`
[00:49:07] +    |             ^^^^^^^^ function or associated item not found in `dyn Bar<_, _>`
[00:49:07] 24 
[00:49:07] 25 error[E0599]: no function or associated item named `lol` found for type `dyn Bar<usize, _>` in the current scope
[00:49:07] +   --> $DIR/unspecified-self-in-trait-ref.rs:26:13
[00:49:07] 27    |
[00:49:07] 27    |
[00:49:07] 28 LL |     let d = Bar::<usize, _>::lol();
[00:49:07] -    |             |
[00:49:07] -    |             |
[00:49:07] -    |             function or associated item not found in `dyn Bar<usize, _>`
[00:49:07] +    |             ^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `dyn Bar<usize, _>`
[00:49:07] 32 
[00:49:07] 33 error[E0393]: the type parameter `A` must be explicitly specified
[00:49:07] 
[00:49:07] 
[00:49:07] The actual stderr differed from the expected stderr.
[00:49:07] The actual stderr differed from the expected stderr.
[00:49:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unspecified-self-in-trait-ref/unspecified-self-in-trait-ref.stderr
[00:49:07] To update references, rerun the tests and pass the `--bless` flag
[00:49:07] To only update this specific test, also pass `--test-args unspecified-self-in-trait-ref.rs`
[00:49:07] error: 1 errors occurred comparing output.
[00:49:07] status: exit code: 1
[00:49:07] status: exit code: 1
 Bar<_, _>` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n