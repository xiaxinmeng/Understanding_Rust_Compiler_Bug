plain
failures:

---- [ui] tests/ui-fulldeps/issue-40001.rs stdout ----

error: auxiliary build of "/checkout/tests/ui-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui-fulldeps=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0053]: method `check_fn` has an incompatible type for trait
  --> fake-test-src-base/auxiliary/issue-40001-plugin.rs:43:13
   |
LL |         id: hir::HirId,
   |             |
   |             |
   |             expected struct `LocalDefId`, found struct `HirId`
   |
   |
   = note: expected signature `fn(&mut MissingAllowedAttrPass, &LateContext<'tcx>, rustc_hir::intravisit::FnKind<'_>, &'tcx rustc_hir::FnDecl<'tcx>, &'tcx rustc_hir::Body<'tcx>, Span, LocalDefId)`
              found signature `fn(&mut MissingAllowedAttrPass, &LateContext<'tcx>, rustc_hir::intravisit::FnKind<'_>, &'tcx rustc_hir::FnDecl<'_>, &'tcx rustc_hir::Body<'_>, Span, HirId)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
------------------------------------------
