plain

---- [ui] src/test/ui/nll/member-constraints/nested-impl-trait-fail.rs stdout ----
diff of stderr:

+ error[E0700]: hidden type for `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>
+    |                     -- hidden type `[&'s u8; 1]` captures the lifetime `'s` as defined here
+ LL |     [a]
+    |     ^^^
+    |
+    |
+ help: to declare that `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures `'s`, you can add an explicit `'s` lifetime bound
+    |
+ LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>> + 's
+    |                                                                                                ++++
+ help: to declare that `impl Cap<'a> + Cap<'b>` captures `'s`, you can add an explicit `'s` lifetime bound
+    |
+ LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b> + 's>
+ 
+ 
1 error[E0700]: hidden type for `impl Cap<'a> + Cap<'b>` captures lifetime that does not appear in bounds
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
3    |


52 LL | ) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b> + 's> {
54 
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
56 
---
To only update this specific test, also pass `--test-args nll/member-constraints/nested-impl-trait-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/member-constraints/nested-impl-trait-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/member-constraints/nested-impl-trait-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/member-constraints/nested-impl-trait-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures lifetime that does not appear in bounds
   |
   |
LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>
   |                     -- hidden type `[&'s u8; 1]` captures the lifetime `'s` as defined here
LL |     [a]
   |     ^^^
   |
   |
help: to declare that `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>> + 's
   |                                                                                                ++++
help: to declare that `impl Cap<'a> + Cap<'b>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b> + 's>


error[E0700]: hidden type for `impl Cap<'a> + Cap<'b>` captures lifetime that does not appear in bounds
   |
   |
LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>
   |                     -- hidden type `&'s u8` captures the lifetime `'s` as defined here
LL |     [a]
   |     ^^^
   |
   |
help: to declare that `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>> + 's
   |                                                                                                ++++
help: to declare that `impl Cap<'a> + Cap<'b>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | fn fail_early_bound<'s, 'a, 'b>(a: &'s u8) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b> + 's>


error[E0700]: hidden type for `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures lifetime that does not appear in bounds
   |
   |
LL | fn fail_late_bound<'s, 'a, 'b>(
   |                    -- hidden type `[&'s u8; 1]` captures the lifetime `'s` as defined here
LL |     [a]
   |     ^^^
   |
   |
help: to declare that `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | ) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>> + 's {
   |                                                       ++++
help: to declare that `impl Cap<'a> + Cap<'b>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | ) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b> + 's> {


error[E0700]: hidden type for `impl Cap<'a> + Cap<'b>` captures lifetime that does not appear in bounds
   |
   |
LL | fn fail_late_bound<'s, 'a, 'b>(
   |                    -- hidden type `&'s u8` captures the lifetime `'s` as defined here
LL |     [a]
   |     ^^^
   |
   |
help: to declare that `impl IntoIterator<Item = impl Cap<'a> + Cap<'b>>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | ) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b>> + 's {
   |                                                       ++++
help: to declare that `impl Cap<'a> + Cap<'b>` captures `'s`, you can add an explicit `'s` lifetime bound
   |
LL | ) -> impl IntoIterator<Item = impl Cap<'a> + Cap<'b> + 's> {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0700`.
