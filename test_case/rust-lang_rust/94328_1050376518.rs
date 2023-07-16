plain
test [ui] ui/cycle-trait/cycle-trait-supertrait-indirect.rs ... ok
test [ui] ui/debuginfo/debuginfo-emit-llvm-ir-and-split-debuginfo.rs ... ok
test [ui] ui/deduplicate-diagnostics.rs#duplicate ... ok
test [ui] ui/deduplicate-diagnostics.rs#deduplicate ... ok
test [ui] ui/debuginfo/debuginfo_with_uninhabitable_field_and_unsized.rs ... ok
test [ui] ui/definition-reachable/private-types.rs ... ok
test [ui] ui/definition-reachable/private-non-types.rs ... ok
test [ui] ui/custom-test-frameworks-simple.rs ... ok
test [ui] ui/cross-crate/xcrate-associated-type-defaults.rs ... ok
---
test [ui (nll)] ui/cycle-trait/cycle-trait-default-type-trait.rs ... ok
test [ui (nll)] ui/cross-crate/issue-64872/issue-64872.rs ... ok
test [ui (nll)] ui/cycle-trait/cycle-trait-supertrait-direct.rs ... ok
test [ui (nll)] ui/cycle-trait/cycle-trait-supertrait-indirect.rs ... ok
test [ui (nll)] ui/debuginfo/debuginfo_with_uninhabitable_field_and_unsized.rs ... ok
test [ui (nll)] ui/deduplicate-diagnostics.rs#deduplicate ... ok
test [ui (nll)] ui/debuginfo/debuginfo-emit-llvm-ir-and-split-debuginfo.rs ... ok
test [ui (nll)] ui/deduplicate-diagnostics.rs#duplicate ... ok
test [ui (nll)] ui/custom-test-frameworks-simple.rs ... ok
---
1 error: lifetime may not live long enough
-   --> $DIR/issue-52742.rs:14:9
+   --> $DIR/issue-52742.rs:12:9
3    |
4 LL |     fn take_bar(&mut self, b: Bar<'_>) {
5    |                 ---------         -- let's call this `'1`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742.nll/issue-52742.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52742.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                 ---------         -- let's call this `'1`
   |                 has type `&mut Foo<'_, '2>`
   |                 has type `&mut Foo<'_, '2>`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
error: aborting due to previous error
------------------------------------------




failures:
    [ui (nll)] ui/nll/issue-52742.rs

test result: FAILED. 12486 passed; 1 failed; 165 ignored; 0 measured; 0 filtered out; finished in 99.31s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
