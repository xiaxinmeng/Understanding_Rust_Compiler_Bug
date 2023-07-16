plain
........................................................................................ 11528/13221
........................................................................................ 11616/13221
......................................................................i........i........ 11704/13221
i.....i.....................i........................................................... 11792/13221
...............................................................F.............F.......... 11880/13221
..............................................................F......................... 11968/13221
........................F...................F.........................F................. 12056/13221
........................................F.....................F......................... 12144/13221
............................F........................................................... 12232/13221
.................................................i...................................... 12408/13221
........................................................................................ 12496/13221
........................................................................................ 12584/13221
........................................................................................ 12672/13221
---
---- [ui] src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-81809.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-81809.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-81809" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-81809/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait StoreIndex: Indexer<u8> + Indexer<u16> {}
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait StoreIndex: Indexer<u8> + Indexer<u16> {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-66768.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66768.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66768" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66768/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL |     Allocator<f64, GeometryDim> + Allocator<f64, NodalDim>
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL |     Allocator<f64, GeometryDim> + Allocator<f64, NodalDim>

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/alias/maybe-bound.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/maybe-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/maybe-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait _2 = _1 + _1;
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait _2 = _1 + _1;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/alias/syntax.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/syntax/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/syntax/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait WithWhere<Art, Thou> = Romeo + Romeo where Fore<(Art, Thou)>: Romeo;
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait WithWhere<Art, Thou> = Romeo + Romeo where Fore<(Art, Thou)>: Romeo;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/duplicate-trait-bounds.rs stdout ----
diff of stderr:

5    |                                                        ^^^^^^^^^^^^^
6    |
7    = note: `#[deny(dup_trait_bounds)]` on by default
- help: Remove this duplicate trait bound
+ help: remove the duplicate trait bound
10    |
10    |
11 LL |     DirectedGraph + WithStartNode + WithPredecessors + WithStartNode + WithSuccessors + WithNumNodes

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/duplicate-trait-bounds/duplicate-trait-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/duplicate-trait-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/duplicate-trait-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/duplicate-trait-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/duplicate-trait-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL |     DirectedGraph + WithStartNode + WithPredecessors + WithStartNode + WithSuccessors + WithNumNodes
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL |     DirectedGraph + WithStartNode + WithPredecessors + WithStartNode + WithSuccessors + WithNumNodes

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/inheritance/repeated-supertrait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inheritance/repeated-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inheritance/repeated-supertrait/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inheritance/repeated-supertrait/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait CompareToInts : CompareTo<i64> + CompareTo<u64> {
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait CompareToInts : CompareTo<i64> + CompareTo<u64> {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/issue-26339.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-26339.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-26339/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-26339/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait A: PartialEq<Foo> + PartialEq<Bar> { }
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait A: PartialEq<Foo> + PartialEq<Bar> { }

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/issue-70944.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-70944.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-70944" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-70944/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | pub trait Foo: Index<KeyA> + Index<KeyB> + Index<KeyC> {}
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | pub trait Foo: Index<KeyA> + Index<KeyB> + Index<KeyC> {}

error: duplicate trait bound
  --> /checkout/src/test/ui/traits/issue-70944.rs:10:44
   |
   |
LL | pub trait Foo: Index<KeyA> + Index<KeyB> + Index<KeyC> {}
   |
help: remove the duplicate trait bound
  --> /checkout/src/test/ui/traits/issue-70944.rs:10:44
   |
   |
LL | pub trait Foo: Index<KeyA> + Index<KeyB> + Index<KeyC> {}

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/traits/normalize-supertrait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/normalize-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/normalize-supertrait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/normalize-supertrait/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait Derived<B: Proj>: Base<B::S> + Base<()> {
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait Derived<B: Proj>: Base<B::S> + Base<()> {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/object/with-self-in-projection-output-repeated-supertrait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/with-self-in-projection-output-repeated-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-repeated-supertrait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-repeated-supertrait/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait NormalizingHelper: Base<Output=<Self as ConstI32>::Out> + Base<Output=i32> {
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait NormalizingHelper: Base<Output=<Self as ConstI32>::Out> + Base<Output=i32> {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/traits/trait-upcasting/correct-supertrait-substitution.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/correct-supertrait-substitution.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/correct-supertrait-substitution/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/correct-supertrait-substitution/auxiliary"
stdout: none
--- stderr -------------------------------
error: duplicate trait bound
   |
   |
LL | trait Foo<T: Default + ToString>: Bar<i32> + Bar<T> {}
   |
   |
   = note: `#[deny(dup_trait_bounds)]` on by default
help: remove the duplicate trait bound
   |
   |
LL | trait Foo<T: Default + ToString>: Bar<i32> + Bar<T> {}

error: aborting due to previous error
------------------------------------------

