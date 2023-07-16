plain
diff of stderr:

2   --> $DIR/tls.rs:12:25
3    |
4 LL |     unsafe { let _val = A; }
-    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[78b0]::A))
+    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[78b067a8dcec65a]::A))
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
7 error[E0080]: could not evaluate static initializer
8   --> $DIR/tls.rs:19:26


9    |
10 LL |     unsafe { let _val = &A; }
-    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[78b0]::A))
+    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[78b067a8dcec65a]::A))
13 warning: skipping const checks
14    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls/tls.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/tls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/tls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL |     unsafe { let _val = A; }
   |                         ^ cannot access thread local static (DefId(0:6 ~ tls[78b067a8dcec65a]::A))
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }
   |                          ^ cannot access thread local static (DefId(0:6 ~ tls[78b067a8dcec65a]::A))
warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:12:25
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:12:25
   |
LL |     unsafe { let _val = A; }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
---
diff of stderr:

9   --> $DIR/generator-print-verbose-1.rs:35:9
10    |
11 LL |         let _non_send_gen = make_non_send_generator();
-    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[749a]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
+    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[749a2c5c77c8001]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
13 LL |         yield;
14    |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later

33    |
34 LL |     || {
35    |     ^^
35    |     ^^
- note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[749a]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
+ note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[749a2c5c77c8001]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
38    |
38    |
39 LL | pub fn make_gen2<T>(t: T) -> impl Generator<Return = T> {
40    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^
40    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[749a]::make_non_send_generator2::{opaque#0}), [])`
+ note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[749a2c5c77c8001]::make_non_send_generator2::{opaque#0}), [])`
43    |
43    |
44 LL | fn make_non_send_generator2() -> impl Generator<Return = Arc<RefCell<i32>>> {
45    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
45    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: required because it captures the following types: `Opaque(DefId(0:42 ~ generator_print_verbose_1[749a]::make_non_send_generator2::{opaque#0}), [])`, `()`
+    = note: required because it captures the following types: `Opaque(DefId(0:42 ~ generator_print_verbose_1[749a2c5c77c8001]::make_non_send_generator2::{opaque#0}), [])`, `()`
47 note: required because it's used within this generator
49    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1/generator-print-verbose-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/print/generator-print-verbose-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/print/generator-print-verbose-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: generator cannot be sent between threads safely
   |
LL |     require_send(send_gen);
LL |     require_send(send_gen);
   |     ^^^^^^^^^^^^ generator is not `Send`
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
note: generator is not `Send` as this value is used across a yield
   |
   |
LL |         let _non_send_gen = make_non_send_generator();
   |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[749a2c5c77c8001]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
LL |         yield;
   |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later
LL |     };
   |     - `_non_send_gen` is later dropped here
note: required by a bound in `require_send`
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`

error[E0277]: `RefCell<i32>` cannot be shared between threads safely
   |
LL |     require_send(send_gen);
LL |     require_send(send_gen);
   |     ^^^^^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
note: required because it's used within this generator
   |
LL |     || {
   |     ^^
   |     ^^
note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[749a2c5c77c8001]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
   |
   |
LL | pub fn make_gen2<T>(t: T) -> impl Generator<Return = T> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[749a2c5c77c8001]::make_non_send_generator2::{opaque#0}), [])`
   |
   |
LL | fn make_non_send_generator2() -> impl Generator<Return = Arc<RefCell<i32>>> {
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: required because it captures the following types: `Opaque(DefId(0:42 ~ generator_print_verbose_1[749a2c5c77c8001]::make_non_send_generator2::{opaque#0}), [])`, `()`
note: required because it's used within this generator
   |
   |
LL |     let send_gen = || {
note: required by a bound in `require_send`
  --> /checkout/src/test/ui/generator/print/generator-print-verbose-1.rs:26:25
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/ty-outlives/impl-trait-captures.rs stdout ----
diff of stderr:

2   --> $DIR/impl-trait-captures.rs:11:5
3    |
4 LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> {
-    |                  -- hidden type `&ReFree(DefId(0:8 ~ impl_trait_captures[1afc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc]::foo::'_), '_)) T` captures the anonymous lifetime defined here
+    |                  -- hidden type `&ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_)) T` captures the anonymous lifetime defined here
7    |     ^
8    |


- help: to declare that the `impl Trait` captures `ReFree(DefId(0:8 ~ impl_trait_captures[1afc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc]::foo::'_), '_))`, you can add an explicit `ReFree(DefId(0:8 ~ impl_trait_captures[1afc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc]::foo::'_), '_))` lifetime bound
+ help: to declare that the `impl Trait` captures `ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_))`, you can add an explicit `ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_))` lifetime bound
10    |
- LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> + ReFree(DefId(0:8 ~ impl_trait_captures[1afc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc]::foo::'_), '_)) {
-    |                                      ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+ LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> + ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_)) {
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args nll/ty-outlives/impl-trait-captures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/impl-trait-captures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/impl-trait-captures" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/impl-trait-captures/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> {
   |                  -- hidden type `&ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_)) T` captures the anonymous lifetime defined here
   |     ^
   |
   |
help: to declare that the `impl Trait` captures `ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_))`, you can add an explicit `ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_))` lifetime bound
   |
LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> + ReFree(DefId(0:8 ~ impl_trait_captures[1afc60e28dcb6dc]::foo), BrNamed(DefId(0:13 ~ impl_trait_captures[1afc60e28dcb6dc]::foo::'_), '_)) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
For more information about this error, try `rustc --explain E0700`.
------------------------------------------


---- [ui] src/test/ui/thir-tree.rs stdout ----
diff of stdout:

- DefId(0:3 ~ thir_tree[8f1d]::main):
+ DefId(0:3 ~ thir_tree[8f1d22a25377c3c]::main):
2 Thir {
3     arms: [],
4     exprs: [
30                 region_scope: Node(2),
31                 lint_level: Explicit(
32                     HirId {
32                     HirId {
-                         owner: DefId(0:3 ~ thir_tree[8f1d]::main),
+                         owner: DefId(0:3 ~ thir_tree[8f1d22a25377c3c]::main),
34                         local_id: 2,
36                 ),


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree/thir-tree.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args thir-tree.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thir-tree.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=thir-tree" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree/auxiliary"
--- stdout -------------------------------
DefId(0:3 ~ thir_tree[8f1d22a25377c3c]::main):
Thir {
    arms: [],
    exprs: [
        Expr {
            ty: (),
            temp_lifetime: Some(
            ),
            ),
            span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
            kind: Block {
                body: Block {
                    targeted_by_break: false,
                    region_scope: Node(1),
                    opt_destruction_scope: None,
                    span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
                    stmts: [],
                    expr: None,
                    safety_mode: Safe,
            },
        },
        Expr {
            ty: (),
            ty: (),
            temp_lifetime: Some(
                Node(2),
            ),
            span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
            kind: Scope {
                region_scope: Node(2),
                lint_level: Explicit(
                    HirId {
                        owner: DefId(0:3 ~ thir_tree[8f1d22a25377c3c]::main),
                        local_id: 2,
                ),
                value: e0,
            },
        },
        },
        Expr {
            ty: (),
            temp_lifetime: Some(
                Node(2),
            ),
            span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
            kind: Scope {
                region_scope: Destruction(2),
                lint_level: Inherited,
                value: e1,
        },
    ],
    stmts: [],
}
