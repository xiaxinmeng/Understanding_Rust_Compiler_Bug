plain
..............................................i......................................... 4664/13618
........................................................................................ 4752/13618
........................................................................................ 4840/13618
........................................................................................ 4928/13618
..............................................................F...F..FFF....F........... 5016/13618
........................................................................................ 5192/13618
.........................................................................i.............. 5280/13618
.......................................................i................................ 5368/13618
........................................................................................ 5456/13618
---
---- [ui] src/test/ui/impl-trait/nested-return-type2-tait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-return-type2-tait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2-tait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2-tait/auxiliary"
stdout: none
--- stderr -------------------------------
error: opaque type `impl Trait<Assoc = Sendable>` does not satisfy its associated type bounds
   |
   |
LL |     type Assoc: Duh;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                 --- this associated type bound is unsatisfied for `Sendable`
...
LL | fn foo() -> impl Trait<Assoc = Sendable> {
   |
   = note: `#[deny(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | type Sendable = impl Send + Duh;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/impl-trait/nested-return-type2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-return-type2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type2/auxiliary"
stdout: none
--- stderr -------------------------------
error: opaque type `impl Trait<Assoc = impl Send>` does not satisfy its associated type bounds
   |
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `impl Send`
...
LL | fn foo() -> impl Trait<Assoc = impl Send> {
   |
   = note: `#[deny(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | fn foo() -> impl Trait<Assoc = impl Send + Duh> {

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/impl-trait/nested-return-type3-tait.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-return-type3-tait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3-tait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3-tait/auxiliary"
stdout: none
--- stderr -------------------------------
error: opaque type `impl Trait<Assoc = Sendable>` does not satisfy its associated type bounds
   |
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `Sendable`
...
LL | fn foo() -> impl Trait<Assoc = Sendable> {
   |
   = note: `#[deny(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | type Sendable = impl Send + Duh;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/impl-trait/nested-return-type3-tait3.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-return-type3-tait3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3-tait3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3-tait3/auxiliary"
stdout: none
--- stderr -------------------------------
error: opaque type `Traitable` does not satisfy its associated type bounds
   |
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `impl Send`
...
LL | type Traitable = impl Trait<Assoc = impl Send>;
   |
   = note: `#[deny(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | type Traitable = impl Trait<Assoc = impl Send + Duh>;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/impl-trait/nested-return-type3-tait2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-return-type3-tait2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3-tait2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3-tait2/auxiliary"
stdout: none
--- stderr -------------------------------
error: opaque type `Traitable` does not satisfy its associated type bounds
   |
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `Sendable`
...
LL | type Traitable = impl Trait<Assoc = Sendable>;
   |
   = note: `#[deny(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | type Sendable = impl Send + Duh;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/impl-trait/nested-return-type3.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/nested-return-type3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/nested-return-type3/auxiliary"
stdout: none
--- stderr -------------------------------
error: opaque type `impl Trait<Assoc = impl Send>` does not satisfy its associated type bounds
   |
   |
LL |     type Assoc: Duh;
   |                 --- this associated type bound is unsatisfied for `impl Send`
...
LL | fn foo() -> impl Trait<Assoc = impl Send> {
   |
   = note: `#[deny(opaque_hidden_inferred_bound)]` on by default
help: add this bound
   |
   |
LL | fn foo() -> impl Trait<Assoc = impl Send + Duh> {

error: aborting due to previous error
------------------------------------------

