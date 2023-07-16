plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b29f1f8d3b3f419522b0eb149b664e5325f7b306)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

---- [ui] tests/ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----
diff of stderr:

- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:13:16
+ error[E0004]: non-exhaustive patterns: `TyKind::Pat(_, _)` not covered
+   --> $DIR/ty_tykind_usage.rs:15:11
- LL |     let kind = TyKind::Bool;
- LL |     let kind = TyKind::Bool;
-    |                ^^^^^^ help: try using `ty::<kind>` directly: `ty`
+ LL |     match kind {
+    |           ^^^^ pattern `TyKind::Pat(_, _)` not covered
- note: the lint level is defined here
-   --> $DIR/ty_tykind_usage.rs:11:8
-   --> $DIR/ty_tykind_usage.rs:11:8
+ note: `TyKind<TyCtxt<'_>>` defined here
+   --> $COMPILER_DIR/rustc_type_ir/src/sty.rs:50:1
+   ::: $COMPILER_DIR/rustc_type_ir/src/sty.rs:233:5
9    |
- LL | #[deny(rustc::usage_of_ty_tykind)]
- 
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:16:9
+    = note: not covered
+    = note: the matched value is of type `TyKind<TyCtxt<'_>>`
+ help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
15    |
- LL |         TyKind::Bool => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:17:9
+ LL ~         TyKind::Error(_) => (),
+ LL ~         TyKind::Pat(_, _) => todo!(),
21    |
- LL |         TyKind::Char => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
24 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:18:9
-    |
- LL |         TyKind::Int(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
+ error: aborting due to previous error
30 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:19:9
-    |
- LL |         TyKind::Uint(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:20:9
-    |
- LL |         TyKind::Float(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:21:9
-    |
- LL |         TyKind::Adt(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:22:9
-    |
- LL |         TyKind::Foreign(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:23:9
-    |
- LL |         TyKind::Str => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:24:9
-    |
- LL |         TyKind::Array(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:25:9
-    |
- LL |         TyKind::Slice(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:26:9
-    |
- LL |         TyKind::RawPtr(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:27:9
-    |
- LL |         TyKind::Ref(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:28:9
-    |
- LL |         TyKind::FnDef(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:29:9
-    |
- LL |         TyKind::FnPtr(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:30:9
-    |
- LL |         TyKind::Dynamic(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:31:9
-    |
- LL |         TyKind::Closure(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:32:9
-    |
- LL |         TyKind::Generator(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:33:9
-    |
- LL |         TyKind::GeneratorWitness(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:34:9
-    |
- LL |         TyKind::GeneratorWitnessMIR(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:35:9
-    |
- LL |         TyKind::Never => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:36:9
-    |
- LL |         TyKind::Tuple(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:37:9
-    |
- LL |         TyKind::Alias(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:38:9
-    |
- LL |         TyKind::Param(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:39:9
-    |
- LL |         TyKind::Bound(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:40:9
-    |
- LL |         TyKind::Placeholder(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:41:9
-    |
- LL |         TyKind::Infer(..) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:42:9
-    |
- LL |         TyKind::Error(_) => (),
-    |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:47:12
-    |
- LL |     if let TyKind::Int(int_ty) = kind {}
-    |            ^^^^^^ help: try using `ty::<kind>` directly: `ty`
- error: usage of `ty::TyKind`
-   --> $DIR/ty_tykind_usage.rs:49:24
-    |
-    |
- LL |     fn ty_kind(ty_bad: TyKind<'_>, ty_good: Ty<'_>) {}
-    |
-    = help: try using `Ty` instead
- 
- error: usage of `ty::TyKind`
- error: usage of `ty::TyKind`
-   --> $DIR/ty_tykind_usage.rs:51:37
-    |
- LL |     fn ir_ty_kind<I: Interner>(bad: IrTyKind<I>) -> IrTyKind<I> {
-    |
-    = help: try using `Ty` instead
- 
- error: usage of `ty::TyKind`
- error: usage of `ty::TyKind`
-   --> $DIR/ty_tykind_usage.rs:51:53
-    |
- LL |     fn ir_ty_kind<I: Interner>(bad: IrTyKind<I>) -> IrTyKind<I> {
-    |
-    = help: try using `Ty` instead
- 
- 
- error: usage of `ty::TyKind::<kind>`
-   --> $DIR/ty_tykind_usage.rs:54:9
-    |
- LL |         IrTyKind::Bool
-    |         |
-    |         |
-    |         help: try using `ty::<kind>` directly: `ty`
- error: aborting due to 33 previous errors
- 
+ For more information about this error, try `rustc --explain E0004`.
215 
---
To only update this specific test, also pass `--test-args internal-lints/ty_tykind_usage.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary" "-Z" "unstable-options"
stdout: none
--- stderr -------------------------------
error[E0004]: non-exhaustive patterns: `TyKind::Pat(_, _)` not covered
  --> fake-test-src-base/internal-lints/ty_tykind_usage.rs:15:11
LL |     match kind {
LL |     match kind {
   |           ^^^^ pattern `TyKind::Pat(_, _)` not covered
   |
note: `TyKind<TyCtxt<'_>>` defined here
  --> /rustc/FAKE_PREFIX/compiler/rustc_type_ir/src/sty.rs:50:1
  ::: /rustc/FAKE_PREFIX/compiler/rustc_type_ir/src/sty.rs:233:5
   = note: not covered
   = note: not covered
   = note: the matched value is of type `TyKind<TyCtxt<'_>>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~         TyKind::Error(_) => (),
LL ~         TyKind::Pat(_, _) => todo!(),             //~ ERROR usage of `ty::TyKind::<kind>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0004`.
