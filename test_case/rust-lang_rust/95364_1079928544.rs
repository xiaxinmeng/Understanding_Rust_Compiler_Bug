plain
To only update this specific test, also pass `--test-args impl-trait/impl_trait_projections.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl_trait_projections.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl_trait_projections" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl_trait_projections/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0667]: `impl Trait` is not allowed in path parameters
   |
   |
LL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {

error[E0667]: `impl Trait` is not allowed in path parameters
  --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:19:9
   |
   |
LL |     -> <impl Iterator as Iterator>::Item
   |         ^^^^^^^^^^^^^

error[E0667]: `impl Trait` is not allowed in path parameters
  --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:26:27
   |
LL |     -> <::std::ops::Range<impl Debug> as Iterator>::Item

error[E0667]: `impl Trait` is not allowed in path parameters
  --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:33:29
   |
   |
LL |     -> <dyn Iterator<Item = impl Debug> as Iterator>::Item

error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/impl-trait/impl_trait_projections.rs:12:50
   |
   |
LL | fn projection_is_disallowed(x: impl Iterator) -> <impl Iterator>::Item {
   |                                                  ^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<impl Iterator as Trait>::Item`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0223, E0667.
For more information about an error, try `rustc --explain E0223`.
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-57979-impl-trait-in-path.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-57979-impl-trait-in-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-57979-impl-trait-in-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-57979-impl-trait-in-path/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0667]: `impl Trait` is not allowed in path parameters
   |
   |
LL | pub fn demo(_: impl Quux<(), Assoc=<() as Quux<impl Bar>>::Assoc>) { }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0667`.
