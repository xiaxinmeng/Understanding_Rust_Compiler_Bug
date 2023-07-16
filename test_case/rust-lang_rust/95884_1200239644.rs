plain
 finished in 0.611 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 157 tests
...............................................F..........F.F........................... 88/157
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] src/test/incremental/hashes/inherent_impls.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
stdout: none
--- stderr -------------------------------
error: `associated_item(Foo::method_privacy)` should be dirty but is not
   |
   |
LL |     pub fn method_privacy() { }

error: aborting due to previous error
------------------------------------------



---- [incremental] src/test/incremental/hashes/trait_defs.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(TraitAddMethodAutoImplementation)` should be dirty but is not
   |
   |
LL | trait TraitAddMethodAutoImplementation {


error: `hir_owner(TraitAddDefaultToAssociatedType)` should be dirty but is not
   |
   |
LL | trait TraitAddDefaultToAssociatedType {


error: `hir_owner(TraitAddInitializerToAssociatedConstant)` should be dirty but is not
   |
   |
LL | trait TraitAddInitializerToAssociatedConstant {


error: `associated_item(TraitAddMethodAutoImplementation::method)` should be dirty but is not
   |
LL |     fn method() {}
   |     ^^^^^^^^^^^


error: `associated_item(TraitAddDefaultToAssociatedType::Associated)` should be dirty but is not
   |
LL |     type Associated = ReferenceType0;
   |     ^^^^^^^^^^^^^^^


error: `associated_item(TraitAddInitializerToAssociatedConstant::Value)` should be dirty but is not
   |
LL |     const Value: u32 = 1;
   |     ^^^^^^^^^^^^^^^^


error: aborting due to 6 previous errors
------------------------------------------


---- [incremental] src/test/incremental/hashes/trait_impls.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/trait_impls.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_impls/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(ChangeHasValueTrait)` should be dirty but is not
   |
   |
LL | pub trait ChangeHasValueTrait {


error: `hir_owner(<Foo as AddDefaultTrait>)` should be dirty but is not
   |
LL | impl AddDefaultTrait for Foo {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(<Foo as AddDefaultTrait>)` should be dirty but is not
   |
LL | impl AddDefaultTrait for Foo {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `associated_item(ChangeHasValueTrait::method_name)` should be dirty but is not
   |
LL |     fn method_name() { }
   |     ^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(<Foo as AddDefaultTrait>::method_name)` should be clean but is not
   |
LL |     default fn method_name() { }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner(<Foo as AddDefaultTrait>::method_name)` should be clean but is not
   |
LL |     default fn method_name() { }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^


error: `associated_item(<Foo as AddDefaultTrait>::method_name)` should be dirty but is not
   |
LL |     default fn method_name() { }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

