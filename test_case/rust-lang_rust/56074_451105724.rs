plain
[00:48:39] 
[00:48:39] ---- [ui] ui/impl-trait/infinite-impl-trait-issue-38064.rs stdout ----
[00:48:39] diff of stderr:
[00:48:39] 
[00:48:39] 1 error[E0720]: opaque type expands to a recursive type
[00:48:39] +   --> $DIR/infinite-impl-trait-issue-38064.rs:8:13
[00:48:39] 3    |
[00:48:39] 3    |
[00:48:39] 4 LL | fn foo() -> impl Quux { //~ opaque type expands to a recursive type
[00:48:39] 5    |             ^^^^^^^^^ expands to self-referential type
[00:48:39] 
[00:48:39] 7    = note: expanded type is `foo::Foo<bar::Bar<impl Quux>>`
[00:48:39] 8 
[00:48:39] 9 error[E0720]: opaque type expands to a recursive type
[00:48:39] +   --> $DIR/infinite-impl-trait-issue-38064.rs:14:13
[00:48:39] 11    |
[00:48:39] 11    |
[00:48:39] 12 LL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type
[00:48:39] 13    |             ^^^^^^^^^ expands to self-referential type
[00:48:39] 
[00:48:39] The actual stderr differed from the expected stderr.
[00:48:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
[00:48:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/infinite-impl-trait-issue-38064.stderr
[00:48:39] To update references, rerun the tests and pass the `--bless` flag
[00:48:39] To only update this specific test, also pass `--test-args impl-trait/infinite-impl-trait-issue-38064.rs`
[00:48:39] error: 1 errors occurred comparing output.
[00:48:39] status: exit code: 1
[00:48:39] status: exit code: 1
[00:48:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/infinite-impl-trait-issue-38064.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/infinite-impl-trait-issue-38064/auxiliary" "-A" "unused"
[00:48:39] ------------------------------------------
[00:48:39] 
[00:48:39] ------------------------------------------
[00:48:39] stderr:
[00:48:39] stderr:
[00:48:39] ------------------------------------------
[00:48:39] {"message":"opaque type expands to a recursive type","code":{"code":"E0720","explanation":"\nAn `impl Trait` type expands to a recursive type.\n\nAn `impl Trait` type must be expandable to a concrete type that contains no\n`impl Trait` types. For example the following example tries to create an\n`impl Trait` type `T` that is equal to `[T, T]`:\n\n